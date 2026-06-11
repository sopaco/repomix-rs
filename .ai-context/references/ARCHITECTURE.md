# ARCHITECTURE

> Last updated: 2026-06-11

## Layout

This is a five-crate Cargo workspace (`resolver = "2"`).

```
repomix-cli      CLI binary (clap derive, --init, --mcp, run_pack)
repomix-mcp      MCP stdio server (rmcp 1.7), four tools, mutex-serialized
    |
    v
repomix-core     pack() orchestrator + 6 submodules
                 (file, tree_sitter, security, output, metrics, git)
    |
    +---> repomix-config   RepomixConfig, RepomixConfig::load, PartialConfig,
                           default ignores, global dir resolution
    +---> repomix-shared   Cross-crate types (RawFile, ProcessedFile,
                           SuspiciousFileResult, logger)
```

Style: **Hexagonal / Ports and Adapters**. CLI and MCP are driving adapters. Core is the port + business logic. File system and git CLI are driven ports.

## The Pipeline (core::pack)

```
search  → collect → validate → process → [git sort] → [git diff/log] → output → metrics
```

`pack(root_dirs, config, progress)` is the single orchestrating call.
- `search`: walks directories, applies .gitignore + custom + default ignore patterns.
- `collect`: reads file contents into `Vec<RawFile>`, tracks skipped files.
- `validate`: runs Secretlint rules; classifies files as safe or suspicious.
- `process`: tree-sitter compression + comment removal + line truncation + token counting. **This is the parallel step** — uses Rayon `par_iter`.
- `git sort` (optional): reorders by recency.
- `git diff` / `git log` (optional): appends these as separate sections.
- `output`: renders the final file(s) in the chosen style; handles split-output.
- `metrics`: computes totals and top-N file breakdown.

## Concurrency

| Area | Mechanism | Scope |
|------|-----------|-------|
| File content processing | Rayon `par_iter` | Per-file tree-sitter + token count |
| MCP tool calls | `Arc<Mutex<()>>` | Serializes concurrent agent requests |
| Main pipeline | Single Tokio task | Sequential by design |

## Key Interfaces

- **`ProgressCallback`** trait — `on_progress`, `on_complete`, `on_error`. Decouples UI from core. CLI prints spinners; MCP could emit JSON-RPC events.
- **`PackOptions`** — builder-pattern wrapper for `pack()`. Library users don't need to manually construct `RepomixConfig`.
- **`PartialConfig`** — CLI overlay for layered config; `Some(v)` overrides, `None` keeps lower layer.

## What Flows Where

All intermediate data structures pass in memory only. There is no zero-copy buffer, no mmap, no streaming writer. Output is written to disk once, as a final step.
