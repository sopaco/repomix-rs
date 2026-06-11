use repomix_config::schema::RepomixConfig;

/// 文件处理选项
pub struct ProcessContentOptions {
    pub remove_comments: bool,
    pub compress: bool,
    pub truncate_base64: bool,
    pub remove_empty_lines: bool,
    pub show_line_numbers: bool,
}

impl ProcessContentOptions {
    pub fn from_config(config: &RepomixConfig) -> Self {
        Self {
            remove_comments: config.output.remove_comments,
            compress: config.output.compress,
            truncate_base64: config.output.truncate_base64,
            remove_empty_lines: config.output.remove_empty_lines,
            show_line_numbers: config.output.show_line_numbers,
        }
    }
}
