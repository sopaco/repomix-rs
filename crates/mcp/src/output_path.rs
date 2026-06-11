//! MCP 输出路径生成与访问校验。

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use repomix_config::schema::OutputStyle;
use rmcp::model::ErrorData;

/// MCP 输出文件默认保留时长（7 天）。
pub const MCP_OUTPUT_TTL_SECS: u64 = 7 * 24 * 60 * 60;

/// MCP pack 产物的唯一 ID 与磁盘路径。
#[derive(Debug, Clone)]
pub struct McpOutputRef {
    pub output_id: String,
    pub path: PathBuf,
}

fn style_extension(s: &OutputStyle) -> &'static str {
    match s {
        OutputStyle::Xml => "xml",
        OutputStyle::Markdown => "md",
        OutputStyle::Json => "json",
        OutputStyle::Plain => "txt",
    }
}

/// 删除 `~/.repomix/outputs/` 中超过 TTL 的旧文件（best-effort）。
pub fn cleanup_stale_mcp_outputs() {
    let Ok(dir) = repomix_config::global_dir::mcp_outputs_dir() else {
        return;
    };
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return;
    };
    let now = SystemTime::now();
    for entry in entries.flatten() {
        let Ok(meta) = entry.metadata() else {
            continue;
        };
        if !meta.is_file() {
            continue;
        }
        let Ok(modified) = meta.modified() else {
            continue;
        };
        let Ok(age) = now.duration_since(modified) else {
            continue;
        };
        if age.as_secs() > MCP_OUTPUT_TTL_SECS
            && let Err(e) = std::fs::remove_file(entry.path())
        {
            tracing::warn!(
                "Failed to remove stale MCP output '{}': {}",
                entry.path().display(),
                e
            );
        }
    }
}

/// 在 `~/.repomix/outputs/` 下创建唯一输出路径。
pub fn make_mcp_output_path(style: &OutputStyle) -> Result<McpOutputRef, ErrorData> {
    cleanup_stale_mcp_outputs();

    let dir = repomix_config::global_dir::mcp_outputs_dir()
        .map_err(|e| ErrorData::internal_error(format!("create mcp outputs dir: {}", e), None))?;
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let output_id = format!("pack_{}_{}", std::process::id(), nanos);
    let path = dir.join(format!("{}.{}", output_id, style_extension(style)));
    Ok(McpOutputRef { output_id, path })
}

/// 校验路径位于 MCP 输出目录内，返回 canonical 路径。
pub fn validate_mcp_output_path(path: &str) -> Result<PathBuf, ErrorData> {
    let path = Path::new(path.trim());
    if path.as_os_str().is_empty() {
        return Err(ErrorData::invalid_params("file_path is empty", None));
    }

    let canonical = path.canonicalize().map_err(|e| {
        ErrorData::invalid_params(format!("file_path not found or inaccessible: {}", e), None)
    })?;

    let allowed = repomix_config::global_dir::mcp_outputs_dir()
        .map_err(|e| ErrorData::internal_error(format!("resolve mcp outputs dir: {}", e), None))?
        .canonicalize()
        .map_err(|e| {
            ErrorData::internal_error(format!("canonicalize mcp outputs dir: {}", e), None)
        })?;

    if !canonical.starts_with(&allowed) {
        return Err(ErrorData::invalid_params(
            format!(
                "file_path must be under '{}' (got '{}')",
                allowed.display(),
                canonical.display()
            ),
            None,
        ));
    }

    Ok(canonical)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn make_mcp_output_path_has_unique_id() {
        let a = make_mcp_output_path(&OutputStyle::Xml).unwrap();
        let b = make_mcp_output_path(&OutputStyle::Xml).unwrap();
        assert_ne!(a.output_id, b.output_id);
        assert_eq!(a.path.extension().and_then(|e| e.to_str()), Some("xml"));
        assert_eq!(
            a.path.file_stem().and_then(|s| s.to_str()),
            Some(a.output_id.as_str())
        );
    }

    #[test]
    fn validate_rejects_path_outside_outputs_dir() {
        assert!(validate_mcp_output_path("/etc/passwd").is_err());
    }

    #[test]
    fn validate_accepts_file_under_outputs_dir() {
        let out = make_mcp_output_path(&OutputStyle::Plain).unwrap();
        fs::write(&out.path, "hello\n").unwrap();

        let validated = validate_mcp_output_path(out.path.to_str().unwrap()).unwrap();
        assert_eq!(validated, out.path.canonicalize().unwrap());

        let _ = fs::remove_file(&out.path);
    }

    #[test]
    fn cleanup_does_not_remove_recent_files() {
        let out = make_mcp_output_path(&OutputStyle::Plain).unwrap();
        fs::write(&out.path, "fresh\n").unwrap();
        cleanup_stale_mcp_outputs();
        assert!(out.path.exists(), "recent MCP output should be kept");
        let _ = fs::remove_file(&out.path);
    }
}
