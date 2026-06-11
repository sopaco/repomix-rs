//! MCP pack 工具共享参数类型。

use rmcp::model::ErrorData;
use rmcp::schemars;
use serde::Deserialize;

use crate::helpers::{McpPackOverrides, parse_style};

/// `pack_codebase` / `pack_remote_repository` 共用的打包选项。
#[derive(Debug, Default, Clone, Deserialize, schemars::JsonSchema)]
pub struct PackSharedParams {
    /// Enable Tree-sitter compression to extract code signatures.
    #[serde(default)]
    pub compress: Option<bool>,
    /// Comma-separated include patterns, e.g. "*.rs,*.toml".
    #[serde(default)]
    pub include_patterns: Option<String>,
    /// Comma-separated ignore patterns, e.g. "target/**,tests/**".
    #[serde(default)]
    pub ignore_patterns: Option<String>,
    /// Remove comments from source files.
    #[serde(default)]
    pub remove_comments: Option<bool>,
    /// Remove empty lines from source files.
    #[serde(default)]
    pub remove_empty_lines: Option<bool>,
    /// Prefix each line with a line number.
    #[serde(default)]
    pub show_line_numbers: Option<bool>,
    /// Truncate long base64-encoded data in output.
    #[serde(default)]
    pub truncate_base64: Option<bool>,
    /// Number of top files to include in metrics breakdown.
    #[serde(default)]
    pub top_files_length: Option<usize>,
    /// Split output when total tokens exceed this threshold (per-part token budget).
    #[serde(default)]
    pub split_output: Option<u64>,
    /// Header text to include in output.
    #[serde(default)]
    pub header_text: Option<String>,
    /// Include git diffs in output.
    #[serde(default)]
    pub include_diffs: Option<bool>,
    /// Include git logs in output.
    #[serde(default)]
    pub include_logs: Option<bool>,
    /// Output style: xml | markdown | plain | json. Default: xml.
    #[serde(default)]
    pub style: Option<String>,
}

impl PackSharedParams {
    pub fn into_mcp_overrides(self) -> Result<McpPackOverrides, ErrorData> {
        Ok(McpPackOverrides {
            include_patterns: self.include_patterns,
            ignore_patterns: self.ignore_patterns,
            compress: self.compress,
            remove_comments: self.remove_comments,
            remove_empty_lines: self.remove_empty_lines,
            show_line_numbers: self.show_line_numbers,
            truncate_base64: self.truncate_base64,
            top_files_length: self.top_files_length,
            split_output: self.split_output,
            header_text: self.header_text,
            include_diffs: self.include_diffs,
            include_logs: self.include_logs,
            style: Some(parse_style(self.style.as_deref())?),
        })
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PackCodebaseParams {
    /// Directory to pack (absolute path). Defaults to current working directory.
    #[serde(default)]
    pub directory: Option<String>,
    #[serde(flatten)]
    pub shared: PackSharedParams,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PackRemoteRepositoryParams {
    /// Git remote URL (https://... or git@...).
    pub url: String,
    #[serde(flatten)]
    pub shared: PackSharedParams,
}
