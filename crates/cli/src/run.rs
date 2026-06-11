use crate::prompts;
use crate::report::print_report;
use crate::spinner::Spinner;
use anyhow::Result;
use repomix_config::schema::RepomixConfig;
use repomix_core::packager::pack;

pub async fn init_config() -> Result<()> {
    let root_dir = std::env::current_dir()?;
    prompts::create_config_file(&root_dir);
    prompts::create_ignore_file(&root_dir);
    Ok(())
}

pub async fn run_mcp_server() -> Result<()> {
    // 真正以 MCP 协议通过 stdio 与客户端通信
    repomix_mcp::server::run_stdio_server().await
}

pub async fn run_pack(cli: crate::Cli) -> Result<()> {
    let remote_url = cli.remote.clone();
    let root_path = cli.root.clone();

    // 远程仓库使用唯一临时目录，退出时由 RAII guard 清理。
    let (root_dir, _temp_dir_guard) = if let Some(remote_url) = &remote_url {
        let temp_dir = make_unique_temp_dir("repomix_remote")
            .map_err(|e| anyhow::anyhow!("无法创建临时目录: {}", e))?;
        let guard = TempDirGuard::new(temp_dir.clone());

        println!("Cloning remote repository: {}", remote_url);
        repomix_core::git::remote::clone_remote_repo(remote_url, &temp_dir)
            .map_err(|e| anyhow::anyhow!("克隆远程仓库失败: {}", e))?;

        (temp_dir, Some(guard))
    } else {
        // 本地目录无需持有
        let dir = root_path.unwrap_or_else(|| std::env::current_dir().unwrap());
        (dir, None)
    };

    let config = build_config(&cli, &std::env::current_dir()?)?;

    let spinner = Spinner::new("Packing repository...");

    let result = pack(vec![root_dir], config, Box::new(spinner)).await?;

    print_report(&result);

    // guard 在此 drop，自动清理临时目录
    drop(_temp_dir_guard);

    Ok(())
}

/// 临时目录 RAII 守卫：drop 时删除目录（best-effort）。
///
/// 复制自 `repomix_mcp::server::TempDirGuard`，避免 CLI 引入
/// 对 mcp crate 的反向依赖。功能等价。
struct TempDirGuard {
    path: Option<std::path::PathBuf>,
}

impl TempDirGuard {
    fn new(path: std::path::PathBuf) -> Self {
        Self { path: Some(path) }
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        if let Some(path) = self.path.take()
            && let Err(e) = std::fs::remove_dir_all(&path)
        {
            tracing::warn!("Failed to clean up temp dir '{}': {}.", path.display(), e);
        }
    }
}

fn build_config(cli: &crate::Cli, config_root: &std::path::Path) -> Result<RepomixConfig> {
    // 配置根目录使用用户当前工作目录，而非 pack 根目录（`--remote` 时为临时克隆目录）。
    // 默认值 → 全局配置 → 项目配置 → CLI 参数
    let partial = repomix_config::load::PartialConfig {
        include: cli
            .include
            .as_ref()
            .map(|s| s.split(',').map(|p| p.trim().to_string()).collect()),
        ignore: cli
            .ignore
            .as_ref()
            .map(|s| s.split(',').map(|p| p.trim().to_string()).collect()),
        style: Some(cli.style.into()),
        compress: if cli.compress { Some(true) } else { None },
        remove_comments: if cli.remove_comments {
            Some(true)
        } else {
            None
        },
        remove_empty_lines: if cli.remove_empty_lines {
            Some(true)
        } else {
            None
        },
        show_line_numbers: if cli.line_numbers { Some(true) } else { None },
        truncate_base64: if cli.truncate_base64 {
            Some(true)
        } else {
            None
        },
        copy_to_clipboard: if cli.copy { Some(true) } else { None },
        output: cli.output.clone(),
        include_empty_directories: if cli.include_empty_directories {
            Some(true)
        } else {
            None
        },
        top_files_length: cli.top_files_length,
        split_output: cli.split_output,
        header_text: cli.header_text.clone(),
        instruction_file_path: cli.instruction_file.clone(),
        include_diffs: if cli.include_diffs { Some(true) } else { None },
        include_logs: if cli.include_logs { Some(true) } else { None },
    };

    RepomixConfig::load(Some(partial), config_root)
}

/// 创建唯一临时目录：PID + 纳秒时间戳 + 哈希随机后缀
fn make_unique_temp_dir(prefix: &str) -> std::io::Result<std::path::PathBuf> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;

    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let mut h = DefaultHasher::new();
    SystemTime::now().hash(&mut h);
    let rand = h.finish();
    let dir = std::env::temp_dir().join(format!(
        "{}_{}_{:x}_{:x}",
        prefix,
        std::process::id(),
        nanos,
        rand
    ));
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}
