# PROJECT-ESSENCE

> Last updated: 2026-06-11

## What It Is

`repomix-rs` is a Rust re-implementation of [Repomix](https://github.com/yamadashy/repomix) that packs an entire codebase into a single, LLM-friendly context file.

**One sentence:** A code-to-text compressor for sending large projects to language models.

## Why It Exists

Sending large projects to an LLM requires either manually copying files or writing scripts. Repomix automates this: it walks the codebase, applies tree-sitter compression (stripping function bodies, keeping signatures), detects secrets via Secretlint, and renders the result in XML, Markdown, plain text, or JSON.

## Who It's For

- **Developers** — who want to feed a local codebase into an LLM without copying files one by one.
- **AI agents** — that need programmatic, MCP-standard access to pack a codebase or repo and consume the result.
- **CI pipelines** — that need a deterministic, headless pack and output artifact.

## Key Features

- Four output styles: XML (default), Markdown, Plain, JSON.
- Tree-sitter compression: function/class bodies replaced with signatures (10 languages).
- Token counting via `tiktoken-rs` (o200k_base / GPT-4o family).
- Secretlint security scan: secret-bearing files excluded from output.
- Git-aware: optional diff, log, and sort-by-change-frequency (uses system `git`).
- Parallel file processing via Rayon; async I/O via Tokio.
- Layered config: defaults → global `~/.repomix/repomix.config.json` → project `./repomix.config.json` → CLI flags.
- Two interfaces: standalone CLI binary and MCP stdio server.

## Core Constraints

- **All processing runs entirely in memory.** No streaming, no mmap, no zero-copy pipe. RAM usage ≈ sum of all file contents. This is a soft ceiling for mono-repos at GB scale.
- **Git is an external dependency.** The `git` CLI must be on `PATH` for diff/log/sorter features. Non-git directories degrade gracefully.
- **C# tree-sitter compression is disabled** (ABI mismatch in `tree-sitter-c-sharp` 0.23 with bundled queries). Known issue, fix tracked in `crates/core/Cargo.toml`.
- **Status: 0.1.0 — schema and API surface are still evolving.**
