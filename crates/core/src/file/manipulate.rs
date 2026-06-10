use std::path::Path;

/// 移除注释
pub fn remove_comments(content: &str, file_path: &Path) -> String {
    // 根据文件扩展名选择注释移除策略
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    match extension {
        "js" | "jsx" | "ts" | "tsx" | "java" | "c" | "cpp" | "h" | "hpp" |
        "cs" | "go" | "rs" | "swift" | "kt" | "scala" => {
            remove_c_style_comments(content)
        }
        "py" | "rb" | "yml" | "yaml" | "toml" | "ini" | "cfg" | "sh" |
        "bash" | "zsh" | "fish" | "r" | "pl" | "pm" => {
            remove_hash_comments(content)
        }
        "html" | "xml" | "svg" => {
            remove_html_comments(content)
        }
        "css" | "scss" | "less" => {
            remove_css_comments(content)
        }
        _ => content.to_string(),
    }
}

/// 移除C风格注释
///
/// 支持 Rust raw strings (`r#"..."#`)、byte strings (`b"..."`)，
/// 避免注释移除破坏字符串内容。
fn remove_c_style_comments(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();
    let mut in_string = false;
    let mut string_char = ' ';
    let mut escaped = false;

    while let Some(c) = chars.next() {
        if in_string {
            result.push(c);
            if escaped {
                // 前一个字符是 `\`，当前字符被转义，重置 escaped 状态
                escaped = false;
            } else if c == '\\' {
                // 当前字符是 `\`，标记下一个字符被转义
                escaped = true;
            } else if c == string_char {
                // 检查是否为 raw string 的闭合引号（后跟 #）
                if string_char == '"' {
                    let hash_count = count_trailing_hashes(&result);
                    if hash_count > 0 {
                        // 检查是否有足够的 # 来闭合 raw string
                        if result.ends_with(&"#".repeat(hash_count)) {
                            in_string = false;
                        }
                    } else {
                        in_string = false;
                    }
                } else {
                    in_string = false;
                }
            }
            continue;
        }

        match c {
            'r' | 'b' => {
                // 检查是否为 Rust raw string 或 byte string
                if let Some(rest) = try_parse_string_literal(&mut chars, c) {
                    result.push_str(&rest);
                } else {
                    result.push(c);
                }
            }
            '"' | '\'' => {
                in_string = true;
                string_char = c;
                escaped = false;
                result.push(c);
            }
            '/' => {
                if let Some(&next) = chars.peek() {
                    if next == '/' {
                        // 单行注释
                        for c in chars.by_ref() {
                            if c == '\n' {
                                result.push('\n');
                                break;
                            }
                        }
                    } else if next == '*' {
                        // 多行注释
                        chars.next(); // 跳过'*'
                        let mut prev = ' ';
                        for c in chars.by_ref() {
                            if prev == '*' && c == '/' {
                                break;
                            }
                            prev = c;
                        }
                    } else {
                        result.push(c);
                    }
                } else {
                    result.push(c);
                }
            }
            _ => result.push(c),
        }
    }

    result
}

/// 计算字符串中尾部连续 # 的数量
fn count_trailing_hashes(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut count = 0;
    let mut i = bytes.len();
    while i > 0 && bytes[i - 1] == b'#' {
        count += 1;
        i -= 1;
    }
    count
}

