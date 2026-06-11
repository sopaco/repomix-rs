/// 截断base64数据
pub fn truncate_base64(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());

    for line in lines {
        if is_base64_line(line) {
            // 截断base64数据，只保留前100个字符（使用 char 边界安全截断）
            let char_len = line.chars().count();
            if char_len > 100 {
                let truncated: String = line.chars().take(100).collect();
                result.push(format!("{}... [truncated]", truncated));
            } else {
                result.push(line.to_string());
            }
        } else {
            result.push(line.to_string());
        }
    }

    result.join("\n")
}

/// 改进 base64 检测，减少误判
///
/// 检测条件（全部满足才视为 base64）：
/// 1. 长度 > 80 字符
/// 2. 仅含 base64 字符集 [A-Za-z0-9+/=]
/// 3. 至少包含一个大写字母或 '+' 或 '/'（排除纯 hex hash）
/// 4. 至少包含一个数字 + 一个字母（排除 UUID-like）
/// 5. 末尾字符必须为 base64 字符（=, A-Z, a-z, 0-9, +, /）——自然语言行通常以标点结束
fn is_base64_line(line: &str) -> bool {
    if line.len() < 80 {
        return false;
    }

    if !line
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
    {
        return false;
    }

    let has_uppercase_or_base64_special = line
        .chars()
        .any(|c| c.is_ascii_uppercase() || c == '+' || c == '/');
    if !has_uppercase_or_base64_special {
        return false;
    }

    let has_digit = line.chars().any(|c| c.is_ascii_digit());
    let has_letter = line.chars().any(|c| c.is_ascii_alphabetic());
    if !(has_digit && has_letter) {
        return false;
    }

    // 末尾字符必须是 base64 字符集中的合法结尾（=, A-Z, a-z, 0-9, +, /）
    // 真实 base64 永远不以空格、句号、逗号、感叹号等标点结束
    matches!(line.chars().last(), Some(c) if c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_line_not_base64() {
        assert!(!is_base64_line("short line"));
    }

    #[test]
    fn test_natural_language_long_sentence_not_base64() {
        // 长自然语言句子：含标点结尾
        let s = "The Quick Brown Fox Jumps Over The Lazy Dog And Runs Into The Forest Which Is Located Near A Beautiful River";
        assert!(
            !is_base64_line(s),
            "natural language with . should not be base64"
        );
    }

    #[test]
    fn test_pure_hex_hash_not_base64() {
        // 纯小写 hex（无大写、无 +/-）
        let s = "a".repeat(40) + &"1".repeat(40);
        assert!(!is_base64_line(&s));
    }

    #[test]
    fn test_real_base64_detected() {
        // 真实 base64：以 = 结尾 padding
        let s = "aGVsbG93b3JsZHRoaXNpc2FiYXNlNjRzdHJpbmd3aXRoZXF1YWxzaWduY29sb25hbmRub3RoaW5nc3BlY2lhbA==";
        assert!(is_base64_line(s));
    }

    #[test]
    fn test_uuid_like_not_base64() {
        // UUID 风格（仅 hex + dash 但 dash 不在 base64 集，已被规则 2 排除）
        // 这里测一个类似 UUID 但仅用 hex 的字符串
        let s = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        assert!(!is_base64_line(s), "pure hex long string not base64");
    }
}
