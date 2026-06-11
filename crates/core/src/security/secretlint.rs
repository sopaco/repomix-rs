use std::sync::OnceLock;

use regex::Regex;
use repomix_shared::types::SuspiciousFileResult;
use std::path::Path;

pub struct SecretRule {
    pub id: String,
    pub name: String,
    pub pattern: Regex,
    pub entropy: Option<f64>,
    pub allowlist: Vec<String>,
}

/// secret 规则全局缓存（OnceLock），避免每次扫描重新编译 Regex
pub static SECRET_RULES: OnceLock<Vec<SecretRule>> = OnceLock::new();

/// 获取 secret 规则的静态切片引用（首次调用时构造并缓存）
pub fn get_secret_rules() -> &'static [SecretRule] {
    SECRET_RULES.get_or_init(build_secret_rules)
}

fn build_secret_rules() -> Vec<SecretRule> {
    vec![
        SecretRule {
            id: "generic-api-key".to_string(),
            name: "Generic API Key".to_string(),
            pattern: safe_compile(
                r#"(?i)(api[_-]?key|apikey)\s*[:=]\s*['"][^'"]+['"]"#,
                "generic-api-key",
            ),
            // 对引号内候选值计熵；2.5 可拦截 "secret123" 等弱口令，同时过滤 "aaaa"
            entropy: Some(2.5),
            allowlist: vec![
                "your-api-key".to_string(),
                "YOUR_API_KEY".to_string(),
                "your_api_key".to_string(),
                "example-api-key".to_string(),
                "placeholder".to_string(),
                "changeme".to_string(),
                "xxx".to_string(),
                "test".to_string(),
            ],
        },
        SecretRule {
            id: "generic-secret".to_string(),
            name: "Generic Secret".to_string(),
            pattern: safe_compile(
                r#"(?i)(secret|password|passwd|pwd)\s*[:=]\s*['"][^'"]+['"]"#,
                "generic-secret",
            ),
            entropy: Some(2.5),
            allowlist: vec![
                "your-secret".to_string(),
                "YOUR_SECRET".to_string(),
                "your_secret".to_string(),
                "changeme".to_string(),
                "example".to_string(),
                "placeholder".to_string(),
                "xxx".to_string(),
                "test".to_string(),
                "default".to_string(),
                "secret_key_base".to_string(),
            ],
        },
        SecretRule {
            id: "generic-token".to_string(),
            name: "Generic Token".to_string(),
            pattern: safe_compile(
                r#"(?i)(token|access[_-]?token|auth[_-]?token)\s*[:=]\s*['"][^'"]+['"]"#,
                "generic-token",
            ),
            entropy: Some(2.5),
            allowlist: vec![
                "your-token".to_string(),
                "YOUR_TOKEN".to_string(),
                "your_token".to_string(),
                "changeme".to_string(),
                "example".to_string(),
                "placeholder".to_string(),
                "xxx".to_string(),
                "test".to_string(),
            ],
        },
        SecretRule {
            id: "aws-access-key".to_string(),
            name: "AWS Access Key".to_string(),
            pattern: safe_compile(r"AKIA[0-9A-Z]{16}", "aws-access-key"),
            entropy: None,
            allowlist: vec!["AKIAIOSFODNN7EXAMPLE".to_string()],
        },
        SecretRule {
            id: "aws-secret-key".to_string(),
            name: "AWS Secret Key".to_string(),
            pattern: safe_compile(
                r#"(?i)aws[_-]?secret[_-]?access[_-]?key\s*[:=]\s*['"][^'"]+['"]"#,
                "aws-secret-key",
            ),
            entropy: None,
            allowlist: vec![
                "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string(),
                "changeme".to_string(),
                "placeholder".to_string(),
            ],
        },
        SecretRule {
            id: "github-token".to_string(),
            name: "GitHub Token".to_string(),
            pattern: safe_compile(r"ghp_[A-Za-z0-9]{36}", "github-token"),
            entropy: None,
            allowlist: vec![],
        },
        SecretRule {
            id: "private-key".to_string(),
            name: "Private Key".to_string(),
            pattern: safe_compile(
                r"-----BEGIN (?:RSA |EC |DSA )?PRIVATE KEY-----",
                "private-key",
            ),
            entropy: None,
            allowlist: vec![],
        },
    ]
}

