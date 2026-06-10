use std::path::Path;
use std::process::Command;

use anyhow::Result;

/// Git Diff结果
pub struct GitDiffResult {
    pub work_tree: String,
    pub staged: String,
}

/// 获取Git Diff
pub fn get_git_diffs(repo_path: &Path) -> Result<GitDiffResult> {
    let work_tree = run_git_diff(repo_path, &["diff"])?;
    let staged = run_git_diff(repo_path, &["diff", "--cached"])?;
    Ok(GitDiffResult { work_tree, staged })
}

fn run_git_diff(repo_path: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
