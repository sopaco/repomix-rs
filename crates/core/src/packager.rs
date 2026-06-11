use anyhow::Result;
use repomix_config::schema::RepomixConfig;
use repomix_shared::types::*;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::file;
use crate::git;
use crate::metrics;
use crate::output;
use crate::path_util::display_path;
use crate::path_util::{effective_pack_root, resolve_output_file_path};
use crate::security;

// Re-export config types for convenience
pub use repomix_config::schema::OutputStyle;

#[derive(Debug, Clone)]
pub struct PackResult {
    pub total_files: usize,
    pub total_characters: usize,
    pub total_tokens: usize,
    pub file_char_counts: HashMap<String, usize>,
    pub file_token_counts: HashMap<String, usize>,
    /// 按 token 数降序的前 N 个文件（N = `RepomixConfig::output.top_files_length`）。
    pub top_files_by_tokens: Vec<(String, usize)>,
    pub git_diff_content: Option<String>,
    pub git_diff_token_count: usize,
    pub git_log_content: Option<String>,
    pub git_log_token_count: usize,
    /// 实际写入磁盘的路径列表（分片模式下有多条）
    pub output_paths: Vec<String>,
    /// 输出内容（与 `output_paths` 一一对应）
    pub output_contents: Vec<String>,
    /// 目录树文本
    pub directory_structure: String,
    pub suspicious_files: Vec<SuspiciousFileResult>,
    pub processed_files: Vec<ProcessedFile>,
    pub safe_file_paths: Vec<PathBuf>,
    pub skipped_files: Vec<SkippedFileInfo>,
}

/// 打包选项
#[derive(Debug, Clone, Default)]
pub struct PackOptions {
    pub root_dirs: Vec<PathBuf>,
    pub config: RepomixConfig,
}

impl PackOptions {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            root_dirs: vec![root_dir],
            config: RepomixConfig::default(),
        }
    }

    pub fn with_config(mut self, config: RepomixConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_style(mut self, style: OutputStyle) -> Self {
        self.config.output.style = style;
        self
    }

    pub fn with_compress(mut self, compress: bool) -> Self {
        self.config.output.compress = compress;
        self
    }

    pub fn with_remove_comments(mut self, remove: bool) -> Self {
        self.config.output.remove_comments = remove;
        self
    }

    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.config.output.show_line_numbers = show;
        self
    }

    pub fn with_include_patterns(mut self, patterns: Vec<String>) -> Self {
        self.config.include.extend(patterns);
        self
    }

    pub fn with_ignore_patterns(mut self, patterns: Vec<String>) -> Self {
        self.config.ignore.custom_ignore.extend(patterns);
        self
    }
}

pub trait ProgressCallback: Send + Sync {
    fn on_progress(&self, message: &str);
    fn on_complete(&self, message: &str);
    fn on_error(&self, message: &str);
}

pub struct NoopProgress;

impl ProgressCallback for NoopProgress {
    fn on_progress(&self, _message: &str) {}
    fn on_complete(&self, _message: &str) {}
    fn on_error(&self, _message: &str) {}
}

