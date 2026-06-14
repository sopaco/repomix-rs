# Agents Guide

This file guides AI coding agents working in this repository.


<!-- mind-mesh:begin env-overview v1 -->
## AI 工程环境（MindMesh）

本仓库由 MindMesh 配置了 AI 工程环境。Coding Agent 请遵循以下约定：

- **知识资产**位于 `.mind-mesh/`（架构上下文、私域知识、源码索引摘要）
- **Skills** 位于 `.agents/skills/`（由 MindMesh 注入，可按需重新集成）
- **工作流**：先读知识 → 再查关系 → 最后读源码；shell 输出优先走 RTK
<!-- mind-mesh:end env-overview -->

<!-- mind-mesh:begin knowledge-guide v1 -->
## MindMesh 知识资产

Coding Agent **必须先加载** `mind-mesh-knowledge-skill`，并按其中分层策略查询 `.mind-mesh/`。

| 层级 | 路径 | 何时使用 |
|------|------|----------|
| 架构 | `.mind-mesh/agent/context.md` | 模块划分、核心流程、系统边界 |
| 私域 | `.mind-mesh/knowledge/` | 业务术语、内部框架/API/脚手架 |
| 源码 | repomix（见 `repomix-context-skill`） | 实现细节、跨文件符号（本地索引，不入库） |
| 关系 | codegraph CLI（见 `codegraph-skill`） | 调用链、依赖关系、影响分析 |

**原则**：先宏观后微观；优先读已生成文档，再 grep 源码索引。
<!-- mind-mesh:end knowledge-guide -->

<!-- mind-mesh:begin skills v1 -->
### 可用 Skills

| Skill | 用途 |
|-------|------|
| `mind-mesh-knowledge-skill` | 读取 `.mind-mesh/` 知识分层与查询顺序 |
| `repomix-context-skill` | grep 本地 repomix 索引（`.mind-mesh/agent/repomix.md`） |
| `codegraph-skill` | `bunx codegraph query/callers/callees/impact` |
| `rtk-skill` | shell 命令优先 `bunx rtk <cmd>`，压缩输出 |
<!-- mind-mesh:end skills -->

<!-- mind-mesh:begin tools v1 -->
### 工具链

| 工具 | 用法 | 场景 |
|------|------|------|
| MindMesh 知识 | 加载 `mind-mesh-knowledge-skill` | 架构、私域知识 |
| Repomix | 见 `repomix-context-skill` | 源码片段（本地索引） |
| Codegraph | `bunx codegraph query/callers/callees/impact` | 符号关系、影响分析 |
| RTK | `bunx rtk <cmd>` | 压缩 git/test/build 等 shell 输出 |

**注意**：不要运行 `codegraph install` 或 `rtk init`（MindMesh 已通过 AGENTS.md + Skills 配置）。
<!-- mind-mesh:end tools -->