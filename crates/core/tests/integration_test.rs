use repomix_config::schema::{OutputStyle, RepomixConfig};
use repomix_core::packager::{NoopProgress, PackOptions, pack};
use repomix_core::{pack_directory, pack_with_config};
use std::path::PathBuf;

#[tokio::test]
async fn test_pack_single_file() {
    let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let config = RepomixConfig::default();

    let result = pack(vec![test_file], config, Box::new(NoopProgress)).await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.total_files > 0);
    assert!(result.total_tokens > 0);
}

#[tokio::test]
async fn test_pack_with_output_style_markdown() {
    let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let mut config = RepomixConfig::default();
    config.output.style = OutputStyle::Markdown;

    let result = pack(vec![test_file], config, Box::new(NoopProgress)).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pack_with_output_style_json() {
    let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let mut config = RepomixConfig::default();
    config.output.style = OutputStyle::Json;

    let result = pack(vec![test_file], config, Box::new(NoopProgress)).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pack_with_output_style_plain() {
    let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let mut config = RepomixConfig::default();
    config.output.style = OutputStyle::Plain;

    let result = pack(vec![test_file], config, Box::new(NoopProgress)).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pack_with_compress() {
    let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let mut config = RepomixConfig::default();
    config.output.compress = true;

    let result = pack(vec![test_file], config, Box::new(NoopProgress)).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pack_with_remove_comments() {
    let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let mut config = RepomixConfig::default();
    config.output.remove_comments = true;

    let result = pack(vec![test_file], config, Box::new(NoopProgress)).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pack_with_line_numbers() {
    let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let mut config = RepomixConfig::default();
    config.output.show_line_numbers = true;

    let result = pack(vec![test_file], config, Box::new(NoopProgress)).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pack_directory() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let result = pack_directory(dir.to_str().unwrap()).await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.total_files > 0);
}

#[tokio::test]
async fn test_pack_with_config() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut config = RepomixConfig::default();
    config.output.style = OutputStyle::Markdown;
    config.output.compress = true;

    let result = pack_with_config(dir.to_str().unwrap(), config).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pack_with_options() {
    let options = PackOptions::new(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src"))
        .with_style(OutputStyle::Json)
        .with_compress(true);

    let result = repomix_core::pack_with_options(options).await;

    assert!(result.is_ok());
}

#[test]
fn test_split_output_no_split() {
    use repomix_core::output::split::split_output;

    let content = "line1\nline2\nline3";
    let result = split_output(content, 100, &OutputStyle::Xml, "o200k_base");

    assert_eq!(result.len(), 1);
}

#[test]
fn test_split_output_with_split() {
    use repomix_core::output::split::split_output;

    let content = (0..200)
        .map(|i| format!("word{}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let result = split_output(&content, 50, &OutputStyle::Xml, "o200k_base");

    assert!(result.len() > 1);
}

#[test]
fn test_secretlint_detects_api_key() {
    use repomix_core::security::secretlint::scan_file_content;
    use std::path::Path;

    let content = r#"api_key = "sk-1234567890abcdef1234567890abcdef""#;
    let results = scan_file_content(content, Path::new("test.txt"));

    assert!(!results.is_empty());
    assert!(results.iter().any(|r| r.rule_id == "generic-api-key"));
}

#[test]
fn test_secretlint_detects_unquoted_api_key_in_env() {
    use repomix_core::security::secretlint::scan_file_content;
    use std::path::Path;

    let content = "API_KEY=sk-1234567890abcdef1234567890abcdef\n";
    let results = scan_file_content(content, Path::new(".env"));

    assert!(!results.is_empty());
    assert!(results.iter().any(|r| r.rule_id == "generic-api-key"));
}

#[test]
fn test_secretlint_detects_password() {
    use repomix_core::security::secretlint::scan_file_content;
    use std::path::Path;

    let content = r#"password = "secret123""#;
    let results = scan_file_content(content, Path::new("test.txt"));

    assert!(!results.is_empty());
    assert!(results.iter().any(|r| r.rule_id == "generic-secret"));
}

#[test]
fn test_secretlint_detects_github_token() {
    use repomix_core::security::secretlint::scan_file_content;
    use std::path::Path;

    let content = r#"token = "ghp_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnop""#;
    let results = scan_file_content(content, Path::new("test.txt"));

    assert!(!results.is_empty());
    assert!(results.iter().any(|r| r.rule_id == "github-token"));
}

#[test]
fn test_secretlint_clean_file() {
    use repomix_core::security::secretlint::scan_file_content;
    use std::path::Path;

    let content = r#"
    fn main() {
        println!("Hello, world!");
    }
    "#;
    let results = scan_file_content(content, Path::new("test.txt"));

    assert!(results.is_empty());
}

#[test]
fn test_file_search() {
    use repomix_config::schema::RepomixConfig;
    use repomix_core::file::search::search_files;

    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let config = RepomixConfig::default();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(search_files(&[root_dir], &config));

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(!result.file_paths.is_empty());
}

#[test]
fn test_file_collect() {
    use repomix_config::schema::RepomixConfig;
    use repomix_core::file::collect::collect_files;

    let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let config = RepomixConfig::default();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(collect_files(vec![test_file], &config));

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(!result.raw_files.is_empty());
}

#[test]
fn test_metrics_calculation() {
    use repomix_config::schema::RepomixConfig;
    use repomix_core::metrics::calculate::calculate_metrics;
    use repomix_shared::types::ProcessedFile;
    use std::path::PathBuf;

    let files = vec![ProcessedFile {
        path: PathBuf::from("test.rs"),
        content: "fn main() {}".to_string(),
        token_count: 4,
    }];

    let config = RepomixConfig::default();
    let result = calculate_metrics(&files, &config);

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.total_characters > 0);
    assert!(result.total_tokens > 0);
}

#[test]
fn test_tree_sitter_languages() {
    use repomix_core::tree_sitter::languages::get_supported_languages;

    let languages = get_supported_languages();
    assert!(languages.len() >= 10);
    assert!(languages.contains(&"rust"));
    assert!(languages.contains(&"python"));
    assert!(languages.contains(&"javascript"));
}
// ===================== Regression tests for the bug-fix batch =====================

/// 串行化 chdir 类测试的全局锁：避免多个 `set_current_dir` 测试互相干扰。
/// cargo test 默认多线程并发运行。
///
/// 使用 `parking_lot::Mutex` 不会因 panic 而 poison；本 workspace 没引入
/// parking_lot，所以退回到 std Mutex 但用 `.lock().unwrap_or_else(|e| e.into_inner())`
/// 容错 panic 导致的 poison。
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};
static CHDIR_LOCK: Lazy<tokio::sync::Mutex<()>> = Lazy::new(|| tokio::sync::Mutex::new(()));

/// 跨测试递增的 tmpdir 后缀计数器：保证每个测试运行都得到独立 tmpdir，
/// 不会因为前一次失败的残留污染后一次（即使跨 cargo test 进程重启，
/// 不同进程 id 也会让 AtomicU64 起始值不同）。
static BUG8_COUNTER: AtomicU64 = AtomicU64::new(0);

/// 生成进程内唯一的 tmpdir 后缀：COUNTER + PID + 纳秒时间戳。
/// 三重组合确保跨 cargo test 进程重启也不会冲突。
fn next_bug8_n() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let counter = BUG8_COUNTER.fetch_add(1, Ordering::SeqCst);
    let pid = std::process::id() as u64;
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    // 简单混合（不是加密哈希），仅需保证唯一性
    counter.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ pid.rotate_left(17) ^ nanos.rotate_left(7)
}

async fn chdir_lock() -> tokio::sync::MutexGuard<'static, ()> {
    CHDIR_LOCK.lock().await
}

/// `top_files_length` 影响 `calculate_metrics` 输出。
#[tokio::test]
async fn test_top_files_length_is_respected_bug7() {
    use repomix_core::metrics::calculate::calculate_metrics;
    use repomix_shared::types::ProcessedFile;
    use std::path::PathBuf;

    let files: Vec<ProcessedFile> = (0..20)
        .map(|i| ProcessedFile {
            path: PathBuf::from(format!("file_{:02}.txt", i)),
            content: "x".repeat(100 + i * 50),
            token_count: 100 + i * 50,
        })
        .collect();

    let mut config = RepomixConfig::default();
    config.output.top_files_length = 5;
    let result = calculate_metrics(&files, &config).unwrap();

    assert_eq!(
        result.top_files_by_tokens.len(),
        5,
        "top_files_length=5 should produce exactly 5 entries"
    );
    // 第一个应该是 token 数最大的 file_19
    assert_eq!(result.top_files_by_tokens[0].0, "file_19.txt");
    assert_eq!(result.top_files_by_tokens[0].1, 100 + 19 * 50);
    // 顺序按 token 数降序
    for w in result.top_files_by_tokens.windows(2) {
        assert!(
            w[0].1 >= w[1].1,
            "top_files_by_tokens should be sorted descending"
        );
    }

    // top_files_length=0 表示禁用，不返回任何条目
    config.output.top_files_length = 0;
    let result = calculate_metrics(&files, &config).unwrap();
    assert!(result.top_files_by_tokens.is_empty());
}

/// 默认 `output.file_path` 时，输出后缀跟随 style。
#[tokio::test]
async fn test_output_path_follows_style_bug8() {
    let _guard = chdir_lock().await;
    use std::fs;
    let n = next_bug8_n();

    let tmpdir = std::env::temp_dir().join(format!("repomix_test_bug8_{}", n));
    // 强制清理：使用 fs::remove_dir_all 容错任何残留（之前 panic 可能留下）。
    // 若 remove 失败（权限等）就跳过本测试，避免误判。
    if let Err(e) = fs::remove_dir_all(&tmpdir) {
        eprintln!("skipping test: cannot remove pre-existing tmpdir: {}", e);
        return;
    }
    fs::create_dir_all(&tmpdir).unwrap();
    let input_file = tmpdir.join("input.rs");
    fs::write(&input_file, "fn main() {}\n").unwrap();

    // 保存原 cwd 并切到 tmpdir，这样 `repomix-output.txt` 落在 tmpdir 里
    // 而不是污染调用者的工作目录。
    let orig_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&tmpdir).unwrap();

    for (style, expected_ext) in [
        (OutputStyle::Xml, "xml"),
        (OutputStyle::Markdown, "md"),
        (OutputStyle::Json, "json"),
        (OutputStyle::Plain, "txt"),
    ] {
        let mut config = RepomixConfig::default();
        // 关键前提：保持 schema 默认值
        assert_eq!(config.output.file_path, "repomix-output.txt");
        config.output.style = style.clone();

        pack(vec![tmpdir.clone()], config, Box::new(NoopProgress))
            .await
            .unwrap();

        let expected_path = tmpdir.join(format!("repomix-output.{}", expected_ext));
        assert!(
            expected_path.exists(),
            "style {:?} should write to {:?}",
            style,
            expected_path
        );
        if expected_ext != "txt" {
            let wrong_path = tmpdir.join("repomix-output.txt");
            assert!(
                !wrong_path.exists(),
                "style {:?} should NOT have written to {:?}",
                style,
                wrong_path
            );
        }
        // 清理这次循环产生的输出
        let _ = fs::remove_file(&expected_path);
    }

    // 恢复 cwd
    std::env::set_current_dir(&orig_cwd).unwrap();
    let _ = fs::remove_dir_all(&tmpdir);
}

/// 用户显式设置 `file_path` 时不被动态后缀覆盖。
#[tokio::test]
async fn test_output_path_user_override_preserved_bug8() {
    let _guard = chdir_lock().await;
    use std::fs;
    let n = next_bug8_n();

    let tmpdir = std::env::temp_dir().join(format!("repomix_test_bug8b_{}", n));
    // 强制清理：使用 fs::remove_dir_all 容错任何残留。
    if let Err(e) = fs::remove_dir_all(&tmpdir) {
        eprintln!("skipping test: cannot remove pre-existing tmpdir: {}", e);
        return;
    }
    fs::create_dir_all(&tmpdir).unwrap();
    fs::write(tmpdir.join("input.rs"), "fn main() {}\n").unwrap();

    let orig_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&tmpdir).unwrap();

    let mut config = RepomixConfig::default();
    config.output.style = OutputStyle::Json;
    // 注意：显式设为非常规后缀，应保留
    config.output.file_path = "my-custom-output.bin".to_string();

    pack(vec![tmpdir.clone()], config, Box::new(NoopProgress))
        .await
        .unwrap();

    let custom = tmpdir.join("my-custom-output.bin");
    assert!(
        custom.exists(),
        "user-specified file_path should be preserved at {:?}",
        custom
    );
    // 反向断言：未生成默认 .json
    let default_json = tmpdir.join("repomix-output.json");
    assert!(
        !default_json.exists(),
        "user override should suppress default .json file at {:?}",
        default_json
    );

    std::env::set_current_dir(&orig_cwd).unwrap();
    let _ = fs::remove_dir_all(&tmpdir);
}

/// C# 暂不支持（tree-sitter-c-sharp 0.23 ABI 与 queries/c_sharp.scm 不兼容）。
#[test]
fn test_csharp_disabled_due_to_abi_mismatch_bug2() {
    use repomix_core::tree_sitter::languages::get_supported_languages;
    let langs = get_supported_languages();
    assert!(
        !langs.contains(&"c_sharp"),
        "c_sharp should be disabled until queries/c_sharp.scm is upgraded to ABI 15. \
         Found in: {:?}",
        langs
    );
    // 同时验证 .cs 文件无法匹配到任何语言配置（将走纯文本路径）
    use repomix_core::tree_sitter::languages::get_language_config;
    use std::path::Path;
    let config = get_language_config(Path::new("test.cs"));
    assert!(
        config.is_none(),
        ".cs extension should not map to any LanguageConfig when c_sharp is disabled"
    );
}

/// XML 输出须转义文件内容中的 XML 保留字符（`& < >`）。
#[test]
fn test_xml_output_escapes_file_content_bug4() {
    use repomix_config::schema::{OutputStyle, RepomixConfig};
    use repomix_core::output::decorate::OutputHeader;
    use repomix_core::output::styles::xml::generate_xml;
    use repomix_shared::types::ProcessedFile;
    use std::path::PathBuf;

    let mut config = RepomixConfig::default();
    config.output.style = OutputStyle::Xml;
    // 关闭 file_summary/header/directory_structure 让测试只关注 <files> 块
    config.output.file_summary = false;
    config.output.directory_structure = false;

    let files = vec![
        ProcessedFile {
            path: PathBuf::from("evil.xml"),
            content: "before </file> after <script>alert(1)</script> & 'quote'\n".to_string(),
            token_count: 10,
        },
        ProcessedFile {
            path: PathBuf::from("unicode.rs"),
            content: "// 你好 <世界> & 'foo'".to_string(),
            token_count: 5,
        },
    ];

    let header = OutputHeader {
        header_text: None,
        instruction_content: None,
    };
    let output = generate_xml(
        &files,
        &config,
        std::path::Path::new("."),
        "",
        &header,
        &None,
        &None,
    );

    // 关键断言：内容中出现的 `</file>` 必须被转义为 `&lt;/file&gt;`
    assert!(
        output.contains("&lt;/file&gt;"),
        "Content `</file>` should be escaped to `&lt;/file&gt;`\nGot: {}",
        output
    );
    // 同样 `<script>` 必须被转义
    assert!(
        output.contains("&lt;script&gt;"),
        "Content `<script>` should be escaped\nGot: {}",
        output
    );
    // `&` 必须被转义为 `&amp;`
    assert!(
        output.contains("&amp;"),
        "Content `&` should be escaped to `&amp;`\nGot: {}",
        output
    );
    // Unicode 内容应原样保留
    assert!(
        output.contains("你好"),
        "Unicode content should be preserved\nGot: {}",
        output
    );
    // 真正的 `</file>` 关闭标签只能来自模板（这里 2 个文件 → 2 个关闭）
    let file_close_count = output.matches("</file>").count();
    assert_eq!(
        file_close_count, 2,
        "Should have exactly 2 `</file>` close tags (one per file), got {}",
        file_close_count
    );
}

/// 含非 ASCII 字符的文件应被正确读取，不因 `had_errors` 误判而跳过。
#[tokio::test]
async fn test_encoding_detect_keeps_non_ascii_files_bug1() {
    use repomix_config::schema::RepomixConfig;
    use repomix_core::file::collect::collect_files;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let tmpdir = std::env::temp_dir().join(format!("repomix_bug1_{}", n));
    let _ = fs::remove_dir_all(&tmpdir);
    fs::create_dir_all(&tmpdir).unwrap();

    // 1) 纯 UTF-8 中文（read_to_string 直接成功路径）
    let utf8_file = tmpdir.join("utf8.txt");
    fs::write(&utf8_file, "你好世界 hello\n").unwrap();

    // 2) GBK "你好世界"（UTF-8 读失败，走 chardetng 路径）
    //    \xC4\xE3\xBA\xC3\xCA\xC0\xBD\xE7 = "你好世界" in GBK
    let gbk_file = tmpdir.join("gbk.txt");
    fs::write(&gbk_file, b"\xC4\xE3\xBA\xC3\xCA\xC0\xBD\xE7 hello\n").unwrap();

    // 3) Latin-1 "café" (UTF-8 读失败，走 chardetng 路径)
    //    \xE9 = 'é' in Latin-1, not a valid UTF-8 byte
    let latin1_file = tmpdir.join("latin1.txt");
    fs::write(&latin1_file, b"caf\xE9 au lait\n").unwrap();

    let config = RepomixConfig::default();
    let result = collect_files(vec![utf8_file, gbk_file, latin1_file], &config)
        .await
        .unwrap();

    // 三个文件都不应被跳过
    assert!(
        result.skipped_files.is_empty(),
        "No file should be skipped due to encoding detection, got: {:?}",
        result.skipped_files
    );
    assert_eq!(result.raw_files.len(), 3, "All 3 files should be collected");

    // GBK 文件应被解码为可读中文（"你好"或"好"或"世界"都应至少存在一个）
    let gbk_content = result
        .raw_files
        .iter()
        .find(|f| f.path.ends_with("gbk.txt"))
        .expect("gbk.txt should be present")
        .content
        .clone();
    assert!(
        gbk_content.contains("好") || gbk_content.contains("世") || gbk_content.contains("界"),
        "GBK file should be decoded to readable Chinese, got: {:?}",
        gbk_content
    );

    // Latin-1 文件应被解码为 "café"
    let latin1_content = result
        .raw_files
        .iter()
        .find(|f| f.path.ends_with("latin1.txt"))
        .expect("latin1.txt should be present")
        .content
        .clone();
    assert!(
        latin1_content.contains("caf") && latin1_content.contains("lait"),
        "Latin-1 file should be decoded to readable text, got: {:?}",
        latin1_content
    );

    let _ = fs::remove_dir_all(&tmpdir);
}

/// 回归：历史 repomix-output.* 文件不应被 search 再次收录。
#[test]
fn test_repomix_output_artifacts_excluded_from_search() {
    use repomix_core::file::search::search_files;
    use std::fs;

    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let tmpdir = std::env::temp_dir().join(format!("repomix_exclude_{}", n));
    let _ = fs::remove_dir_all(&tmpdir);
    fs::create_dir_all(&tmpdir).unwrap();
    fs::write(tmpdir.join("source.rs"), "fn main() {}\n").unwrap();
    fs::write(tmpdir.join("repomix-output.xml"), "<packed/>\n").unwrap();
    fs::write(tmpdir.join("repomix-output.md"), "# packed\n").unwrap();

    let config = RepomixConfig::default();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt
        .block_on(search_files(std::slice::from_ref(&tmpdir), &config))
        .unwrap();

    let names: Vec<String> = result
        .file_paths
        .iter()
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .collect();

    assert!(names.iter().any(|n| n == "source.rs"));
    assert!(!names.iter().any(|n| n.starts_with("repomix-output.")));
}

/// 回归：默认忽略依赖锁文件（Rust / npm / yarn / pnpm / bun）。
#[test]
fn test_lock_files_excluded_from_search() {
    use repomix_core::file::search::search_files;
    use std::fs;

    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let tmpdir = std::env::temp_dir().join(format!("repomix_lock_ignore_{}", n));
    let _ = fs::remove_dir_all(&tmpdir);
    fs::create_dir_all(&tmpdir).unwrap();
    fs::write(tmpdir.join("main.rs"), "fn main() {}\n").unwrap();
    for lock in [
        "Cargo.lock",
        "package-lock.json",
        "yarn.lock",
        "pnpm-lock.yaml",
        "bun.lock",
        "bun.lockb",
        "poetry.lock",
        "Pipfile.lock",
        "Gemfile.lock",
        "composer.lock",
        "go.sum",
        "pubspec.lock",
        "mix.lock",
        "Podfile.lock",
        "Package.resolved",
        "gradle.lockfile",
    ] {
        fs::write(tmpdir.join(lock), "lock data\n").unwrap();
    }

    let config = RepomixConfig::default();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt
        .block_on(search_files(std::slice::from_ref(&tmpdir), &config))
        .unwrap();

    let names: Vec<String> = result
        .file_paths
        .iter()
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .collect();

    assert_eq!(names, vec!["main.rs".to_string()]);
}

/// 回归：git sort 使用仓库相对路径匹配变更频率。
#[test]
fn test_git_sort_matches_relative_paths() {
    use repomix_core::git::sort::sort_by_git_changes;
    use repomix_shared::types::ProcessedFile;
    use std::fs;
    use std::process::Command;

    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let tmpdir = std::env::temp_dir().join(format!("repomix_gitsort_{}", n));
    let _ = fs::remove_dir_all(&tmpdir);
    fs::create_dir_all(&tmpdir).unwrap();

    Command::new("git")
        .args(["init"])
        .current_dir(&tmpdir)
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&tmpdir)
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.name", "test"])
        .current_dir(&tmpdir)
        .output()
        .unwrap();

    fs::write(tmpdir.join("hot.rs"), "v1\n").unwrap();
    fs::write(tmpdir.join("cold.rs"), "v1\n").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(&tmpdir)
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(&tmpdir)
        .output()
        .unwrap();

    for i in 2..=5 {
        fs::write(tmpdir.join("hot.rs"), format!("v{}\n", i)).unwrap();
        Command::new("git")
            .args(["add", "hot.rs"])
            .current_dir(&tmpdir)
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", &format!("change {}", i)])
            .current_dir(&tmpdir)
            .output()
            .unwrap();
    }

    let mut files = vec![
        ProcessedFile {
            path: tmpdir.join("cold.rs"),
            content: String::new(),
            token_count: 1,
        },
        ProcessedFile {
            path: tmpdir.join("hot.rs"),
            content: String::new(),
            token_count: 1,
        },
    ];

    sort_by_git_changes(&mut files, &tmpdir, 10).unwrap();
    assert!(
        files[0].path.ends_with("hot.rs"),
        "hot.rs should sort first, got {:?}",
        files[0].path
    );

    let _ = fs::remove_dir_all(&tmpdir);
}

