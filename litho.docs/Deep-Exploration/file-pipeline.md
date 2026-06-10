# Deep-Exploration：文件流水线

文件流水线是 `repomix-core` 最核心、最长的子模块，占据了 `crates/core/src/file/` 下九个实现文件，是这个工具从磁盘上的一个目录，到最后输出文件的那张"运输网"。如果把 `pack()` 比作一家工厂的装配线，文件流水线就是压铸车间：原材料进，成品出。

## 这个模块在做什么

文件流水线做了三件事，按顺序排列：

1. **搜索**（`file::search`）—— 确定"要打包哪些文件"。
2. **收集**（`file::collect`）—— 把文件的原始内容读进内存。
3. **处理**（`file::process`，含 `process_content`、`manipulate`、`truncate_base64`）—— 压缩、删注释、删空行、加行号、截断 base64，最后加上 token 统计。
4. **树生成**（`file::tree_generate`）—— 输出开头的目录结构部分。

这四个步骤是顺序依赖的，输出一个变成下一个的输入，没有缓存，没有重试，走通就走到终。

## 核心组件

`crates/core/src/file/types.rs` 是所有组件的类型布局的起点。它定义了搜索参数 `FileSearchOptions`、收集参数 `FileCollectOptions`，所有流经流水线的数据结构都在这个文件里。设计上分成三层：最轻的搜索参数、中间的原始文件 `RawFile`，和最终加工后的 `ProcessedFile`。这个分层让每个阶段只关心自己需要的那几个字段，不需要加载整个 `RepomixConfig`。

`crates/core/src/file/search.rs` 是入口。它基于 `ignore` crate 做目录遍历，然后把 `.gitignore` 的规则、用户显式 `--include` / `--ignore`、以及库自带的 `default_ignore` 模式全部叠加在一起，最终得出一个 `Vec<PathBuf>` 的文件清单和一个可选的空目录列表（后者用于满足 `include_full_directory_structure` 选项）。

`crates/core/src/file/collect.rs` 读文件内容，生成 `Vec<RawFile> { path, content, size }`。每个文件最多读到 `RepomixConfig::input.max_file_size` 为止，超限的就记入 `skipped_files` 列表。收集步骤用 async 读操作，因为磁盘 I/O 是这里的主要瓶颈。

`crates/core/src/file/process.rs` 是实际重量最重的部分。它接收 `Vec<RawFile>`，并行地对每个文件做变换：
- **tree-sitter 压缩**：如果 `config.output.compress` 打开，就用 tree-sitter 的目标语言 grammar 解析文件，提取函数/类签名，丢弃函数体。
- **移除注释**：`file::manipulate::remove_comments`，解析 token 流，过滤掉注释 token。
- **移除空行**：`file::manipulate::remove_empty_lines`，去掉连续空行和纯粹空白行。
- **截断 base64**：`file::truncate_base64`，把超长的 base64 blob 截成 `...[truncated N bytes]`。
- **加行号**：`file::manipulate::add_line_numbers`，逐行前缀行号。
- **token 计数**：用 tiktoken-rs 对最终内容计数，失败时回退到 `split_whitespace` 估计。

整个 `process_files` 函数用 `rayon` 的 `par_iter` 把文件分给多核 CPU。这意味着对于 1000 个文件的项目，CPU 利用率能跑满，把这几个变换的耗时摊到所有可用核心上。

`crates/core/src/file/tree_generate.rs` 是输出阶段前面的一道工序，负责把目录结构转成 ASCII 树文本，插到输出文件的开头。它支持三种模式：仅文件节点、带空目录、完全展开目录树。

`crates/core/src/file/process_content.rs` 是处理单文件的核心，用 `ProcessContentOptions` 来开关各个变换。选项结构体内嵌在 config 的 `output` 子字段里，这样处理层不需要直接接触大 config，只需要知道"要不要压缩、要不要删注释"这几个问题。

## 内部数据流

```text
FileSearchOptions { include_patterns, ignore_patterns, include_empty_directories }
        ↓
file::search
        ↓
FileSearchResult { file_paths: Vec<PathBuf>, empty_dir_paths: Vec<PathBuf> }
        ↓
file::collect (async I/O, parallel? 当前是顺序读)
        ↓
FileCollectResult { raw_files: Vec<RawFile>, skipped: Vec<SkippedFileInfo> }
        ↓
security::validate → ValidationResult { suspicious, safe_paths }
        ↓
file::process (rayon par_iter over RawFile)
        ↓
Vec<ProcessedFile> { path, content, token_count }
        ↓
filter_suspicious → 去掉可疑文件
        ↓
output::generate (含 tree::generate) → 写入磁盘
```

 RawFile → ProcessedFile 的变换是纯函数式的：给定一个 `RawFile` 和一组选项，输出一个 `ProcessedFile`，不依赖外部状态。这个性质让 rayon 的 `par_iter` 能安全地并行调用，而 `ProcessedFile` 一旦产出，其 `content` 不会再被修改。

## 扩展点

要加一个新的文件处理变换（比如格式化、内容规范化、或者添加文件哈希），只需：

1. 在 `file/manipulate.rs` 里加一个新函数。
2. 在 `ProcessContentOptions` 里加一个布尔开关。
3. 在 `FileCollectResult` 的 `RepomixConfig` schema 里加一个配置键。
4. 在 `process_single_file` 的末尾加上对新函数的调用。

五个文件，改动位于一个 crate 内，不影响对外 API。

## 性能考量

- **并行化的粒度是每个文件**，不是文件的字节流。把一个 100MB 的文件交给一个 rayon worker 处理，即使文件内部有多个函数，树-sitter 解析也不会被拆分到多核。大文件的压缩速度受限于单核。
- **收集步骤是顺序 I/O**。`file::collect::collect_files` 对每个文件做一次 read，没有 batch read 或 async 批量。对于上千个小文件的项目，文件数量本身的 syscall 开销是显著的。
- **`max_file_size` 的默认值**以字节为单位，一旦超限整个文件就被跳过而不读取任何内容，避免 OOM。

## 与其他模块的接口

输入来源：`file::search` 读 `config::output::include` / `ignore` / `use_gitignore` / `custom_ignore`。  
下游消费者：`security::validate` 读 `Vec<RawFile>`，`output::generate` 读 `Vec<ProcessedFile>` 和 `empty_dir_paths`。  
管道参数：`tree_sitter::compress` 接受 `ProcessedFile`，`metrics::token_count` 也在处理阶段被调用。  
边界：`file::search::search_files` 是外部 crate 能调用的唯一 async 入口，返回 `FileSearchResult`。

---

## 置信度评分：8 / 10

> 说明：本模块的分析依赖于对 `crates/core/src/file/` 下所有九个实现文件的直接读取（search、collect、process、process_content、manipulate、truncate_base64、tree_generate、types 和 mod），主干流程准确。`file::search` 的具体 glob 合并逻辑细节和 `tree_generate` 的完整递归实现没有全量展开，因此扣 2 分。建议对这两个文件做一次针对性全量阅读来补足余量。
