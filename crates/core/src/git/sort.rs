use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use anyhow::Result;
use repomix_shared::types::ProcessedFile;

use crate::path_util::{git_relative_path, git_repo_root};

/// 获取文件的变更频率（基于 git log）
pub fn get_file_change_counts(repo_path: &Path, max_commits: usize) -> Result<HashMap<String, usize>> {
    let output = Command::new("git")
        .args([
            "log",
            &format!("-{}", max_commits),
            "--pretty=format:",
            "--name-only",
        ])
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "git log failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let mut change_counts: HashMap<String, usize> = HashMap::new();

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let line = line.trim().to_string();
        if !line.is_empty() {
            *change_counts.entry(line).or_insert(0) += 1;
        }
    }

    Ok(change_counts)
}

/// 按 git 变更频率排序文件（最常变更的排在前面）
pub fn sort_by_git_changes(
    files: &mut [ProcessedFile],
    repo_path: &Path,
    max_commits: usize,
) -> Result<()> {
    let change_counts = get_file_change_counts(repo_path, max_commits)?;
    let repo_root = git_repo_root(repo_path);

    files.sort_by(|a, b| {
        let key_a = git_relative_path(&a.path, &repo_root);
        let key_b = git_relative_path(&b.path, &repo_root);
        let count_a = change_counts.get(&key_a).unwrap_or(&0);
        let count_b = change_counts.get(&key_b).unwrap_or(&0);
        count_b.cmp(count_a)
    });

    Ok(())
}
