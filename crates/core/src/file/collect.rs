use std::path::{Path, PathBuf};
use anyhow::Result;
use rayon::prelude::*;
use repomix_config::schema::RepomixConfig;
use repomix_shared::types::*;
use crate::file::types::FileCollectOptions;

/// 收集文件内容
///
/// P3 修复（Bug #11）：移除未使用的 `root_dirs` 参数（下划线前缀表示作者已意识到）。
/// 该参数是设计残留：根目录信息已在调用方（packager）持有，此处不需要重复传递。
pub async fn collect_files(
    file_paths: Vec<PathBuf>,
    config: &RepomixConfig,
) -> Result<FileCollectResult> {
    let options = FileCollectOptions::from_config(config);
    let max_file_size = options.max_file_size;
    
    // 并行读取文件
    let results: Vec<_> = file_paths
        .par_iter()
        .map(|path| {
            read_raw_file(path, max_file_size)
        })
        .collect();
    
    let mut raw_files = Vec::new();
    let mut skipped_files = Vec::new();
    
    for result in results {
        match result {
            Ok(raw_file) => raw_files.push(raw_file),
            Err(skipped) => skipped_files.push(skipped),
        }
    }
    
    Ok(FileCollectResult {
        raw_files,
        skipped_files,
    })
}

/// 读取原始文件（带编码检测）
///
/// P0 修复：合并二进制检测和内容读取为单次 I/O，
/// 避免 TOCTOU 竞争和冗余文件打开。
fn read_raw_file(path: &PathBuf, max_file_size: u64) -> std::result::Result<RawFile, SkippedFileInfo> {
    // 检查文件大小
    let metadata = std::fs::metadata(path).map_err(|e| SkippedFileInfo {
        path: path.clone(),
        reason: format!("无法读取文件元数据: {}", e),
    })?;

    if metadata.len() > max_file_size {
        return Err(SkippedFileInfo {
            path: path.clone(),
            reason: format!("文件大小超过限制: {} > {}", metadata.len(), max_file_size),
        });
    }

    // 单次读取文件所有内容
    let bytes = std::fs::read(path).map_err(|e| SkippedFileInfo {
        path: path.clone(),
        reason: format!("无法读取文件内容: {}", e),
    })?;

    // UTF-16 BOM：带 BOM 的 UTF-16 文本含 NULL 字节，需在二进制检测前处理
    if let Some(content) = decode_utf16_if_bom(&bytes) {
        return Ok(RawFile {
            path: path.clone(),
            content,
            size: metadata.len() as usize,
        });
    }

    // 二进制检测：检查前 64KB 是否包含 NULL 字节
    let check_size = bytes.len().min(65536);
    if bytes[..check_size].contains(&0) {
        return Err(SkippedFileInfo {
            path: path.clone(),
            reason: "二进制文件".to_string(),
        });
    }

    // 编码检测
    let content = decode_bytes(&bytes, path)?;

    Ok(RawFile {
        path: path.clone(),
        content,
        size: metadata.len() as usize,
    })
}

/// 若存在 UTF-16 BOM，解码为字符串；否则返回 None。
fn decode_utf16_if_bom(bytes: &[u8]) -> Option<String> {
    let (le, skip) = if bytes.starts_with(&[0xFF, 0xFE]) {
        (true, 2)
    } else if bytes.starts_with(&[0xFE, 0xFF]) {
        (false, 2)
    } else {
        return None;
    };

    if bytes.len() <= skip {
        return Some(String::new());
    }

    let u16_units: Vec<u16> = bytes[skip..]
        .chunks_exact(2)
        .map(|chunk| {
            if le {
                u16::from_le_bytes([chunk[0], chunk[1]])
            } else {
                u16::from_be_bytes([chunk[0], chunk[1]])
            }
        })
        .collect();

    Some(String::from_utf16_lossy(&u16_units))
}

/// 从字节序列解码为字符串
fn decode_bytes(bytes: &[u8], path: &Path) -> std::result::Result<String, SkippedFileInfo> {
    // 先尝试直接以 UTF-8 读取
    if let Ok(content) = std::str::from_utf8(bytes) {
        return Ok(content.to_string());
    }

    // UTF-8 失败，尝试检测编码
    let mut detector = chardetng::EncodingDetector::new();
    detector.feed(bytes, true);
    let encoding = detector.guess(None, true);

    // P0 改进（Bug #1）：`encoding.decode` 返回的 `had_errors=true` 表示解码过程中
    // 有字节被替换为 U+FFFD（replacement character），但 `decoded` 仍可能是可用
    // 内容。**正确的"无法解码"判断应该是"输入非空但解码后为空"**：这种情况极少见
    // （例如纯控制字符的二进制流），chardetng 误判为 ASCII，decode 后全被丢弃。
    // 修复前用 `had_errors` 作为唯一判据会误跳过大量合法文件（实测含非 ASCII 字符
    // 的 GBK/Latin1 文件经常 had_errors=true 但内容完全可用）。详见 #1。
    let (decoded, _, _had_errors) = encoding.decode(bytes);
    if decoded.is_empty() {
        Err(SkippedFileInfo {
            path: path.to_path_buf(),
            reason: "无法检测文件编码".to_string(),
        })
    } else {
        Ok(decoded.into_owned())
    }
}