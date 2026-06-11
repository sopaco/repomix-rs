use std::path::Path;

use repomix_config::schema::RepomixConfig;
use repomix_shared::types::ProcessedFile;
use serde::{Deserialize, Serialize};
use crate::output::decorate::{OutputHeader, format_header};
use crate::path_util::display_path;

/// JSON输出结构
#[derive(Serialize, Deserialize)]
struct JsonOutput {
    metadata: JsonMetadata,
    custom_instructions: Option<String>,
    directory_structure: Option<String>,
    files: Vec<JsonFile>,
    git_diff: Option<String>,
    git_log: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_count_tree: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct JsonMetadata {
    /// 默认启用；可通过 `output.json.no_timestamp` 关闭以保持输出可复现
    #[serde(skip_serializing_if = "Option::is_none")]
    packed_at: Option<String>,
    total_files: usize,
    total_tokens: usize,
}

#[derive(Serialize, Deserialize)]
struct JsonFile {
    path: String,
    content: String,
    token_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    char_count: Option<usize>,
}

/// 生成JSON格式输出
pub fn generate_json(
    files: &[ProcessedFile],
    config: &RepomixConfig,
    pack_root: &Path,
    tree_string: &str,
    header: &OutputHeader,
    git_diff_content: &Option<String>,
    git_log_content: &Option<String>,
    token_count_tree: Option<&str>,
) -> String {
    let include_timestamp = !config.output.json.no_timestamp;
    let metadata = JsonMetadata {
        packed_at: if include_timestamp {
            Some(chrono::Utc::now().to_rfc3339())
        } else {
            None
        },
        total_files: files.len(),
        total_tokens: files.iter().map(|f| f.token_count).sum(),
    };

    let header_text = format_header(header);
    let custom_instructions = if header_text.is_empty() {
        None
    } else {
        Some(header_text)
    };

    let directory_structure = if config.output.directory_structure && !tree_string.is_empty() {
        Some(tree_string.to_string())
    } else {
        None
    };

    let json_files: Vec<JsonFile> = files
        .iter()
        .map(|file| JsonFile {
            path: display_path(&file.path, pack_root),
            content: file.content.clone(),
            token_count: file.token_count,
            char_count: if config.output.parsable_style {
                Some(file.content.len())
            } else {
                None
            },
        })
        .collect();

    let output = JsonOutput {
        metadata,
        custom_instructions,
        directory_structure,
        files: json_files,
        git_diff: git_diff_content.as_ref().and_then(|d| if d.is_empty() { None } else { Some(d.clone()) }),
        git_log: git_log_content.as_ref().and_then(|l| if l.is_empty() { None } else { Some(l.clone()) }),
        token_count_tree: token_count_tree.map(|t| t.to_string()),
    };

    serde_json::to_string_pretty(&output).unwrap_or_default()
}
