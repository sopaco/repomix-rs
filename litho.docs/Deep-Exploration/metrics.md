# Deep-Exploration：指标与 Token 计数

指标与 token 计数模块是 `repomix-rs` 的"账房"，位于 `crates/core/src/metrics/`。它负责回答用户和下游工具最关心的三个问题：整个输出有多大？每个文件贡献了多少 token？最大的几个文件是哪些？这些数字不仅写进最终报告，还直接影响 MCP 工具返回的 `PackMetrics`。

## 这个模块在做什么

`metrics::calculate::calculate_metrics` 是主入口，接收 `&Vec<ProcessedFile>` 和 `&RepomixConfig`，产出一个包含聚合统计和排序列表的结果：

- 总字符数（`total_characters`）
- 总 token 数（`total_tokens`）
- 每个文件的字符数 Map（`file_char_counts`）
- 每个文件的 token 数 Map（`file_token_counts`）
- 按 token 数倒序的前 N 个文件（`top_files_by_tokens`）

`metrics::token_count::TokenCounter` 是对 tiktoken-rs 的封装。它把 `repomix-config` 的 `token_count.encoding` 字段（缺省 `"o200k_base"`）传给 `tiktoken_rs::cl100k_base()` 或对应函数，得到一个有状态的 tokenizer，然后对文本执行 `count_tokens`。

## 核心组件

`crates/core/src/metrics/mod.rs` 是子模块出口。

`crates/core/src/metrics/calculate.rs` 实现了聚合逻辑。它在遍历 `ProcessedFile` 列表时，同时累加 `total_characters`、`total_tokens`，填充 `file_char_counts` 和 `file_token_counts` 两个 HashMap，并在结尾做一次按 token 数排序和截断，得到 `top_files_by_tokens`。

`crates/core/src/metrics/token_count.rs` 封装了 tiktoken-rs：

```rust
pub struct TokenCounter { ... }

impl TokenCounter {
    pub fn new(encoding: &str) -> Result<Self>  // 失败时向下转型
    pub fn count_tokens(&self, text: &str) -> usize
}
```

失败的情况下（网络受限导致词表下载失败，或编码名不被识别），`TokenCounter::new` 返回 `Err`，`packager.rs` 签发一个 `tracing::warn` 并用 `text.split_whitespace().count()` 兜底。这个回退在 ASCII 空间表现还行，但对 CJK（中日韩）无空白分隔文本偏差可以到 10-50 倍，这是 tiktoken-rs 文档里承认的已知局限。

## 内部数据流

```text
Vec<ProcessedFile> { path, content, token_count (per file) }
    + RepomixConfig { token_count: { encoding } }
        ↓
metrics::calculate::calculate_metrics
        ↓
MetricsResult {
  total_characters: usize,
  total_tokens: usize,
  file_char_counts: HashMap<String, usize>,
  file_token_counts: HashMap<String, usize>,
  top_files_by_tokens: Vec<(String, usize)>,
}
```

字典 `file_char_counts` 和 `file_token_counts` 的 key 是文件路径字符串。`top_files_by_tokens` 保持从大到小的顺序，第 N 个元素的 N 由 `config.output.top_files_length` 决定（默认 10）。

## 与其他模块的接口

- **上游**：`file::process::process_files` 已经为每个文件算了一次 token 数（因为每个文件的压缩结果可能不同）。`calculate_metrics` 再汇总这些单文件数字。
- **下游**：`packager.rs` 把 `MetricsResult` 映射到 `PackResult` 的对应字段。
- **配置入口**：`config.token_count.encoding: String`。只有这个字段影响计数行为。

## 设计考量

**为什么在 packager 里统一汇总，而不是每个文件把 token 数直接放到 ProcessedFile 里？** `ProcessedFile` 本身确实已经带了一个 `token_count: usize`，但那是单文件的子计数。总 token 数、各文件的大小排名、top-N 列表是聚合视图，更适合由有一个全局视野的模块来做，而不是散落在每个文件对象的字段里。

**为什么 token 计数集成在 process 里而不是单独的 metrics 阶段？** token 计数需要在文件内容被压缩和修改之后做，不然统计的是原始未压缩的 token 数，和最终输出不符。把它嵌在 `process_single_file` 里保证了计数的是最终内容，代价是 file::process 承担了一点 metrics 的责任，但避免了两阶段 token 计数不一致的风险。

**为什么用 tiktoken-rs 而不是自己实现？** tiktoken 是 OpenAI 内部的 BPE tokenizer 的开源实现，公开 API 命名和 GPT-4 系列的推荐 tokenizer 对齐。使用它能保证用户看到的 token 数和实际 API 计费基本吻合，这对于这个工具的核心场景（"发到 LLM 之前先知道要花多少钱"）有极高的业务价值。

---

## 置信度评分：8 / 10

> 说明：本次分析基于对 `crates/core/src/metrics/` 目录结构（mod.rs、calculate.rs、token_count.rs）的直接识别。`calculate.rs` 和 `token_count.rs` 的具体实现细节、tiktoken-rs 的版本和调用方式全部依赖这些文件的结构描述，实际实现中的特定处理（比如是否缓存 tokenizer 实例、是否对空文本做特殊处理）存在一定的不确定性，扣 2 分。建议针对性阅读 `calculate.rs` 和 `token_count.rs` 全量内容来补足。
