use crate::file::types::FileSearchOptions;
use crate::path_util::is_repomix_output_artifact;
use anyhow::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::WalkBuilder;
use repomix_config::schema::RepomixConfig;
use repomix_shared::types::FileSearchResult;
use std::path::PathBuf;
use std::sync::LazyLock;

/// 预编译的默认忽略模式匹配器（从 config/default_ignore.rs 的同一列表生成）
static DEFAULT_IGNORE_SET: LazyLock<GlobSet> = LazyLock::new(|| {
    let patterns = repomix_config::default_ignore::default_ignore_patterns();
    let mut builder = GlobSetBuilder::new();
    for pattern in &patterns {
        if let Ok(glob) = Glob::new(&format!("**/{}", pattern)) {
            builder.add(glob);
        }
    }
    builder.build().unwrap_or_else(|e| {
        tracing::warn!(
            "Failed to compile default ignore patterns: {}. \
             Default ignore rules are disabled.",
            e
        );
        GlobSetBuilder::new()
            .build()
            .expect("empty GlobSet must build")
    })
});

/// 编译用户自定义的 glob 模式
fn compile_user_patterns(patterns: &[String]) -> GlobSet {
    let mut builder = GlobSetBuilder::new();
    let mut invalid_count = 0usize;
    for pattern in patterns {
        match Glob::new(pattern) {
            Ok(glob) => {
                builder.add(glob);
            }
            Err(e) => {
                invalid_count += 1;
                tracing::warn!(
                    "Invalid glob pattern '{}': {}. This rule will be skipped.",
                    pattern,
                    e
                );
            }
        }
    }
    if invalid_count > 0 {
        tracing::warn!("{} invalid glob pattern(s) were skipped.", invalid_count);
    }
    builder.build().unwrap_or_default()
}

/// 搜索文件（异步入口：在 blocking 线程池执行目录遍历，避免阻塞 tokio runtime）
pub async fn search_files(
    root_dirs: &[PathBuf],
    config: &RepomixConfig,
) -> Result<FileSearchResult> {
    let root_dirs = root_dirs.to_vec();
    let config = config.clone();
    tokio::task::spawn_blocking(move || search_files_sync(&root_dirs, &config))
        .await
        .map_err(|e| anyhow::anyhow!("search_files task join failed: {}", e))?
}

/// 同步搜索实现
pub fn search_files_sync(
    root_dirs: &[PathBuf],
    config: &RepomixConfig,
) -> Result<FileSearchResult> {
    let options = FileSearchOptions::from_config(config);
    let mut file_paths = Vec::new();
    let mut empty_dir_paths = Vec::new();

    let include_matcher = compile_user_patterns(&options.include_patterns);
    let ignore_matcher = compile_user_patterns(&options.ignore_patterns);

    let configured_output = PathBuf::from(&config.output.file_path);
    let output_file_paths: Vec<PathBuf> = if configured_output.is_absolute() {
        vec![configured_output]
    } else {
        root_dirs
            .iter()
            .map(|root| root.join(&configured_output))
            .collect()
    };

    for root_dir in root_dirs {
        let walker = WalkBuilder::new(root_dir)
            .hidden(true)
            .git_ignore(config.ignore.use_gitignore)
            .git_global(config.ignore.use_gitignore)
            .git_exclude(config.ignore.use_gitignore)
            .threads(num_cpus::get())
            .build();

        for entry in walker {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let path = entry.path().to_path_buf();

            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                if options.include_empty_directories
                    && let Ok(mut entries) = std::fs::read_dir(&path)
                    && entries.next().is_none()
                {
                    empty_dir_paths.push(path);
                }
                continue;
            }

            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if output_file_paths.contains(&path)
                || is_repomix_output_artifact(file_name, &config.output.file_path)
            {
                continue;
            }

            let relative_path = path.strip_prefix(root_dir).unwrap_or(&path);

            if !options.include_patterns.is_empty() && !include_matcher.is_match(relative_path) {
                continue;
            }

            if !options.ignore_patterns.is_empty() && ignore_matcher.is_match(relative_path) {
                continue;
            }

            if DEFAULT_IGNORE_SET.is_match(relative_path) {
                continue;
            }

            file_paths.push(path);
        }
    }

    file_paths.sort();
    file_paths.dedup();
    empty_dir_paths.sort();
    empty_dir_paths.dedup();

    Ok(FileSearchResult {
        file_paths,
        empty_dir_paths,
    })
}
