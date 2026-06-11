use std::path::Path;
use std::process::Command;
use anyhow::Result;

/// 克隆远程仓库
pub fn clone_remote_repo(url: &str, target_dir: &Path) -> Result<()> {
    let status = Command::new("git")
        .args(["clone", url, target_dir.to_str().unwrap()])
        .status()?;
    
    if !status.success() {
        anyhow::bail!("Failed to clone repository: {}", url);
    }
    
    Ok(())
}

/// 检查是否为 Git 仓库（`rev-parse --show-toplevel`，子目录中也能正确检测）
pub fn is_git_repo(path: &Path) -> bool {
    Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(path)
        .stderr(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}