pub async fn pack(
    root_dirs: Vec<PathBuf>,
    mut config: RepomixConfig,
    progress: Box<dyn ProgressCallback>,
) -> Result<PackResult> {
    progress.on_progress("Starting pack...");

    // 默认 file_path 时根据 style 动态调整后缀；用户显式设置的路径不受影响。
    if config.output.file_path == "repomix-output.txt" {
        let ext = match config.output.style {
            OutputStyle::Xml => "xml",
            OutputStyle::Markdown => "md",
            OutputStyle::Json => "json",
            OutputStyle::Plain => "txt",
        };
        config.output.file_path = format!("repomix-output.{}", ext);
    }

    // 相对 output.file_path 解析到 pack 根目录，便于 search 排除与磁盘写入一致
    if let Some(pack_root) = root_dirs.first() {
        config.output.file_path = resolve_output_file_path(&config.output.file_path, pack_root);
    }

    progress.on_progress("Searching files...");
    let search_result = file::search::search_files(&root_dirs, &config).await?;

    // 提取空目录路径（用于 include_full_directory_structure）
    let empty_dir_strs: Vec<String> = search_result
        .empty_dir_paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    progress.on_progress("Collecting file contents...");
    let collect_result = file::collect::collect_files(search_result.file_paths, &config).await?;

    progress.on_progress("Running security checks...");
    let raw_files_for_validate = collect_result.raw_files.clone();
    let config_for_validate = config.clone();
    let validation = tokio::task::spawn_blocking(move || {
        security::validate::validate_file_safety(&raw_files_for_validate, &config_for_validate)
    })
    .await
    .map_err(|e| anyhow::anyhow!("validate_file_safety task join failed: {}", e))??;

    progress.on_progress("Processing file contents...");
    let raw_files = collect_result.raw_files;
    let config_for_process = config.clone();
    let mut processed = tokio::task::spawn_blocking(move || {
        file::process::process_files(&raw_files, &config_for_process)
    })
    .await
    .map_err(|e| anyhow::anyhow!("process_files task join failed: {}", e))??;

    // Sort by git changes if enabled
    if config.output.git.sort_by_changes {
        progress.on_progress("Sorting by git changes...");
        if let Some(root_dir) = root_dirs.first()
            && git::remote::is_git_repo(root_dir)
            && let Err(e) = git::sort::sort_by_git_changes(
                &mut processed,
                root_dir,
                config.output.git.sort_by_changes_max_commits,
            )
        {
            // 用 tracing::warn 替代 eprintln，统一日志通道
            tracing::warn!("Failed to sort by git changes: {}", e);
        }
    }

    let filtered = filter_suspicious(processed, &validation);

    // Git diff 和 log（在生成输出之前获取，以便写入输出文件）
    let mut git_diff_content = None;
    let mut git_diff_token_count = 0usize;
    let mut git_log_content = None;
    let mut git_log_token_count = 0usize;
    if let Some(root_dir) = root_dirs.first()
        && git::remote::is_git_repo(root_dir)
    {
        if config.output.git.include_diffs {
            match git::diff::get_git_diffs(root_dir) {
                Ok(diff_result) => {
                    let diff_content = format!("{}\n{}", diff_result.work_tree, diff_result.staged);
                    git_diff_token_count =
                        metrics::token_count::TokenCounter::new(&config.token_count.encoding)
                            .map(|c| c.count_tokens(&diff_content))
                            .unwrap_or_else(|_| {
                                metrics::token_count::estimate_tokens_fallback(&diff_content)
                            });
                    git_diff_content = Some(diff_content);
                }
                Err(e) => {
                    tracing::warn!("Failed to get git diffs: {}", e);
                }
            }
        }
        if config.output.git.include_logs {
            match git::log::get_git_logs(root_dir, config.output.git.include_logs_count) {
                Ok(log_result) => {
                    let log_content = log_result.logs.join("\n");
                    git_log_token_count =
                        metrics::token_count::TokenCounter::new(&config.token_count.encoding)
                            .map(|c| c.count_tokens(&log_content))
                            .unwrap_or_else(|_| {
                                metrics::token_count::estimate_tokens_fallback(&log_content)
                            });
                    git_log_content = Some(log_content);
                }
                Err(e) => {
                    tracing::warn!("Failed to get git logs: {}", e);
                }
            }
        }
    }

    progress.on_progress("Generating output...");
    let pack_root = root_dirs
        .first()
        .map(|p| effective_pack_root(p))
        .unwrap_or_else(|| PathBuf::from("."));
    let output = output::generate::produce_output(
        &filtered,
        &config,
        &pack_root,
        &git_diff_content,
        &git_log_content,
        &empty_dir_strs,
    )?;

    progress.on_progress("Calculating metrics...");
    let metrics = metrics::calculate::calculate_metrics(&filtered, &config)?;

    progress.on_complete("Pack complete");

    let relativize_map = |map: HashMap<String, usize>| {
        map.into_iter()
            .map(|(path, count)| (display_path(std::path::Path::new(&path), &pack_root), count))
            .collect()
    };
    let top_files_by_tokens: Vec<(String, usize)> = metrics
        .top_files_by_tokens
        .into_iter()
        .map(|(path, count)| (display_path(std::path::Path::new(&path), &pack_root), count))
        .collect();

    Ok(PackResult {
        total_files: filtered.len(),
        total_characters: metrics.total_characters,
        total_tokens: metrics.total_tokens,
        file_char_counts: relativize_map(metrics.file_char_counts),
        file_token_counts: relativize_map(metrics.file_token_counts),
        top_files_by_tokens,
        git_diff_content,
        git_diff_token_count,
        git_log_content,
        git_log_token_count,
        output_paths: output.written_paths,
        output_contents: output.contents,
        directory_structure: output.directory_structure,
        suspicious_files: validation.suspicious,
        processed_files: filtered,
        safe_file_paths: validation.safe_paths,
        skipped_files: collect_result.skipped_files,
    })
}

fn filter_suspicious(
    processed: Vec<ProcessedFile>,
    validation: &ValidationResult,
) -> Vec<ProcessedFile> {
    let suspicious_paths: std::collections::HashSet<_> =
        validation.suspicious.iter().map(|s| &s.path).collect();

    processed
        .into_iter()
        .filter(|file| !suspicious_paths.contains(&file.path))
        .collect()
}
