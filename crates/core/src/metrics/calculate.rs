use repomix_config::schema::RepomixConfig;
use repomix_shared::types::ProcessedFile;
use std::collections::HashMap;

/// 指标结果
pub struct MetricsResult {
    pub total_characters: usize,
    pub total_tokens: usize,
    pub file_char_counts: HashMap<String, usize>,
    pub file_token_counts: HashMap<String, usize>,
    /// 按 token 数降序排列的前 N 个文件（N = `config.output.top_files_length`）。
    pub top_files_by_tokens: Vec<(String, usize)>,
}

/// 计算指标（含按 token 数降序的前 N 个文件列表）
pub fn calculate_metrics(
    files: &[ProcessedFile],
    config: &RepomixConfig,
) -> Result<MetricsResult, anyhow::Error> {
    let mut total_characters = 0;
    let mut total_tokens = 0;
    let mut file_char_counts = HashMap::new();
    let mut file_token_counts = HashMap::new();

    for file in files {
        let char_count = file.content.len();
        let token_count = file.token_count;

        total_characters += char_count;
        total_tokens += token_count;

        let path_str = file.path.to_string_lossy().to_string();
        file_char_counts.insert(path_str.clone(), char_count);
        file_token_counts.insert(path_str, token_count);
    }

    // 计算 top-N 文件：按 token 数降序。
    // top_files_length 为 0 时返回空向量（用户可显式禁用此功能）；
    // 也避免 `take(0)` 误返回全部。
    let top_n = config.output.top_files_length;
    let mut sorted: Vec<(String, usize)> = file_token_counts
        .iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let top_files_by_tokens = if top_n == 0 {
        Vec::new()
    } else {
        sorted.into_iter().take(top_n).collect()
    };

    Ok(MetricsResult {
        total_characters,
        total_tokens,
        file_char_counts,
        file_token_counts,
        top_files_by_tokens,
    })
}
