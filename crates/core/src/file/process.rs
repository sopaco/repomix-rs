use anyhow::Result;
use rayon::prelude::*;
use repomix_config::schema::RepomixConfig;
use repomix_shared::types::*;
use crate::file::process_content::ProcessContentOptions;
use crate::file::truncate_base64::truncate_base64;
use crate::file::manipulate::remove_comments;
use crate::file::manipulate::remove_empty_lines;
use crate::file::manipulate::trim_content;
use crate::file::manipulate::add_line_numbers;
use crate::metrics::token_count::TokenCounter;

/// 处理文件内容
pub fn process_files(
    raw_files: &[RawFile],
    config: &RepomixConfig,
) -> Result<Vec<ProcessedFile>> {
    let options = ProcessContentOptions::from_config(config);

    // 创建 token 计数器
    // P1 修复（Bug #5）：`tiktoken_rs::o200k_base()` 等初始化需要下载 vocabulary 文件
    // （tiktoken-rs 0.5 内置），在离线 / 受限网络 / CI 环境下可能失败。失败时
    // 当前会静默降级到 `split_whitespace()`，对 CJK 等无空白分隔的语言估算偏差巨大
    // （可能低 10-50 倍）。现在失败时打印明确 warning，让用户知道 token 统计不准。
    let token_counter = match TokenCounter::new(&config.token_count.encoding) {
        Ok(c) => Some(c),
        Err(e) => {
            tracing::warn!(
                "Failed to initialize token counter for encoding '{}': {}. \
                 Falling back to whitespace-based estimate, which severely underestimates \
                 CJK (Chinese/Japanese/Korean) text. Top-N file ranking may be unreliable.",
                config.token_count.encoding, e
            );
            None
        }
    };

    // 并行处理文件
    let processed: Vec<ProcessedFile> = raw_files
        .par_iter()
        .map(|raw_file| {
            process_single_file(raw_file, &options, token_counter.as_ref())
        })
        .collect();

    Ok(processed)
}

/// 处理单个文件
fn process_single_file(raw_file: &RawFile, options: &ProcessContentOptions, token_counter: Option<&TokenCounter>) -> ProcessedFile {
    // 检查是否需要任何变换，避免不必要的 clone
    let needs_transform = options.remove_comments
        || options.compress
        || options.truncate_base64
        || options.remove_empty_lines
        || options.show_line_numbers;

    if !needs_transform {
        // 无变换，直接使用原始内容（clone 在构造 ProcessedFile 时发生）
        let token_count = count_tokens(&raw_file.content, token_counter);
        return ProcessedFile {
            path: raw_file.path.clone(),
            content: raw_file.content.clone(),
            token_count,
        };
    }

    let mut content = raw_file.content.clone();

    // 阶段1: 重型变换（CPU密集，rayon并行）
    if options.remove_comments {
        content = remove_comments(&content, &raw_file.path);
    }

    if options.compress {
        if let Some(lang_config) = crate::tree_sitter::languages::get_language_config(&raw_file.path) {
            match crate::tree_sitter::compress::compress_file(&content, &raw_file.path, lang_config) {
                Ok(Some(compressed)) => content = compressed,
                Ok(None) => {}
                Err(e) => {
                    tracing::warn!(
                        "Tree-sitter compression failed for {}: {}. Using original content.",
                        raw_file.path.display(),
                        e
                    );
                }
            }
        }
    }

    // 阶段2: 轻量变换（顺序敏感，串行执行）
    if options.truncate_base64 {
        content = truncate_base64(&content);
    }

    if options.remove_empty_lines {
        content = remove_empty_lines(&content);
    }

    content = trim_content(&content);

    if options.show_line_numbers {
        content = add_line_numbers(&content);
    }

    // 计算token数量
    let token_count = count_tokens(&content, token_counter);

    ProcessedFile {
        path: raw_file.path.clone(),
        content,
        token_count,
    }
}

/// 计算token数量
fn count_tokens(content: &str, token_counter: Option<&TokenCounter>) -> usize {
    if let Some(counter) = token_counter {
        counter.count_tokens(content)
    } else {
        crate::metrics::token_count::estimate_tokens_fallback(content)
    }
}