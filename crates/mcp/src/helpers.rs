//! MCP 工具共享辅助函数（可单测）。

use repomix_config::load::PartialConfig;
use repomix_config::schema::OutputStyle;
use rmcp::model::ErrorData;

/// MCP pack 工具共用的配置覆盖项（`pack_codebase` / `pack_remote_repository`）。
#[derive(Debug, Default, Clone)]
pub struct McpPackOverrides {
    pub include_patterns: Option<String>,
    pub ignore_patterns: Option<String>,
    pub compress: Option<bool>,
    pub remove_comments: Option<bool>,
    pub remove_empty_lines: Option<bool>,
    pub show_line_numbers: Option<bool>,
    pub truncate_base64: Option<bool>,
    pub top_files_length: Option<usize>,
    pub split_output: Option<u64>,
    pub header_text: Option<String>,
    pub include_diffs: Option<bool>,
    pub include_logs: Option<bool>,
    pub style: Option<OutputStyle>,
}

impl McpPackOverrides {
    pub fn into_partial_config(self) -> PartialConfig {
        PartialConfig {
            include: self.include_patterns.as_deref().map(|s| split_csv(Some(s))),
            ignore: self.ignore_patterns.as_deref().map(|s| split_csv(Some(s))),
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
            style: self.style,
            ..Default::default()
        }
    }
}

/// 解析输出风格；未知值返回 `invalid_params` 错误，空值默认 XML。
pub fn parse_style(s: Option<&str>) -> Result<OutputStyle, ErrorData> {
    match s {
        None | Some("") | Some("xml") => Ok(OutputStyle::Xml),
        Some("markdown") => Ok(OutputStyle::Markdown),
        Some("plain") => Ok(OutputStyle::Plain),
        Some("json") => Ok(OutputStyle::Json),
        Some(other) => Err(ErrorData::invalid_params(
            format!(
                "invalid style '{}': expected one of xml, markdown, plain, json",
                other
            ),
            None,
        )),
    }
}

/// 验证远程仓库 URL 的基本合法性。
pub fn validate_remote_url(url: &str) -> Result<(), ErrorData> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return Err(ErrorData::invalid_params("remote url is empty", None));
    }
    let ok = trimmed.starts_with("https://")
        || trimmed.starts_with("http://")
        || trimmed.starts_with("git://")
        || trimmed.starts_with("ssh://")
        || (trimmed.contains('@') && trimmed.contains(':') && !trimmed.contains(' '));
    if !ok {
        return Err(ErrorData::invalid_params(
            format!(
                "remote url '{}' is not a recognized git url \
                 (expected https://, http://, git://, ssh:// or user@host:path)",
                trimmed
            ),
            None,
        ));
    }
    Ok(())
}

pub fn split_csv(s: Option<&str>) -> Vec<String> {
    s.map(|v| {
        v.split(',')
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect()
    })
    .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_style_defaults_to_xml() {
        assert!(matches!(parse_style(None).unwrap(), OutputStyle::Xml));
        assert!(matches!(parse_style(Some("")).unwrap(), OutputStyle::Xml));
        assert!(matches!(
            parse_style(Some("xml")).unwrap(),
            OutputStyle::Xml
        ));
    }

    #[test]
    fn parse_style_rejects_unknown() {
        assert!(parse_style(Some("yaml")).is_err());
    }

    #[test]
    fn validate_remote_url_accepts_https() {
        assert!(validate_remote_url("https://github.com/owner/repo").is_ok());
    }

    #[test]
    fn validate_remote_url_rejects_empty() {
        assert!(validate_remote_url("").is_err());
    }

    #[test]
    fn split_csv_trims_and_skips_empty() {
        assert_eq!(
            split_csv(Some(" *.rs , , *.toml ")),
            vec!["*.rs".to_string(), "*.toml".to_string()]
        );
    }
}
