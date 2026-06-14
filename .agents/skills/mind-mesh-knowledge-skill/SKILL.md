---
name: mind-mesh-knowledge-skill
description: Use when a coding agent needs project knowledge from MindMesh .mind-mesh/ assets. Guides layered reading of context, private knowledge, and repomix index.
version: 1.0.0
---

# MindMesh Knowledge Skill

MindMesh stores **AI knowledge assets** under `.mind-mesh/` in this repository.

## Knowledge layers (mandatory order)

1. **Architecture** — `.mind-mesh/agent/context.md`
   - Module map, core flows, system boundaries, tech stack
   - Check `.mind-mesh/agent/context-meta.json` or `meta.json` for freshness

2. **Private domain** — `.mind-mesh/knowledge/**/*.md`
   - Business glossary, internal frameworks, APIs, scaffolding guides
   - Team-maintained markdown; read in filename sort order when surveying

3. **Structured meta** — `.mind-mesh/agent/meta-inputs.md`
   - Compiled from `mind-mesh-meta.json` and knowledge scans

4. **Source index** — see `repomix-context-skill`
   - Local `.mind-mesh/agent/repomix.md` (gitignored; regenerate via MindMesh scan)

## Query workflow

```
Task received
  → Read context.md (or relevant section)
  → If business/internal terms → scan knowledge/
  → If implementation detail → repomix-context-skill
  → If call graph / impact → codegraph-skill
```

## Rules

- **Do not** invent module names that contradict `context.md` or `meta-inputs.md`
- **Do not** read the entire live repository tree when indexed assets exist
- **Do not** load full `repomix.md` into context — grep slices only
- Prefer `.mind-mesh/` over guessing project structure

## Private knowledge directory

`.mind-mesh/knowledge/` — developers add markdown here; MindMesh scans on context generation.

Example files:
- `00-glossary.md` — domain terms
- `10-internal-framework.md` — internal libs
- `20-api-usage.md` — internal APIs
- `30-scaffolding.md` — project generators

## Human docs (optional)

`.mind-mesh/human/` — Litho-generated docs for humans; useful for onboarding context but denser than `context.md`.
