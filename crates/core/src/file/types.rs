use repomix_config::schema::RepomixConfig;

/// 文件搜索选项
pub struct FileSearchOptions {
    pub include_patterns: Vec<String>,
    pub ignore_patterns: Vec<String>,
    pub include_empty_directories: bool,
}

impl FileSearchOptions {
    pub fn from_config(config: &RepomixConfig) -> Self {
        Self {
            include_patterns: config.include.clone(),
            ignore_patterns: config.ignore.custom_ignore.clone(),
            include_empty_directories: config.output.include_empty_directories,
        }
    }
}

/// 文件收集选项
pub struct FileCollectOptions {
    pub max_file_size: u64,
}

impl FileCollectOptions {
    pub fn from_config(config: &RepomixConfig) -> Self {
        Self {
            max_file_size: config.input.max_file_size,
        }
    }
}