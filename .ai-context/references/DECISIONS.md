# DECISIONS

> Last updated: 2026-06-11

## Key Design Choices

### 1. System `git` CLI instead of `libgit2`
**Chosen:** Shell out to `git` via `std::process::Command`.
**Rationale:** Smaller binary, no ABI compatibility burden, no extra Rust dependency.
**Trade-off:** Git must be on `PATH`. Features silently skip on non-git directories or missing git with a warning.
**Revisit when:** A user requests offline/bundled git support or git-free environments are a priority.

### 2. Search single-threaded, processing multi-threaded
**Chosen:** Single-threaded directory walk (using `ignore` crate), Rayon `par_iter` for per-file processing.
**Rationale:** `ignore` crate's parallel walker doesn't honor glob rules correctly in this project's use case. File search is I/O-light (filenames only); processing is CPU-heavy.
**Trade-off:** Slight latency at scale; negligible in practice.

### 3. In-memory-only pipeline
**Chosen:** All intermediate structs stored in heap-allocated `Vec`s and `HashMap`s.
**Rationale:** Simplicity, correctness (preserves ordering for git sort), easier testing.
**Trade-off:** RAM usage = sum of all file sizes. No streaming means large mono-repos may OOM.
**Revisit when:** A GB-scale repo case is a known user scenario.

### 4. MCP tool calls serialized via Mutex
**Chosen:** `RepomixMcpServer` holds `Arc<Mutex<()>>`; one `pack()` at a time.
**Rationale:** `pack()` writes to a shared temp directory and uses shared logger state. Serialization avoids race conditions without refactoring.
**Trade-off:** Single-agent MCP is fine; concurrent agent workloads will stall.
**Revisit when:** Multi-agent simultaneous tool calls are a requirement.

### 5. Layered config with `PartialConfig` semantics
**Chosen:** `RepomixConfig::load(partial, config_root)` overlays `PartialConfig` atop lower-level config. `None` means "don't override".
**Rationale:** Predictable CLI behavior — `repomix --compress .` doesn't accidentally change `style` to default.
**Trade-off:** Users must understand the layering model to debug unexpected config values.

## Known Constraints / Legacy

- **C# tree-sitter compression disabled.** `tree-sitter-c-sharp` 0.23 has an ABI mismatch with `queries/c_sharp.scm`. Tracked in `crates/core/Cargo.toml`.
- **Historical bug-fix comments (P0/P1/P2, Bug #N) are preserved in source.** Document the evolution but add noise. Cleanup is planned but not yet prioritized.
- **`target/` directory is gitignored** but includes sample output files (`repomix-output.*`) in `core/src/` for testing.
