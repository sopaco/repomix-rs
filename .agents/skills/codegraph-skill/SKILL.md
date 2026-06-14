---
name: codegraph-skill
description: Use when a coding agent needs symbol relationships, callers, callees, or change impact. Guides Codegraph CLI usage (not MCP).
version: 1.0.0
---

# Codegraph Skill

[Codegraph](https://colbymchenry.github.io/codegraph/) provides a pre-indexed code graph for this project.

## Prerequisites

- Installed via `bun add -d @colbymchenry/codegraph`
- Project indexed: `.codegraph/` (run `bunx codegraph init -i` if missing)

## CLI commands (use `bunx codegraph …`)

| Intent | Command |
|--------|---------|
| Find symbol by name | `bunx codegraph query <name>` |
| Who calls X | `bunx codegraph callers <symbol>` |
| What X calls | `bunx codegraph callees <symbol>` |
| Change blast radius | `bunx codegraph impact <symbol>` |
| Tests affected by file changes | `bunx codegraph affected <files…>` |
| Index health | `bunx codegraph status` |
| Refresh after edits | `bunx codegraph sync` |

## When to use vs MindMesh knowledge

| Use Codegraph | Use MindMesh `.mind-mesh/` |
|---------------|---------------------------|
| Symbol lookup, call chains | Architecture, modules, business rules |
| Impact before refactor | Private domain knowledge |
| File/symbol relationships | High-level flows and boundaries |

## Workflow

1. Load `mind-mesh-knowledge-skill` first for architectural context
2. Use `codegraph query` to locate symbols
3. Use `callers` / `callees` / `impact` for relationship questions
4. Use `repomix-context-skill` for full source slices when needed

## Do not

- Run `codegraph install` (configures MCP/agents — MindMesh manages AGENTS.md)
- Re-verify Codegraph AST results with blind `grep` across the whole repo
- Chain `query` + manual reads when `impact` or `explore` intent is clear

## Staleness

If `codegraph status` reports pending files, run `bunx codegraph sync` before structural queries.
