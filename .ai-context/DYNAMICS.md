# DYNAMICS

> Last updated: 2026-06-11

## Active Development

- **Status: 0.1.0** — CLI, library API, and MCP server are usable. Configuration schema may still change before 1.0.

## Active Constraints

- **All processing is in-memory.** Expect soft failures (OOM) on multi-GB mono-repos. No workaround currently.
- **C# tree-sitter compression disabled.** No workaround provided; files in C# will skip compression silently.
- **MCP serialized via Mutex.** Concurrent agent requests queue, not parallelize. Likely acceptable at current usage levels.

## Graceful Degradation (Expected Behavior)

These failures do NOT abort the pack run; they print a warning and skip the step:

- `git` not on `PATH` or directory is not a git repo → git steps (diff, log, sort) skipped; warning logged.
- `tiktoken-rs` initialization fails (e.g., offline) → falls back to whitespace-split token estimation; warning logged.
- Secretlint hits rules on a file → file goes to `suspicious_files`, excluded from output; other files unaffected.
- File exceeds `max_file_size` → recorded as `SkippedFileInfo`, processing continues.

These failures DO abort:

- `file::search` I/O error → hard failure, error propagated to caller.
- Remote `git clone` failure → hard failure, error propagated with git error message.
- MCP tool execution error → wrapped in `rmcp::Error`, returned over JSON-RPC.

## Recently Resolved (Brief)

- TempDirGuard RAII cleanup implemented to avoid `/tmp` leaks on remote clone.
- `PartialConfig` layering semantics fixed to use `None` = keep-lower, not default-override.

## Open Questions / Risks

- Schema stability: `RepomixConfig` shape may change before 1.0. Library users should pin versions.
- C# tree-sitter: fix pending; no ETA in code.
- MCP concurrency: not yet stress-tested under multi-agent load.
