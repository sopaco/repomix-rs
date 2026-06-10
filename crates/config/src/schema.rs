use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct RepomixConfig {
    pub input: InputConfig,
    pub output: OutputConfig,
    pub include: Vec<String>,
    pub ignore: IgnoreConfig,
    pub security: SecurityConfig,
    pub token_count: TokenCountConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct InputConfig {
    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            max_file_size: default_max_file_size(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct OutputConfig {
    #[serde(default = "default_file_path")]
    pub file_path: String,
    #[serde(default = "default_style")]
    pub style: OutputStyle,
    #[serde(default)]
    /// 使输出风格更易于机器解析（XML 中附加 tokens/chars 属性，Markdown/Plain 添加结构化标记）
    pub parsable_style: bool,
    #[serde(default)]
    pub header_text: Option<String>,
    #[serde(default)]
    pub instruction_file_path: Option<String>,
    #[serde(default = "default_true")]
    pub file_summary: bool,
    #[serde(default = "default_true")]
    pub directory_structure: bool,
    #[serde(default = "default_true")]
    pub files: bool,
    #[serde(default)]
    pub remove_comments: bool,
    #[serde(default)]
    pub remove_empty_lines: bool,
    #[serde(default)]
    pub compress: bool,
    #[serde(default = "default_top_files_length")]
    pub top_files_length: usize,
    #[serde(default)]
    pub show_line_numbers: bool,
    #[serde(default)]
    pub truncate_base64: bool,
    #[serde(default)]
    pub copy_to_clipboard: bool,
    #[serde(default)]
    pub include_empty_directories: bool,
    #[serde(default)]
    /// 包含完整目录结构（含空目录）
    pub include_full_directory_structure: bool,
    #[serde(default)]
    /// 当输出总 token 数超过此阈值时拆成多个文件（按 token 计，非字节）。
    /// XML 在文件边界切分以保证每片结构完整；Markdown/Plain/JSON 按行切分。
    pub split_output: Option<u64>,
    #[serde(default)]
    pub token_count_tree: TokenCountTreeConfig,
    pub git: GitOutputConfig,
    #[serde(default)]
    pub json: JsonOutputConfig,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            file_path: default_file_path(),
            style: default_style(),
            parsable_style: false,
            header_text: None,
            instruction_file_path: None,
            file_summary: default_true(),
            directory_structure: default_true(),
            files: default_true(),
            remove_comments: false,
            remove_empty_lines: false,
            compress: false,
            top_files_length: default_top_files_length(),
            show_line_numbers: false,
            truncate_base64: false,
            copy_to_clipboard: false,
            include_empty_directories: false,
            include_full_directory_structure: false,
            split_output: None,
            token_count_tree: TokenCountTreeConfig::default(),
            git: GitOutputConfig::default(),
            json: JsonOutputConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct IgnoreConfig {
    #[serde(default = "default_true")]
    pub use_gitignore: bool,
    #[serde(default)]
    pub custom_ignore: Vec<String>,
}

impl Default for IgnoreConfig {
    fn default() -> Self {
        Self {
            use_gitignore: default_true(),
            custom_ignore: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct SecurityConfig {
    #[serde(default = "default_true")]
    pub enable_secretlint: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_secretlint: default_true(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct TokenCountConfig {
    #[serde(default = "default_encoding")]
    pub encoding: String,
}

impl Default for TokenCountConfig {
    fn default() -> Self {
        Self {
            encoding: default_encoding(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TokenCountTreeConfig {
    #[serde(default)]
    /// 在输出中显示 token 计数树
    pub show_tree: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct GitOutputConfig {
    #[serde(default = "default_true")]
    pub sort_by_changes: bool,
    #[serde(default = "default_100")]
    pub sort_by_changes_max_commits: usize,
    #[serde(default)]
    pub include_diffs: bool,
    #[serde(default)]
    pub include_logs: bool,
    #[serde(default = "default_50")]
    pub include_logs_count: usize,
}

impl Default for GitOutputConfig {
    fn default() -> Self {
        Self {
            sort_by_changes: default_true(),
            sort_by_changes_max_commits: default_100(),
            include_diffs: false,
            include_logs: false,
            include_logs_count: default_50(),
        }
    }
}

/// P3 修复（Bug #14）：JSON 输出专属配置
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct JsonOutputConfig {
    /// 关闭后 JSON metadata 中不含 packed_at 字段，
    /// 两次打包同一仓库可产生完全一致的 JSON（便于版本控制/缓存）
    #[serde(default)]
    pub no_timestamp: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputStyle {
    Xml,
    Markdown,
    Plain,
    Json,
}

fn default_max_file_size() -> u64 {
    50 * 1024 * 1024 // 50MB
}

fn default_file_path() -> String {
    "repomix-output.txt".to_string()
}

fn default_style() -> OutputStyle {
    OutputStyle::Xml
}

fn default_true() -> bool {
    true
}

fn default_top_files_length() -> usize {
    10
}

fn default_encoding() -> String {
    "o200k_base".to_string()
}

fn default_100() -> usize {
    100
}

fn default_50() -> usize {
    50
}

