# MAINTENANCE

> Last updated: 2026-06-11

## How to Keep This Knowledge Base Fresh

### Review Cadence

| File | When to update |
|------|---------------|
| `PROJECT-ESSENCE.md` | Quarterly, or on any major version change |
| `ARCHITECTURE.md` | Monthly, or after any crate restructure |
| `DECISIONS.md` | On every non-trivial design change or new tech choice |
| `DYNAMICS.md` | Immediately when an active issue is resolved or a new one opens |

### What to Check Each Session

- [ ] Any new crate added or removed from `Cargo.toml` → update `ARCHITECTURE.md`
- [ ] Any change to `pack()` signature or `RepomixConfig` → update `ARCHITECTURE.md` + `DECISIONS.md`
- [ ] Any new graceful/hard failure behavior → update `DYNAMICS.md`
- [ ] Any new crate dependency (libgit2, new tree-sitter language, etc.) → update `DECISIONS.md`

### Token Budget

Target total across all files: < 4,000 tokens. Each file should be under 150 lines unless the complexity genuinely warrants it.

### Sources of Truth

These files are always authoritative. When they change, verify `.ai-context/` stays in sync:

- `Cargo.toml` (workspace members, crate definitions)
- `crates/core/src/packager.rs` (pipeline order, `pack()` signature)
- `crates/config/src/schema.rs` (config shape)
- `crates/mcp/src/server.rs` (MCP tool list)
- `repomix-rs/litho.docs/` (existing C4 architecture documentation)