/// 尝试解析字符串字面量（支持 Rust raw strings 和 byte strings）
///
/// 如果成功解析，将消耗的字符追加到已读取的字符（包括前缀）并返回完整字符串。
/// 如果不是字符串字面量，返回 None（不消耗任何字符）。
fn try_parse_string_literal(chars: &mut std::iter::Peekable<std::str::Chars>, prefix: char) -> Option<String> {
    let mut result = String::new();
    result.push(prefix);

    // 检查是否为 raw string: r"...", r#"..."#, r##"..."## 等
    if prefix == 'r' {
        if let Some(&'"') = chars.peek() {
            // r"..." 或 r#"..."#
            result.push(chars.next().unwrap());

            // 计算 # 的数量
            let mut hash_count = 0;
            while let Some(&'#') = chars.peek() {
                result.push(chars.next().unwrap());
                hash_count += 1;
            }

            // 查找匹配的闭合
            let closing = format!("\"{}", "#".repeat(hash_count));
            for c in chars.by_ref() {
                result.push(c);
                if result.ends_with(&closing) && result.len() > closing.len() {
                    break;
                }
            }

            return Some(result);
        }
    }

    // 检查是否为 byte string: b"...", br"...", br#"..."# 等
    if prefix == 'b' {
        let next_char = chars.peek().copied();
        match next_char {
            Some('"') => {
                // b"..."
                result.push(chars.next().unwrap());
                let mut escaped = false;
                for c in chars.by_ref() {
                    result.push(c);
                    if escaped {
                        escaped = false;
                    } else if c == '\\' {
                        escaped = true;
                    } else if c == '"' {
                        return Some(result);
                    }
                }
                return None;
            }
            Some('r') => {
                // br"..." 或 br#"..."#
                result.push(chars.next().unwrap());
                if let Some(&'"') = chars.peek() {
                    result.push(chars.next().unwrap());

                    let mut hash_count = 0;
                    while let Some(&'#') = chars.peek() {
                        result.push(chars.next().unwrap());
                        hash_count += 1;
                    }

                    let closing = format!("\"{}", "#".repeat(hash_count));
                    for c in chars.by_ref() {
                        result.push(c);
                        if result.ends_with(&closing) && result.len() > closing.len() + 1 {
                            break;
                        }
                    }
                    return Some(result);
                }
            }
            _ => {}
        }
    }

    None
}

/// 移除#注释
fn remove_hash_comments(content: &str) -> String {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                // 整行是注释，移除整行（不保留空行）
                None
            } else {
                // 保留非注释行，但移除行内注释
                let mut result = String::with_capacity(line.len());
                let mut in_string = false;
                let mut string_char = ' ';

                for c in line.chars() {
                    if in_string {
                        result.push(c);
                        if c == string_char {
                            in_string = false;
                        }
                        continue;
                    }

                    match c {
                        '"' | '\'' => {
                            in_string = true;
                            string_char = c;
                            result.push(c);
                        }
                        '#' => {
                            // 行内注释，跳过剩余部分
                            break;
                        }
                        _ => result.push(c),
                    }
                }
                Some(result)
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// 移除HTML注释
fn remove_html_comments(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '<' && chars.peek() == Some(&'!') {
            // 检查是否为注释开始
            let mut comment_start = String::new();
            comment_start.push(c);
            comment_start.push(chars.next().unwrap());

            // 检查是否为<!--
            if chars.peek() == Some(&'-') {
                comment_start.push(chars.next().unwrap());
                if chars.peek() == Some(&'-') {
                    comment_start.push(chars.next().unwrap());

                    // 读取注释内容直到-->
                    let mut comment_body = String::new();
                    let mut prev = ' ';
                    let mut prev2 = ' ';
                    let mut closed = false;
                    for c in chars.by_ref() {
                        if prev2 == '-' && prev == '-' && c == '>' {
                            closed = true;
                            break;
                        }
                        comment_body.push(c);
                        prev2 = prev;
                        prev = c;
                    }
                    // 如果注释未闭合，将已消耗的内容恢复到结果中
                    if !closed {
                        result.push_str(&comment_start);
                        result.push_str(&comment_body);
                    }
                    // 闭合的注释被跳过
                    continue;
                }
            }

            // 不是注释，添加到结果
            result.push_str(&comment_start);
        } else {
            result.push(c);
        }
    }

    result
}

/// 移除CSS注释
fn remove_css_comments(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '/' && chars.peek() == Some(&'*') {
            // 跳过注释开始
            chars.next(); // 跳过'*'
            let mut comment_body = String::new();
            let mut prev = ' ';
            let mut closed = false;
            for c in chars.by_ref() {
                if prev == '*' && c == '/' {
                    closed = true;
                    break;
                }
                comment_body.push(c);
                prev = c;
            }
            // 如果注释未闭合，将已消耗的内容恢复到结果中
            if !closed {
                result.push('/');
                result.push('*');
                result.push_str(&comment_body);
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// 移除空行
pub fn remove_empty_lines(content: &str) -> String {
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n")
}

/// 去除首尾空白
pub fn trim_content(content: &str) -> String {
    content.trim().to_string()
}

/// 添加行号
pub fn add_line_numbers(content: &str) -> String {
    content
        .lines()
        .enumerate()
        .map(|(i, line)| format!("{}: {}", i + 1, line))
        .collect::<Vec<String>>()
        .join("\n")
}