/// 安全编译 regex：失败时打 warning 并返回 never-match 占位
fn safe_compile(pattern: &str, name: &str) -> Regex {
    match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(
                "Invalid regex for secret rule '{}': {}. This rule will be disabled.",
                name, e
            );
            Regex::new(r"\u{FEFF}never_present_marker").unwrap_or_else(|_| {
                Regex::new("(?-u)\\A\\z").expect("hardcoded empty-anchored regex is valid")
            })
        }
    }
}

/// 跟踪 `#[test]` / `#[cfg(test)]` 块，跳过测试夹具中的假阳性。
#[derive(Default)]
struct TestRegionState {
    brace_depth: u32,
    test_block_start: Option<u32>,
    cfg_test_start: Option<u32>,
    pending_test_attr: bool,
    pending_cfg_test: bool,
}

impl TestRegionState {
    fn is_inside_test_region(&self) -> bool {
        self.test_block_start
            .is_some_and(|start| self.brace_depth >= start)
            || self.cfg_test_start
                .is_some_and(|start| self.brace_depth >= start)
    }

    fn update_for_line(&mut self, line: &str) {
        let trimmed = line.trim();
        if trimmed.starts_with("#[") {
            if trimmed.contains("#[test")
                || trimmed.contains("#[tokio::test")
                || trimmed.contains("#[async_std::test")
            {
                self.pending_test_attr = true;
            }
            if trimmed.contains("cfg(test)") {
                self.pending_cfg_test = true;
            }
        }

        let open = line.chars().filter(|&c| c == '{').count() as u32;
        let close = line.chars().filter(|&c| c == '}').count() as u32;

        if self.pending_test_attr && open > 0 {
            self.test_block_start = Some(self.brace_depth + 1);
            self.pending_test_attr = false;
        }
        if self.pending_cfg_test && open > 0 {
            self.cfg_test_start = Some(self.brace_depth + 1);
            self.pending_cfg_test = false;
        }

        self.brace_depth += open;
        self.brace_depth = self.brace_depth.saturating_sub(close);

        if let Some(start) = self.test_block_start {
            if self.brace_depth < start {
                self.test_block_start = None;
            }
        }
        if let Some(start) = self.cfg_test_start {
            if self.brace_depth < start {
                self.cfg_test_start = None;
            }
        }
    }
}

/// 从 `key = "value"` / `key: 'value'` 形式赋值中提取引号内的秘密候选值。
fn extract_assign_quoted_value(line: &str) -> Option<&str> {
    let assign_pos = line
        .char_indices()
        .find(|(_, c)| *c == '=' || *c == ':')
        .map(|(i, _)| i)?;
    let mut rest = line[assign_pos + 1..].trim_start();
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    rest = &rest[quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some(&rest[..end])
}

/// 返回用于熵计算 / allowlist 判断的候选子串（非整行）。
fn secret_candidate<'a>(line: &'a str, rule: &SecretRule) -> &'a str {
    if matches!(
        rule.id.as_str(),
        "generic-api-key" | "generic-secret" | "generic-token" | "aws-secret-key"
    ) {
        if let Some(value) = extract_assign_quoted_value(line) {
            return value;
        }
    }
    rule.pattern
        .find(line)
        .map(|m| m.as_str())
        .unwrap_or(line)
}

