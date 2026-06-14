# Tree-Sitter 领域

**模块路径**：`crates/core/src/tree_sitter/`
**生成日期**：2026-06-14
**分析置信度**：8/10

---

## 概述

Tree-sitter 模块是 repomix-rs 的"代码翻译机"——它能理解代码的语法结构，提取函数签名、类定义、类型声明等"骨架"信息，同时把函数体等"血肉"剥离。这对 LLM 场景来说是一举两得的好处：AI 不需要看到每个函数的完整实现细节就能理解代码的模块结构和 API 设计，而 token 消耗可以降低 40-60%。

这个模块是"性能投资"的典型例子：引入 tree-sitter 依赖增加了编译时间（它需要编译 10 种语言的语法解析器），但换来的是相比纯文本打包显著的 token 节省。

---

## 核心功能点

1. **单文件压缩**：`compress_file()`（`crates/core/src/tree_sitter/compress.rs:11`）接收文件内容、路径和 `LanguageConfig`，使用 tree-sitter 解析源码编译 AST，通过预定义的 S-expression query 提取匹配节点，按源码顺序拼接输出。

2. **10 语言支持**：`get_language_config()` 根据文件扩展名匹配语言配置。C# 因 `tree-sitter-c-sharp` 0.23 的 ABI 不匹配临时禁用。

3. **源码顺序保证**：`QueryCursor::captures()` 按源码位置有序返回捕获（tree-sitter 文档保证），确保压缩输出中的符号顺序与源文件一致。

---

## 关键组件

| 组件/类型 | 文件路径 | 核心职责 |
|---------|---------|---------|
| `compress_file()` | `crates/core/src/tree_sitter/compress.rs:11` | 压缩单个文件 |
| `get_language_config()` | `crates/core/src/tree_sitter/languages.rs` | 文件扩展名→语言配置 |
| `LanguageConfig` | `crates/core/src/tree_sitter/languages.rs` | Language + compress_query |

---

## 内部数据流

```mermaid
flowchart TD
    A["compress_file(content, path, config)<br/>tree_sitter/compress.rs:11"] --> B["Parser::new()"]
    B --> C["set_language(&config.language)"]
    C --> D["parser.parse(content)<br/>→ 语法树"]
    D --> E["QueryCursor::captures()<br/>按源码顺序迭代"]
    E --> F{"byte_range<br/>为 0?"}
    F -->|是| SKIP["跳过"]
    F -->|否| G{"与上一个<br/>重叠?"}
    G -->|是| SKIP
    G -->|否| H["content[range]<br/>切片提取"]
    H --> I["parts.push(text)"]
    I --> E
    E --> J{"还有更多<br/>capture?"}
    J -->|是| F
    J -->|否| K["parts.join(\"\\n⋮----\\n\")"]
    K --> L{"parts 为空?"}
    L -->|是| NONE["返回 None"]
    L -->|否| SOME["返回 compressed"]
```

**关键步骤说明**：
1. `last_byte_end` 机制跳过嵌套 capture 的重复——同一源码位置如果被父 capture 和子 capture 同时匹配，只保留外层
2. capture 按 byte_range 从原始 `content` 直接切片，零拷贝

---

## 关键接口与扩展点

添加新语言的步骤：
1. 在 `languages.rs` 中调用 `LanguageConfig::new(Language::new(), query_content)`  
2. 添加查询文件到 `queries/` 目录（S-expression 格式）  
3. 在 `get_language_config()` 的 match 中增加文件扩展名映射  

---

## 与其他模块的交互

| 交互模块 | 方向 | 接口/协议 | 说明 |
|---------|------|---------|------|
| file::process | 被依赖 | `compress_file()` | process.rs 在每个文件处理时按需调用 |

---

## 跨模块协作场景

**在流水线的加工车间**：tree_sitter 模块在 file::process 内部被调用，是"Phase 1（rayon 并行）"的一部分。当 `config.output.compress` 为 true 时，`process_single_file()` 首先尝试从语言配置中获取匹配项，然后调 `compress_file()` 进行压缩。

---

## 性能考量

- 每个文件创建独立的 `Parser` 实例（Parse 对象不可跨线程共享）
- `LanguageConfig` 可跨线程共享（`Language` 是 `&'static` 引用）
- `QueryCursor` 每次重新创建，避免状态残留
- 跳过检测失败的场景会回退到原始内容（发 warning 不报错）
- 不支持的扩展名直接返回 None，不走 tree-sitter 路径

---

## 实现亮点

- **重叠 capture 去重**：`last_byte_end` 变量跟踪上一个 capture 的末尾偏移，新 capture 如果 `range.start < last_byte_end` 则跳过——这处理了嵌套 capture 带来的重复（`crates/core/src/tree_sitter/compress.rs:36-52`）
- **无输出短路**：如果 `parts` 为空（如空文件或语法树无匹配），返回 `None`，调用方使用原始内容

---

**分析置信度说明**：8/10 — 完整阅读了 `compress.rs` 和测试代码，确认了核心压缩逻辑、源码顺序保证、重叠去重机制。未阅读 queries 目录中的 S-expression 文件内容，未逐行阅读 `languages.rs` 的全部语言配置。
