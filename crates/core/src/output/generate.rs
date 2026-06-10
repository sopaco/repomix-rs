use std::path::Path;

use anyhow::Result;
use repomix_config::schema::{RepomixConfig, OutputStyle};
use repomix_shared::types::ProcessedFile;
use crate::path_util::display_path;
use crate::output::styles::{xml, markdown, plain, json};
use crate::output::decorate::collect_header;
use crate::output::split;
use crate::file::tree_generate::{generate_tree_string, generate_tree_string_with_line_counts, calculate_file_line_counts};

/// 输出结果
pub struct OutputResult {
    /// 实际写入磁盘的路径列表（分片模式下有多条）
    pub written_paths: Vec<String>,
    /// 输出内容（与 `written_paths` 一一对应）
    pub contents: Vec<String>,
    /// 目录树文本（供 MCP / 报告使用）
    pub directory_structure: String,
}

/// 生成输出
pub fn produce_output(
    files: &[ProcessedFile],
    config: &RepomixConfig,
    pack_root: &Path,
    git_diff_content: &Option<String>,
    git_log_content: &Option<String>,
    empty_dir_paths: &[String],
) -> Result<OutputResult> {
    // 收集头部信息（不再修改文件内容）
    let header = collect_header(config);
    
    // 生成相对 pack 根目录的文件路径列表
    let file_paths: Vec<String> = files
        .iter()
        .map(|f| display_path(&f.path, pack_root))
        .collect();
    
    // 生成目录树
    let tree_string = if config.output.directory_structure {
        let empty_dirs: Vec<String> = if config.output.include_full_directory_structure {
            empty_dir_paths
                .iter()
                .map(|p| display_path(Path::new(p), pack_root))
                .collect()
        } else {
            Vec::new()
        };
        generate_tree_string(&file_paths, &empty_dirs)
    } else {
        String::new()
    };
    
    // 计算文件行数
    let contents: Vec<String> = files.iter().map(|f| f.content.clone()).collect();
    let line_counts = calculate_file_line_counts(&file_paths, &contents);
    
    // 根据风格生成输出
    // Token 计数树（JSON 输出需要在生成前准备好，以便嵌入到 JSON 结构中）
    let token_tree = if config.output.token_count_tree.show_tree {
        let token_counts: std::collections::HashMap<String, usize> = file_paths
            .iter()
            .zip(files.iter())
            .map(|(path, f)| (path.clone(), f.token_count))
            .collect();
        let empty_dirs: Vec<String> = if config.output.include_full_directory_structure {
            empty_dir_paths
                .iter()
                .map(|p| display_path(Path::new(p), pack_root))
                .collect()
        } else {
            Vec::new()
        };
        Some(generate_tree_string_with_line_counts(
            &file_paths,
            &token_counts,
            &empty_dirs,
        ))
    } else {
        None
    };

    // 分割大文件（XML 按文件边界；其余风格按行切分已渲染文本）
    let output_contents = if let Some(threshold) = config.output.split_output {
        match config.output.style {
            OutputStyle::Xml => split::split_xml_by_files(
                files,
                config,
                pack_root,
                &tree_string,
                &header,
                git_diff_content,
                git_log_content,
                token_tree.as_deref(),
                threshold,
                &config.token_count.encoding,
            ),
            _ => {
                let mut output_content = match config.output.style {
                    OutputStyle::Markdown => markdown::generate_markdown(
                        files,
                        config,
                        pack_root,
                        &tree_string,
                        &line_counts,
                        &header,
                        git_diff_content,
                        git_log_content,
                    ),
                    OutputStyle::Plain => plain::generate_plain(
                        files,
                        config,
                        pack_root,
                        &tree_string,
                        &line_counts,
                        &header,
                        git_diff_content,
                        git_log_content,
                    ),
                    OutputStyle::Json => json::generate_json(
                        files,
                        config,
                        pack_root,
                        &tree_string,
                        &header,
                        git_diff_content,
                        git_log_content,
                        token_tree.as_deref(),
                    ),
                    OutputStyle::Xml => unreachable!(),
                };
                if let Some(ref tree) = token_tree {
                    if config.output.style != OutputStyle::Json {
                        output_content
                            .push_str(&format_token_count_tree(tree, &config.output.style));
                    }
                }
                split::split_output(
                    &output_content,
                    threshold,
                    &config.output.style,
                    &config.token_count.encoding,
                )
            }
        }
    } else {
        let mut output_content = match config.output.style {
            OutputStyle::Xml => xml::generate_xml(
                files,
                config,
                pack_root,
                &tree_string,
                &header,
                git_diff_content,
                git_log_content,
            ),
            OutputStyle::Markdown => markdown::generate_markdown(
                files,
                config,
                pack_root,
                &tree_string,
                &line_counts,
                &header,
                git_diff_content,
                git_log_content,
            ),
            OutputStyle::Plain => plain::generate_plain(
                files,
                config,
                pack_root,
                &tree_string,
                &line_counts,
                &header,
                git_diff_content,
                git_log_content,
            ),
            OutputStyle::Json => json::generate_json(
                files,
                config,
                pack_root,
                &tree_string,
                &header,
                git_diff_content,
                git_log_content,
                token_tree.as_deref(),
            ),
        };
        if let Some(ref tree) = token_tree {
            if config.output.style != OutputStyle::Json {
                output_content.push_str(&format_token_count_tree(tree, &config.output.style));
            }
        }
        vec![output_content]
    };

    let base_output_path = config.output.file_path.clone();
    let mut written_paths = Vec::with_capacity(output_contents.len());

    for (i, content) in output_contents.iter().enumerate() {
        let output_path = if i == 0 {
            base_output_path.clone()
        } else {
            format!("{}.{}", base_output_path, i + 1)
        };
        std::fs::write(&output_path, content)?;
        written_paths.push(output_path);
    }
    
    // 复制到剪贴板
    if config.output.copy_to_clipboard {
        if let Some(content) = output_contents.first() {
            if let Err(e) = copy_to_clipboard(content) {
                tracing::warn!(
                    "Failed to copy to clipboard ({}). Pack succeeded but clipboard was not set. \
                     This is expected in headless/SSH/CI environments.",
                    e
                );
            }
        }
    }
    
    Ok(OutputResult {
        written_paths,
        contents: output_contents,
        directory_structure: tree_string,
    })
}

/// 复制到剪贴板
fn copy_to_clipboard(content: &str) -> Result<()> {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(content.to_string())?;
    Ok(())
}

/// 格式化 token 计数树
fn format_token_count_tree(tree: &str, style: &OutputStyle) -> String {
    match style {
        OutputStyle::Xml => format!("\n<token_count_tree>\n{}\n</token_count_tree>\n", tree),
        OutputStyle::Markdown => format!("\n## Token Count Tree\n\n```\n{}\n```\n\n", tree),
        OutputStyle::Plain => format!("\nToken Count Tree\n----------------\n{}\n\n", tree),
        OutputStyle::Json => format!("\nToken Count Tree:\n{}\n", tree),
    }
}
