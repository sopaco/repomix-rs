use std::path::Path;

use repomix_config::schema::{OutputStyle, RepomixConfig};
use repomix_shared::types::ProcessedFile;

use crate::metrics::token_count::{TokenCounter, estimate_tokens_fallback};
use crate::output::decorate::OutputHeader;
use crate::output::styles::xml::{XmlSplitMeta, render_xml_part};

/// 按 **token 数**（非字节）分割已渲染的文本输出。
///
/// 行为契约：
/// - `content` 总 token 数 `<= threshold` 时，原样返回单 part（不包裹任何元数据）。
/// - 多 part 时按行切分，每段不超过 `threshold` token。
/// - `style == Json` 时多 part 输出一个 JSON 数组，每个元素仍为合法 JSON。
/// - `style == Markdown | Plain` 时多 part 包裹 `<!-- file_part -->` 注释。
///
/// **XML 风格请使用 [`split_xml_by_files`]**：在文件边界切分并保证每片含完整
/// `<files>...</files>` 结构，避免行级切分破坏 XML。
///
/// `encoding` 与主流程 `TokenCounter` 使用同一 tiktoken 编码（失败时降级到 CJK 友好估算）。
pub fn split_output(
    content: &str,
    token_threshold: u64,
    style: &OutputStyle,
    encoding: &str,
) -> Vec<String> {
    let counter = TokenCounter::new(encoding).ok();
    let count_tokens = |text: &str| -> usize {
        if let Some(c) = &counter {
            c.count_tokens(text)
        } else {
            estimate_tokens_fallback(text)
        }
    };

    let total_tokens = count_tokens(content);
    let threshold = token_threshold as usize;

    if total_tokens <= threshold {
        return vec![content.to_string()];
    }

    let parts = split_lines_by_tokens(content, threshold, &count_tokens);
    let total = parts.len();

    if total <= 1 {
        return parts;
    }

    match style {
        OutputStyle::Json => {
            let mut combined = String::from("[\n");
            for (i, part) in parts.iter().enumerate() {
                if i > 0 {
                    combined.push_str(",\n");
                }
                let part_json = serde_json::json!({
                    "part": i + 1,
                    "total_parts": total,
                    "content": part,
                });
                combined.push_str(&serde_json::to_string(&part_json).unwrap_or_else(|_| {
                    format!(
                        "{{\"part\":{},\"total_parts\":{},\"content\":{}}}",
                        i + 1,
                        total,
                        serde_json::Value::String(part.clone()),
                    )
                }));
            }
            combined.push_str("\n]");
            vec![combined]
        }
        // XML 应由 split_xml_by_files 在 generate 阶段按文件边界分片。
        OutputStyle::Xml => parts,
        OutputStyle::Markdown | OutputStyle::Plain => parts
            .into_iter()
            .enumerate()
            .map(|(i, part)| {
                format!(
                    "<!-- file_part id=\"{}\" total_parts=\"{}\" -->\n{}\n<!-- /file_part -->",
                    i + 1,
                    total,
                    part
                )
            })
            .collect(),
    }
}

