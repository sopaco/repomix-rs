use tiktoken_rs::CoreBPE;

pub struct TokenCounter {
    encoder: CoreBPE,
}

impl TokenCounter {
    /// 创建 token 计数器
    ///
    /// 支持的 encoding：
    /// - `o200k_base`（默认，gpt-4o）
    /// - `cl100k_base`（gpt-4 / gpt-3.5-turbo）
    /// - `p50k_base`（legacy Codex / text-embedding-ada-002）
    /// - `p50k_edit`（edit 模型）
    /// - `r50k_base`（gpt-3）
    ///
    /// 未识别的 encoding 名称 fallback 到 `o200k_base` 并打 warning。
    pub fn new(encoding: &str) -> Result<Self, anyhow::Error> {
        let encoder = match encoding {
            "o200k_base" => tiktoken_rs::o200k_base()?,
            "cl100k_base" => tiktoken_rs::cl100k_base()?,
            "p50k_base" => tiktoken_rs::p50k_base()?,
            "p50k_edit" => tiktoken_rs::p50k_edit()?,
            "r50k_base" => tiktoken_rs::r50k_base()?,
            "gpt-4o" | "gpt-4" | "gpt-3.5-turbo" | "gpt-3" => {
                // 友好别名
                tiktoken_rs::get_bpe_from_model(encoding)?
            }
            other => {
                tracing::warn!(
                    "Unknown token encoding '{}', falling back to 'o200k_base'. \
                     Supported: o200k_base, cl100k_base, p50k_base, p50k_edit, r50k_base.",
                    other
                );
                tiktoken_rs::o200k_base()?
            }
        };
        Ok(Self { encoder })
    }

    pub fn count_tokens(&self, text: &str) -> usize {
        self.encoder.encode_ordinary(text).len()
    }
}

pub fn create_default_token_counter() -> Result<TokenCounter, anyhow::Error> {
    TokenCounter::new("o200k_base")
}

/// tiktoken 不可用时的降级估算：CJK 按字符计 token，其余按空白分词。
pub fn estimate_tokens_fallback(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    let cjk_count = text.chars().filter(|c| is_cjk(*c)).count();
    let non_cjk: String = text.chars().filter(|c| !is_cjk(*c)).collect();
    let word_tokens = non_cjk
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .count();
    (cjk_count + word_tokens).max(1)
}

fn is_cjk(ch: char) -> bool {
    matches!(
        ch,
        '\u{4E00}'..='\u{9FFF}'
            | '\u{3400}'..='\u{4DBF}'
            | '\u{3040}'..='\u{30FF}'
            | '\u{AC00}'..='\u{D7AF}'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_encoding_works() {
        let counter = TokenCounter::new("o200k_base").expect("o200k_base should init");
        let count = counter.count_tokens("hello world");
        assert_eq!(count, 2);
    }

    #[test]
    fn test_cl100k_encoding_works() {
        let counter = TokenCounter::new("cl100k_base").expect("cl100k_base should init");
        let count = counter.count_tokens("hello world");
        // cl100k 编码 "hello world" 也是 2 token
        assert_eq!(count, 2);
    }

    #[test]
    fn test_unknown_encoding_falls_back() {
        // 未知 encoding 应 fallback 到 o200k_base 而非 panic
        let counter = TokenCounter::new("not-a-real-encoding").expect("should fall back");
        let count = counter.count_tokens("hello world");
        assert_eq!(count, 2);
    }

    #[test]
    fn test_count_chinese() {
        let counter = TokenCounter::new("o200k_base").expect("init");
        // 中文每个字大约 1-3 token
        let count = counter.count_tokens("你好世界");
        assert!(count > 0 && count <= 12);
    }

    #[test]
    fn test_estimate_tokens_fallback_cjk() {
        // 纯中文不应被 whitespace 估算为 0
        let count = estimate_tokens_fallback("你好世界");
        assert_eq!(count, 4);
    }

    #[test]
    fn test_estimate_tokens_fallback_mixed() {
        let count = estimate_tokens_fallback("hello 你好");
        assert!(count >= 3);
    }
}
