---
name: repomix-context-skill
description: Use when an agent needs source code from the local repomix index under .mind-mesh/agent/repomix.md (not committed; regenerate via MindMesh scan).
version: 1.1.0
---

# Repomix Context Skill

MindMesh stores a **local repomix index** at `.mind-mesh/agent/repomix.md` (gitignored, fast to regenerate).

Read **architecture first** via `mind-mesh-knowledge-skill` → `.mind-mesh/agent/context.md`.

## When to use

- Implementation details after reading `context.md`
- Cross-file symbol search within the indexed snapshot
- Locating handlers, types, routes

## Query strategy (mandatory)

1. **Read meta** — `.mind-mesh/agent/meta.json` (`total_tokens`, `synced_at`, `top_files_by_tokens`)
2. **Grep the pack** — search `repomix.md` for symbols, paths, routes (never load entire file)
3. **Read slices** — extract matching `### path` sections only
4. **Refresh** — if `meta.json.synced_at` is stale, ask user to run MindMesh **Pack Context** / scan

## Paths

| File | Purpose |
|------|---------|
| `.mind-mesh/agent/repomix.md` | Full indexed snapshot (local only) |
| `.mind-mesh/agent/meta.json` | Pack metrics |
| `.mind-mesh/agent/context.md` | Architecture (read first) |

## Do not

- Commit or assume `repomix.md` exists in git
- Read the live repository tree when the pack covers the question
- Dump all of `repomix.md` into context

## MindMesh desktop / CLI

If `repomix.md` is missing, regenerate with MindMesh scan or:

```bash
mind-mesh assets pack-agent <repo-path>
```