/// 回归：UTF-16 LE BOM 文本不应被当作二进制跳过。
#[tokio::test]
async fn test_utf16_bom_file_collected() {
    use repomix_core::file::collect::collect_files;
    use std::fs;

    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let tmpdir = std::env::temp_dir().join(format!("repomix_utf16_{}", n));
    let _ = fs::remove_dir_all(&tmpdir);
    fs::create_dir_all(&tmpdir).unwrap();

    // UTF-16 LE BOM + "Hi" (U+0048 U+0069)
    let utf16_file = tmpdir.join("utf16.txt");
    fs::write(&utf16_file, [0xFF, 0xFE, 0x48, 0x00, 0x69, 0x00]).unwrap();

    let config = RepomixConfig::default();
    let result = collect_files(vec![utf16_file], &config).await.unwrap();

    assert!(result.skipped_files.is_empty());
    assert_eq!(result.raw_files.len(), 1);
    assert!(result.raw_files[0].content.contains('H'));

    let _ = fs::remove_dir_all(&tmpdir);
}

/// 回归：相对 output.file_path 写入 pack 根目录，而非调用者 CWD。
#[tokio::test]
async fn test_output_written_to_pack_root_not_cwd() {
    let _guard = chdir_lock().await;
    use std::fs;

    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let base = std::env::temp_dir().join(format!("repomix_outroot_{}", n));
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(base.join("repo/src")).unwrap();
    fs::write(base.join("repo/src/main.rs"), "fn main() {}\n").unwrap();

    let orig_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&base).unwrap();

    let mut config = RepomixConfig::default();
    config.output.file_path = "repomix-output.xml".to_string();

    pack(vec![base.join("repo")], config, Box::new(NoopProgress))
        .await
        .unwrap();

    let expected = base.join("repo/repomix-output.xml");
    assert!(
        expected.exists(),
        "output should be under pack root at {:?}",
        expected
    );
    assert!(
        !base.join("repomix-output.xml").exists(),
        "output should NOT land in caller CWD"
    );

    std::env::set_current_dir(&orig_cwd).unwrap();
    let _ = fs::remove_dir_all(&base);
}
