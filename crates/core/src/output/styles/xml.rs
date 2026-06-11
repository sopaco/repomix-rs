use std::path::Path;

use crate::output::decorate::{OutputHeader, format_header};
use crate::path_util::display_path;
use repomix_config::schema::RepomixConfig;
use repomix_shared::types::ProcessedFile;

/// 对插入 XML 属性 / Markdown / Plain 的路径进行转义
fn xml_attr_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// 对插入 XML 文本节点的文件内容做最小转义（保留换行/制表/回车）
fn xml_text_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// 分片元数据：用于在 token 阈值拆分时生成结构完整的 XML 分片。
#[derive(Debug, Clone, Copy)]
pub struct XmlSplitMeta {
    pub part: usize,
    pub total_parts: usize,
    pub is_first_part: bool,
    pub is_last_part: bool,
}

/// 渲染 XML 输出（支持分片模式）。
///
/// `split_meta == None` 时行为与历史 `generate_xml` 一致（git 段落始终输出）。
/// 分片模式下：`file_summary` / 自定义 header 仅出现在第一片；`git_*` 与
/// `token_count_tree` 仅出现在最后一片；中间片带 `<split_info>` 标记。
#[allow(clippy::too_many_arguments)]
pub(crate) fn render_xml_part(
    files: &[ProcessedFile],
    config: &RepomixConfig,
    pack_root: &Path,
    tree_string: &str,
    header: &OutputHeader,
    git_diff_content: &Option<String>,
    git_log_content: &Option<String>,
    token_count_tree: Option<&str>,
    split_meta: Option<XmlSplitMeta>,
) -> String {
    let mut output = String::new();

    let is_first = split_meta.map(|m| m.is_first_part).unwrap_or(true);
    let is_last = split_meta.map(|m| m.is_last_part).unwrap_or(true);

    if let Some(meta) = split_meta
        && !meta.is_first_part
    {
        output.push_str(&format!(
            "<split_info part=\"{}\" total_parts=\"{}\">\n",
            meta.part, meta.total_parts
        ));
        output.push_str(
            "Continuation of a split repomix output. Earlier parts contain preceding files.\n",
        );
        output.push_str("</split_info>\n\n");
    }

    // 头部信息
    if config.output.file_summary && is_first {
        output.push_str("<file_summary>\n");
        output.push_str("This section contains a summary of this file.\n\n");
        output.push_str("<purpose>\n");
        output.push_str("This file contains the packed representation of the repository.\n");
        output.push_str("</purpose>\n\n");
        output.push_str("<file_format>\n");
        output.push_str("The content is organized as follows:\n");
        output.push_str("1. This header section contains metadata about the packing process.\n");
        output.push_str("2. This directory structure section shows the repository structure.\n");
        output.push_str("3. Multiple file entries, each consisting of:\n");
        output.push_str("   - File path as an attribute\n");
        output.push_str("   - Full contents of the file\n");
        output.push_str("</file_format>\n");
        output.push_str("</file_summary>\n\n");
    }

    if is_first {
        let header_text = format_header(header);
        if !header_text.is_empty() {
            output.push_str("<header>\n");
            output.push_str(&header_text);
            output.push_str("\n</header>\n\n");
        }
    }

    // 目录结构
    if config.output.directory_structure && !tree_string.is_empty() {
        output.push_str("<directory_structure>\n");
        output.push_str(tree_string);
        output.push_str("\n</directory_structure>\n\n");
    }

    // 文件内容
    if config.output.files {
        output.push_str("<files>\n");
        for file in files {
            let path_escaped = xml_attr_escape(&display_path(&file.path, pack_root));
            if config.output.parsable_style {
                output.push_str(&format!(
                    "<file path=\"{}\" tokens=\"{}\" chars=\"{}\">\n",
                    path_escaped,
                    file.token_count,
                    file.content.len()
                ));
            } else {
                output.push_str(&format!("<file path=\"{}\">\n", path_escaped));
            }
            output.push_str(&xml_text_escape(&file.content));
            output.push_str("\n</file>\n\n");
        }
        output.push_str("</files>\n");
    }

    // Git Diff / Log（分片时仅最后一片携带）
    if is_last {
        if let Some(diff) = git_diff_content
            && !diff.is_empty()
        {
            output.push_str("\n<git_diff>\n");
            output.push_str(diff);
            output.push_str("\n</git_diff>\n");
        }

        if let Some(log) = git_log_content
            && !log.is_empty()
        {
            output.push_str("\n<git_log>\n");
            output.push_str(log);
            output.push_str("\n</git_log>\n");
        }

        if let Some(tree) = token_count_tree
            && !tree.is_empty()
        {
            output.push_str("\n<token_count_tree>\n");
            output.push_str(tree);
            output.push_str("\n</token_count_tree>\n");
        }
    }

    output
}

/// 生成 XML 格式输出（单片、非分片）。
pub fn generate_xml(
    files: &[ProcessedFile],
    config: &RepomixConfig,
    pack_root: &Path,
    tree_string: &str,
    header: &OutputHeader,
    git_diff_content: &Option<String>,
    git_log_content: &Option<String>,
) -> String {
    render_xml_part(
        files,
        config,
        pack_root,
        tree_string,
        header,
        git_diff_content,
        git_log_content,
        None,
        None,
    )
}
