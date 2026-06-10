use std::path::Path;

use tree_sitter::{Parser, QueryCursor};
use streaming_iterator::StreamingIterator;

use crate::tree_sitter::languages::LanguageConfig;

/// 压缩单个文件：使用 tree-sitter 提取捕获节点，输出保留源码顺序的签名列表
///
/// P1 修复（Bug #5）：用 `cursor.captures(...)` 替代 `cursor.matches(...)`，
/// tree-sitter 的 captures API 按源码位置**严格有序**迭代捕获节点，
/// 避免当 query 包含多个 capture 名时不同 match 之间乱序。
pub fn compress_file(
    content: &str,
    file_path: &Path,
    config: &LanguageConfig,
) -> Result<Option<String>, anyhow::Error> {
    let mut parser = Parser::new();
    parser.set_language(&config.language)?;

    let tree = parser
        .parse(content, None)
        .ok_or_else(|| anyhow::anyhow!("Parse failed for {}", file_path.display()))?;

    let query = match config.compress_query.as_ref() {
        Some(q) => q,
        None => return Ok(None),
    };

    let mut cursor = QueryCursor::new();
    // captures() 按源码顺序返回 (QueryMatch, capture_index) 元组
    // (tree-sitter 文档保证 captures 顺序严格按源码位置)
    let mut captures = cursor.captures(query, tree.root_node(), content.as_bytes());

    let mut parts: Vec<String> = Vec::new();
    // 用 last_byte_end 跳过同一位置的重复 capture（tree-sitter captures() 可能对
    // 嵌套 capture 返回多次相同位置节点）
    let mut last_byte_end: usize = 0;

    while let Some((m, capture_index)) = captures.next() {
        // m.captures 是该 match 内所有 capture 的列表
        // capture_index 指向该 match 内的具体 capture
        let node = m.captures[*capture_index].node;
        let range = node.byte_range();

        // 跳过空范围或重复
        if range.start >= range.end {
            continue;
        }
        if range.start < last_byte_end {
            // 与上一个 capture 重叠（嵌套），跳过
            continue;
        }
        last_byte_end = range.end;

        // 安全切片：byte_range 一定在 content 范围内（来自同一棵树）
        let text = &content[range];
        parts.push(text.to_string());
    }

    if parts.is_empty() {
        return Ok(None);
    }

    Ok(Some(parts.join("\n⋮----\n")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_sitter::languages::get_language_config;
    use std::path::Path;

    #[test]
    fn test_compress_rust_preserves_order() {
        let content = r#"
fn alpha() { println!("first"); }
fn beta() { println!("second"); }
fn gamma() { println!("third"); }
"#;
        let config = get_language_config(Path::new("test.rs")).expect("rust config");
        let result = compress_file(content, Path::new("test.rs"), config)
            .expect("compress ok")
            .expect("should produce output");

        let alpha_pos = result.find("alpha").expect("alpha in result");
        let beta_pos = result.find("beta").expect("beta in result");
        let gamma_pos = result.find("gamma").expect("gamma in result");

        assert!(
            alpha_pos < beta_pos && beta_pos < gamma_pos,
            "captures must follow source order: alpha={} beta={} gamma={}",
            alpha_pos, beta_pos, gamma_pos
        );
    }

    #[test]
    fn test_compress_empty_file_returns_none() {
        let content = "";
        let config = get_language_config(Path::new("test.rs")).expect("rust config");
        let result = compress_file(content, Path::new("test.rs"), config)
            .expect("compress ok");
        assert!(result.is_none());
    }
}
