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

    let suspicious: Vec<SuspiciousFileResult> = raw_files
        .par_iter()
        .flat_map(|file| scan_file_content(&file.content, &file.path))
        .collect();

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
