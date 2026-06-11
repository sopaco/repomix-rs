# ai-context for repomix-rs

> Last updated: 2026-06-11

## Activation Rules

Activate this context when any of the following apply:

- Working on any file under `crates/` (core, config, shared, cli, mcp)
- Tracking architecture, design decisions, or active constraints
- Creating new features or fixing bugs in this repo
- First session with this project and you need to understand it

## Quick Map

| File | Topic | Read when... |
|------|-------|--------------|
| `references/PROJECT-ESSENCE.md` | What this is, who it's for | You need context on purpose, features, constraints |
| `references/ARCHITECTURE.md` | System layout, data flow, concurrency | You need to understand how the crates relate or how a pack run works |
| `references/DECISIONS.md` | Key design choices and trade-offs | You're changing behavior in core, mcp, or cli |
| `DYNAMICS.md` | Active constraints, known issues, open risks | Before making changes that touch those areas |

## What Is This Repo

`repomix-rs` packs an entire codebase into a single AI-friendly file. Key entry points:

- **`crates/cli/src/main.rs`** — CLI binary (`repomix`)
- **`crates/mcp/src/server.rs`** — MCP stdio server (`repomix --mcp`)
- **`crates/core/src/packager.rs`** — core `pack()` orchestrator
- **`crates/config/src/`** — `RepomixConfig`, layered loading, schema
