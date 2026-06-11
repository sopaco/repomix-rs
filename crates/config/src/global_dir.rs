use anyhow::Result;
use std::path::PathBuf;

/// 获取全局配置目录
pub fn global_config_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

    let config_dir = home.join(".repomix");

    // 如果目录不存在，则创建
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)?;
    }

    Ok(config_dir)
}

/// 获取全局配置文件路径
pub fn global_config_path() -> Result<PathBuf> {
    Ok(global_config_dir()?.join("repomix.config.json"))
}

/// MCP / API 持久化输出目录（`~/.repomix/outputs/`）
pub fn mcp_outputs_dir() -> Result<PathBuf> {
    let dir = global_config_dir()?.join("outputs");
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

/// 获取全局缓存目录
pub fn global_cache_dir() -> Result<PathBuf> {
    let cache_dir =
        dirs::cache_dir().ok_or_else(|| anyhow::anyhow!("Could not find cache directory"))?;

    let repomix_cache = cache_dir.join("repomix");

    if !repomix_cache.exists() {
        std::fs::create_dir_all(&repomix_cache)?;
    }

    Ok(repomix_cache)
}
