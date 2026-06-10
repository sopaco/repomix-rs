use std::path::{Path, PathBuf};

fn normalize_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

/// 将绝对路径转为相对 pack 根目录的显示路径（统一 `/` 分隔符）。
///
/// 优先用 `strip_prefix`（无 syscall）；仅在直接匹配失败时再 canonicalize。
pub fn display_path(path: &Path, pack_root: &Path) -> String {
    if let Ok(rel) = path.strip_prefix(pack_root) {
        return rel.to_string_lossy().replace('\\', "/");
    }

    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let root = pack_root.canonicalize().unwrap_or_else(|_| pack_root.to_path_buf());
    path.strip_prefix(&root)
        .unwrap_or(&path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// 解析 pack 根目录：传入单文件时使用其父目录。
pub fn effective_pack_root(path: &Path) -> PathBuf {
    if path.is_file() {
        path.parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| path.to_path_buf())
    } else {
        path.to_path_buf()
    }
}

/// 解析输出文件路径：绝对路径保持不变，相对路径相对于 pack 根目录（结果始终为绝对路径）。
pub fn resolve_output_file_path(file_path: &str, pack_root: &Path) -> String {
    let p = Path::new(file_path);
    if p.is_absolute() {
        return file_path.to_string();
    }
    let root = effective_pack_root(pack_root);
    let root = if root.is_absolute() {
        root
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(root)
    };
    root.join(p).to_string_lossy().into_owned()
}

/// 解析 git 仓库根目录（失败时回退为 `repo_path`）。
pub fn git_repo_root(repo_path: &Path) -> PathBuf {
    std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(repo_path)
        .stderr(std::process::Stdio::null())
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| {
            let s = String::from_utf8(o.stdout).ok()?;
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(normalize_path(Path::new(trimmed)))
            }
        })
        .unwrap_or_else(|| normalize_path(repo_path))
}

/// 将文件路径转为 git log `--name-only` 使用的相对路径键。
pub fn git_relative_path(path: &Path, repo_root: &Path) -> String {
    display_path(path, repo_root)
}

/// 判断文件名是否为 repomix 历史/分片输出产物，应避免再次打包。
pub fn is_repomix_output_artifact(file_name: &str, configured_output: &str) -> bool {
    let configured_base = Path::new(configured_output)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(configured_output);

    if file_name == configured_base {
        return true;
    }
    // split 分片：`repomix-output.xml.2`
    if let Some(suffix) = file_name.strip_prefix(&format!("{configured_base}.")) {
        if suffix.parse::<u32>().is_ok() {
            return true;
        }
    }
    file_name.starts_with("repomix-output.")
}
