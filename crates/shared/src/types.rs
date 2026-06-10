use std::path::PathBuf;

/// 原始文件内容
#[derive(Debug, Clone)]
pub struct RawFile {
    pub path: PathBuf,
    pub content: String,
    pub size: usize,
}

/// 处理后的文件
#[derive(Debug, Clone)]
pub struct ProcessedFile {
    pub path: PathBuf,
    pub content: String,
    pub token_count: usize,
}

/// 跳过的文件信息
#[derive(Debug, Clone)]
pub struct SkippedFileInfo {
    pub path: PathBuf,
    pub reason: String,
}

/// 可疑文件结果
#[derive(Debug, Clone)]
pub struct SuspiciousFileResult {
    pub path: PathBuf,
    pub line: usize,
    pub message: String,
    pub rule_id: String,
}

/// 文件搜索结果
#[derive(Debug, Clone)]
pub struct FileSearchResult {
    pub file_paths: Vec<PathBuf>,
    pub empty_dir_paths: Vec<PathBuf>,
}

/// 文件收集结果
#[derive(Debug, Clone)]
pub struct FileCollectResult {
    pub raw_files: Vec<RawFile>,
    pub skipped_files: Vec<SkippedFileInfo>,
}

/// 安全验证结果
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub suspicious: Vec<SuspiciousFileResult>,
    pub safe_paths: Vec<PathBuf>,
}