pub fn scan_file_content(content: &str, file_path: &Path) -> Vec<SuspiciousFileResult> {
    let rules = get_secret_rules();
    let mut results = Vec::new();
    let mut test_state = TestRegionState::default();

    for (line_num, line) in content.lines().enumerate() {
        if test_state.is_inside_test_region() {
            test_state.update_for_line(line);
            continue;
        }

        for rule in rules {
            if !rule.pattern.is_match(line) {
                continue;
            }

            let candidate = secret_candidate(line, rule);

            if rule
                .allowlist
                .iter()
                .any(|allow| candidate.contains(allow) || line.contains(allow))
            {
                continue;
            }

            if let Some(min_entropy) = rule.entropy {
                if calculate_entropy(candidate) < min_entropy {
                    continue;
                }
            }

            results.push(SuspiciousFileResult {
                path: file_path.to_path_buf(),
                line: line_num + 1,
                message: format!("Detected potential secret or credential: {}", rule.name),
                rule_id: rule.id.clone(),
            });
        }

        test_state.update_for_line(line);
    }

    results
}

pub(crate) fn calculate_entropy(s: &str) -> f64 {
    if s.is_empty() {
        return 0.0;
    }

    let mut counts = [0u32; 256];
    let len = s.len() as f64;

    for &byte in s.as_bytes() {
        counts[byte as usize] += 1;
    }

    let mut entropy = 0.0;
    for &count in &counts {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }

    entropy
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_entropy_empty() {
        assert_eq!(calculate_entropy(""), 0.0);
    }

    #[test]
    fn test_entropy_repeated_char() {
        assert_eq!(calculate_entropy("aaaa"), 0.0);
    }

    #[test]
    fn test_entropy_distributed() {
        let e = calculate_entropy("abcdefghijklmnop");
        assert!(e > 3.5, "expected >3.5, got {}", e);
    }

    #[test]
    fn test_safe_compile_invalid_returns_never_match() {
        let r = safe_compile("(unclosed", "test");
        assert!(!r.is_match("anything"));
    }

    #[test]
    fn test_allowlist_skips_placeholder() {
        let content = r#"api_key = "your-api-key""#;
        let results = scan_file_content(content, Path::new("test.txt"));
        assert!(results.is_empty(), "placeholder should be allowlisted");
    }

    #[test]
    fn test_entropy_filter_blocks_low_entropy() {
        let content = r#"api_key = "aaaa""#;
        let results = scan_file_content(content, Path::new("test.txt"));
        assert!(
            !results.iter().any(|r| r.rule_id == "generic-api-key"),
            "low entropy value should be filtered out, got: {:?}",
            results
        );
    }

    #[test]
    fn test_test_function_body_skipped() {
        let content = r##"
fn production() {
    api_key = "sk-1234567890abcdef1234567890abcdef"
}

#[test]
fn test_fixture() {
    let x = r#"api_key = "sk-1234567890abcdef1234567890abcdef""#;
}
"##;
        let results = scan_file_content(content, Path::new("lib.rs"));
        assert_eq!(results.len(), 1, "only production code should match: {:?}", results);
        assert_eq!(results[0].rule_id, "generic-api-key");
    }

    #[test]
    fn test_cfg_test_module_skipped() {
        let content = r##"
fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn t() {
        let content = r#"api_key = "aaaa""#;
    }
}
"##;
        let results = scan_file_content(content, Path::new("secretlint.rs"));
        assert!(results.is_empty(), "cfg(test) module should be skipped: {:?}", results);
    }

    #[test]
    fn test_scanner_source_no_false_positive() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/security/secretlint.rs");
        let content = fs::read_to_string(&path).expect("read secretlint.rs");
        let results = scan_file_content(&content, &path);
        assert!(
            results.is_empty(),
            "secretlint.rs should not flag itself, got: {:?}",
            results
        );
    }

    #[test]
    fn test_integration_test_fixtures_skipped() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/integration_test.rs");
        let content = fs::read_to_string(&path).expect("read integration_test.rs");
        let results = scan_file_content(&content, &path);
        assert!(
            results.is_empty(),
            "integration_test.rs fixtures should be skipped, got: {:?}",
            results
        );
    }
}
