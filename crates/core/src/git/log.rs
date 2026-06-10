use std::path::Path;
use std::process::Command;
use anyhow::Result;

/// Git Log结果
pub struct GitLogResult {
    pub logs: Vec<String>,
}

/// 获取Git Log
pub fn get_git_logs(repo_path: &Path, max_count: usize) -> Result<GitLogResult> {
    let output = Command::new("git")
        .args(["log", &format!("-{}", max_count), "--oneline"])
        .current_dir(repo_path)
        .output()?;
    
    let logs = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.to_string())
        .collect();
    
    Ok(GitLogResult { logs })
}