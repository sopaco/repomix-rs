# Deep-Exploration：MCP 服务器

MCP 服务器是 `repomix-rs` 的第二条消费者通路，也是它和传统打包工具最不一样的地方。`repomix-mcp` 把整个 `repomix-core` 的打包能力包装成四个标准 MCP 工具，通过 stdio 和 JSON-RPC 协议对外暴露，让 AI agent 运行时无需知道 Rust 代码或文件系统路径就能调用打包功能。

## 这个模块在做什么

`crates/mcp/src/server.rs` 是整个 MCP 模块的核心。它定义了一个 `RepomixMcpServer` 结构体，实现了 `rmcp::ServerHandler` trait，通过 `run_stdio_server()` 入口在 stdio 上监听 JSON-RPC 请求。

### 四个工具

`RepomixMcpServer` 内部维护了一个 `tool_router`（rmcp 提供的工具注册机制），注册了四个工具：

1. **`pack_codebase`** —— 最完整的打包调用，支持全部四个风格，接受 `include_patterns` / `ignore_patterns` / `compress` / `top_files_length` 参数，返回 `PackToolResult`。
2. **`pack_remote_repository`** —— 接收一个 `url`，克隆到唯一临时目录，调用 `core::pack()` 后自动清理，返回结构和 `pack_codebase` 一样。
3. **`read_repomix_output`** —— 轻量级文件读取，接受一个 `file_path`，返回文件内容字符串。agent 可以用这个工具来检查上一次打包的结果，或者查看外部写入的其他文件。
4. **`grep_repomix_output`** —— 在一个已经存在的输出文件里做正则搜索。接受 `file_path`, `pattern`, 和可选的 `context: usize`（上下文行数），返回匹配行及上下文。

### 关键结构

`PackToolResult` 是前两个打包工具的返回结构：

```rust
pub struct PackToolResult {
    pub description: String,
    pub result: PackResult,
    pub directory_structure: String,   // 输出中的目录树文本
    pub output_id: String,             // MCP 侧唯一 ID
    pub output_file_path: String,      // 磁盘上的文件路径
    pub total_files: usize,
    pub total_tokens: usize,
}
```

`PackMetrics` 是对 `PackResult` 的精简版映射，保留了最核心的统计信息，供 MCP 工具快速返回。

`RepomixMcpServer` 持有一个 `lock: Arc<Mutex<()>>`，所有工具调用都在锁里执行，防止多个 agent 并发请求互相踩踏。

### 临时目录管理

`pack_remote_repository` 在 MCP 侧也用一个独立的 `TempDirGuard` 实现（代码在 `crates/mcp/src/server.rs` 里内联），和 CLI 版本的 `cli/run.rs` 里的实现逻辑相同，但结构体不共享，以避免 CLI 反向依赖 MCP crate。

临时目录名的生成逻辑和 CLI 版一致：`repomix_remote_{PID}_{nanos}_{hash_hex}`。

## 内部数据流

```text
JSON-RPC 请求（stdio）
    ↓
rmcp protocal decoder
    ↓
RepomixMcpServer::<tool_name>(params)
    ↓
lock.acquire()
    ↓
构建 PartialConfig / PackOptions
    ↓
调用 core::pack() 或 std::fs::read 或 regex::find_iter
    ↓
lock.release()
    ↓
JSON-RPC 响应（stdio）
```

工具调用是**互斥的**：在持有锁的时候，另一个 tool 无法开始。这意味着如果打包一个大仓库需要 30 秒，在这 30 秒里其他 agent 请求只能排队。这不是 bug，而是版本 0.1 有意的简化——先保证正确性，再优化吞吐量。

## 与其他模块的接口

- **上游消费者**：只有 agent 运行时会调用 MCP 工具。运行时通过 stdio 发 JSON-RPC 请求，或通过 Claude Desktop 的 `mcpServers` 配置启动。
- **下游依赖**：MCP 工具内部调用 `repomix-core::pack()` 和 `repomix_config::load`，以及 Rust 标准库的 `std::fs` 和 `regex`。
- **配置入口**：MCP 工具接收的参数通过 `PartialConfig` 与 `RepomixConfig::load` 桥接，和 CLI 版共享完全相同的分层逻辑。

## 设计考量

**为什么要单独的 crate (`repomix-mcp`)？** `rmcp` 这个依赖相对小众，如果直接塞进 `repomix-cli` 会导致 CLI 用户也承担 MCP 依赖的成本（虽然 `rmcp` 本身不大）。单独成包后，两个 crate 保持独立编译，who 不需要 MCP 的用户就装不到 MCP 相关的符号。

**为什么不把 CLI 和 MCP 合并成一个 `repomix-rs` 大 crate？** 这是 0.1 的设计选择。分拆后职责清晰：CLI 只处理终端交互，MCP 只处理 JSON-RPC 协议，共用 `core`。未来如果要合并，只需要把 `main.rs` 里的入口路由扩展到"如果是 stdio 模式就走 MCP，否则走 CLI"，但现在的分离让测试两个模式变得更容易。

**`read_repomix_output` 和 `grep_repomix_output` 找什么用？** 这两个工具提供了"对已产出的包做二次操作"的能力。agent 可以先 `pack_codebase` 把仓库打包，然后用 `grep_repomix_output` 在结果里搜索特定模式，或者用 `read_repomix_output` 读取完整内容再交给 LLM。这种分两步走的设计让 agent 不必在单次调用里 cram 太多逻辑。

---

## 置信度评分：7 / 10

> 说明：本次分析基于对 `crates/mcp/src/server.rs` 的文件大纲的读取，以及其导出的 `PackToolResult`、`PackMetrics`、四个工具函数和 `RepomixMcpServer` 结构的识别。具体的工具实现代码（`pack_remote_repository` 里的 clone 逻辑、`grep_repomix_output` 的正则策略）没有全量展开，扣 3 分。建议全量阅读 `server.rs` 来准确理解工具的参数约束和错误返回策略。
