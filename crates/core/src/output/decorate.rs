use repomix_config::schema::RepomixConfig;

/// 输出头部信息（由 decorate 生成，在 produce_output 中使用）
pub struct OutputHeader {
    pub header_text: Option<String>,
    pub instruction_content: Option<String>,
}

/// 收集输出头部信息
pub fn collect_header(config: &RepomixConfig) -> OutputHeader {
    let header_text = config.output.header_text.clone();

    let instruction_content = config
        .output
        .instruction_file_path
        .as_ref()
        .and_then(|path| std::fs::read_to_string(path).ok());

    OutputHeader {
        header_text,
        instruction_content,
    }
}

/// 格式化头部文本（供各样式生成器使用）
pub fn format_header(header: &OutputHeader) -> String {
    let mut parts = Vec::new();

    if let Some(text) = &header.header_text {
        parts.push(text.clone());
    }

    if let Some(text) = &header.instruction_content {
        parts.push(text.clone());
    }

    parts.join("\n\n")
}
