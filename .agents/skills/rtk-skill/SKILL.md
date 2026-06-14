---
name: rtk-skill
description: Use when running shell commands (git, test, build, package managers). RTK filters output to save 60-90% tokens.
version: 1.0.0
---

# RTK Skill (Rust Token Killer)

[RTK](https://github.com/rtk-ai/rtk) compresses CLI command output before it reaches the LLM.

## Verify installation

```bash
bunx rtk gain
```

Must show token savings stats. If it fails, the wrong `rtk` package may be installed.

## Usage

Prefix shell commands with `rtk` when available on PATH, or via `bunx`:

```bash
rtk git status
rtk git log --oneline -20
# or if only project-local:
bunx rtk git status
```

## When to use RTK

| Use RTK | Use MindMesh / Codegraph instead |
|---------|----------------------------------|
| git status/log/diff | Architecture (`context.md`) |
| test/build output | Symbol relationships (codegraph) |
| package manager output | Source code slices (repomix) |

## Built-in Read/Grep tools

IDE built-in `Read`/`Grep` may bypass shell hooks. For token savings on file reads, prefer:

```bash
bunx rtk read path/to/file.rs
# or shell equivalents: head, tail, rg via rtk
```

## Do not

- Run `rtk init` or `rtk init -g` — MindMesh configures guidance via AGENTS.md (no global hook)
- Use RTK for reading `.mind-mesh/agent/context.md` — read that file directly (it's already dense)

## Complements other skills

- **mind-mesh-knowledge-skill** — what to read
- **codegraph-skill** — structural queries
- **rtk-skill** — how to run shell commands efficiently
