---
type: human
project: repomix-rs
title: Agent Architecture Context
source: /Users/bjsttlp485/Workspace/SAW/repomix-rs
---

## 项目概览

**repomix-rs** 是一个基于 Rust 开发的多模态代码打包工具，旨在将代码仓库的内容转换为特定格式（JSON、Markdown、Plain、XML）的压缩包，支持 Git 差异分析、安全敏感信息检测、文件截断、代码搜索及指标统计等功能。项目核心模块 `repomix-rs` 采用 Rust 编写，提供了高性能、跨平台的命令行为 CLI，并内置了可独立使用的 Rust 库（`repomix-core`, `repomix-config`, `repomix-mcp`）和 Node.js 包（`repomix` npm package）。项目结构清晰，遵循模块化设计，将 CLI、核心逻辑、配置、MCP（Model Context Protocol）和 npm 模板解耦，支持多平台部署（Node.js/npm 与 Rust/cargo）。

## 架构设计

项目采用 **模块化分层架构**：- **core**: 核心逻辑层，负责文件收集、处理、树构建（Tree-sitter）、内容截断、Git 差异分析、安全校验（Secretlint）及输出生成。- **config**: 配置加载与解析层，支持全局配置与命令行覆盖，提供 Schema 验证。- **mcp**: MCP Server 层，为 LLM 代理提供工具（Tools），支持参数校验与输出格式化。- **shared**: 共享层，提供日志（Logger）、通用数据类型（Types）、模式匹配工具（Pattern Utils）。- **cli**: CLI 入口层，负责解析用户输入、编排核心模块调用、处理命令行标志（Flags）。- **npm**: NPM 分发层，基于 Node.js 封装 Rust 逻辑，支持模板驱动的多语言构建脚本。**数据流**：CLI 接收命令与参数 -> `config` 加载配置 -> `core` 执行收集/处理/搜索 -> `mcp` 封装工具（可选） -> 生成压缩包 -> 输出结果。

## 模块地图

| 模块 (Module) | 路径 (Path) | 职责 (Responsibility) |
| :--- | :--- | :--- |
| `repomix-rs` | Root | 项目根入口，定义 crate 组织与 npm 包装 |
| `repomix-core` | `crates/core` | 文件处理核心引擎，包含文件收集、树构建、Git 处理、输出格式化 |
| `repomix-config` | `crates/config` | 全局与 CLI 配置加载、校验与默认值处理 |
| `repomix-mcp` | `crates/mcp` | MCP Server 实现，提供 LLM 可调用工具 |
| `repomix` (npm) | `npm/repomix-rs` | 基于 Node.js 的 CLI 包装，提供多语言构建脚本 |
| `repomix-core` -> `file` | `crates/core/src/file` | 文件系统操作：收集文件、过滤、截断、Base64 编码、搜索 |
| `repomix-core` -> `git` | `crates/core/src/git` | Git 仓库操作：差异分析、日志分析、远程处理 |
| `repomix-core` -> `metrics` | `crates/core/src/metrics` | 指标计算：Token 计数、行数统计 |
| `repomix-core` -> `output` | `crates/core/src/output` | 输出渲染：Markdown、Plain、JSON、XML 格式化 |
| `repomix-core` -> `security` | `crates/core/src/security` | 安全策略：Secretlint 检测、安全校验 |
| `repomix-core` -> `tree_sitter` | `crates/core/src/tree_sitter` | AST 构建与压缩：语言查询加载、压缩逻辑 |
| `repomix-mcp` -> `tools` | `crates/mcp/src/tools` | MCP 工具定义与实现 |

## 核心流程

### 1. CLI 启动与配置

加载用户运行 `repomix` 命令，传入路径与参数。`cli` 模块解析输入，`config` 模块读取全局 `~/.config/repomix/config` 与 CLI 覆盖参数，合并生成当前运行配置，并通过 Schema 验证。

### 2. 文件收集与处理 (Collection & Processing)

`core` 模块中的 `file` 子模块启动：- **collect**: 扫描指定路径下的文件列表。- **manipulate**: 应用 `pattern_utils` 进行模式匹配与过滤。- **process**: 对文件进行截断（limit tokens）、编码（text -> base64）。- **search**: 若配置启用搜索，使用 `tree_sitter` 进行代码搜索。- **process_content**: 组装文件元数据与内容。

### 3. AST 构建与压缩 (AST & Compression)

`core` 中的 `tree_sitter` 子模块：- 加载对应语言的 `.scm` 查询（Query）文件。- 对文件内容进行解析，生成 AST（语法树）。- `compress` 模块对 AST 进行压缩，移除注释行、空白行，提取关键代码节点。- 输出压缩后的代码树。

### 4. Git 差异分析 (Git Analysis)

若检测到 `.git` 目录，`core` 中的 `git` 子模块：- `diff`: 分析当前仓库与远程仓库的差异，提取变更文件。- `log`: 分析 git log，提取提交历史摘要。- `remote`: 获取远程仓库地址。- `sort`: 对差异文件进行排序。

### 5. 安全检测 (Security)

