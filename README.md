# repomix-rs

A Rust implementation of [Repomix](https://github.com/yamadashy/repomix) — a tool that packs your entire codebase into a single, AI-friendly file. It is a drop-in replacement written for speed, safety, and embedding into AI agents via the Model Context Protocol (MCP).

> **Status:** `2.0.0` — under active development. CLI, library, and MCP server are usable; configuration schema may still evolve.

---

## Features

- **Multiple output formats** — XML (default), Markdown, Plain text, JSON
- **Tree-sitter compression** — extract code signatures while stripping implementation bodies (10 languages)
- **Git-aware output** — sort by change frequency, include `git diff` and `git log` (via the system `git` CLI)
- **Token counting** — accurate counts via `tiktoken-rs` (`o200k_base` by default, GPT-4o family)
- **Security scanning** — detect and exclude files containing secrets via Secretlint
- **Parallel processing** — `rayon` for file collection, `tokio` for I/O
- **Layered configuration** — defaults → `~/.repomix/repomix.config.json` → `./repomix.config.json` → CLI flags
- **Two consumption modes** — standalone CLI binary **and** an `rmcp`-based MCP server for AI agents

---

## Workspace layout

This repository is a Cargo workspace with five crates:

| Crate | Purpose |
|---|---|
| `repomix-core` | Library: file collection, processing, tree-sitter compression, metrics, output generation, git operations |
| `repomix-config` | Typed configuration schema, default ignore patterns, global config path resolution, layered `RepomixConfig::load` |
| `repomix-shared` | Cross-crate types (`ProcessedFile`, `SuspiciousFileResult`, …) and the tracing-based logger |
| `repomix-cli` | The `repomix` binary (clap-based) |
| `repomix-mcp` | The MCP server exposing `pack_codebase`, `pack_remote_repository`, `read_repomix_output`, `grep_repomix_output` |

---

## Installation

### npm (recommended)

Install the Rust build from npm. The npm package is named **`repomix-rs`** (to distinguish it from the [original TypeScript Repomix](https://www.npmjs.com/package/repomix)); the terminal command is **`repomix`**.

```bash
# Global install → `repomix` on your PATH
npm install -g repomix-rs

# One-off run (no global install)
npx repomix-rs .

# MCP server for AI agents
npx -y repomix-rs --mcp
```

Supported platforms: Linux (x64, arm64), macOS (x64, arm64), Windows (x64).

> If both `repomix` (TypeScript) and `repomix-rs` (Rust) are installed globally, the last install wins for the `repomix` command. Install only the one you need, or use `npx repomix-rs` / `npx repomix` explicitly.

### From source

```bash
# Install the CLI to ~/.cargo/bin/repomix
cargo install --path crates/cli

# Or build a release binary in ./target/release/repomix
cargo build --release
```

The Cargo package is `repomix-cli`; a `[[bin]]` section in `crates/cli/Cargo.toml` produces a binary named **`repomix`**. The `clap` command name shown in `--help` is `repomix-rs` (to match the repo name).

### Prerequisites

Git-related features (`sort_by_changes`, `--include-diffs`, `--include-logs`, `--remote`, MCP `pack_remote_repository`) shell out to the **`git` executable** on your `PATH`. No Cargo feature flag is required.

- Install [Git](https://git-scm.com/) and ensure `git` is available in your shell.
- When packing a non-git directory, or when `git` is missing, git-aware steps are skipped with a warning rather than failing the whole pack.

---

## Usage

### CLI quickstart

```bash
# Pack the current directory
repomix .

# Pack a remote repository directly (cloned into a unique temp dir, cleaned up on exit)
repomix --remote https://github.com/owner/repo

# Choose output style
repomix --style markdown --output output.md .
repomix --style json --output output.json .
repomix --style plain --output output.txt .

# Compress code (tree-sitter) and remove comments
repomix --compress --remove-comments --remove-empty-lines .

# Filter files
repomix --include "*.rs,*.toml,Cargo.*" --ignore "target/**,tests/**" .

# Show top-N token-heavy files in the report (default: 10)
repomix --top-files-length 20 .

# Interactively scaffold a project-level config and a .repomixignore file
repomix --init

# Run as an MCP server (talks JSON-RPC over stdio)
repomix --mcp
```

#### Full CLI reference

```
repomix [OPTIONS] [ROOT]              # pack local directory
repomix --remote <URL> [OPTIONS]      # pack a remote git repository
repomix --init                        # interactively write repomix.config.json + .repomixignore
repomix --mcp                         # start the MCP server on stdio
```

| Flag | Description | Default |
|---|---|---|
| `ROOT` | Directory to pack (positional) | current dir |
| `--remote <URL>` | Clone and pack a remote git repo (`https://`, `http://`, `git://`, `ssh://`, or `user@host:path`) | — |
| `--include <LIST>` | Comma-separated glob patterns to include (appended to config) | — |
| `--ignore <LIST>` | Comma-separated glob patterns to ignore (appended to config) | — |
| `--style <xml\|markdown\|plain\|json>` | Output style | `xml` |
| `--output <PATH>` | Output file path | `repomix-output.txt` (style-dependent) |
| `--compress` | Enable tree-sitter signature extraction | off |
| `--remove-comments` | Strip comments from output | off |
| `--remove-empty-lines` | Collapse blank lines | off |
| `--line-numbers` | Prefix every output line with its number | off |
| `--truncate-base64` | Truncate long base64 blobs in the output | off |
| `--copy` | Copy the output to the system clipboard | off |
| `--include-empty-directories` | Include empty dirs in the tree section | off |
| `--top-files-length <N>` | Number of top token-heavy files to print in the report | `10` |
| `--split-output <TOKENS>` | Split output into chunks of at most N tokens (XML splits at file boundaries) | — |
| `--header-text <TEXT>` | Custom header text prepended to the output | — |
| `--instruction-file <PATH>` | Path to a file whose contents are appended as instructions | — |
| `--include-diffs` | Append `git diff` to the output (requires `git` on `PATH` and a `.git` repo) | off |
| `--include-logs` | Append `git log` to the output (requires `git` on `PATH` and a `.git` repo) | off |
| `-v`, `-vv`, `-vvv` | Verbose logging (count-based) | off |
| `--init` | Interactively create `repomix.config.json` and `.repomixignore`, then exit | — |
| `--mcp` | Run as an MCP server on stdio, then exit | — |

> **Behavior note:** `--include` and `--ignore` *append* to the patterns already present in the project/global config. Multiple `--include` flags accumulate, and they never silently replace patterns from your config file.
>
> **Logging note:** `-v` toggles the tracing level (default `INFO`, with `-v` set to `DEBUG`). The `RUST_LOG` environment variable, if set, is honored via `tracing_subscriber::EnvFilter`.

### Library API

The public surface lives in `repomix_core`:

```rust
use repomix_core::{
    pack, pack_directory, pack_with_config, pack_with_options,
    NoopProgress, OutputStyle, PackOptions, RepomixConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. One-shot default packing
    let result = pack_directory("/path/to/repo").await?;
    println!("Packed {} files, {} tokens", result.total_files, result.total_tokens);

    // 2. Custom config
    let mut config = RepomixConfig::default();
    config.output.style = OutputStyle::Markdown;
    config.output.compress = true;
    config.output.show_line_numbers = true;
    let result = pack_with_config("/path/to/repo", config).await?;

    // 3. Fluent PackOptions builder
    let options = PackOptions::new("/path/to/repo".into())
        .with_style(OutputStyle::Json)
        .with_compress(true)
        .with_line_numbers(true)
        .with_include_patterns(vec!["*.rs".into(), "*.toml".into()])
        .with_ignore_patterns(vec!["target/**".into()]);
    let result = pack_with_options(options).await?;

    // 4. Full control with a progress callback
    struct MyProgress;
    impl repomix_core::ProgressCallback for MyProgress {
        fn on_progress(&self, msg: &str) { println!("… {msg}"); }
        fn on_complete(&self, msg: &str) { println!("✓ {msg}"); }
        fn on_error(&self, msg: &str)    { eprintln!("✗ {msg}"); }
    }
    let result = pack(
        vec!["/path/to/repo".into()],
        RepomixConfig::default(),
        Box::new(MyProgress),
    )
    .await?;

    // result.total_files, .total_tokens, .total_characters,
    // .top_files_by_tokens, .suspicious_files, .skipped_files, ...
    Ok(())
}
```

`pack` is the canonical entry point; the convenience wrappers (`pack_directory`, `pack_with_config`, `pack_with_options`) all delegate to it. `RepomixConfig` is re-exported from `repomix_core::config` for convenience, but its canonical home is the `repomix-config` crate.

> The library is async (uses `tokio`); initialize a runtime as shown above. The CLI itself uses `#[tokio::main]`.

### MCP server

Start the server with `repomix --mcp`. It speaks the Model Context Protocol over stdio (JSON-RPC) using the `rmcp` crate.

#### Tools

| Tool | Description | Parameters |
|---|---|---|
| `pack_codebase` | Pack a local directory | `directory?`, `compress?`, `include_patterns?`, `ignore_patterns?`, `top_files_length?`, `style?` (`xml` \| `markdown` \| `plain` \| `json`) |
| `pack_remote_repository` | Clone and pack a remote git repo | `url` (required), `style?` |
| `read_repomix_output` | Read a previously generated repomix output file | `file_path` (required) |
| `grep_repomix_output` | Regex search within a repomix output file, with optional context lines | `file_path` (required), `pattern` (required), `context?` |

`pack_codebase` and `pack_remote_repository` return a JSON object shaped like:

```json
{
  "description": "Successfully packed 42 files (12345 tokens) from repository",
  "result": "{ ... PackMetrics JSON: total_files, total_tokens, total_characters, file_token_counts, file_char_counts, top_files_by_tokens ... }",
  "directory_structure": "",
  "output_id": "packed_output",
  "output_file_path": "/tmp/repomix_mcp_xxx/pack.xml",
  "total_files": 42,
  "total_tokens": 12345
}
```

> `directory_structure` is currently always `""` in the MCP response (the directory tree is included inside the packed output file at `output_file_path`); use `read_repomix_output` to inspect it.

Unknown `style` values (e.g. `"yaml"`) are rejected with a structured `invalid_params` error rather than silently falling back to XML. Pack operations are serialized internally (a `tokio::Mutex` per server instance) to avoid concurrent `git clone` / `rayon` work on the same repo.

#### Client configuration examples

**Claude Desktop** (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "repomix": {
      "command": "repomix",
      "args": ["--mcp"]
    }
  }
}
```

**Cursor** (Settings → MCP → Add new global MCP server):

```json
{
  "mcpServers": {
    "repomix": {
      "command": "repomix",
      "args": ["--mcp"]
    }
  }
}
```

---

## Configuration

`repomix-rs` reads configuration from four layers, merged in order (later wins):

1. **Built-in defaults**
2. **Global config** — `~/.repomix/repomix.config.json`
3. **Project config** — `./repomix.config.json` (relative to the current working directory)
4. **CLI flags** / MCP tool parameters

Use `repomix --init` to interactively scaffold a project-level `repomix.config.json` and a `.repomixignore` file (similar in spirit to `.gitignore`). Existing files trigger a `dialoguer::Confirm` prompt before being overwritten.

### Full schema

```jsonc
{
  "input": {
    "max_file_size": 52428800  // 50 MB; files larger are skipped
  },
  "output": {
    "file_path": "repomix-output.txt",
    "style": "xml",            // "xml" | "markdown" | "plain" | "json"
    "parsable_style": false,   // adds tokens/chars attrs (XML) or structured markers
    "header_text": null,       // string prepended to the output
    "instruction_file_path": null, // path whose contents are appended as instructions
    "file_summary": true,      // include a per-file summary section
    "directory_structure": true,
    "files": true,             // include the files section
    "remove_comments": false,
    "remove_empty_lines": false,
    "compress": false,         // tree-sitter signature extraction
    "top_files_length": 10,    // N for the top-N token-heavy files in metrics
    "show_line_numbers": false,
    "truncate_base64": false,
    "copy_to_clipboard": false,
    "include_empty_directories": false,
    "include_full_directory_structure": false,
    "split_output": null,      // max tokens per chunk; null = single file
    "token_count_tree": {
      "show_tree": false       // include a per-directory token tree
    },
    "git": {
      "sort_by_changes": true,
      "sort_by_changes_max_commits": 100,
      "include_diffs": false,
      "include_logs": false,
      "include_logs_count": 50
    },
    "json": {
      "no_timestamp": false    // omit `packed_at` from JSON metadata (deterministic output)
    }
  },
  "include": [],               // additional glob patterns to include (e.g. ["*.rs", "*.toml"])
  "ignore": {
    "use_gitignore": true,     // honor .gitignore when collecting
    "custom_ignore": []        // extra glob patterns to ignore
  },
  "security": {
    "enable_secretlint": true  // detect and exclude files containing secrets
  },
  "token_count": {
    "encoding": "o200k_base"   // any encoding supported by tiktoken-rs
  }
}
```

### Default ignore patterns

In addition to whatever you put in `ignore.custom_ignore` and your `.gitignore`, the following are always ignored: `.git`, `node_modules`, `__pycache__`, `.DS_Store`, and binary/archive/media extensions (`*.pyc`, `*.pyo`, `*.class`, `*.jar`, `*.war`, `*.ear`, `*.zip`, `*.tar.gz`, `*.tar.bz2`, `*.tgz`, `*.rar`, `*.7z`, `*.exe`, `*.dll`, `*.so`, `*.dylib`, `*.pdf`, `*.doc*`, `*.xls*`, `*.ppt*`, `*.mp3`, `*.mp4`, `*.avi`, `*.mov`, `*.wav`, `*.flac`, `*.ogg`, `*.jpg`, `*.jpeg`, `*.png`, `*.gif`, `*.bmp`, `*.ico`, `*.svg`, `*.webp`, `*.woff`, `*.woff2`, `*.ttf`, `*.eot`, `*.otf`, `*.wasm`, `*.whl`, `*.egg`). The list lives in `repomix_config::default_ignore::default_ignore_patterns()`.

---

## Supported languages (tree-sitter compression)

| Family | Languages |
|---|---|
| Web | JavaScript (`.js`, `.jsx`), TypeScript (`.ts`, `.tsx`) |
| Systems | C (`.c`, `.h`), C++ (`.cpp`, `.cxx`, `.cc`, `.hpp`, `.hxx`), Rust (`.rs`), Go (`.go`) |
| Scripting | Python (`.py`), Ruby (`.rb`), PHP (`.php`) |
| Enterprise | Java (`.java`) |

> **Note:** C# (`.cs`) compression is **temporarily disabled** in this version due to an ABI mismatch between `tree-sitter-c-sharp` 0.23 (language version 15) and the bundled query file. `.cs` files fall back to plain-text processing until the queries are upgraded.

---

## Performance

The Rust implementation is designed to be substantially faster than the Node.js original by replacing single-threaded globby + `promisePool` with `ignore` (multi-threaded traversal) and `rayon` (data-parallel processing), and by using native tree-sitter instead of WASM-in-a-Worker.

Representative numbers from a synthetic 5,000-file repository on a recent multi-core machine:

| Metric | Node.js (Repomix) | `repomix-rs` | Speedup |
|---|---|---|---|
| File search | `globby` (single-thread) | `ignore` crate (multi-thread) | 3–5× |
| File collection | `promisePool(50)` | `rayon` + zero-copy reads | 2–3× |
| Tree-sitter compression | WASM + Worker | Native + `rayon` | 5–10× |
| End-to-end (≈5k files) | ~15 s | ~2–3 s | 5–7× |

> Run your own benchmarks with `cargo build --release` followed by `time ./target/release/repomix <large-repo>`. Numbers above are indicative, not guarantees.

---

## Development

```bash
# Build the whole workspace
cargo build

# Run all tests
cargo test

# Run clippy with warnings as errors (recommended before sending a PR)
cargo clippy --workspace --all-targets -- -D warnings

# Format the code
cargo fmt --all

# Build a release CLI binary (produces ./target/release/repomix)
cargo build --release -p repomix-cli

# Run the CLI against the current directory
cargo run -p repomix-cli --release -- .

# Run the CLI as an MCP server (useful for ad-hoc debugging with an MCP client)
cargo run -p repomix-cli -- --mcp
```

### Project conventions

- All public APIs in `repomix-core` and `repomix-config` are re-exported from the crate root.
- Configuration loading is centralized in `repomix_config::load::RepomixConfig::load`; CLI flags map to `repomix_config::load::PartialConfig`.
- The CLI and MCP server should be thin wrappers — all real work happens in `repomix-core`.
- Default ignore patterns live in `repomix_config::default_ignore::default_ignore_patterns()`; add new ones there, not inline.

---

## License

MIT — see the original [Repomix](https://github.com/yamadashy/repomix) project for the reference TypeScript implementation.

## Acknowledgments

- [Repomix](https://github.com/yamadashy/repomix) — original TypeScript implementation by [@yamadashy](https://github.com/yamadashy)
- [tree-sitter](https://tree-sitter.github.io/) — parser generator
- [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) — Rust binding for OpenAI's tokenizer
- [rmcp](https://github.com/anthropics/rust-mcp-sdk) — official Rust MCP SDK
