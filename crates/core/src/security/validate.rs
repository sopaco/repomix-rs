use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::Result;
use rayon::prelude::*;
use repomix_config::schema::RepomixConfig;
use repomix_shared::types::*;

use crate::security::secretlint::scan_file_content;

pub fn validate_file_safety(
    raw_files: &[RawFile],
    config: &RepomixConfig,
) -> Result<ValidationResult> {
    if !config.security.enable_secretlint {
        return Ok(ValidationResult {
            suspicious: Vec::new(),
            safe_paths: raw_files.iter().map(|f| f.path.clone()).collect(),
        });
    }

    // P1 修复（Bug #6）：scan_file_content 内部使用 OnceLock 全局缓存，
    // 避免 rayon worker 重复构造 7 个 regex
    let suspicious: Vec<SuspiciousFileResult> = raw_files
        .par_iter()
        .flat_map(|file| scan_file_content(&file.content, &file.path))
        .collect();

    // P1 修复（Bug #4）：用 HashSet 做 O(1) 查找替代 Vec::contains O(N×M)
    let suspicious_paths: HashSet<&PathBuf> = suspicious.iter().map(|s| &s.path).collect();
    let safe_paths: Vec<PathBuf> = raw_files
        .iter()
        .filter(|f| !suspicious_paths.contains(&f.path))
        .map(|f| f.path.clone())
        .collect();

    Ok(ValidationResult {
        suspicious,
        safe_paths,
    })
}
