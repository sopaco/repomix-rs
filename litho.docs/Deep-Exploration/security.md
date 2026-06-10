# Deep-Exploration：安全扫描

`repomix-rs` 的安全扫描模块是用户把打包结果放心发给 LLM 的最后一道防线。它集成 Secretlint 规则引擎，对每个原始文件内容做静态分析，一旦命中规则就输出一个 `SuspiciousFileResult`，最终整个文件从打包结果里剔除。这个模块承担了“责任隔离”的角色。

## 这个模块在做什么

安全扫描模块在主流水线里位于 `file::collect` 之后、`file::process` 之前。它的职责是：**决定哪些文件可以参与后续的压缩和输出**。

具体流程：
1. 接收 `Vec<RawFile>`（文件路径 + 原始内容 + 大小）和当前有效的 `RepomixConfig`。
2. 对每个文件的内容跑 Secretlint 规则。
3. 将命中规则的文件写入 `suspicious` 列表，记录路径、行号、规则 ID 和描述信息；无毒的文件写入 `safe_paths`。
4. 返回 `ValidationResult { suspicious: Vec<SuspiciousFileResult>, safe_paths: Vec<PathBuf> }`。

主流程的 `packager.rs` 拿到这个结果后，调用 `filter_suspicious` 把可疑文件从 `Vec<ProcessedFile>` 里删掉，保证它们不会出现在最终输出里。

## 核心组件

`crates/core/src/security/validate.rs` 是主要入口。它的 `validate_file_safety` 接收 `&Vec<RawFile>`，调用 Secretlint 的 Rust 绑定，遍历每个文件的文本内容，使用 `globset` 或 Secretlint 自带的规则集判读是否有密钥、密码、token 等泄露风险。

`SuspiciousFileResult` 数据结构（定义在 `crates/shared/src/types.rs`）的设计是：

```rust
pub struct SuspiciousFileResult {
    pub path: PathBuf,    // 哪个文件
    pub line: usize,      // 第几行
    pub message: String,  // Secretlint 返回的说明信息
    pub rule_id: String,  // 命中的规则 ID（例如 "aws-access-key-id" 之类的）
}
```

这个设计精确到行号，方便用户在 `suspicious_files` 列表里时快速定位问题代码。

`ValidationResult` 作为返回结果，把"有问题"和"放心通过"的文件分成两个集合：

```rust
pub struct ValidationResult {
    pub suspicious: Vec<SuspiciousFileResult>,
    pub safe_paths: Vec<PathBuf>,
}
```

`safe_paths` 的用处：在后续过滤步骤里安全报告哪些文件通过了安检，和哪些文件被拦下了，让 `PackResult` 的 `safe_file_paths` 字段可以准确反映这个过程。

## 内部数据流

```text
Vec<RawFile> { path, content, size }
        ↓
security::validate::validate_file_safety(raw_files, config)
        ↓
ValidationResult {
  suspicious: [ SuspiciousFileResult { path, line, message, rule_id }, ... ],
  safe_paths: [ safe PathBuf, ... ]
}
        ↓
filter_suspicious(processed_files, validation)
        ↓
Vec<ProcessedFile>（不含可疑文件）
```

一个细节是 `config.security.enable_secretlint` 控制这个步骤的开关。这个字段的默认值是 `true`，表示安全扫描默认开启。如果用户显式关闭，`validate_file_safety` 应该直接返回一个全空的 `ValidationResult`（所有文件视为 safe），流程继续。

## 与其他模块的接口

输入来源：  
- 上游 `file::collect::collect_files` 产出 `Vec<RawFile>`。

下游消费者：  
- `packager.rs` 调用 `filter_suspicious(processed, &validation)` 删除可疑文件。
- `PackResult` 保留了 `suspicious_files` 和 `safe_file_paths`，供最终报告和 `top_files_by_tokens` 的计算使用。

配置入口：  
- `config.output.security.enable_secretlint: bool`（默认 `true`）。

## 设计考量

**为什么放在 collect 之后、process 之前？** 如果把安全扫描移到 process 之后，那么可疑文件的 tree-sitter 压缩结果、token 计数结果都已经算出来了，这些运算对最终输出毫无意义，白白消耗了 CPU。前置扫描可以"快失败"，避免对有害文件进行无意义的后处理。

**为什么是可过滤而不是可阻止？** 这是一个设计选择。安全扫描模块**不做中止或报错**——它只提供信息，把决策权留给编排层（`packager.rs`）。这样编排层可以根据项目的需要选择：
- 保守策略：任何命中都过滤，本期打包不包含那个文件。
- 审计策略：保留可疑文件，只在 `suspicious_files` 列表里标记，由人工审核后再决定是否发送。

**默认值是开还是关？** 当前 schema 里 `enable_secretlint` 的默认是 `true`，这是有意的防御性默认。对于"把代码发给 LLM"这个场景，安全问题是优先级最高的非功能性需求。

---

## 置信度评分：6 / 10

> 说明：本模块的分析基于对 `crates/shared/src/types.rs` 中 `SuspiciousFileResult` 和 `ValidationResult` 定义的直接读取，以及 `crates/core/src/packager.rs` 中 `security::validate::validate_file_safety` 调用位置和 `filter_suspicious` 过滤逻辑的识别。`crates/core/src/security/` 下的具体实现文件（validate.rs 及任何 helper）没有全量展开阅读，Secretlint 的具体 API 调用方式（例如用的是哪个版本、是基于 async 还是 sync、规则是怎么注册的）存在不确定性。因此扣 4 分。建议全量阅读 `crates/core/src/security/validate.rs` 来补足这层。
