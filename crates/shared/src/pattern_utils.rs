use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::Path;

/// 构建glob匹配器
pub fn build_glob_matcher(patterns: &[String]) -> Result<GlobSet, globset::Error> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(Glob::new(pattern)?);
    }
    builder.build()
}

/// 检查路径是否匹配给定的glob模式
pub fn matches_glob_pattern(path: &Path, patterns: &[String]) -> bool {
    if patterns.is_empty() {
        return false;
    }
    
    let matcher = build_glob_matcher(patterns).unwrap_or_default();
    matcher.is_match(path)
}

/// 将逗号分隔的模式字符串转换为Vec<String>
pub fn parse_patterns(pattern_str: Option<&str>) -> Vec<String> {
    pattern_str
        .map(|s| {
            s.split(',')
                .map(|p| p.trim().to_string())
                .filter(|p| !p.is_empty())
                .collect()
        })
        .unwrap_or_default()
}