/// 按文件边界将 XML 输出拆成多片，每片均为结构完整的 XML（含闭合的 `<files>`）。
#[allow(clippy::too_many_arguments)]
pub fn split_xml_by_files(
    files: &[ProcessedFile],
    config: &RepomixConfig,
    pack_root: &Path,
    tree_string: &str,
    header: &OutputHeader,
    git_diff_content: &Option<String>,
    git_log_content: &Option<String>,
    token_count_tree: Option<&str>,
    token_threshold: u64,
    encoding: &str,
) -> Vec<String> {
    let count_tokens = make_token_counter_fn(encoding);
    let threshold = token_threshold as usize;

    let full = render_xml_part(
        files,
        config,
        pack_root,
        tree_string,
        header,
        git_diff_content,
        git_log_content,
        token_count_tree,
        None,
    );
    if count_tokens(&full) <= threshold {
        return vec![full];
    }

    let mut chunks: Vec<Vec<usize>> = Vec::new();
    let mut current: Vec<usize> = Vec::new();

    for (i, _file) in files.iter().enumerate() {
        let mut trial: Vec<usize> = current.clone();
        trial.push(i);

        let trial_files: Vec<ProcessedFile> = trial.iter().map(|&j| files[j].clone()).collect();
        let is_first = chunks.is_empty();
        let trial_meta = XmlSplitMeta {
            part: chunks.len() + 1,
            total_parts: 2,
            is_first_part: is_first,
            is_last_part: false,
        };
        let trial_xml = render_xml_part(
            &trial_files,
            config,
            pack_root,
            tree_string,
            header,
            git_diff_content,
            git_log_content,
            None,
            Some(trial_meta),
        );

        if count_tokens(&trial_xml) > threshold && !current.is_empty() {
            chunks.push(current);
            current = vec![i];
        } else {
            current.push(i);
        }
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    if chunks.is_empty() {
        return vec![full];
    }

    let total_parts = chunks.len();
    chunks
        .into_iter()
        .enumerate()
        .map(|(part_idx, indices)| {
            let part_files: Vec<ProcessedFile> =
                indices.iter().map(|&j| files[j].clone()).collect();
            let meta = XmlSplitMeta {
                part: part_idx + 1,
                total_parts,
                is_first_part: part_idx == 0,
                is_last_part: part_idx + 1 == total_parts,
            };
            let tree = if meta.is_last_part {
                token_count_tree
            } else {
                None
            };
            render_xml_part(
                &part_files,
                config,
                pack_root,
                tree_string,
                header,
                git_diff_content,
                git_log_content,
                tree,
                Some(meta),
            )
        })
        .collect()
}

fn make_token_counter_fn(encoding: &str) -> impl Fn(&str) -> usize {
    let counter = TokenCounter::new(encoding).ok();
    move |text: &str| -> usize {
        if let Some(c) = &counter {
            c.count_tokens(text)
        } else {
            estimate_tokens_fallback(text)
        }
    }
}

fn split_lines_by_tokens(
    content: &str,
    threshold: usize,
    count_tokens: &dyn Fn(&str) -> usize,
) -> Vec<String> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return vec![content.to_string()];
    }

    let mut parts: Vec<String> = Vec::new();
    let mut current_lines: Vec<&str> = Vec::new();
    let mut current_tokens: usize = 0;

    for line in &lines {
        let line_tokens = count_tokens(line);

        if current_tokens + line_tokens > threshold && !current_lines.is_empty() {
            parts.push(current_lines.join("\n"));
            current_lines.clear();
            current_tokens = 0;
        }

        current_lines.push(*line);
        current_tokens += line_tokens;
    }

    if !current_lines.is_empty() {
        parts.push(current_lines.join("\n"));
    }

    if parts.is_empty() {
        vec![content.to_string()]
    } else {
        parts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ENCODING: &str = "o200k_base";

    #[test]
    fn test_split_output_no_split() {
        let content = "line1\nline2\nline3";
        let result = split_output(content, 100, &OutputStyle::Xml, TEST_ENCODING);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], content);
        assert!(!result[0].contains("<file_part"));
    }

    #[test]
    fn test_split_output_with_split() {
        let content = (0..200)
            .map(|i| format!("word{}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let result = split_output(&content, 50, &OutputStyle::Xml, TEST_ENCODING);
        assert!(result.len() > 1);
    }

    #[test]
    fn test_split_xml_by_files_produces_balanced_structure() {
        use repomix_config::schema::RepomixConfig;
        use repomix_shared::types::ProcessedFile;
        use std::path::PathBuf;

        let files: Vec<ProcessedFile> = (0..8)
            .map(|i| ProcessedFile {
                path: PathBuf::from(format!("src/file_{}.rs", i)),
                content: format!(
                    "fn func_{i}() {{\n{body}\n}}\n",
                    body = "let x = 1;\n".repeat(40 + i * 5)
                ),
                token_count: 50 + i * 10,
            })
            .collect();

        let config = RepomixConfig::default();
        let header = OutputHeader {
            header_text: None,
            instruction_content: None,
        };
        let parts = split_xml_by_files(
            &files,
            &config,
            Path::new("."),
            "src/\n",
            &header,
            &None,
            &None,
            None,
            200,
            TEST_ENCODING,
        );

        assert!(
            parts.len() > 1,
            "expected multiple XML parts, got {}",
            parts.len()
        );
        for (i, part) in parts.iter().enumerate() {
            assert_eq!(
                part.matches("<files>").count(),
                1,
                "part {} should have exactly one <files> open tag",
                i + 1
            );
            assert_eq!(
                part.matches("</files>").count(),
                1,
                "part {} should have exactly one </files> close tag",
                i + 1
            );
            assert_eq!(
                part.matches("<file ").count() + part.matches("<file>").count(),
                part.matches("</file>").count(),
                "part {} should have balanced <file> tags",
                i + 1
            );
        }
        assert!(
            parts[0].contains("<file_summary>"),
            "first part should include file_summary"
        );
        assert!(
            !parts[1].contains("<file_summary>"),
            "continuation parts should not repeat file_summary"
        );
        assert!(
            parts[1].contains("<split_info"),
            "continuation parts should include split_info"
        );
    }

    #[test]
    fn test_split_output_json_keeps_valid_json() {
        let content = (0..200)
            .map(|i| format!("word{}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let result = split_output(&content, 50, &OutputStyle::Json, TEST_ENCODING);
        assert_eq!(result.len(), 1);
        let parsed: serde_json::Value =
            serde_json::from_str(&result[0]).expect("split_output JSON 风格必须输出合法 JSON");
        assert!(parsed.is_array());
        assert!(parsed.as_array().unwrap().len() > 1);
    }

    #[test]
    fn test_split_output_markdown_uses_html_comment() {
        let content = (0..200)
            .map(|i| format!("word{}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let result = split_output(&content, 50, &OutputStyle::Markdown, TEST_ENCODING);
        assert!(result.len() > 1);
        for part in &result {
            assert!(part.contains("<!-- file_part"));
        }
    }

    #[test]
    fn test_split_output_single_line_exceeds_threshold() {
        let content = "x ".repeat(200);
        let result = split_output(&content, 50, &OutputStyle::Xml, TEST_ENCODING);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], content);
    }
}