`core` 中的 `security` 子模块：- `secretlint`: 运行 Secretlint 检测配置文件（`.env` 等）中的敏感信息。- `validate`: 验证文件安全性策略。

### 6. 输出生成 (Output Generation)

`core` 中的 `output` 子模块：- 根据配置选择输出格式（markdown/json/plain/xml）。- `generate`: 组装文件列表、AST 数据、Git 信息、安全警告，渲染最终内容。- `split`: 若需分块输出，进行分段处理。- `decorate`: 应用 Markdown 样式等装饰。

### 7. MCP 工具封装

若启用 MCP，`mcp` 模块：- `server`: 初始化 MCP Server。- `params`: 校验工具参数。- `output_path`: 处理文件输出路径逻辑。- `helpers`: 通用辅助函数。- `tools`: 提供具体工具（如 `file_search`, `code_analysis` 等）。

## 技术选型

- **核心语言**: Rust。利用 Rust 的高性能、低内存占用特性，实现文件 IO、树构建、压缩等计算密集型任务。
- **构建工具**: `cargo` (Rust), `npm` (Node.js)。
- **AST 库**: `tree-sitter`。用于高性能代码语法分析，支持多语言。
- **模式匹配**: `glob` (Rust crate)。用于文件过滤与包含/排除规则。
- **安全检测**: `secretlint`。用于配置文件中的敏感信息检测。
- **输出渲染**: `pulldown-cmark` (Rust crate, Markdown), `serde_json` (JSON), `xml-rs` (XML)。
- **MCP**: 基于 `tower`/`hyper` 实现 MCP Server 协议。
- **模板引擎**: `handlebars` (Node.js).
- **打包工具**: `tar` (Rust crate) / `tar` (Node.js)。

## 系统边界

- **CLI vs Library**: `repomix` (CLI) 调用 `repomix-core` (Library) 和 `repomix-config`。CLI 不直接处理核心业务逻辑，仅作参数解析与流程编排。
- **Core vs Tree-sitter**: `repomix-core` 集成 `tree-sitter` crate。`core` 负责调用 API，`tree-sitter` 负责解析。
- **Rust vs Node.js**: Rust 核心逻辑 (`crates/`) 编译后作为 Node.js 模块 (`.node`) 被 `npm/repomix-rs` 加载。两者之间通过 **FFI** 或 **shared crate** 作为边界，但在本项目中，`npm` 部分更多是直接编译 Rust crate 为 NPM 包或调用编译后的二进制/动态库。
- **Core vs Security**: `secretlint` 作为外部插件或集成调用。`security` 模块负责调用 `secretlint` 并处理结果。
- **Core vs Git**: `git` 模块负责封装 `git CLI` 调用，处理 Git 仓库逻辑。
- **MCP**: `repomix-mcp` 作为一个独立的 MCP Server 实现，可嵌入任何 MCP 客户端。

## 代码映射索引

| 模块 | 文件路径 | 说明 |
| :--- | :--- | :--- |
| 根入口 | `Cargo.toml` | 项目 Cargo 配置，定义 crates |
| CLI 入口 | `crates/cli/src/main.rs` | CLI 程序入口 |
| CLI 主逻辑 | `crates/cli/src/run.rs` | 命令行参数解析与执行编排 |
| CLI 配置 | `crates/cli/src/prompts/mod.rs` | CLI 交互提示逻辑 |
| 配置 | `crates/config/src/lib.rs` | 配置模块入口 |
| 配置加载 | `crates/config/src/load.rs` | 读取全局与 CLI 配置 |
| 配置 Schema | `crates/config/src/schema.rs` | 配置结构体与验证逻辑 |
| 核心逻辑 | `crates/core/src/lib.rs` | Core 模块入口 |
| 文件收集 | `crates/core/src/file/collect.rs` | 文件目录扫描与收集 |
| 文件处理 | `crates/core/src/file/process.rs` | 文件截断、编码处理 |
| AST 构建 | `crates/core/src/tree_sitter/mod.rs` | Tree-sitter 集成入口 |
| AST 查询 | `crates/core/src/tree_sitter/queries/rust.scm` | Rust 语言查询文件 |
| AST 压缩 | `crates/core/src/tree_sitter/compress.rs` | AST 逻辑压缩 |
| Git 分析 | `crates/core/src/git/diff.rs` | Git 差异文件处理 |
| Git 日志 | `crates/core/src/git/log.rs` | Git 提交日志处理 |
| 输出 | `crates/core/src/output/generate.rs` | 内容生成与格式化 |
| 输出 Markdown | `crates/core/src/output/styles/markdown.rs` | Markdown 样式生成 |
| 安全 | `crates/core/src/security/validate.rs` | 文件安全校验 |
| MCP | `crates/mcp/src/server.rs` | MCP Server 初始化 |
| MCP 工具 | `crates/mcp/src/tools/mod.rs` | MCP 工具列表 |
| npm 包 | `npm/repomix-rs/package.json` | NPM 包定义 |
| npm 构建 | `scripts/publish-npm.mjs` | NPM 发布脚本 |
