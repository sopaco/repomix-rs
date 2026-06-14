---
type: agent_context
project: repomix-rs
title: Agent Architecture Context
source: /Users/bjsttlp485/Workspace/SAW/repomix-rs
---

## 项目概览

repomix-rs 是一个用 Rust 重写的 **Repomix** 工具，旨在将代码库转化为 AI 可一次性摄入的结构化文档。它接收一个项目目录或远程 Git 仓库，输出包含目录树、文件内容、精确 token 计数、活跃度排序及安全扫描结果的“项目全景图”。**核心价值**：解决大语言模型上下文窗口限制问题。将原本耗时 15 秒（TS 版）的处理过程压缩至 2-3 秒（Rust 版），适用于 CI/CD 流水线、MCP 服务器等实时场景。

## 架构设计

系统基于 **Rust** 构建，采用模块化 crate 设计，通过 CLI 层统一调度，核心处理逻辑在 `core` 中完成。### C4 系统全景- **边界层**：CLI (`cli`) 接受 `--path` 或 `--git-url` 参数，负责输入解析、缓存管理与输出写入。- **配置层**：`config` 负责加载用户本地/全局配置及内置默认规则，解析为 JSON Schema。- **核心引擎**：`core` 是系统的主体，包含文件收集、git 差异分析、Tree-sitter 语法压缩、Token 统计及多格式输出生成。- **外部服务**：通过 MCP 协议或本地命令行交互；依赖 Tree-sitter CLI 进行二进制语法压缩。### 核心处理逻辑1. **输入阶段**：CLI 识别来源（本地目录/Git URL），调用 Git 获取远程代码（如适用）。2. **预处理**：加载全局配置，对文件进行安全扫描（Secretlint），过滤敏感信息。3. **核心解析**：遍历文件，通过 Tree-sitter 提取语法结构（压缩函数体），计算 token 计数。4. **排序与合并**：根据 Git 活跃度或统计权重对文件进行排序，组装成结构化的 JSON 中间件。5. **输出渲染**：根据用户请求格式（XML/Markdown/Plain/JSON）渲染最终文档。

## 模块地图

| 模块 | 功能简述 | 核心逻辑 |
|------|---------|---------|
| **cli** | 命令入口 | 参数解析、工作流调度、文件缓存管理 |
| **config** | 配置解析 | 加载默认规则、验证用户规则、Schema 校验 |
| **core** | 核心引擎 | 文件收集、语法压缩、token 统计、输出生成 |
| **mcp** | MCP 服务 | MCP 协议实现、参数转换、响应处理 |
| **shared** | 公共基础 | 日志、类型定义、正则工具类 |

## 核心流程

### 1. CLI 入口与参数解析
用户执行 `repomix-rs` 命令，传入路径或 Git URL。CLI 解析 `--output-format`（XML 默认）、`--git-url` 等参数。若为 Git URL，自动触发 git 拉取流程并暂存二进制结果。
### 2. 配置加载
从指定路径加载用户配置，合并全局默认配置，生成最终解析后的配置对象（`Config`）。
### 3. 安全扫描（并行）
CLI 启动多个子进程，对所有输入目录进行安全扫描。使用 Secretlint 正则检测并熵值检测。仅保留无敏感信息的文件清单。
### 4. 文件收集与树构建
遍历剩余安全文件，构建文件树结构，标记文件属性（大小、行数等）。
### 5. Git 差异分析（可选）
若 Git URL 有效，运行 Git 命令获取最近提交历史，分析文件变更频率，生成活跃度评分。
### 6. 核心压缩与统计
遍历每个文件，调用 Tree-sitter CLI。- **压缩逻辑**：解析 AST，提取函数/类定义及类型声明（头部），保留 `fn`/`impl`/`class` 关键字及参数，丢弃完整函数体逻辑。- **Token 统计**：基于压缩后的代码内容，调用统计工具计算 token 数。
### 7. 结果组装
将所有文件数据（路径、压缩内容、token 数、活跃度、git hash）合并为 JSON 数组。
### 8. 多格式渲染
根据输出格式：- **JSON**：直接输出组装后的数据。- **XML**：将 JSON 结构转换为 XML Schema 结构。- **Markdown**：渲染目录树及文件内容。- **Plain**：仅输出核心摘要。

## 技术选型

| 领域 | 选型 | 理由 |
|------|------|------|
| **语言** | Rust | 编译型语言，处理大文件时内存与速度性能远超 TypeScript。 |
| **语法解析** | Tree-sitter | 提供增量、高效的增量/完整解析器，支持多种语言 AST。 |
| **安全扫描** | Secretlint | 社区成熟规则集，支持正则与熵检测，可白名单。 |
| **输出渲染** | 自研模板引擎 | 实现 JSON/XML/Markdown/Plain 四种格式转换。 |
| **Git 交互** | `git binary` | 直接调用 Git 二进制命令解析历史，避免 Node.js 版本兼容问题。 |

## 系统边界

| 维度 | 边界描述 |
|------|---------|
| **输入** | 本地目录路径；Git 远程 URL (HTTP/Ssh)。 |
| **输出** | 结构化文档（JSON/XML/Markdown/Plain）。 |
| **依赖** | Rust 编译环境；Tree-sitter 二进制文件；Git 命令行工具。 |
| **范围** | 仅包含代码文件及特定配置文件；排除 `target/`、`.git/`、`node_modules/` 等噪声目录。 |

## 代码映射索引

| 代码位置 | 说明 |
|---------|------|
| `crates/core/src/lib.rs` | 核心库入口，调度各模块 |
| `crates/core/src/packager.rs` | 打包逻辑，处理输入输出目录结构 |
| `crates/core/src/security/mod.rs` | 安全扫描模块入口 |
| `crates/core/src/security/secretlint.rs` | Secretlint 扫描规则实现 |
| `crates/core/src/tree_sitter/compress.rs` | Tree-sitter AST 压缩逻辑 |
| `crates/core/src/output/mod.rs` | 多格式输出渲染器入口 |
| `crates/config/src/lib.rs` | 配置解析入口 |
| `crates/mcp/src/lib.rs` | MCP 服务端入口 |
| `crates/mcp/src/helpers.rs` | MCP 工具辅助函数 |
| `src/main.rs` | CLI 主入口 |
| `Cargo.toml` | 项目依赖管理 |
| `README.md` | 项目文档 |
| `.gitignore` | 忽略规则定义 |
