use std::collections::HashMap;
use std::path::Path;

use repomix_config::schema::RepomixConfig;
use repomix_shared::types::ProcessedFile;
use crate::output::decorate::{OutputHeader, format_header};
use crate::path_util::display_path;

/// 生成纯文本格式输出
pub fn generate_plain(
    files: &[ProcessedFile],
    config: &RepomixConfig,
    pack_root: &Path,
    tree_string: &str,
    line_counts: &HashMap<String, usize>,
    header: &OutputHeader,
    git_diff_content: &Option<String>,
    git_log_content: &Option<String>,
) -> String {
    let mut output = String::new();
    
    // 头部信息
    output.push_str("Repository Packed for AI Analysis\n");
    output.push_str("====================================\n\n");
    output.push_str("This file contains the packed representation of the repository.\n\n");

    let header_text = format_header(header);
    if !header_text.is_empty() {
        output.push_str("Custom Instructions\n");
        output.push_str("------------------\n");
        output.push_str(&header_text);
        output.push_str("\n\n");
    }
    
    // 目录结构
    if config.output.directory_structure && !tree_string.is_empty() {
        output.push_str("Directory Structure\n");
        output.push_str("------------------\n");
        output.push_str(tree_string);
        output.push_str("\n\n");
    }
    
    // 文件内容
    if config.output.files {
        output.push_str("Files\n");
        output.push_str("-----\n\n");
        for file in files {
            let path_str = display_path(&file.path, pack_root);
            let path_safe = path_str.replace(['\n', '\r', '\t'], " ");
            let line_count = line_counts.get(&path_str).unwrap_or(&0);
            if config.output.parsable_style {
                output.push_str(&format!("======== FILE: {} ({} lines, {} tokens) ========\n", path_safe, line_count, file.token_count));
            } else {
                output.push_str(&format!("File: {} ({} lines)\n", path_safe, line_count));
                output.push_str(&"-".repeat(40));
            }
            output.push('\n');
            output.push_str(&file.content);
            output.push_str("\n\n");
        }
    }
    
    // Git Diff
    if let Some(ref diff) = git_diff_content {
        if !diff.is_empty() {
            output.push_str("Git Diff\n");
            output.push_str("--------\n");
            output.push_str(diff);
            output.push_str("\n\n");
        }
    }
    
    // Git Log
    if let Some(ref log) = git_log_content {
        if !log.is_empty() {
            output.push_str("Git Log\n");
            output.push_str("-------\n");
            output.push_str(log);
            output.push_str("\n\n");
        }
    }
    
    output
}
