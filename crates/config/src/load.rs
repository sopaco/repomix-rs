use crate::schema::RepomixConfig;
use crate::global_dir;
use std::path::Path;
use anyhow::Result;

impl RepomixConfig {
    /// 加载配置：默认值 → 全局配置 → 项目配置 → CLI 参数
    pub fn load(cli_overrides: Option<PartialConfig>, cwd: &Path) -> Result<Self> {
        let mut config = Self::default();

        // 全局配置：~/.repomix/repomix.config.json
        match Self::load_from_file(&global_dir::global_config_path()?) {
            Ok(Some(global)) => config.merge_global(global),
            Ok(None) => {},
            Err(e) => tracing::warn!("Failed to load global config: {}", e),
        }

        // 项目配置：./repomix.config.json
        match Self::load_from_file(&cwd.join("repomix.config.json")) {
            Ok(Some(local)) => config.merge_local(local),
            Ok(None) => {},
            Err(e) => tracing::warn!("Failed to load project config: {}", e),
        }

        // CLI 参数覆盖
        if let Some(overrides) = cli_overrides {
            config.merge_cli(overrides);
        }

        config.validate()?;
        Ok(config)
    }

    /// 从文件加载配置
    pub fn load_from_file(path: &Path) -> Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read config file {}: {}", path.display(), e))?;
        let config: Self = serde_json::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse config file {}: {}", path.display(), e))?;
        Ok(Some(config))
    }

    /// 合并全局配置
    fn merge_global(&mut self, other: Self) {
        let defaults = RepomixConfig::default();
        
        // 合并include模式（追加）
        self.include.extend(other.include);
        
        // 合并ignore配置
        self.ignore.custom_ignore.extend(other.ignore.custom_ignore);
        if other.ignore.use_gitignore != defaults.ignore.use_gitignore {
            self.ignore.use_gitignore = other.ignore.use_gitignore;
        }
        
        // 合并input配置
        if other.input.max_file_size != defaults.input.max_file_size {
            self.input.max_file_size = other.input.max_file_size;
        }
        
        // 合并output配置（仅覆盖非默认值）
        if other.output.file_path != defaults.output.file_path {
            self.output.file_path = other.output.file_path;
        }
        if other.output.style != defaults.output.style {
            self.output.style = other.output.style;
        }
        if other.output.parsable_style != defaults.output.parsable_style {
            self.output.parsable_style = other.output.parsable_style;
        }
        if other.output.header_text != defaults.output.header_text {
            self.output.header_text = other.output.header_text;
        }
        if other.output.instruction_file_path != defaults.output.instruction_file_path {
            self.output.instruction_file_path = other.output.instruction_file_path;
        }
        if other.output.file_summary != defaults.output.file_summary {
            self.output.file_summary = other.output.file_summary;
        }
        if other.output.directory_structure != defaults.output.directory_structure {
            self.output.directory_structure = other.output.directory_structure;
        }
        if other.output.files != defaults.output.files {
            self.output.files = other.output.files;
        }
        if other.output.remove_comments != defaults.output.remove_comments {
            self.output.remove_comments = other.output.remove_comments;
        }
        if other.output.remove_empty_lines != defaults.output.remove_empty_lines {
            self.output.remove_empty_lines = other.output.remove_empty_lines;
        }
        if other.output.compress != defaults.output.compress {
            self.output.compress = other.output.compress;
        }
        if other.output.top_files_length != defaults.output.top_files_length {
            self.output.top_files_length = other.output.top_files_length;
        }
        if other.output.show_line_numbers != defaults.output.show_line_numbers {
            self.output.show_line_numbers = other.output.show_line_numbers;
        }
        if other.output.truncate_base64 != defaults.output.truncate_base64 {
            self.output.truncate_base64 = other.output.truncate_base64;
        }
        if other.output.copy_to_clipboard != defaults.output.copy_to_clipboard {
            self.output.copy_to_clipboard = other.output.copy_to_clipboard;
        }
        if other.output.include_empty_directories != defaults.output.include_empty_directories {
            self.output.include_empty_directories = other.output.include_empty_directories;
        }
        if other.output.include_full_directory_structure != defaults.output.include_full_directory_structure {
            self.output.include_full_directory_structure = other.output.include_full_directory_structure;
        }
        if other.output.split_output != defaults.output.split_output {
            self.output.split_output = other.output.split_output;
        }
        if other.output.token_count_tree.show_tree != defaults.output.token_count_tree.show_tree {
            self.output.token_count_tree.show_tree = other.output.token_count_tree.show_tree;
        }
        // 合并git输出配置
        if other.output.git.sort_by_changes != defaults.output.git.sort_by_changes {
            self.output.git.sort_by_changes = other.output.git.sort_by_changes;
        }
        if other.output.git.sort_by_changes_max_commits != defaults.output.git.sort_by_changes_max_commits {
            self.output.git.sort_by_changes_max_commits = other.output.git.sort_by_changes_max_commits;
        }
        if other.output.git.include_diffs != defaults.output.git.include_diffs {
            self.output.git.include_diffs = other.output.git.include_diffs;
        }
        if other.output.git.include_logs != defaults.output.git.include_logs {
            self.output.git.include_logs = other.output.git.include_logs;
        }
        if other.output.git.include_logs_count != defaults.output.git.include_logs_count {
            self.output.git.include_logs_count = other.output.git.include_logs_count;
        }

        // 合并 json 配置
        if other.output.json.no_timestamp != defaults.output.json.no_timestamp {
            self.output.json.no_timestamp = other.output.json.no_timestamp;
        }

        // 合并security配置
        if other.security.enable_secretlint != defaults.security.enable_secretlint {
            self.security.enable_secretlint = other.security.enable_secretlint;
        }
        
        // 合并token_count配置
        if other.token_count.encoding != defaults.token_count.encoding {
            self.token_count.encoding = other.token_count.encoding;
        }
    }

    /// 合并本地配置
    fn merge_local(&mut self, other: Self) {
        // 本地配置优先级高于全局配置
        self.merge_global(other);
    }

    /// 合并 CLI 参数（include / ignore 追加模式，与 `merge_global` / `merge_local` 一致）
    pub(crate) fn merge_cli(&mut self, overrides: PartialConfig) {
        if let Some(mut include) = overrides.include {
            self.include.append(&mut include);
        }

        if let Some(mut ignore) = overrides.ignore {
            self.ignore.custom_ignore.append(&mut ignore);
        }
        
        if let Some(style) = overrides.style {
            self.output.style = style;
        }
        
        if let Some(compress) = overrides.compress {
            self.output.compress = compress;
        }
        
        if let Some(remove_comments) = overrides.remove_comments {
            self.output.remove_comments = remove_comments;
        }
        
        if let Some(remove_empty_lines) = overrides.remove_empty_lines {
            self.output.remove_empty_lines = remove_empty_lines;
        }
        
        if let Some(show_line_numbers) = overrides.show_line_numbers {
            self.output.show_line_numbers = show_line_numbers;
        }
        
        if let Some(truncate_base64) = overrides.truncate_base64 {
            self.output.truncate_base64 = truncate_base64;
        }
        
        if let Some(copy_to_clipboard) = overrides.copy_to_clipboard {
            self.output.copy_to_clipboard = copy_to_clipboard;
        }
        
        if let Some(output) = overrides.output {
            self.output.file_path = output;
        }
        
        if let Some(include_empty_directories) = overrides.include_empty_directories {
            self.output.include_empty_directories = include_empty_directories;
        }
        
        if let Some(top_files_length) = overrides.top_files_length {
            self.output.top_files_length = top_files_length;
        }
        
        if let Some(split_output) = overrides.split_output {
            self.output.split_output = Some(split_output);
        }
        
        if let Some(header_text) = overrides.header_text {
            self.output.header_text = Some(header_text);
        }
        
        if let Some(instruction_file_path) = overrides.instruction_file_path {
            self.output.instruction_file_path = Some(instruction_file_path);
        }
        
        if let Some(include_diffs) = overrides.include_diffs {
            self.output.git.include_diffs = include_diffs;
        }
        
        if let Some(include_logs) = overrides.include_logs {
            self.output.git.include_logs = include_logs;
        }
    }

    /// 验证配置
    fn validate(&self) -> Result<()> {
        // 验证输出路径
        if self.output.file_path.is_empty() {
            anyhow::bail!("Output file path cannot be empty");
        }
        
        // 验证文件大小限制
        if self.input.max_file_size == 0 {
            anyhow::bail!("Max file size cannot be zero");
        }
        
        Ok(())
    }
}

/// 部分配置（用于CLI参数覆盖）
#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub include: Option<Vec<String>>,
    pub ignore: Option<Vec<String>>,
    pub style: Option<crate::schema::OutputStyle>,
    pub compress: Option<bool>,
    pub remove_comments: Option<bool>,
    pub remove_empty_lines: Option<bool>,
    pub show_line_numbers: Option<bool>,
    pub truncate_base64: Option<bool>,
    pub copy_to_clipboard: Option<bool>,
    pub output: Option<String>,
    pub include_empty_directories: Option<bool>,
    pub top_files_length: Option<usize>,
    pub split_output: Option<u64>,
    pub header_text: Option<String>,
    pub instruction_file_path: Option<String>,
    pub include_diffs: Option<bool>,
    pub include_logs: Option<bool>,
}