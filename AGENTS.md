# AGENTS.md — Working Handbook for `repomix-rs`

> This file is the **operational** guide for coding agents. It explains how to work
> with this codebase efficiently. Big-picture context lives in `.ai-context/` (see
> "Where to find context" below). The two documents are intentionally non-overlapping.

---

## 1. Where to find context

Before writing any code, read these in this order:

1. `.ai-context/SKILL.md` — activation rules and a table of which `.ai-context/*` file
   to read for each type of question.
2. `.ai-context/references/PROJECT-ESSENCE.md` — what this project provides and its hard
   constraints (e.g., all processing is in-memory).
3. `.ai-context/references/ARCHITECTURE.md` — crate dependency graph, the 7-stage pipeline,
   and the data types that flow between stages.
4. `.ai-context/references/DECISIONS.md` — every non-obvious design choice and its
   trade-off. Read before changing behavior in `core`, `cli`, or `mcp`.
5. `.ai-context/DYNAMICS.md` — active constraints and known issues (e.g., C# tree-sitter
   disabled, MCP serialization caveat).

> **Do not regenerate `.ai-context/` unless drift is confirmed.** Use `meta/MAINTENANCE.md`
> as a checklist before doing so.

---

## 2. Pre-existing scoped assets

Three skill directories live under `.agents/skills/`. Activate them by name; they contain
detailed instructions beyond the summary here.

| Skill | Activate when | One-line purpose |
|---|---|---|
| `ai-context-generator` | `.ai-context/` is stale or the user asks to regenerate it | Regenerates `.ai-context/` from source-of-truth files |
| `repomix-rs-explorer` | "Analyze this repo", "what's the structure", patterns discovery across many files | Packs this repo with `repomix` then reads the output |
| `litho-documents-skill` | User asks to "generate project documentation", "generate C4 architecture docs", or any of the Chinese trigger phrases | Produces `litho.docs/*.md` C4 documentation autonomously |

Use the skill's `SKILL.md` as the authoritative instruction set; the table above is only
a trigger reminder.

---

## 3. Project conventions (must-follow)

Conventions come from `README.md § 项目规范` and from the source itself.

### 3.1 Crate boundary rules

- `repomix-core` owns the pipeline. **All real work** (search, collect, process, output,
  metrics, git) lives here. `cli` and `mcp` are thin wrappers.
- `repomix-config` owns `RepomixConfig`, `PartialConfig`, the layered load, and
  `default_ignore_patterns()`. **Never hard-code an ignore pattern outside this crate.**
- `repomix-shared` owns cross-crate types (`RawFile`, `ProcessedFile`,
  `SuspiciousFileResult`, `SkippedFileInfo`, `ValidationResult`) and the tracing logger.
  Types shared by ≥ 2 crates must live here.
- `repomix-cli` and `repomix-mcp` must not import each other (would create a cycle).

### 3.2 All public APIs re-exported from crate root

`repomix-core` and `repomix-config` re-export their public surface from the crate root.
Library consumers must not need to know internal module paths.

### 3.3 Config loading contract

```
CLI flags  →  PartialConfig  →  RepomixConfig::load(partial, config_root)
                                           ↓
                            defaults  →  global  →  project  →  CLI
```

`PartialConfig` fields set to `None` mean "keep the layer below". This is the only safe
way to implement CLI flags that should not force defaults over user config files.

Side-effect: **do not** apply a `PartialConfig` default value before calling `load()`;
doing so would overwrite user-set values in lower layers.

---

## 4. Development commands

```bash
# Build the whole workspace
cargo build

# Run all tests
cargo test

# Lint (treat warnings as errors — CI gate)
cargo clippy --workspace --all-targets -- -D warnings

# Format
cargo fmt --all

# Release CLI binary
cargo build --release -p repomix-cli

# Run pack against the current workspace directory
cargo run -p repomix-cli --release -- .

# Start MCP server (useful for manual testing with an MCP client)
cargo run -p repomix-cli -- --mcp
```

**Before submitting a PR:** run `clippy` and `cargo fmt --all -- --check`.

---

## 5. Code style

- **Edition:** Rust 2024 for all crates.
- **Error handling:** Use `anyhow::Error` inside `cli` and `core` internal functions.
  Public `repomix-core` errors use `thiserror`.
- **Async:** `pack()` is `async` (Tokio). Internal file I/O uses `tokio::fs`. CPU-bound
  processing delegates to Rayon via `par_iter`. Do not mix blocking calls in async
  contexts without `spawn_blocking`.
- **Logging:** Use `tracing` / `tracing::warn!`, never `eprintln!`.
- **Comments:** The codebase preserves P0/P1/P2 and `Bug #N` inline comments that document
  why non-obvious code exists. Do not remove these without also removing the corresponding
  workaround.
- **Naming in `core/src/`:** submodules are flat in `core/src/` (e.g., `file/search.rs`,
  `file/collect.rs`, `security/validate.rs`). Follow this pattern for new submodules.

---
