---
type: agent_context
project: repomix-rs
title: Agent Architecture Context
source: /Users/bjsttlp485/Workspace/SAW/repomix-rs
---

## 项目概览

repomix-rs 是一个用 Rust 重写的高性能代码打包工具。它的目标是将复杂的代码库（包含数百甚至数千个文件）转化为 AI 能够一次性理解的结构化文档。**核心价值**：- **极速**：相比原版 TypeScript 实现的 Repomix (5000 文件项目约 15 秒)，repomix-rs 利用 Rust 的编译型特性将打包时间压缩至 **2-3 秒**。- **结构化**：输出包含目录树、文件内容、Token 计数、按活跃度排序的文件排名、安全扫描报告及 Git 变更记录。- **多格式**：支持 XML（默认，含元数据）、Markdown（人可读）、JSON（易处理）和 Plain（轻量）四种格式。

## 架构设计

系统采用单 binary（`repomix-rs`）启动模式，核心逻辑封装在 `crates/` 下的各个库中，通过 CLI 参数接收输入。

```mermaid
graph LR
    subgraph Entry [入口 CLI]
        A[接收目录或 Git URL]
    end

    subgraph Pipeline [核心流水线]
        B[配置加载与验证]
        C[文件扫描与清理]
        D[文件内容处理 & Token 计算]
        E[Git 变更提取]
        F[安全扫描 Secretlint]
        G[输出组装与格式化]
    end

    subgraph Storage [输出与缓存]
        H[磁盘临时文件]
        I[最终打包文件 .xml/.md/.json]
        J[临时文件清理]
    end

    A --> B
    B --> C
    C --> D
    D --> E
    E --> F
    F --> G
    G --> H
    H --> I
    I --> J
```

#

## 核心流程

 (核心逻辑)

1. **初始化与输入解析**
   - 接收根目录路径或 Git URL。如果是 Git URL，先进行克隆。
   - 加载全局配置（`.mind-mesh` 覆盖）。

2. **文件遍历与过滤**
   - 递归遍历目录树。
   - 应用忽略规则（`.gitignore` 等）过滤掉文件。
   - **安全扫描**：对所有文件内容调用 `secretlint`，识别并剔除包含敏感信息（API Key、密码等）的文件。
   - **活跃度排序**：计算每个文件的 Git 修改历史（最近一次提交时间），按时间倒序排列文件列表。

3. **智能打包 (Packaging)**
   - **Tree-sitter 解析**：对保留的文件进行语法解析，提取符号（函数、类、变量）及类型信息。
   - **智能压缩 (Truncation)**：仅保留 `function_signature`、`type_implementation` 和 `mod` 信息，去除高耗时的函数体实现。
   - **Token 计数**：统计剩余代码的 Token 数量。

4. **内容组装**
   - 构建目录树结构（DirectoryTree）。
   - 填充每个文件的压缩内容、Token 数、行号范围。
   - 添加 Git 摘要（变更文件及提交信息）。

5. **格式渲染**
   - 根据用户指定的 `--format` 参数，组装成 XML/Markdown/JSON/Plain。
   - 输出最终文件。

## 模块地图

| Module | Path | 职责 | 依赖 |
| :--- | :--- | :--- | :--- |
| CLI | `crates/cli/` | 命令解析、入口点、Spinner UI | shared, core, config, mcp |
| Config | `crates/config/` | 加载全局/本地配置、Schema 验证 | shared |
| Core | `crates/core/` | 核心业务逻辑：文件处理、打包算法 | tree-sitter-rs, git2 |
| | `file/` | 文件收集、内容读取、Tree-sitter 集成 | |
| | `metrics/` | 文件 Token 统计、计算大小 | |
| | `output/` | 结果生成与格式化渲染 | |
| | `security/` | Secretlint 扫描、敏感信息检测 | |
| | `tree_sitter/` | 代码语法分析与压缩策略 | tree-sitter-rs |
| MCP | `crates/mcp/` | MCP Server 服务实现 | shared, core |
| | `tools/` | MCP 标准工具定义 | |
| Shared | `crates/shared/` | 公共类型定义、日志、工具函数 | |
| Utils | `scripts/` | 构建、发布、版本检查脚本 | |

#

## 代码映射索引

| Module | Path |
| :--- | :--- |
| CLI | `crates/cli/src/main.rs` |
| CLI | `crates/cli/src/prompts/mod.rs` |
| CLI | `crates/cli/src/report.rs` |
| CLI | `crates/cli/src/run.rs` |
| CLI | `crates/cli/src/spinner.rs` |
| Config | `crates/config/src/load.rs` |
| Config | `crates/config/src/schema.rs` |
| Core | `crates/core/src/packager.rs` |
| Core | `crates/core/src/file/collect.rs` |
| Core | `crates/core/src/file/process.rs` |
| Core | `crates/core/src/file/truncate_base64.rs` |
| Core | `crates/core/src/metrics/calculate.rs` |
| Core | `crates/core/src/output/generate.rs` |
| Core | `crates/core/src/security/secretlint.rs` |
| Core | `crates/core/src/tree_sitter/compress.rs` |
| MCP | `crates/mcp/src/server.rs` |
| Shared | `crates/shared/src/types.rs` |

## 系统边界

| Entity | Boundary | Relationship |
| :--- | :--- | :--- |
| repomix-rs | Local File System / Remote Git | Input Source |
| Config Loader | Local `Config` struct | Dependency |
| File Processor | `File` struct & `Content` | Dependency |
| Packager | `Packager` struct | Orchestration |
| Token Calculator | `Metrics` struct | Dependency |
| Secret Scanner | `Secretlint` (External) | External Service |
| Syntax Analyzer | `Tree-sitter` (External) | External Service |
| Output Renderer | `Output` struct | Dependency || MCP Server | External Client (Agent) | Dependency |

## 技术选型

| 技术 | 版本/特性 | 用途 | 替代方案 || :--- | :--- | :--- | :--- || **语言** | Rust 2021 Edition | 整体开发语言 | Node.js (原版 Repomix) || **构建工具** | Cargo | 包管理与编译 | npm (原版) || **依赖管理** | tree-sitter-rs | 多语言 AST 解析 | || | git2 | Git 历史操作 | || | secretlint | 敏感信息检测 | || **设计模式** | Command Pattern | CLI 参数处理 | || | Builder Pattern | Config 构建 | || **序列化** | serde | 配置与输出处理 | || **并发模型** | Rayon | 文件遍历/Token 统计 | |```
