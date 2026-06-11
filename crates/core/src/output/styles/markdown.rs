use std::collections::HashMap;
use std::path::Path;

use crate::output::decorate::{OutputHeader, format_header};
use crate::path_util::display_path;
use repomix_config::schema::RepomixConfig;
use repomix_shared::types::ProcessedFile;

/// 根据内容中最长连续反引号 run 选择 fence 长度，避免 `` ``` `` 提前闭合代码块。
fn wrap_markdown_code_block(content: &str) -> String {
    let mut fence_len = 3usize;
    let bytes = content.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'`' {
            let start = i;
            while i < bytes.len() && bytes[i] == b'`' {
                i += 1;
            }
            fence_len = fence_len.max(i - start + 1);
        } else {
            i += 1;
        }
    }
    let fence = "`".repeat(fence_len);
    format!("{fence}\n{content}\n{fence}\n\n")
}

/// 生成Markdown格式输出
#[allow(clippy::too_many_arguments)]
pub fn generate_markdown(
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
    output.push_str("# Repository Packed for AI Analysis\n\n");
    output.push_str("This file contains the packed representation of the repository.\n\n");
    output.push_str("## Purpose\n\n");
    output.push_str("This file contains the packed representation of the repository.\n\n");
    output.push_str("## File Format\n\n");
    output.push_str("The content is organized as follows:\n");
    output.push_str("1. This header section contains metadata about the packing process.\n");
    output.push_str("2. This directory structure section shows the repository structure.\n");
    output.push_str("3. Multiple file entries, each consisting of:\n");
    output.push_str("   - File path as a heading\n");
    output.push_str("   - Full contents of the file in a code block\n\n");

    let header_text = format_header(header);
    if !header_text.is_empty() {
        output.push_str("## Custom Instructions\n\n");
        output.push_str(&header_text);
        output.push_str("\n\n");
    }

    // 目录结构
    if config.output.directory_structure && !tree_string.is_empty() {
        output.push_str("## Directory Structure\n\n");
        output.push_str("```\n");
        output.push_str(tree_string);
        output.push_str("\n```\n\n");
    }

    // 文件内容
    if config.output.files {
        output.push_str("## Files\n\n");
        for file in files {
            let path_str = display_path(&file.path, pack_root);
            let path_safe = path_str.replace('|', "\\|");
            let line_count = line_counts.get(&path_str).unwrap_or(&0);
            if config.output.parsable_style {
                output.push_str(&format!(
                    "### {} ({} lines, {} tokens)\n\n",
                    path_safe, line_count, file.token_count
                ));
            } else {
                output.push_str(&format!("### {} ({} lines)\n\n", path_safe, line_count));
            }
            output.push_str(&wrap_markdown_code_block(&file.content));
        }
    }

    // Git Diff
    if let Some(diff) = git_diff_content
        && !diff.is_empty()
    {
        output.push_str("## Git Diff\n\n");
        output.push_str("```diff\n");
        output.push_str(diff);
        output.push_str("\n```\n\n");
    }

    // Git Log
    if let Some(log) = git_log_content
        && !log.is_empty()
    {
        output.push_str("## Git Log\n\n");
        output.push_str("```\n");
        output.push_str(log);
        output.push_str("\n```\n\n");
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_markdown_code_block_handles_inline_backticks() {
        let content = "fn main() {\n    let s = \"code with ``` inside\";\n}";
        let wrapped = wrap_markdown_code_block(content);
        assert!(wrapped.starts_with("````\n"));
        assert!(wrapped.ends_with("````\n\n"));
    }
}
