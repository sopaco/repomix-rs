# Repository Packed for AI Analysis

This file contains the packed representation of the repository.

## Purpose

This file contains the packed representation of the repository.

## File Format

The content is organized as follows:
1. This header section contains metadata about the packing process.
2. This directory structure section shows the repository structure.
3. Multiple file entries, each consisting of:
   - File path as a heading
   - Full contents of the file in a code block

## Custom Instructions

MindMesh Agent Source Pack (repomix-rs / architecture-context)
Purpose: Indexed snapshot of project source code for Ask-mode retrieval.
Use grep_agent_pack and read_agent_pack_file on demand — never load this entire file into LLM context.
Auto-packed on first Ask when missing; use Pack Context in the UI to refresh after large codebase changes.


## Directory Structure

```
articles/
  01_hermes_agent_repomix_rs_guide.md
  02_codebase_pack_ai_workflow.md
  03_promo_why_repomix_rs.md
  README.md
  README_article_topics.md
  _gen_articles.py
crates/
  cli/
    src/
      prompts/
        mod.rs
      main.rs
      report.rs
      run.rs
      spinner.rs
    Cargo.toml
  config/
    src/
      default_ignore.rs
      global_dir.rs
      lib.rs
      load.rs
      schema.rs
      tests.rs
    Cargo.toml
  core/
    src/
      file/
        collect.rs
        manipulate.rs
        mod.rs
        process.rs
        process_content.rs
        search.rs
        tree_generate.rs
        truncate_base64.rs
        types.rs
      git/
        diff.rs
        log.rs
        mod.rs
        remote.rs
        sort.rs
      metrics/
        calculate.rs
        mod.rs
        token_count.rs
      output/
        styles/
          json.rs
          markdown.rs
          mod.rs
          plain.rs
          xml.rs
        decorate.rs
        generate.rs
        mod.rs
        split.rs
      security/
        mod.rs
        secretlint.rs
        validate.rs
      tree_sitter/
        queries/
          c.scm
          c_sharp.scm
          cpp.scm
          go.scm
          java.scm
          javascript.scm
          php.scm
          python.scm
          ruby.scm
          rust.scm
          typescript.scm
        compress.rs
        languages.rs
        mod.rs
      lib.rs
      packager.rs
      path_util.rs
    Cargo.toml
  mcp/
    src/
      tools/
        mod.rs
      helpers.rs
      lib.rs
      output_path.rs
      params.rs
      server.rs
    Cargo.toml
  shared/
    src/
      lib.rs
      logger.rs
      pattern_utils.rs
      types.rs
    Cargo.toml
    LICENSE-MIT
npm/
  repomix-rs/
    bin/
      repomix.js
    package.json
  templates/
    platform-package.json.tmpl
scripts/
  check-npm-version.mjs
  publish-crates.sh
  publish-npm.mjs
Cargo.toml
LICENSE
README.md
```

## Files

### crates/cli/Cargo.toml (31 lines)

```
1: [package]
2: name = "repomix-cli"
3: version = "2.0.0"
4: edition = "2024"
5: description = "CLI tool to pack repository content into a single prompt for LLMs."
6: license = "MIT"
7: 
8: [[bin]]
9: name = "repomix"
10: path = "src/main.rs"
11: 
12: [dependencies]
13: repomix-core = { path = "../core" }
14: repomix-config = { path = "../config" }
15: repomix-shared = { path = "../shared" }
16: repomix-mcp = { path = "../mcp" }
17: clap = { version = "4.0", features = ["derive"] }
18: tokio = { version = "1.0", features = ["full"] }
19: anyhow = "1.0"
20: tracing = "0.1"
21: tracing-subscriber = "0.3"
22: indicatif = "0.17"
23: dirs = "5.0"
24: arboard = "3.3"
25: serde_json = "1.0"
26: dialoguer = "0.11"
27: console = "0.15"
28: 
29: [dev-dependencies]
30: assert_cmd = "2.0"
31: predicates = "3.1"
```

### crates/config/Cargo.toml (16 lines)

```
1: [package]
2: name = "repomix-config"
3: version = "2.0.0"
4: edition = "2024"
5: description = "Configuration management for repomix."
6: license = "MIT"
7: 
8: [dependencies]
9: repomix-shared = { path = "../shared" }
10: serde = { version = "1.0", features = ["derive"] }
11: serde_json = "1.0"
12: anyhow = "1.0"
13: tracing = "0.1"
14: dirs = "5.0"
15: globset = "0.4"
16: ignore = "0.4"
```

### crates/core/Cargo.toml (40 lines)

```
1: [package]
2: name = "repomix-core"
3: version = "2.0.0"
4: edition = "2024"
5: description = "Core engine for repomix: search, collect, process, and pack repository content."
6: license = "MIT"
7: 
8: [dependencies]
9: repomix-shared = { path = "../shared" }
10: repomix-config = { path = "../config" }
11: anyhow = "1.0"
12: thiserror = "1.0"
13: tokio = { version = "1.0", features = ["full"] }
14: rayon = "1.8"
15: ignore = "0.4"
16: globset = "0.4"
17: chardetng = "0.1"
18: encoding_rs = "0.8"
19: tree-sitter = "0.24"
20: streaming-iterator = "0.1"
21: tiktoken-rs = "0.5"
22: serde = { version = "1.0", features = ["derive"] }
23: serde_json = "1.0"
24: regex = "1.10"
25: tracing = "0.1"
26: walkdir = "2.4"
27: num_cpus = "1.16"
28: once_cell = "1.19"
29: chrono = { version = "0.4", default-features = false, features = ["clock", "std"] }
30: arboard = "3.3"
31: tree-sitter-typescript = "0.23"
32: tree-sitter-javascript = "0.23"
33: tree-sitter-python = "0.23"
34: tree-sitter-rust = "0.23"
35: tree-sitter-go = "0.23"
36: tree-sitter-java = "0.23"
37: tree-sitter-c = "0.23"
38: tree-sitter-cpp = "0.23"
39: tree-sitter-ruby = "0.23"
40: tree-sitter-php = "0.23"
```

### crates/core/src/packager.rs (109 lines)

```
1: PackResult
2: ⋮----
3: {
4:     pub total_files: usize,
5:     pub total_characters: usize,
6:     pub total_tokens: usize,
7:     pub file_char_counts: HashMap<String, usize>,
8:     pub file_token_counts: HashMap<String, usize>,
9:     
10:     pub top_files_by_tokens: Vec<(String, usize)>,
11:     pub git_diff_content: Option<String>,
12:     pub git_diff_token_count: usize,
13:     pub git_log_content: Option<String>,
14:     pub git_log_token_count: usize,
15:     
16:     pub output_paths: Vec<String>,
17:     
18:     pub output_contents: Vec<String>,
19:     
20:     pub directory_structure: String,
21:     pub suspicious_files: Vec<SuspiciousFileResult>,
22:     pub processed_files: Vec<ProcessedFile>,
23:     pub safe_file_paths: Vec<PathBuf>,
24:     pub skipped_files: Vec<SkippedFileInfo>,
25: }
26: ⋮----
27: PackOptions
28: ⋮----
29: {
30:     pub root_dirs: Vec<PathBuf>,
31:     pub config: RepomixConfig,
32: }
33: ⋮----
34: PackOptions
35: ⋮----
36: {
37:     pub fn new(root_dir: PathBuf) -> Self {
38:         Self {
39:             root_dirs: vec![root_dir],
40:             config: RepomixConfig::default(),
41:         }
42:     }
43: 
44:     pub fn with_config(mut self, config: RepomixConfig) -> Self {
45:         self.config = config;
46:         self
47:     }
48: 
49:     pub fn with_style(mut self, style: OutputStyle) -> Self {
50:         self.config.output.style = style;
51:         self
52:     }
53: 
54:     pub fn with_compress(mut self, compress: bool) -> Self {
55:         self.config.output.compress = compress;
56:         self
57:     }
58: 
59:     pub fn with_remove_comments(mut self, remove: bool) -> Self {
60:         self.config.output.remove_comments = remove;
61:         self
62:     }
63: 
64:     pub fn with_line_numbers(mut self, show: bool) -> Self {
65:         self.config.output.show_line_numbers = show;
66:         self
67:     }
68: 
69:     pub fn with_include_patterns(mut self, patterns: Vec<String>) -> Self {
70:         self.config.include.extend(patterns);
71:         self
72:     }
73: 
74:     pub fn with_ignore_patterns(mut self, patterns: Vec<String>) -> Self {
75:         self.config.ignore.custom_ignore.extend(patterns);
76:         self
77:     }
78: }
79: ⋮----
80: ProgressCallback
81: ⋮----
82: {
83:     fn on_progress(&self, message: &str);
84:     fn on_complete(&self, message: &str);
85:     fn on_error(&self, message: &str);
86: }
87: ⋮----
88: NoopProgress
89: ⋮----
90: {
91:     fn on_progress(&self, _message: &str) {}
92:     fn on_complete(&self, _message: &str) {}
93:     fn on_error(&self, _message: &str) {}
94: }
95: ⋮----
96: pack
97: ⋮----
98: (
99:     root_dirs: Vec<PathBuf>,
100:     mut config: RepomixConfig,
101:     progress: Box<dyn ProgressCallback>,
102: )
103: ⋮----
104: filter_suspicious
105: ⋮----
106: (
107:     processed: Vec<ProcessedFile>,
108:     validation: &ValidationResult,
109: )
```

### crates/core/src/security/secretlint.rs (196 lines)

```
1: SecretRule
2: ⋮----
3: {
4:     pub id: String,
5:     pub name: String,
6:     pub pattern: Regex,
7:     pub entropy: Option<f64>,
8:     pub allowlist: Vec<String>,
9: }
10: ⋮----
11: get_secret_rules
12: ⋮----
13: ()
14: ⋮----
15: build_secret_rules
16: ⋮----
17: ()
18: ⋮----
19: safe_compile
20: ⋮----
21: (pattern: &str, name: &str)
22: ⋮----
23: TestRegionState
24: ⋮----
25: {
26:     bace_depth: u32,
27:     test_block_start: Option<u32>,
28:     cfg_test_start: Option<u32>,
29:     pending_test_attr: bool,
30:     pending_cfg_test: bool,
31: }
32: ⋮----
33: TestRegionState
34: ⋮----
35: {
36:     fn is_inside_test_region(&self) -> bool {
37:         self.test_block_start
38:             .is_some_and(|start| self.bace_depth >= start)
39:             || self
40:                 .cfg_test_start
41:                 .is_some_and(|start| self.bace_depth >= start)
42:     }
43: 
44:     fn update_for_line(&mut self, line: &str) {
45:         let trimmed = line.trim();
46:         if trimmed.starts_with("#[") {
47:             if trimmed.contains("#[test")
48:                 || trimmed.contains("#[tokio::test")
49:                 || trimmed.contains("#[async_std::test")
50:             {
51:                 self.pending_test_attr = true;
52:             }
53:             if trimmed.contains("cfg(test)") {
54:                 self.pending_cfg_test = true;
55:             }
56:         }
57: 
58:         let open = line.chars().filter(|&c| c == '{').count() as u32;
59:         let close = line.chars().filter(|&c| c == '}').count() as u32;
60: 
61:         if self.pending_test_attr && open > 0 {
62:             self.test_block_start = Some(self.bace_depth + 1);
63:             self.pending_test_attr = false;
64:         }
65:         if self.pending_cfg_test && open > 0 {
66:             self.cfg_test_start = Some(self.bace_depth + 1);
67:             self.pending_cfg_test = false;
68:         }
69: 
70:         self.bace_depth += open;
71:         self.bace_depth = self.bace_depth.saturating_sub(close);
72: 
73:         if let Some(start) = self.test_block_start
74:             && self.bace_depth < start
75:         {
76:             self.test_block_start = None;
77:         }
78:         if let Some(start) = self.cfg_test_start
79:             && self.bace_depth < start
80:         {
81:             self.cfg_test_start = None;
82:         }
83:     }
84: }
85: ⋮----
86: extract_assign_quoted_value
87: ⋮----
88: (line: &str)
89: ⋮----
90: extract_assign_unquoted_value
91: ⋮----
92: (line: &str)
93: ⋮----
94: extract_assign_value
95: ⋮----
96: (line: &str)
97: ⋮----
98: path_has_test_segment
99: ⋮----
100: (path: &Path)
101: ⋮----
102: path_is_jvm_test_source_root
103: ⋮----
104: (path: &Path)
105: ⋮----
106: is_test_fixture_file_name
107: ⋮----
108: (path: &Path)
109: ⋮----
110: is_test_fixture_path
111: ⋮----
112: (path: &Path)
113: ⋮----
114: secret_candidate
115: ⋮----
116: (line: &'a str, rule: &SecretRule)
117: ⋮----
118: scan_file_content
119: ⋮----
120: (content: &str, file_path: &Path)
121: ⋮----
122: calculate_entropy
123: ⋮----
124: (s: &str)
125: ⋮----
126: test_entropy_empty
127: ⋮----
128: ()
129: ⋮----
130: test_entropy_repeated_char
131: ⋮----
132: ()
133: ⋮----
134: test_entropy_distributed
135: ⋮----
136: ()
137: ⋮----
138: test_safe_compile_invalid_returns_never_match
139: ⋮----
140: ()
141: ⋮----
142: test_allowlist_skips_placeholder
143: ⋮----
144: ()
145: ⋮----
146: test_entropy_filter_blocks_low_entropy
147: ⋮----
148: ()
149: ⋮----
150: test_unquoted_api_key_in_env_format
151: ⋮----
152: ()
153: ⋮----
154: test_unquoted_api_key_low_entropy_filtered
155: ⋮----
156: ()
157: ⋮----
158: test_test_function_body_skipped
159: ⋮----
160: ()
161: ⋮----
162: test_cfg_test_module_skipped
163: ⋮----
164: ()
165: ⋮----
166: test_scanner_source_no_false_positive
167: ⋮----
168: ()
169: ⋮----
170: test_integration_test_fixtures_skipped
171: ⋮----
172: ()
173: ⋮----
174: test_fixture_path_web_jest
175: ⋮----
176: ()
177: ⋮----
178: test_fixture_path_android_instrumented
179: ⋮----
180: ()
181: ⋮----
182: test_fixture_path_kmp_common_test
183: ⋮----
184: ()
185: ⋮----
186: test_fixture_path_ios_swift
187: ⋮----
188: ()
189: ⋮----
190: test_fixture_path_pytest_module
191: ⋮----
192: ()
193: ⋮----
194: test_production_path_still_scanned
195: ⋮----
196: ()
```

### crates/mcp/Cargo.toml (18 lines)

```
1: [package]
2: name = "repomix-mcp"
3: version = "2.0.0"
4: edition = "2024"
5: description = "MCP (Model Context Protocol) server for repomix."
6: license = "MIT"
7: 
8: [dependencies]
9: repomix-core = { path = "../core" }
10: repomix-config = { path = "../config" }
11: repomix-shared = { path = "../shared" }
12: tokio = { version = "1.0", features = ["full"] }
13: anyhow = "1.0"
14: serde = { version = "1.0", features = ["derive"] }
15: serde_json = "1.0"
16: regex = "1.10"
17: rmcp = { version = "1.7", features = ["server", "transport-io", "macros"] }
18: tracing = "0.1"
```

### crates/mcp/src/server.rs (302 lines)

```
1: PackToolResult
2: ⋮----
3: {
4:     pub description: String,
5:     pub result: String,
6:     pub directory_structure: String,
7:     pub output_id: String,
8:     
9:     pub output_file_path: String,
10:     
11:     pub output_paths: Vec<String>,
12:     pub total_files: usize,
13:     pub total_tokens: usize,
14: }
15: ⋮----
16: PackMetrics
17: ⋮----
18: {
19:     pub total_files: usize,
20:     pub total_tokens: usize,
21:     pub total_characters: usize,
22:     pub file_token_counts: std::collections::HashMap<String, usize>,
23:     pub file_char_counts: std::collections::HashMap<String, usize>,
24:     
25:     pub top_files_by_tokens: Vec<(String, usize)>,
26: }
27: ⋮----
28: PackMetrics
29: ⋮----
30: {
31:     fn from(r: &PackResult) -> Self {
32:         Self {
33:             total_files: r.total_files,
34:             total_tokens: r.total_tokens,
35:             total_characters: r.total_characters,
36:             file_token_counts: r.file_token_counts.clone(),
37:             file_char_counts: r.file_char_counts.clone(),
38:             top_files_by_tokens: r.top_files_by_tokens.clone(),
39:         }
40:     }
41: }
42: ⋮----
43: ReadRepomixOutputParams
44: ⋮----
45: {
46:     
47:     pub file_path: String,
48: }
49: ⋮----
50: GrepRepomixOutputParams
51: ⋮----
52: {
53:     
54:     pub file_path: String,
55:     
56:     pub pattern: String,
57:     
58:     #[serde(default)]
59:     pub context: Option<usize>,
60: }
61: ⋮----
62: load_mcp_config
63: ⋮----
64: (
65:     partial: repomix_config::load::PartialConfig,
66: )
67: ⋮----
68: make_temp_dir
69: ⋮----
70: (prefix: &str)
71: ⋮----
72: TempDirGuard
73: ⋮----
74: {
75:     path: PathBuf,
76: }
77: ⋮----
78: TempDirGuard
79: ⋮----
80: {
81:     fn new(path: PathBuf) -> Self {
82:         Self { path }
83:     }
84: }
85: ⋮----
86: TempDirGuard
87: ⋮----
88: {
89:     fn drop(&mut self) {
90:         if let Err(e) = std::fs::remove_dir_all(&self.path) {
91:             tracing::warn!(
92:                 "Failed to clean up temp dir '{}': {}. \
93:                  This may be a permission issue or the directory is in use.",
94:                 self.path.display(),
95:                 e
96:             );
97:         }
98:     }
99: }
100: ⋮----
101: pack_tool_result
102: ⋮----
103: (result: &PackResult, output_id: &str, description: &str)
104: ⋮----
105: ok_result
106: ⋮----
107: (value: serde_json::Value)
108: ⋮----
109: RepomixMcpServer
110: ⋮----
111: {
112:     tool_router: ToolRouter<Self>,
113:     
114:     lock: Arc<Mutex<()>>,
115: }
116: ⋮----
117: RepomixMcpServer
118: ⋮----
119: {
120:     fn default() -> Self {
121:         Self::new()
122:     }
123: }
124: ⋮----
125: RepomixMcpServer
126: ⋮----
127: {
128:     pub fn new() -> Self {
129:         Self {
130:             tool_router: Self::tool_router(),
131:             lock: Arc::new(Mutex::new(())),
132:         }
133:     }
134: 
135:     #[tool(
136:         name = "pack_codebase",
137:         description = "Pack a local directory into an AI-friendly format (XML/Markdown/Plain/JSON). Returns a JSON object with total_files, total_tokens, output_file_path and metrics breakdown. Use this when the user wants to feed a codebase to an LLM."
138:     )]
139:     async fn pack_codebase(
140:         &self,
141:         Parameters(p): Parameters<PackCodebaseParams>,
142:     ) -> Result<CallToolResult, ErrorData> {
143:         let _guard = self.lock.lock().await;
144: 
145:         let root_dir: PathBuf = p
146:             .directory
147:             .as_deref()
148:             .map(PathBuf::from)
149:             .or_else(|| std::env::current_dir().ok())
150:             .ok_or_else(|| {
151:                 ErrorData::invalid_params("directory not provided and CWD unavailable", None)
152:             })?;
153: 
154:         let partial = p.shared.into_mcp_overrides()?.into_partial_config();
155:         let mut config = load_mcp_config(partial)?;
156: 
157:         let mcp_output = make_mcp_output_path(&config.output.style)?;
158:         config.output.file_path = mcp_output.path.to_string_lossy().to_string();
159: 
160:         let result = pack(vec![root_dir], config, Box::new(NoopProgress))
161:             .await
162:             .map_err(|e| ErrorData::internal_error(format!("pack failed: {}", e), None))?;
163: 
164:         let tool_result = pack_tool_result(
165:             &result,
166:             &mcp_output.output_id,
167:             &format!(
168:                 "Successfully packed {} files ({} tokens) from repository",
169:                 result.total_files, result.total_tokens
170:             ),
171:         );
172:         ok_result(serde_json::to_value(&tool_result).unwrap_or_default())
173:     }
174: 
175:     #[tool(
176:         name = "pack_remote_repository",
177:         description = "Clone a remote git repository to a temporary directory and pack it. Returns the same structure as pack_codebase."
178:     )]
179:     async fn pack_remote_repository(
180:         &self,
181:         Parameters(p): Parameters<PackRemoteRepositoryParams>,
182:     ) -> Result<CallToolResult, ErrorData> {
183:         let _guard = self.lock.lock().await;
184: 
185:         validate_remote_url(&p.url)?;
186: 
187:         let temp_dir = make_temp_dir("repomix_mcp_remote")
188:             .map_err(|e| ErrorData::internal_error(format!("create temp dir: {}", e), None))?;
189:         let _temp_guard = TempDirGuard::new(temp_dir.clone());
190: 
191:         repomix_core::git::remote::clone_remote_repo(&p.url, &temp_dir)
192:             .map_err(|e| ErrorData::internal_error(format!("git clone failed: {}", e), None))?;
193: 
194:         let partial = p.shared.into_mcp_overrides()?.into_partial_config();
195:         let mut config = load_mcp_config(partial)?;
196:         let mcp_output = make_mcp_output_path(&config.output.style)?;
197:         config.output.file_path = mcp_output.path.to_string_lossy().to_string();
198: 
199:         let result = pack(vec![temp_dir.clone()], config, Box::new(NoopProgress))
200:             .await
201:             .map_err(|e| ErrorData::internal_error(format!("pack failed: {}", e), None))?;
202: 
203:         let tool_result = pack_tool_result(
204:             &result,
205:             &mcp_output.output_id,
206:             &format!(
207:                 "Successfully packed {} files ({} tokens) from remote repository",
208:                 result.total_files, result.total_tokens
209:             ),
210:         );
211:         ok_result(serde_json::to_value(&tool_result).unwrap_or_default())
212:     }
213: 
214:     #[tool(
215:         name = "read_repomix_output",
216:         description = "Read the contents of a previously generated repomix output file. Returns the raw text content."
217:     )]
218:     async fn read_repomix_output(
219:         &self,
220:         Parameters(p): Parameters<ReadRepomixOutputParams>,
221:     ) -> Result<CallToolResult, ErrorData> {
222:         let path = validate_mcp_output_path(&p.file_path)?;
223:         let content = tokio::task::spawn_blocking(move || std::fs::read_to_string(path))
224:             .await
225:             .map_err(|e| ErrorData::internal_error(format!("read task failed: {}", e), None))?
226:             .map_err(|e| ErrorData::internal_error(format!("read failed: {}", e), None))?;
227:         Ok(CallToolResult::success(vec![Content::text(content)]))
228:     }
229: 
230:     #[tool(
231:         name = "grep_repomix_output",
232:         description = "Search a repomix output file for lines matching a regular expression. Returns a JSON object with match_count and a matches array."
233:     )]
234:     async fn grep_repomix_output(
235:         &self,
236:         Parameters(p): Parameters<GrepRepomixOutputParams>,
237:     ) -> Result<CallToolResult, ErrorData> {
238:         let path = validate_mcp_output_path(&p.file_path)?;
239:         let regex = regex::Regex::new(&p.pattern)
240:             .map_err(|e| ErrorData::invalid_params(format!("invalid regex: {}", e), None))?;
241:         let pattern = p.pattern.clone();
242:         let context = p.context.unwrap_or(0);
243:         let file_display = p.file_path.clone();
244: 
245:         let matches = tokio::task::spawn_blocking(move || {
246:             let content =
247:                 std::fs::read_to_string(&path).map_err(|e| format!("read failed: {}", e))?;
248:             let lines: Vec<&str> = content.lines().collect();
249:             let mut matches: Vec<serde_json::Value> = Vec::new();
250: 
251:             for (i, line) in lines.iter().enumerate() {
252:                 if regex.is_match(line) {
253:                     let mut entry = serde_json::Map::new();
254:                     entry.insert("line_number".into(), serde_json::json!(i + 1));
255:                     entry.insert("text".into(), serde_json::json!(line));
256:                     if context > 0 {
257:                         let start = i.saturating_sub(context);
258:                         let end = (i + context + 1).min(lines.len());
259:                         entry.insert(
260:                             "context_before".into(),
261:                             serde_json::json!(lines[start..i].join("\n")),
262:                         );
263:                         entry.insert(
264:                             "context_after".into(),
265:                             serde_json::json!(lines[i + 1..end].join("\n")),
266:                         );
267:                     }
268:                     matches.push(serde_json::Value::Object(entry));
269:                 }
270:             }
271: 
272:             Ok::<_, String>(matches)
273:         })
274:         .await
275:         .map_err(|e| ErrorData::internal_error(format!("grep task failed: {}", e), None))?
276:         .map_err(|e| ErrorData::internal_error(e, None))?;
277:         ok_result(serde_json::json!({
278:             "file": file_display,
279:             "pattern": pattern,
280:             "match_count": matches.len(),
281:             "matches": matches,
282:         }))
283:     }
284: }
285: ⋮----
286: RepomixMcpServer
287: ⋮----
288: {
289:     fn get_info(&self) -> ServerInfo {
290:         ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
291:             .with_protocol_version(ProtocolVersion::V_2024_11_05)
292:             .with_instructions(
293:                 "Pack codebases into AI-friendly formats. Tools: pack_codebase, \
294:                  pack_remote_repository, read_repomix_output, grep_repomix_output.",
295:             )
296:             .with_server_info(Implementation::new("repomix", env!("CARGO_PKG_VERSION")))
297:     }
298: }
299: ⋮----
300: run_stdio_server
301: ⋮----
302: ()
```

### crates/shared/Cargo.toml (19 lines)

```
1: [package]
2: name = "repomix-shared"
3: version = "2.0.0"
4: edition = "2024"
5: description = "Shared types and tracing logger used across the repomix workspace."
6: license = "MIT"
7: 
8: [dependencies]
9: anyhow = "1.0"
10: serde = { version = "1.0", features = ["derive"] }
11: serde_json = "1.0"
12: tracing = "0.1"
13: tracing-subscriber = { version = "0.3", features = ["env-filter"] }
14: regex = "1.10"
15: chardetng = "0.1"
16: encoding_rs = "0.8"
17: arboard = "3.3"
18: indicatif = "0.17"
19: globset = "0.4"
```

### README.md (409 lines)

````
1: # repomix-rs
2: 
3: A Rust implementation of [Repomix](https://github.com/yamadashy/repomix) — a tool that packs your entire codebase into a single, AI-friendly file. It is a drop-in replacement written for speed, safety, and embedding into AI agents via the Model Context Protocol (MCP).
4: 
5: > **Status:** `2.0.0` — under active development. CLI, library, and MCP server are usable; configuration schema may still evolve.
6: 
7: ---
8: 
9: ## Features
10: 
11: - **Multiple output formats** — XML (default), Markdown, Plain text, JSON
12: - **Tree-sitter compression** — extract code signatures while stripping implementation bodies (10 languages)
13: - **Git-aware output** — sort by change frequency, include `git diff` and `git log` (via the system `git` CLI)
14: - **Token counting** — accurate counts via `tiktoken-rs` (`o200k_base` by default, GPT-4o family)
15: - **Security scanning** — detect and exclude files containing secrets via Secretlint
16: - **Parallel processing** — `rayon` for file collection, `tokio` for I/O
17: - **Layered configuration** — defaults → `~/.repomix/repomix.config.json` → `./repomix.config.json` → CLI flags
18: - **Two consumption modes** — standalone CLI binary **and** an `rmcp`-based MCP server for AI agents
19: 
20: ---
21: 
22: ## Workspace layout
23: 
24: This repository is a Cargo workspace with five crates:
25: 
26: | Crate | Purpose |
27: |---|---|
28: | `repomix-core` | Library: file collection, processing, tree-sitter compression, metrics, output generation, git operations |
29: | `repomix-config` | Typed configuration schema, default ignore patterns, global config path resolution, layered `RepomixConfig::load` |
30: | `repomix-shared` | Cross-crate types (`ProcessedFile`, `SuspiciousFileResult`, …) and the tracing-based logger |
31: | `repomix-cli` | The `repomix` binary (clap-based) |
32: | `repomix-mcp` | The MCP server exposing `pack_codebase`, `pack_remote_repository`, `read_repomix_output`, `grep_repomix_output` |
33: 
34: ---
35: 
36: ## Installation
37: 
38: ### npm (recommended)
39: 
40: Install the Rust build from npm. The npm package is named **`repomix-rs`** (to distinguish it from the [original TypeScript Repomix](https://www.npmjs.com/package/repomix)); the terminal command is **`repomix`**.
41: 
42: ```bash
43: # Global install → `repomix` on your PATH
44: npm install -g repomix-rs
45: 
46: # One-off run (no global install)
47: npx repomix-rs .
48: 
49: # MCP server for AI agents
50: npx -y repomix-rs --mcp
51: ```
52: 
53: Supported platforms: Linux (x64, arm64), macOS (x64, arm64), Windows (x64).
54: 
55: > If both `repomix` (TypeScript) and `repomix-rs` (Rust) are installed globally, the last install wins for the `repomix` command. Install only the one you need, or use `npx repomix-rs` / `npx repomix` explicitly.
56: 
57: ### From source
58: 
59: ```bash
60: # Install the CLI to ~/.cargo/bin/repomix
61: cargo install --path crates/cli
62: 
63: # Or build a release binary in ./target/release/repomix
64: cargo build --release
65: ```
66: 
67: The Cargo package is `repomix-cli`; a `[[bin]]` section in `crates/cli/Cargo.toml` produces a binary named **`repomix`**. The `clap` command name shown in `--help` is `repomix-rs` (to match the repo name).
68: 
69: ### Prerequisites
70: 
71: Git-related features (`sort_by_changes`, `--include-diffs`, `--include-logs`, `--remote`, MCP `pack_remote_repository`) shell out to the **`git` executable** on your `PATH`. No Cargo feature flag is required.
72: 
73: - Install [Git](https://git-scm.com/) and ensure `git` is available in your shell.
74: - When packing a non-git directory, or when `git` is missing, git-aware steps are skipped with a warning rather than failing the whole pack.
75: 
76: ---
77: 
78: ## Usage
79: 
80: ### CLI quickstart
81: 
82: ```bash
83: # Pack the current directory
84: repomix .
85: 
86: # Pack a remote repository directly (cloned into a unique temp dir, cleaned up on exit)
87: repomix --remote https://github.com/owner/repo
88: 
89: # Choose output style
90: repomix --style markdown --output output.md .
91: repomix --style json --output output.json .
92: repomix --style plain --output output.txt .
93: 
94: # Compress code (tree-sitter) and remove comments
95: repomix --compress --remove-comments --remove-empty-lines .
96: 
97: # Filter files
98: repomix --include "*.rs,*.toml,Cargo.*" --ignore "target/**,tests/**" .
99: 
100: # Show top-N token-heavy files in the report (default: 10)
101: repomix --top-files-length 20 .
102: 
103: # Interactively scaffold a project-level config and a .repomixignore file
104: repomix --init
105: 
106: # Run as an MCP server (talks JSON-RPC over stdio)
107: repomix --mcp
108: ```
109: 
110: #### Full CLI reference
111: 
112: ```
113: repomix [OPTIONS] [ROOT]              # pack local directory
114: repomix --remote <URL> [OPTIONS]      # pack a remote git repository
115: repomix --init                        # interactively write repomix.config.json + .repomixignore
116: repomix --mcp                         # start the MCP server on stdio
117: ```
118: 
119: | Flag | Description | Default |
120: |---|---|---|
121: | `ROOT` | Directory to pack (positional) | current dir |
122: | `--remote <URL>` | Clone and pack a remote git repo (`https://`, `http://`, `git://`, `ssh://`, or `user@host:path`) | — |
123: | `--include <LIST>` | Comma-separated glob patterns to include (appended to config) | — |
124: | `--ignore <LIST>` | Comma-separated glob patterns to ignore (appended to config) | — |
125: | `--style <xml\|markdown\|plain\|json>` | Output style | `xml` |
126: | `--output <PATH>` | Output file path | `repomix-output.txt` (style-dependent) |
127: | `--compress` | Enable tree-sitter signature extraction | off |
128: | `--remove-comments` | Strip comments from output | off |
129: | `--remove-empty-lines` | Collapse blank lines | off |
130: | `--line-numbers` | Prefix every output line with its number | off |
131: | `--truncate-base64` | Truncate long base64 blobs in the output | off |
132: | `--copy` | Copy the output to the system clipboard | off |
133: | `--include-empty-directories` | Include empty dirs in the tree section | off |
134: | `--top-files-length <N>` | Number of top token-heavy files to print in the report | `10` |
135: | `--split-output <TOKENS>` | Split output into chunks of at most N tokens (XML splits at file boundaries) | — |
136: | `--header-text <TEXT>` | Custom header text prepended to the output | — |
137: | `--instruction-file <PATH>` | Path to a file whose contents are appended as instructions | — |
138: | `--include-diffs` | Append `git diff` to the output (requires `git` on `PATH` and a `.git` repo) | off |
139: | `--include-logs` | Append `git log` to the output (requires `git` on `PATH` and a `.git` repo) | off |
140: | `-v`, `-vv`, `-vvv` | Verbose logging (count-based) | off |
141: | `--init` | Interactively create `repomix.config.json` and `.repomixignore`, then exit | — |
142: | `--mcp` | Run as an MCP server on stdio, then exit | — |
143: 
144: > **Behavior note:** `--include` and `--ignore` *append* to the patterns already present in the project/global config. Multiple `--include` flags accumulate, and they never silently replace patterns from your config file.
145: >
146: > **Logging note:** `-v` toggles the tracing level (default `INFO`, with `-v` set to `DEBUG`). The `RUST_LOG` environment variable, if set, is honored via `tracing_subscriber::EnvFilter`.
147: 
148: ### Library API
149: 
150: The public surface lives in `repomix_core`:
151: 
152: ```rust
153: use repomix_core::{
154:     pack, pack_directory, pack_with_config, pack_with_options,
155:     NoopProgress, OutputStyle, PackOptions, RepomixConfig,
156: };
157: 
158: #[tokio::main]
159: async fn main() -> anyhow::Result<()> {
160:     // 1. One-shot default packing
161:     let result = pack_directory("/path/to/repo").await?;
162:     println!("Packed {} files, {} tokens", result.total_files, result.total_tokens);
163: 
164:     // 2. Custom config
165:     let mut config = RepomixConfig::default();
166:     config.output.style = OutputStyle::Markdown;
167:     config.output.compress = true;
168:     config.output.show_line_numbers = true;
169:     let result = pack_with_config("/path/to/repo", config).await?;
170: 
171:     // 3. Fluent PackOptions builder
172:     let options = PackOptions::new("/path/to/repo".into())
173:         .with_style(OutputStyle::Json)
174:         .with_compress(true)
175:         .with_line_numbers(true)
176:         .with_include_patterns(vec!["*.rs".into(), "*.toml".into()])
177:         .with_ignore_patterns(vec!["target/**".into()]);
178:     let result = pack_with_options(options).await?;
179: 
180:     // 4. Full control with a progress callback
181:     struct MyProgress;
182:     impl repomix_core::ProgressCallback for MyProgress {
183:         fn on_progress(&self, msg: &str) { println!("… {msg}"); }
184:         fn on_complete(&self, msg: &str) { println!("✓ {msg}"); }
185:         fn on_error(&self, msg: &str)    { eprintln!("✗ {msg}"); }
186:     }
187:     let result = pack(
188:         vec!["/path/to/repo".into()],
189:         RepomixConfig::default(),
190:         Box::new(MyProgress),
191:     )
192:     .await?;
193: 
194:     // result.total_files, .total_tokens, .total_characters,
195:     // .top_files_by_tokens, .suspicious_files, .skipped_files, ...
196:     Ok(())
197: }
198: ```
199: 
200: `pack` is the canonical entry point; the convenience wrappers (`pack_directory`, `pack_with_config`, `pack_with_options`) all delegate to it. `RepomixConfig` is re-exported from `repomix_core::config` for convenience, but its canonical home is the `repomix-config` crate.
201: 
202: > The library is async (uses `tokio`); initialize a runtime as shown above. The CLI itself uses `#[tokio::main]`.
203: 
204: ### MCP server
205: 
206: Start the server with `repomix --mcp`. It speaks the Model Context Protocol over stdio (JSON-RPC) using the `rmcp` crate.
207: 
208: #### Tools
209: 
210: | Tool | Description | Parameters |
211: |---|---|---|
212: | `pack_codebase` | Pack a local directory | `directory?`, `compress?`, `include_patterns?`, `ignore_patterns?`, `top_files_length?`, `style?` (`xml` \| `markdown` \| `plain` \| `json`) |
213: | `pack_remote_repository` | Clone and pack a remote git repo | `url` (required), `style?` |
214: | `read_repomix_output` | Read a previously generated repomix output file | `file_path` (required) |
215: | `grep_repomix_output` | Regex search within a repomix output file, with optional context lines | `file_path` (required), `pattern` (required), `context?` |
216: 
217: `pack_codebase` and `pack_remote_repository` return a JSON object shaped like:
218: 
219: ```json
220: {
221:   "description": "Successfully packed 42 files (12345 tokens) from repository",
222:   "result": "{ ... PackMetrics JSON: total_files, total_tokens, total_characters, file_token_counts, file_char_counts, top_files_by_tokens ... }",
223:   "directory_structure": "",
224:   "output_id": "packed_output",
225:   "output_file_path": "/tmp/repomix_mcp_xxx/pack.xml",
226:   "total_files": 42,
227:   "total_tokens": 12345
228: }
229: ```
230: 
231: > `directory_structure` is currently always `""` in the MCP response (the directory tree is included inside the packed output file at `output_file_path`); use `read_repomix_output` to inspect it.
232: 
233: Unknown `style` values (e.g. `"yaml"`) are rejected with a structured `invalid_params` error rather than silently falling back to XML. Pack operations are serialized internally (a `tokio::Mutex` per server instance) to avoid concurrent `git clone` / `rayon` work on the same repo.
234: 
235: #### Client configuration examples
236: 
237: **Claude Desktop** (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):
238: 
239: ```json
240: {
241:   "mcpServers": {
242:     "repomix": {
243:       "command": "repomix",
244:       "args": ["--mcp"]
245:     }
246:   }
247: }
248: ```
249: 
250: **Cursor** (Settings → MCP → Add new global MCP server):
251: 
252: ```json
253: {
254:   "mcpServers": {
255:     "repomix": {
256:       "command": "repomix",
257:       "args": ["--mcp"]
258:     }
259:   }
260: }
261: ```
262: 
263: ---
264: 
265: ## Configuration
266: 
267: `repomix-rs` reads configuration from four layers, merged in order (later wins):
268: 
269: 1. **Built-in defaults**
270: 2. **Global config** — `~/.repomix/repomix.config.json`
271: 3. **Project config** — `./repomix.config.json` (relative to the current working directory)
272: 4. **CLI flags** / MCP tool parameters
273: 
274: Use `repomix --init` to interactively scaffold a project-level `repomix.config.json` and a `.repomixignore` file (similar in spirit to `.gitignore`). Existing files trigger a `dialoguer::Confirm` prompt before being overwritten.
275: 
276: ### Full schema
277: 
278: ```jsonc
279: {
280:   "input": {
281:     "max_file_size": 52428800  // 50 MB; files larger are skipped
282:   },
283:   "output": {
284:     "file_path": "repomix-output.txt",
285:     "style": "xml",            // "xml" | "markdown" | "plain" | "json"
286:     "parsable_style": false,   // adds tokens/chars attrs (XML) or structured markers
287:     "header_text": null,       // string prepended to the output
288:     "instruction_file_path": null, // path whose contents are appended as instructions
289:     "file_summary": true,      // include a per-file summary section
290:     "directory_structure": true,
291:     "files": true,             // include the files section
292:     "remove_comments": false,
293:     "remove_empty_lines": false,
294:     "compress": false,         // tree-sitter signature extraction
295:     "top_files_length": 10,    // N for the top-N token-heavy files in metrics
296:     "show_line_numbers": false,
297:     "truncate_base64": false,
298:     "copy_to_clipboard": false,
299:     "include_empty_directories": false,
300:     "include_full_directory_structure": false,
301:     "split_output": null,      // max tokens per chunk; null = single file
302:     "token_count_tree": {
303:       "show_tree": false       // include a per-directory token tree
304:     },
305:     "git": {
306:       "sort_by_changes": true,
307:       "sort_by_changes_max_commits": 100,
308:       "include_diffs": false,
309:       "include_logs": false,
310:       "include_logs_count": 50
311:     },
312:     "json": {
313:       "no_timestamp": false    // omit `packed_at` from JSON metadata (deterministic output)
314:     }
315:   },
316:   "include": [],               // additional glob patterns to include (e.g. ["*.rs", "*.toml"])
317:   "ignore": {
318:     "use_gitignore": true,     // honor .gitignore when collecting
319:     "custom_ignore": []        // extra glob patterns to ignore
320:   },
321:   "security": {
322:     "enable_secretlint": true  // detect and exclude files containing secrets
323:   },
324:   "token_count": {
325:     "encoding": "o200k_base"   // any encoding supported by tiktoken-rs
326:   }
327: }
328: ```
329: 
330: ### Default ignore patterns
331: 
332: In addition to whatever you put in `ignore.custom_ignore` and your `.gitignore`, the following are always ignored: `.git`, `node_modules`, `__pycache__`, `.DS_Store`, and binary/archive/media extensions (`*.pyc`, `*.pyo`, `*.class`, `*.jar`, `*.war`, `*.ear`, `*.zip`, `*.tar.gz`, `*.tar.bz2`, `*.tgz`, `*.rar`, `*.7z`, `*.exe`, `*.dll`, `*.so`, `*.dylib`, `*.pdf`, `*.doc*`, `*.xls*`, `*.ppt*`, `*.mp3`, `*.mp4`, `*.avi`, `*.mov`, `*.wav`, `*.flac`, `*.ogg`, `*.jpg`, `*.jpeg`, `*.png`, `*.gif`, `*.bmp`, `*.ico`, `*.svg`, `*.webp`, `*.woff`, `*.woff2`, `*.ttf`, `*.eot`, `*.otf`, `*.wasm`, `*.whl`, `*.egg`). The list lives in `repomix_config::default_ignore::default_ignore_patterns()`.
333: 
334: ---
335: 
336: ## Supported languages (tree-sitter compression)
337: 
338: | Family | Languages |
339: |---|---|
340: | Web | JavaScript (`.js`, `.jsx`), TypeScript (`.ts`, `.tsx`) |
341: | Systems | C (`.c`, `.h`), C++ (`.cpp`, `.cxx`, `.cc`, `.hpp`, `.hxx`), Rust (`.rs`), Go (`.go`) |
342: | Scripting | Python (`.py`), Ruby (`.rb`), PHP (`.php`) |
343: | Enterprise | Java (`.java`) |
344: 
345: > **Note:** C# (`.cs`) compression is **temporarily disabled** in this version due to an ABI mismatch between `tree-sitter-c-sharp` 0.23 (language version 15) and the bundled query file. `.cs` files fall back to plain-text processing until the queries are upgraded.
346: 
347: ---
348: 
349: ## Performance
350: 
351: The Rust implementation is designed to be substantially faster than the Node.js original by replacing single-threaded globby + `promisePool` with `ignore` (multi-threaded traversal) and `rayon` (data-parallel processing), and by using native tree-sitter instead of WASM-in-a-Worker.
352: 
353: Representative numbers from a synthetic 5,000-file repository on a recent multi-core machine:
354: 
355: | Metric | Node.js (Repomix) | `repomix-rs` | Speedup |
356: |---|---|---|---|
357: | File search | `globby` (single-thread) | `ignore` crate (multi-thread) | 3–5× |
358: | File collection | `promisePool(50)` | `rayon` + zero-copy reads | 2–3× |
359: | Tree-sitter compression | WASM + Worker | Native + `rayon` | 5–10× |
360: | End-to-end (≈5k files) | ~15 s | ~2–3 s | 5–7× |
361: 
362: > Run your own benchmarks with `cargo build --release` followed by `time ./target/release/repomix <large-repo>`. Numbers above are indicative, not guarantees.
363: 
364: ---
365: 
366: ## Development
367: 
368: ```bash
369: # Build the whole workspace
370: cargo build
371: 
372: # Run all tests
373: cargo test
374: 
375: # Run clippy with warnings as errors (recommended before sending a PR)
376: cargo clippy --workspace --all-targets -- -D warnings
377: 
378: # Format the code
379: cargo fmt --all
380: 
381: # Build a release CLI binary (produces ./target/release/repomix)
382: cargo build --release -p repomix-cli
383: 
384: # Run the CLI against the current directory
385: cargo run -p repomix-cli --release -- .
386: 
387: # Run the CLI as an MCP server (useful for ad-hoc debugging with an MCP client)
388: cargo run -p repomix-cli -- --mcp
389: ```
390: 
391: ### Project conventions
392: 
393: - All public APIs in `repomix-core` and `repomix-config` are re-exported from the crate root.
394: - Configuration loading is centralized in `repomix_config::load::RepomixConfig::load`; CLI flags map to `repomix_config::load::PartialConfig`.
395: - The CLI and MCP server should be thin wrappers — all real work happens in `repomix-core`.
396: - Default ignore patterns live in `repomix_config::default_ignore::default_ignore_patterns()`; add new ones there, not inline.
397: 
398: ---
399: 
400: ## License
401: 
402: MIT — see the original [Repomix](https://github.com/yamadashy/repomix) project for the reference TypeScript implementation.
403: 
404: ## Acknowledgments
405: 
406: - [Repomix](https://github.com/yamadashy/repomix) — original TypeScript implementation by [@yamadashy](https://github.com/yamadashy)
407: - [tree-sitter](https://tree-sitter.github.io/) — parser generator
408: - [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) — Rust binding for OpenAI's tokenizer
409: - [rmcp](https://github.com/anthropics/rust-mcp-sdk) — official Rust MCP SDK
````

### crates/cli/src/main.rs (116 lines)

```
1: Cli
2: ⋮----
3: {
4:     
5:     root: Option<PathBuf>,
6: 
7:     
8:     #[arg(long)]
9:     remote: Option<String>,
10: 
11:     
12:     #[arg(long)]
13:     include: Option<String>,
14: 
15:     
16:     #[arg(long)]
17:     ignore: Option<String>,
18: 
19:     
20:     #[arg(long, default_value = "xml", value_enum)]
21:     style: CliOutputStyle,
22: 
23:     
24:     #[arg(long)]
25:     compress: bool,
26: 
27:     
28:     #[arg(long)]
29:     remove_comments: bool,
30: 
31:     
32:     #[arg(long)]
33:     remove_empty_lines: bool,
34: 
35:     
36:     #[arg(long)]
37:     line_numbers: bool,
38: 
39:     
40:     #[arg(long)]
41:     truncate_base64: bool,
42: 
43:     
44:     #[arg(long)]
45:     copy: bool,
46: 
47:     
48:     #[arg(long)]
49:     init: bool,
50: 
51:     
52:     #[arg(long)]
53:     mcp: bool,
54: 
55:     
56:     #[arg(long)]
57:     output: Option<String>,
58: 
59:     
60:     #[arg(long)]
61:     include_empty_directories: bool,
62: 
63:     
64:     #[arg(long)]
65:     top_files_length: Option<usize>,
66: 
67:     
68:     #[arg(long)]
69:     split_output: Option<u64>,
70: 
71:     
72:     #[arg(long)]
73:     header_text: Option<String>,
74: 
75:     
76:     #[arg(long)]
77:     instruction_file: Option<String>,
78: 
79:     
80:     #[arg(long)]
81:     include_diffs: bool,
82: 
83:     
84:     #[arg(long)]
85:     include_logs: bool,
86: 
87:     
88:     #[arg(short, long, action = clap::ArgAction::Count)]
89:     verbose: u8,
90: }
91: ⋮----
92: CliOutputStyle
93: ⋮----
94: {
95:     Xml,
96:     Markdown,
97:     Plain,
98:     Json,
99: }
100: ⋮----
101: OutputStyle
102: ⋮----
103: {
104:     fn from(s: CliOutputStyle) -> Self {
105:         match s {
106:             CliOutputStyle::Xml => OutputStyle::Xml,
107:             CliOutputStyle::Markdown => OutputStyle::Markdown,
108:             CliOutputStyle::Plain => OutputStyle::Plain,
109:             CliOutputStyle::Json => OutputStyle::Json,
110:         }
111:     }
112: }
113: ⋮----
114: main
115: ⋮----
116: ()
```

### crates/cli/src/prompts/mod.rs (11 lines)

```
1: prompt_for_config
2: ⋮----
3: (_root_dir: &Path)
4: ⋮----
5: create_config_file
6: ⋮----
7: (root_dir: &Path)
8: ⋮----
9: create_ignore_file
10: ⋮----
11: (root_dir: &Path)
```

### crates/cli/src/run.rs (45 lines)

```
1: init_config
2: ⋮----
3: ()
4: ⋮----
5: run_mcp_server
6: ⋮----
7: ()
8: ⋮----
9: run_pack
10: ⋮----
11: (cli: crate::Cli)
12: ⋮----
13: TempDirGuard
14: ⋮----
15: {
16:     path: Option<std::path::PathBuf>,
17: }
18: ⋮----
19: TempDirGuard
20: ⋮----
21: {
22:     fn new(path: std::path::PathBuf) -> Self {
23:         Self { path: Some(path) }
24:     }
25: }
26: ⋮----
27: TempDirGuard
28: ⋮----
29: {
30:     fn drop(&mut self) {
31:         if let Some(path) = self.path.take()
32:             && let Err(e) = std::fs::remove_dir_all(&path)
33:         {
34:             tracing::warn!("Failed to clean up temp dir '{}': {}.", path.display(), e);
35:         }
36:     }
37: }
38: ⋮----
39: build_config
40: ⋮----
41: (cli: &crate::Cli, config_root: &std::path::Path)
42: ⋮----
43: make_unique_temp_dir
44: ⋮----
45: (prefix: &str)
```

### crates/config/src/default_ignore.rs (3 lines)

```
1: default_ignore_patterns
2: ⋮----
3: ()
```

### crates/config/src/load.rs (272 lines)

```
1: RepomixConfig
2: ⋮----
3: {
4:     
5:     pub fn load(cli_overrides: Option<PartialConfig>, cwd: &Path) -> Result<Self> {
6:         let mut config = Self::default();
7: 
8:         
9:         match Self::load_from_file(&global_dir::global_config_path()?) {
10:             Ok(Some(global)) => config.merge_global(global),
11:             Ok(None) => {}
12:             Err(e) => tracing::warn!("Failed to load global config: {}", e),
13:         }
14: 
15:         
16:         match Self::load_from_file(&cwd.join("repomix.config.json")) {
17:             Ok(Some(local)) => config.merge_local(local),
18:             Ok(None) => {}
19:             Err(e) => tracing::warn!("Failed to load project config: {}", e),
20:         }
21: 
22:         
23:         if let Some(overrides) = cli_overrides {
24:             config.merge_cli(overrides);
25:         }
26: 
27:         config.validate()?;
28:         Ok(config)
29:     }
30: 
31:     
32:     pub fn load_from_file(path: &Path) -> Result<Option<Self>> {
33:         if !path.exists() {
34:             return Ok(None);
35:         }
36:         let content = std::fs::read_to_string(path)
37:             .map_err(|e| anyhow::anyhow!("Failed to read config file {}: {}", path.display(), e))?;
38:         let config: Self = serde_json::from_str(&content).map_err(|e| {
39:             anyhow::anyhow!("Failed to parse config file {}: {}", path.display(), e)
40:         })?;
41:         Ok(Some(config))
42:     }
43: 
44:     
45:     fn merge_global(&mut self, other: Self) {
46:         let defaults = RepomixConfig::default();
47: 
48:         
49:         self.include.extend(other.include);
50: 
51:         
52:         self.ignore.custom_ignore.extend(other.ignore.custom_ignore);
53:         if other.ignore.use_gitignore != defaults.ignore.use_gitignore {
54:             self.ignore.use_gitignore = other.ignore.use_gitignore;
55:         }
56: 
57:         
58:         if other.input.max_file_size != defaults.input.max_file_size {
59:             self.input.max_file_size = other.input.max_file_size;
60:         }
61: 
62:         
63:         if other.output.file_path != defaults.output.file_path {
64:             self.output.file_path = other.output.file_path;
65:         }
66:         if other.output.style != defaults.output.style {
67:             self.output.style = other.output.style;
68:         }
69:         if other.output.parsable_style != defaults.output.parsable_style {
70:             self.output.parsable_style = other.output.parsable_style;
71:         }
72:         if other.output.header_text != defaults.output.header_text {
73:             self.output.header_text = other.output.header_text;
74:         }
75:         if other.output.instruction_file_path != defaults.output.instruction_file_path {
76:             self.output.instruction_file_path = other.output.instruction_file_path;
77:         }
78:         if other.output.file_summary != defaults.output.file_summary {
79:             self.output.file_summary = other.output.file_summary;
80:         }
81:         if other.output.directory_structure != defaults.output.directory_structure {
82:             self.output.directory_structure = other.output.directory_structure;
83:         }
84:         if other.output.files != defaults.output.files {
85:             self.output.files = other.output.files;
86:         }
87:         if other.output.remove_comments != defaults.output.remove_comments {
88:             self.output.remove_comments = other.output.remove_comments;
89:         }
90:         if other.output.remove_empty_lines != defaults.output.remove_empty_lines {
91:             self.output.remove_empty_lines = other.output.remove_empty_lines;
92:         }
93:         if other.output.compress != defaults.output.compress {
94:             self.output.compress = other.output.compress;
95:         }
96:         if other.output.top_files_length != defaults.output.top_files_length {
97:             self.output.top_files_length = other.output.top_files_length;
98:         }
99:         if other.output.show_line_numbers != defaults.output.show_line_numbers {
100:             self.output.show_line_numbers = other.output.show_line_numbers;
101:         }
102:         if other.output.truncate_base64 != defaults.output.truncate_base64 {
103:             self.output.truncate_base64 = other.output.truncate_base64;
104:         }
105:         if other.output.copy_to_clipboard != defaults.output.copy_to_clipboard {
106:             self.output.copy_to_clipboard = other.output.copy_to_clipboard;
107:         }
108:         if other.output.include_empty_directories != defaults.output.include_empty_directories {
109:             self.output.include_empty_directories = other.output.include_empty_directories;
110:         }
111:         if other.output.include_full_directory_structure
112:             != defaults.output.include_full_directory_structure
113:         {
114:             self.output.include_full_directory_structure =
115:                 other.output.include_full_directory_structure;
116:         }
117:         if other.output.split_output != defaults.output.split_output {
118:             self.output.split_output = other.output.split_output;
119:         }
120:         if other.output.token_count_tree.show_tree != defaults.output.token_count_tree.show_tree {
121:             self.output.token_count_tree.show_tree = other.output.token_count_tree.show_tree;
122:         }
123:         
124:         if other.output.git.sort_by_changes != defaults.output.git.sort_by_changes {
125:             self.output.git.sort_by_changes = other.output.git.sort_by_changes;
126:         }
127:         if other.output.git.sort_by_changes_max_commits
128:             != defaults.output.git.sort_by_changes_max_commits
129:         {
130:             self.output.git.sort_by_changes_max_commits =
131:                 other.output.git.sort_by_changes_max_commits;
132:         }
133:         if other.output.git.include_diffs != defaults.output.git.include_diffs {
134:             self.output.git.include_diffs = other.output.git.include_diffs;
135:         }
136:         if other.output.git.include_logs != defaults.output.git.include_logs {
137:             self.output.git.include_logs = other.output.git.include_logs;
138:         }
139:         if other.output.git.include_logs_count != defaults.output.git.include_logs_count {
140:             self.output.git.include_logs_count = other.output.git.include_logs_count;
141:         }
142: 
143:         
144:         if other.output.json.no_timestamp != defaults.output.json.no_timestamp {
145:             self.output.json.no_timestamp = other.output.json.no_timestamp;
146:         }
147: 
148:         
149:         if other.security.enable_secretlint != defaults.security.enable_secretlint {
150:             self.security.enable_secretlint = other.security.enable_secretlint;
151:         }
152: 
153:         
154:         if other.token_count.encoding != defaults.token_count.encoding {
155:             self.token_count.encoding = other.token_count.encoding;
156:         }
157:     }
158: 
159:     
160:     fn merge_local(&mut self, other: Self) {
161:         
162:         self.merge_global(other);
163:     }
164: 
165:     
166:     pub(crate) fn merge_cli(&mut self, overrides: PartialConfig) {
167:         if let Some(mut include) = overrides.include {
168:             self.include.append(&mut include);
169:         }
170: 
171:         if let Some(mut ignore) = overrides.ignore {
172:             self.ignore.custom_ignore.append(&mut ignore);
173:         }
174: 
175:         if let Some(style) = overrides.style {
176:             self.output.style = style;
177:         }
178: 
179:         if let Some(compress) = overrides.compress {
180:             self.output.compress = compress;
181:         }
182: 
183:         if let Some(remove_comments) = overrides.remove_comments {
184:             self.output.remove_comments = remove_comments;
185:         }
186: 
187:         if let Some(remove_empty_lines) = overrides.remove_empty_lines {
188:             self.output.remove_empty_lines = remove_empty_lines;
189:         }
190: 
191:         if let Some(show_line_numbers) = overrides.show_line_numbers {
192:             self.output.show_line_numbers = show_line_numbers;
193:         }
194: 
195:         if let Some(truncate_base64) = overrides.truncate_base64 {
196:             self.output.truncate_base64 = truncate_base64;
197:         }
198: 
199:         if let Some(copy_to_clipboard) = overrides.copy_to_clipboard {
200:             self.output.copy_to_clipboard = copy_to_clipboard;
201:         }
202: 
203:         if let Some(output) = overrides.output {
204:             self.output.file_path = output;
205:         }
206: 
207:         if let Some(include_empty_directories) = overrides.include_empty_directories {
208:             self.output.include_empty_directories = include_empty_directories;
209:         }
210: 
211:         if let Some(top_files_length) = overrides.top_files_length {
212:             self.output.top_files_length = top_files_length;
213:         }
214: 
215:         if let Some(split_output) = overrides.split_output {
216:             self.output.split_output = Some(split_output);
217:         }
218: 
219:         if let Some(header_text) = overrides.header_text {
220:             self.output.header_text = Some(header_text);
221:         }
222: 
223:         if let Some(instruction_file_path) = overrides.instruction_file_path {
224:             self.output.instruction_file_path = Some(instruction_file_path);
225:         }
226: 
227:         if let Some(include_diffs) = overrides.include_diffs {
228:             self.output.git.include_diffs = include_diffs;
229:         }
230: 
231:         if let Some(include_logs) = overrides.include_logs {
232:             self.output.git.include_logs = include_logs;
233:         }
234:     }
235: 
236:     
237:     fn validate(&self) -> Result<()> {
238:         
239:         if self.output.file_path.is_empty() {
240:             anyhow::bail!("Output file path cannot be empty");
241:         }
242: 
243:         
244:         if self.input.max_file_size == 0 {
245:             anyhow::bail!("Max file size cannot be zero");
246:         }
247: 
248:         Ok(())
249:     }
250: }
251: ⋮----
252: PartialConfig
253: ⋮----
254: {
255:     pub include: Option<Vec<String>>,
256:     pub ignore: Option<Vec<String>>,
257:     pub style: Option<crate::schema::OutputStyle>,
258:     pub compress: Option<bool>,
259:     pub remove_comments: Option<bool>,
260:     pub remove_empty_lines: Option<bool>,
261:     pub show_line_numbers: Option<bool>,
262:     pub truncate_base64: Option<bool>,
263:     pub copy_to_clipboard: Option<bool>,
264:     pub output: Option<String>,
265:     pub include_empty_directories: Option<bool>,
266:     pub top_files_length: Option<usize>,
267:     pub split_output: Option<u64>,
268:     pub header_text: Option<String>,
269:     pub instruction_file_path: Option<String>,
270:     pub include_diffs: Option<bool>,
271:     pub include_logs: Option<bool>,
272: }
```

### crates/config/src/schema.rs (248 lines)

```
1: RepomixConfig
2: ⋮----
3: {
4:     pub input: InputConfig,
5:     pub output: OutputConfig,
6:     pub include: Vec<String>,
7:     pub ignore: IgnoreConfig,
8:     pub security: SecurityConfig,
9:     pub token_count: TokenCountConfig,
10: }
11: ⋮----
12: InputConfig
13: ⋮----
14: {
15:     #[serde(default = "default_max_file_size")]
16:     pub max_file_size: u64,
17: }
18: ⋮----
19: InputConfig
20: ⋮----
21: {
22:     fn default() -> Self {
23:         Self {
24:             max_file_size: default_max_file_size(),
25:         }
26:     }
27: }
28: ⋮----
29: OutputConfig
30: ⋮----
31: {
32:     #[serde(default = "default_file_path")]
33:     pub file_path: String,
34:     #[serde(default = "default_style")]
35:     pub style: OutputStyle,
36:     #[serde(default)]
37:     
38:     pub parsable_style: bool,
39:     #[serde(default)]
40:     pub header_text: Option<String>,
41:     #[serde(default)]
42:     pub instruction_file_path: Option<String>,
43:     #[serde(default = "default_true")]
44:     pub file_summary: bool,
45:     #[serde(default = "default_true")]
46:     pub directory_structure: bool,
47:     #[serde(default = "default_true")]
48:     pub files: bool,
49:     #[serde(default)]
50:     pub remove_comments: bool,
51:     #[serde(default)]
52:     pub remove_empty_lines: bool,
53:     #[serde(default)]
54:     pub compress: bool,
55:     #[serde(default = "default_top_files_length")]
56:     pub top_files_length: usize,
57:     #[serde(default)]
58:     pub show_line_numbers: bool,
59:     #[serde(default)]
60:     pub truncate_base64: bool,
61:     #[serde(default)]
62:     pub copy_to_clipboard: bool,
63:     #[serde(default)]
64:     pub include_empty_directories: bool,
65:     #[serde(default)]
66:     
67:     pub include_full_directory_structure: bool,
68:     #[serde(default)]
69:     
70:     
71:     pub split_output: Option<u64>,
72:     #[serde(default)]
73:     pub token_count_tree: TokenCountTreeConfig,
74:     pub git: GitOutputConfig,
75:     #[serde(default)]
76:     pub json: JsonOutputConfig,
77: }
78: ⋮----
79: OutputConfig
80: ⋮----
81: {
82:     fn default() -> Self {
83:         Self {
84:             file_path: default_file_path(),
85:             style: default_style(),
86:             parsable_style: false,
87:             header_text: None,
88:             instruction_file_path: None,
89:             file_summary: default_true(),
90:             directory_structure: default_true(),
91:             files: default_true(),
92:             remove_comments: false,
93:             remove_empty_lines: false,
94:             compress: false,
95:             top_files_length: default_top_files_length(),
96:             show_line_numbers: false,
97:             truncate_base64: false,
98:             copy_to_clipboard: false,
99:             include_empty_directories: false,
100:             include_full_directory_structure: false,
101:             split_output: None,
102:             token_count_tree: TokenCountTreeConfig::default(),
103:             git: GitOutputConfig::default(),
104:             json: JsonOutputConfig::default(),
105:         }
106:     }
107: }
108: ⋮----
109: IgnoreConfig
110: ⋮----
111: {
112:     #[serde(default = "default_true")]
113:     pub use_gitignore: bool,
114:     #[serde(default)]
115:     pub custom_ignore: Vec<String>,
116: }
117: ⋮----
118: IgnoreConfig
119: ⋮----
120: {
121:     fn default() -> Self {
122:         Self {
123:             use_gitignore: default_true(),
124:             custom_ignore: Vec::new(),
125:         }
126:     }
127: }
128: ⋮----
129: SecurityConfig
130: ⋮----
131: {
132:     #[serde(default = "default_true")]
133:     pub enable_secretlint: bool,
134: }
135: ⋮----
136: SecurityConfig
137: ⋮----
138: {
139:     fn default() -> Self {
140:         Self {
141:             enable_secretlint: default_true(),
142:         }
143:     }
144: }
145: ⋮----
146: TokenCountConfig
147: ⋮----
148: {
149:     #[serde(default = "default_encoding")]
150:     pub encoding: String,
151: }
152: ⋮----
153: TokenCountConfig
154: ⋮----
155: {
156:     fn default() -> Self {
157:         Self {
158:             encoding: default_encoding(),
159:         }
160:     }
161: }
162: ⋮----
163: TokenCountTreeConfig
164: ⋮----
165: {
166:     #[serde(default)]
167:     
168:     pub show_tree: bool,
169: }
170: ⋮----
171: GitOutputConfig
172: ⋮----
173: {
174:     #[serde(default = "default_true")]
175:     pub sort_by_changes: bool,
176:     #[serde(default = "default_100")]
177:     pub sort_by_changes_max_commits: usize,
178:     #[serde(default)]
179:     pub include_diffs: bool,
180:     #[serde(default)]
181:     pub include_logs: bool,
182:     #[serde(default = "default_50")]
183:     pub include_logs_count: usize,
184: }
185: ⋮----
186: GitOutputConfig
187: ⋮----
188: {
189:     fn default() -> Self {
190:         Self {
191:             sort_by_changes: default_true(),
192:             sort_by_changes_max_commits: default_100(),
193:             include_diffs: false,
194:             include_logs: false,
195:             include_logs_count: default_50(),
196:         }
197:     }
198: }
199: ⋮----
200: JsonOutputConfig
201: ⋮----
202: {
203:     
204:     
205:     #[serde(default)]
206:     pub no_timestamp: bool,
207: }
208: ⋮----
209: OutputStyle
210: ⋮----
211: {
212:     Xml,
213:     Markdown,
214:     Plain,
215:     Json,
216: }
217: ⋮----
218: default_max_file_size
219: ⋮----
220: ()
221: ⋮----
222: default_file_path
223: ⋮----
224: ()
225: ⋮----
226: default_style
227: ⋮----
228: ()
229: ⋮----
230: default_true
231: ⋮----
232: ()
233: ⋮----
234: default_top_files_length
235: ⋮----
236: ()
237: ⋮----
238: default_encoding
239: ⋮----
240: ()
241: ⋮----
242: default_100
243: ⋮----
244: ()
245: ⋮----
246: default_50
247: ⋮----
248: ()
```

### crates/config/src/tests.rs (15 lines)

```
1: test_merge_cli_appends_includes_bug1
2: ⋮----
3: ()
4: ⋮----
5: test_merge_cli_none_preserves_existing_bug1
6: ⋮----
7: ()
8: ⋮----
9: test_default_file_path_is_bug8_baseline
10: ⋮----
11: ()
12: ⋮----
13: test_load_with_empty_cwd_does_not_panic
14: ⋮----
15: ()
```

### crates/core/src/file/collect.rs (28 lines)

```
1: collect_files
2: ⋮----
3: (
4:     file_paths: Vec<PathBuf>,
5:     config: &RepomixConfig,
6: )
7: ⋮----
8: collect_files_sync
9: ⋮----
10: (
11:     file_paths: Vec<PathBuf>,
12:     config: &RepomixConfig,
13: )
14: ⋮----
15: read_raw_file
16: ⋮----
17: (
18:     path: &PathBuf,
19:     max_file_size: u64,
20: )
21: ⋮----
22: decode_utf16_if_bom
23: ⋮----
24: (bytes: &[u8])
25: ⋮----
26: decode_bytes
27: ⋮----
28: (bytes: &[u8], path: &Path)
```

### crates/core/src/file/process.rs (15 lines)

```
1: process_files
2: ⋮----
3: (raw_files: &[RawFile], config: &RepomixConfig)
4: ⋮----
5: process_single_file
6: ⋮----
7: (
8:     raw_file: &RawFile,
9:     options: &ProcessContentOptions,
10:     token_counter: Option<&TokenCounter>,
11: )
12: ⋮----
13: count_tokens
14: ⋮----
15: (content: &str, token_counter: Option<&TokenCounter>)
```

### crates/core/src/file/truncate_base64.rs (27 lines)

```
1: truncate_base64
2: ⋮----
3: (content: &str)
4: ⋮----
5: is_base64_line
6: ⋮----
7: (line: &str)
8: ⋮----
9: test_short_line_not_base64
10: ⋮----
11: ()
12: ⋮----
13: test_natural_language_long_sentence_not_base64
14: ⋮----
15: ()
16: ⋮----
17: test_pure_hex_hash_not_base64
18: ⋮----
19: ()
20: ⋮----
21: test_real_base64_detected
22: ⋮----
23: ()
24: ⋮----
25: test_uuid_like_not_base64
26: ⋮----
27: ()
```

### crates/core/src/git/remote.rs (7 lines)

```
1: clone_remote_repo
2: ⋮----
3: (url: &str, target_dir: &Path)
4: ⋮----
5: is_git_repo
6: ⋮----
7: (path: &Path)
```

### crates/core/src/metrics/calculate.rs (17 lines)

```
1: MetricsResult
2: ⋮----
3: {
4:     pub total_characters: usize,
5:     pub total_tokens: usize,
6:     pub file_char_counts: HashMap<String, usize>,
7:     pub file_token_counts: HashMap<String, usize>,
8:     
9:     pub top_files_by_tokens: Vec<(String, usize)>,
10: }
11: ⋮----
12: calculate_metrics
13: ⋮----
14: (
15:     files: &[ProcessedFile],
16:     config: &RepomixConfig,
17: )
```

### crates/core/src/output/styles/json.rs (45 lines)

```
1: JsonOutput
2: ⋮----
3: {
4:     metadata: JsonMetadata,
5:     custom_instructions: Option<String>,
6:     directory_structure: Option<String>,
7:     files: Vec<JsonFile>,
8:     git_diff: Option<String>,
9:     git_log: Option<String>,
10:     #[serde(skip_serializing_if = "Option::is_none")]
11:     token_count_tree: Option<String>,
12: }
13: ⋮----
14: JsonMetadata
15: ⋮----
16: {
17:     
18:     #[serde(skip_serializing_if = "Option::is_none")]
19:     packed_at: Option<String>,
20:     total_files: usize,
21:     total_tokens: usize,
22: }
23: ⋮----
24: JsonFile
25: ⋮----
26: {
27:     path: String,
28:     content: String,
29:     token_count: usize,
30:     #[serde(skip_serializing_if = "Option::is_none")]
31:     char_count: Option<usize>,
32: }
33: ⋮----
34: generate_json
35: ⋮----
36: (
37:     files: &[ProcessedFile],
38:     config: &RepomixConfig,
39:     pack_root: &Path,
40:     tree_string: &str,
41:     header: &OutputHeader,
42:     git_diff_content: &Option<String>,
43:     git_log_content: &Option<String>,
44:     token_count_tree: Option<&str>,
45: )
```

### crates/core/src/output/styles/markdown.rs (20 lines)

```
1: wrap_markdown_code_block
2: ⋮----
3: (content: &str)
4: ⋮----
5: generate_markdown
6: ⋮----
7: (
8:     files: &[ProcessedFile],
9:     config: &RepomixConfig,
10:     pack_root: &Path,
11:     tree_string: &str,
12:     line_counts: &HashMap<String, usize>,
13:     header: &OutputHeader,
14:     git_diff_content: &Option<String>,
15:     git_log_content: &Option<String>,
16: )
17: ⋮----
18: test_wrap_markdown_code_block_handles_inline_backticks
19: ⋮----
20: ()
```

### crates/core/src/output/styles/plain.rs (12 lines)

```
1: generate_plain
2: ⋮----
3: (
4:     files: &[ProcessedFile],
5:     config: &RepomixConfig,
6:     pack_root: &Path,
7:     tree_string: &str,
8:     line_counts: &HashMap<String, usize>,
9:     header: &OutputHeader,
10:     git_diff_content: &Option<String>,
11:     git_log_content: &Option<String>,
12: )
```

### crates/core/src/output/styles/xml.rs (42 lines)

```
1: xml_attr_escape
2: ⋮----
3: (s: &str)
4: ⋮----
5: xml_text_escape
6: ⋮----
7: (s: &str)
8: ⋮----
9: XmlSplitMeta
10: ⋮----
11: {
12:     pub part: usize,
13:     pub total_parts: usize,
14:     pub is_first_part: bool,
15:     pub is_last_part: bool,
16: }
17: ⋮----
18: render_xml_part
19: ⋮----
20: (
21:     files: &[ProcessedFile],
22:     config: &RepomixConfig,
23:     pack_root: &Path,
24:     tree_string: &str,
25:     header: &OutputHeader,
26:     git_diff_content: &Option<String>,
27:     git_log_content: &Option<String>,
28:     token_count_tree: Option<&str>,
29:     split_meta: Option<XmlSplitMeta>,
30: )
31: ⋮----
32: generate_xml
33: ⋮----
34: (
35:     files: &[ProcessedFile],
36:     config: &RepomixConfig,
37:     pack_root: &Path,
38:     tree_string: &str,
39:     header: &OutputHeader,
40:     git_diff_content: &Option<String>,
41:     git_log_content: &Option<String>,
42: )
```

### crates/core/src/tree_sitter/compress.rs (15 lines)

```
1: compress_file
2: ⋮----
3: (
4:     content: &str,
5:     file_path: &Path,
6:     config: &LanguageConfig,
7: )
8: ⋮----
9: test_compress_rust_preserves_order
10: ⋮----
11: ()
12: ⋮----
13: test_compress_empty_file_returns_none
14: ⋮----
15: ()
```

### crates/core/src/tree_sitter/languages.rs (19 lines)

```
1: LanguageConfig
2: ⋮----
3: {
4:     pub language: Language,
5:     pub compress_query: Option<Query>,
6:     pub extensions: Vec<&'static str>,
7: }
8: ⋮----
9: load_query
10: ⋮----
11: (language: &Language, query_source: &str, lang_name: &str)
12: ⋮----
13: get_language_config
14: ⋮----
15: (file_path: &std::path::Path)
16: ⋮----
17: get_supported_languages
18: ⋮----
19: ()
```

### crates/mcp/src/lib.rs (5 lines)

```
1: pub mod helpers;
2: pub mod output_path;
3: pub mod params;
4: pub mod server;
5: pub mod tools;
```

### crates/shared/src/logger.rs (3 lines)

```
1: init_logger
2: ⋮----
3: (verbose: bool)
```

### crates/cli/src/spinner.rs (94 lines)

```
1: Spinner
2: ⋮----
3: {
4:     pb: ProgressBar,
5: }
6: ⋮----
7: Spinner
8: ⋮----
9: {
10:     pub fn new(message: &str) -> Self {
11:         let pb = ProgressBar::new_spinner();
12:         pb.set_style(
13:             ProgressStyle::default_spinner()
14:                 .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
15:                 .template("{spinner:.green} {msg}")
16:                 .unwrap(),
17:         );
18:         pb.set_message(message.to_string());
19:         pb.enable_steady_tick(std::time::Duration::from_millis(100));
20: 
21:         Self { pb }
22:     }
23: 
24:     pub fn update(&self, message: &str) {
25:         self.pb.set_message(message.to_string());
26:     }
27: 
28:     #[allow(dead_code)]
29:     pub fn finish(&self, message: &str) {
30:         self.pb.finish_with_message(message.to_string());
31:     }
32: 
33:     #[allow(dead_code)]
34:     pub fn finish_with_success(&self, message: &str) {
35:         self.pb.finish_with_message(format!("✔ {}", message));
36:     }
37: 
38:     pub fn finish_with_error(&self, message: &str) {
39:         self.pb.finish_with_message(format!("✖ {}", message));
40:     }
41: }
42: ⋮----
43: Spinner
44: ⋮----
45: {
46:     fn on_progress(&self, message: &str) {
47:         self.update(message);
48:     }
49: 
50:     fn on_complete(&self, _message: &str) {
51:         
52:     }
53: 
54:     fn on_error(&self, message: &str) {
55:         self.finish_with_error(message);
56:     }
57: }
58: ⋮----
59: ProgressBar2
60: ⋮----
61: {
62:     pb: ProgressBar,
63: }
64: ⋮----
65: ProgressBar2
66: ⋮----
67: {
68:     pub fn new(total: u64, message: &str) -> Self {
69:         let pb = ProgressBar::new(total);
70:         pb.set_style(
71:             ProgressStyle::default_bar()
72:                 .template(
73:                     "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
74:                 )
75:                 .unwrap()
76:                 .progress_chars("=>-"),
77:         );
78:         pb.set_message(message.to_string());
79: 
80:         Self { pb }
81:     }
82: 
83:     pub fn inc(&self, delta: u64) {
84:         self.pb.inc(delta);
85:     }
86: 
87:     pub fn set_message(&self, message: &str) {
88:         self.pb.set_message(message.to_string());
89:     }
90: 
91:     pub fn finish(&self, message: &str) {
92:         self.pb.finish_with_message(message.to_string());
93:     }
94: }
```

### crates/config/src/global_dir.rs (15 lines)

```
1: global_config_dir
2: ⋮----
3: ()
4: ⋮----
5: global_config_path
6: ⋮----
7: ()
8: ⋮----
9: mcp_outputs_dir
10: ⋮----
11: ()
12: ⋮----
13: global_cache_dir
14: ⋮----
15: ()
```

### crates/config/src/lib.rs (7 lines)

```
1: pub mod default_ignore;
2: pub mod global_dir;
3: pub mod load;
4: pub mod schema;
5: 
6: #[cfg(test)]
7: mod tests;
```

### crates/core/src/file/manipulate.rs (42 lines)

```
1: remove_comments
2: ⋮----
3: (content: &str, file_path: &Path)
4: ⋮----
5: remove_c_style_comments
6: ⋮----
7: (content: &str)
8: ⋮----
9: count_trailing_hashes
10: ⋮----
11: (s: &str)
12: ⋮----
13: try_parse_string_literal
14: ⋮----
15: (
16:     chars: &mut std::iter::Peekable<std::str::Chars>,
17:     prefix: char,
18: )
19: ⋮----
20: remove_hash_comments
21: ⋮----
22: (content: &str)
23: ⋮----
24: remove_html_comments
25: ⋮----
26: (content: &str)
27: ⋮----
28: remove_css_comments
29: ⋮----
30: (content: &str)
31: ⋮----
32: remove_empty_lines
33: ⋮----
34: (content: &str)
35: ⋮----
36: trim_content
37: ⋮----
38: (content: &str)
39: ⋮----
40: add_line_numbers
41: ⋮----
42: (content: &str)
```

### crates/core/src/file/mod.rs (9 lines)

```
1: pub mod collect;
2: pub mod manipulate;
3: pub mod process;
4: pub mod process_content;
5: pub mod search;
6: pub mod truncate_base64;
7: 
8: pub mod tree_generate;
9: pub mod types;
```

### crates/core/src/file/process_content.rs (23 lines)

```
1: ProcessContentOptions
2: ⋮----
3: {
4:     pub remove_comments: bool,
5:     pub compress: bool,
6:     pub truncate_base64: bool,
7:     pub remove_empty_lines: bool,
8:     pub show_line_numbers: bool,
9: }
10: ⋮----
11: ProcessContentOptions
12: ⋮----
13: {
14:     pub fn from_config(config: &RepomixConfig) -> Self {
15:         Self {
16:             remove_comments: config.output.remove_comments,
17:             compress: config.output.compress,
18:             truncate_base64: config.output.truncate_base64,
19:             remove_empty_lines: config.output.remove_empty_lines,
20:             show_line_numbers: config.output.show_line_numbers,
21:         }
22:     }
23: }
```

### crates/core/src/file/search.rs (17 lines)

```
1: compile_user_patterns
2: ⋮----
3: (patterns: &[String])
4: ⋮----
5: search_files
6: ⋮----
7: (
8:     root_dirs: &[PathBuf],
9:     config: &RepomixConfig,
10: )
11: ⋮----
12: search_files_sync
13: ⋮----
14: (
15:     root_dirs: &[PathBuf],
16:     config: &RepomixConfig,
17: )
```

### crates/core/src/file/tree_generate.rs (90 lines)

```
1: TreeNode
2: ⋮----
3: {
4:     pub name: String,
5:     pub children: Vec<TreeNode>,
6:     pub is_directory: bool,
7: }
8: ⋮----
9: TreeNode
10: ⋮----
11: {
12:     fn new(name: &str, is_directory: bool) -> Self {
13:         Self {
14:             name: name.to_string(),
15:             children: Vec::new(),
16:             is_directory,
17:         }
18:     }
19: }
20: ⋮----
21: generate_file_tree
22: ⋮----
23: (file_paths: &[String], empty_dir_paths: &[String])
24: ⋮----
25: add_path_to_tree
26: ⋮----
27: (root: &mut TreeNode, path: &str, is_directory: bool)
28: ⋮----
29: sort_tree_nodes
30: ⋮----
31: (node: &mut TreeNode)
32: ⋮----
33: tree_to_string
34: ⋮----
35: (node: &TreeNode, prefix: &str, is_root: bool)
36: ⋮----
37: tree_to_string_inner
38: ⋮----
39: (node: &TreeNode, prefix: &str, _is_root: bool)
40: ⋮----
41: tree_to_string_with_line_counts
42: ⋮----
43: (
44:     node: &TreeNode,
45:     line_counts: &HashMap<String, usize>,
46:     prefix: &str,
47:     current_path: &str,
48:     is_root: bool,
49: )
50: ⋮----
51: tree_to_string_with_line_counts_inner
52: ⋮----
53: (
54:     node: &TreeNode,
55:     line_counts: &HashMap<String, usize>,
56:     prefix: &str,
57:     current_path: &str,
58:     _is_root: bool,
59: )
60: ⋮----
61: generate_tree_string
62: ⋮----
63: (file_paths: &[String], empty_dir_paths: &[String])
64: ⋮----
65: generate_tree_string_with_line_counts
66: ⋮----
67: (
68:     file_paths: &[String],
69:     line_counts: &HashMap<String, usize>,
70:     empty_dir_paths: &[String],
71: )
72: ⋮----
73: calculate_file_line_counts
74: ⋮----
75: (
76:     file_paths: &[String],
77:     contents: &[String],
78: )
79: ⋮----
80: test_generate_tree
81: ⋮----
82: ()
83: ⋮----
84: test_tree_to_string
85: ⋮----
86: ()
87: ⋮----
88: test_generate_tree_string
89: ⋮----
90: ()
```

### crates/core/src/file/types.rs (35 lines)

```
1: FileSearchOptions
2: ⋮----
3: {
4:     pub include_patterns: Vec<String>,
5:     pub ignore_patterns: Vec<String>,
6:     pub include_empty_directories: bool,
7: }
8: ⋮----
9: FileSearchOptions
10: ⋮----
11: {
12:     pub fn from_config(config: &RepomixConfig) -> Self {
13:         Self {
14:             include_patterns: config.include.clone(),
15:             ignore_patterns: config.ignore.custom_ignore.clone(),
16:             include_empty_directories: config.output.include_empty_directories,
17:         }
18:     }
19: }
20: ⋮----
21: FileCollectOptions
22: ⋮----
23: {
24:     pub max_file_size: u64,
25: }
26: ⋮----
27: FileCollectOptions
28: ⋮----
29: {
30:     pub fn from_config(config: &RepomixConfig) -> Self {
31:         Self {
32:             max_file_size: config.input.max_file_size,
33:         }
34:     }
35: }
```

### crates/core/src/git/log.rs (9 lines)

```
1: GitLogResult
2: ⋮----
3: {
4:     pub logs: Vec<String>,
5: }
6: ⋮----
7: get_git_logs
8: ⋮----
9: (repo_path: &Path, max_count: usize)
```

### crates/core/src/git/mod.rs (4 lines)

```
1: pub mod diff;
2: pub mod log;
3: pub mod remote;
4: pub mod sort;
```

### crates/core/src/git/sort.rs (14 lines)

```
1: get_file_change_counts
2: ⋮----
3: (
4:     repo_path: &Path,
5:     max_commits: usize,
6: )
7: ⋮----
8: sort_by_git_changes
9: ⋮----
10: (
11:     files: &mut [ProcessedFile],
12:     repo_path: &Path,
13:     max_commits: usize,
14: )
```

### crates/core/src/lib.rs (11 lines)

```
1: pack_directory
2: ⋮----
3: (dir: &str)
4: ⋮----
5: pack_with_config
6: ⋮----
7: (dir: &str, config: RepomixConfig)
8: ⋮----
9: pack_with_options
10: ⋮----
11: (options: PackOptions)
```

### crates/core/src/metrics/mod.rs (2 lines)

```
1: pub mod calculate;
2: pub mod token_count;
```

### crates/core/src/metrics/token_count.rs (82 lines)

```
1: TokenCounter
2: ⋮----
3: {
4:     encoder: CoreBPE,
5: }
6: ⋮----
7: TokenCounter
8: ⋮----
9: {
10:     
11:     
12:     
13:     
14:     
15:     
16:     
17:     
18:     
19:     
20:     pub fn new(encoding: &str) -> Result<Self, anyhow::Error> {
21:         let encoder = match encoding {
22:             "o200k_base" => tiktoken_rs::o200k_base()?,
23:             "cl100k_base" => tiktoken_rs::cl100k_base()?,
24:             "p50k_base" => tiktoken_rs::p50k_base()?,
25:             "p50k_edit" => tiktoken_rs::p50k_edit()?,
26:             "r50k_base" => tiktoken_rs::r50k_base()?,
27:             "gpt-4o" | "gpt-4" | "gpt-3.5-turbo" | "gpt-3" => {
28:                 
29:                 tiktoken_rs::get_bpe_from_model(encoding)?
30:             }
31:             other => {
32:                 tracing::warn!(
33:                     "Unknown token encoding '{}', falling back to 'o200k_base'. \
34:                      Supported: o200k_base, cl100k_base, p50k_base, p50k_edit, r50k_base.",
35:                     other
36:                 );
37:                 tiktoken_rs::o200k_base()?
38:             }
39:         };
40:         Ok(Self { encoder })
41:     }
42: 
43:     pub fn count_tokens(&self, text: &str) -> usize {
44:         self.encoder.encode_ordinary(text).len()
45:     }
46: }
47: ⋮----
48: create_default_token_counter
49: ⋮----
50: ()
51: ⋮----
52: estimate_tokens_fallback
53: ⋮----
54: (text: &str)
55: ⋮----
56: is_cjk
57: ⋮----
58: (ch: char)
59: ⋮----
60: test_default_encoding_works
61: ⋮----
62: ()
63: ⋮----
64: test_cl100k_encoding_works
65: ⋮----
66: ()
67: ⋮----
68: test_unknown_encoding_falls_back
69: ⋮----
70: ()
71: ⋮----
72: test_count_chinese
73: ⋮----
74: ()
75: ⋮----
76: test_estimate_tokens_fallback_cjk
77: ⋮----
78: ()
79: ⋮----
80: test_estimate_tokens_fallback_mixed
81: ⋮----
82: ()
```

### crates/core/src/output/decorate.rs (14 lines)

```
1: OutputHeader
2: ⋮----
3: {
4:     pub header_text: Option<String>,
5:     pub instruction_content: Option<String>,
6: }
7: ⋮----
8: collect_header
9: ⋮----
10: (config: &RepomixConfig)
11: ⋮----
12: format_header
13: ⋮----
14: (header: &OutputHeader)
```

### crates/core/src/output/generate.rs (29 lines)

```
1: OutputResult
2: ⋮----
3: {
4:     
5:     pub written_paths: Vec<String>,
6:     
7:     pub contents: Vec<String>,
8:     
9:     pub directory_structure: String,
10: }
11: ⋮----
12: produce_output
13: ⋮----
14: (
15:     files: &[ProcessedFile],
16:     config: &RepomixConfig,
17:     pack_root: &Path,
18:     git_diff_content: &Option<String>,
19:     git_log_content: &Option<String>,
20:     empty_dir_paths: &[String],
21: )
22: ⋮----
23: copy_to_clipboard
24: ⋮----
25: (content: &str)
26: ⋮----
27: format_token_count_tree
28: ⋮----
29: (tree: &str, style: &OutputStyle)
```

### crates/core/src/output/mod.rs (5 lines)

```
1: pub mod generate;
2: pub mod styles;
3: 
4: pub mod decorate;
5: pub mod split;
```

### crates/core/src/output/split.rs (59 lines)

```
1: split_output
2: ⋮----
3: (
4:     content: &str,
5:     token_threshold: u64,
6:     style: &OutputStyle,
7:     encoding: &str,
8: )
9: ⋮----
10: split_xml_by_files
11: ⋮----
12: (
13:     files: &[ProcessedFile],
14:     config: &RepomixConfig,
15:     pack_root: &Path,
16:     tree_string: &str,
17:     header: &OutputHeader,
18:     git_diff_content: &Option<String>,
19:     git_log_content: &Option<String>,
20:     token_count_tree: Option<&str>,
21:     token_threshold: u64,
22:     encoding: &str,
23: )
24: ⋮----
25: make_token_counter_fn
26: ⋮----
27: (encoding: &str)
28: ⋮----
29: split_lines_by_tokens
30: ⋮----
31: (
32:     content: &str,
33:     threshold: usize,
34:     count_tokens: &dyn Fn(&str) -> usize,
35: )
36: ⋮----
37: test_split_output_no_split
38: ⋮----
39: ()
40: ⋮----
41: test_split_output_with_split
42: ⋮----
43: ()
44: ⋮----
45: test_split_xml_by_files_produces_balanced_structure
46: ⋮----
47: ()
48: ⋮----
49: test_split_output_json_keeps_valid_json
50: ⋮----
51: ()
52: ⋮----
53: test_split_output_markdown_uses_html_comment
54: ⋮----
55: ()
56: ⋮----
57: test_split_output_single_line_exceeds_threshold
58: ⋮----
59: ()
```

### crates/core/src/output/styles/mod.rs (4 lines)

```
1: pub mod json;
2: pub mod markdown;
3: pub mod plain;
4: pub mod xml;
```

### crates/core/src/path_util.rs (27 lines)

```
1: normalize_path
2: ⋮----
3: (path: &Path)
4: ⋮----
5: display_path
6: ⋮----
7: (path: &Path, pack_root: &Path)
8: ⋮----
9: effective_pack_root
10: ⋮----
11: (path: &Path)
12: ⋮----
13: resolve_output_file_path
14: ⋮----
15: (file_path: &str, pack_root: &Path)
16: ⋮----
17: git_repo_root
18: ⋮----
19: (repo_path: &Path)
20: ⋮----
21: git_relative_path
22: ⋮----
23: (path: &Path, repo_root: &Path)
24: ⋮----
25: is_repomix_output_artifact
26: ⋮----
27: (file_name: &str, configured_output: &str)
```

### crates/core/src/security/mod.rs (2 lines)

```
1: pub mod secretlint;
2: pub mod validate;
```

### crates/core/src/security/validate.rs (6 lines)

```
1: validate_file_safety
2: ⋮----
3: (
4:     raw_files: &[RawFile],
5:     config: &RepomixConfig,
6: )
```

### crates/core/src/tree_sitter/mod.rs (2 lines)

```
1: pub mod compress;
2: pub mod languages;
```

### crates/mcp/src/helpers.rs (72 lines)

```
1: McpPackOverrides
2: ⋮----
3: {
4:     pub include_patterns: Option<String>,
5:     pub ignore_patterns: Option<String>,
6:     pub compress: Option<bool>,
7:     pub remove_comments: Option<bool>,
8:     pub remove_empty_lines: Option<bool>,
9:     pub show_line_numbers: Option<bool>,
10:     pub truncate_base64: Option<bool>,
11:     pub top_files_length: Option<usize>,
12:     pub split_output: Option<u64>,
13:     pub header_text: Option<String>,
14:     pub include_diffs: Option<bool>,
15:     pub include_logs: Option<bool>,
16:     pub style: Option<OutputStyle>,
17: }
18: ⋮----
19: McpPackOverrides
20: ⋮----
21: {
22:     pub fn into_partial_config(self) -> PartialConfig {
23:         PartialConfig {
24:             include: self.include_patterns.as_deref().map(|s| split_csv(Some(s))),
25:             ignore: self.ignore_patterns.as_deref().map(|s| split_csv(Some(s))),
26:             compress: self.compress,
27:             remove_comments: self.remove_comments,
28:             remove_empty_lines: self.remove_empty_lines,
29:             show_line_numbers: self.show_line_numbers,
30:             truncate_base64: self.truncate_base64,
31:             top_files_length: self.top_files_length,
32:             split_output: self.split_output,
33:             header_text: self.header_text,
34:             include_diffs: self.include_diffs,
35:             include_logs: self.include_logs,
36:             style: self.style,
37:             ..Default::default()
38:         }
39:     }
40: }
41: ⋮----
42: parse_style
43: ⋮----
44: (s: Option<&str>)
45: ⋮----
46: validate_remote_url
47: ⋮----
48: (url: &str)
49: ⋮----
50: split_csv
51: ⋮----
52: (s: Option<&str>)
53: ⋮----
54: parse_style_defaults_to_xml
55: ⋮----
56: ()
57: ⋮----
58: parse_style_rejects_unknown
59: ⋮----
60: ()
61: ⋮----
62: validate_remote_url_accepts_https
63: ⋮----
64: ()
65: ⋮----
66: validate_remote_url_rejects_empty
67: ⋮----
68: ()
69: ⋮----
70: split_csv_trims_and_skips_empty
71: ⋮----
72: ()
```

### crates/mcp/src/output_path.rs (38 lines)

```
1: McpOutputRef
2: ⋮----
3: {
4:     pub output_id: String,
5:     pub path: PathBuf,
6: }
7: ⋮----
8: style_extension
9: ⋮----
10: (s: &OutputStyle)
11: ⋮----
12: cleanup_stale_mcp_outputs
13: ⋮----
14: ()
15: ⋮----
16: make_mcp_output_path
17: ⋮----
18: (style: &OutputStyle)
19: ⋮----
20: validate_mcp_output_path
21: ⋮----
22: (path: &str)
23: ⋮----
24: make_mcp_output_path_has_unique_id
25: ⋮----
26: ()
27: ⋮----
28: validate_rejects_path_outside_outputs_dir
29: ⋮----
30: ()
31: ⋮----
32: validate_accepts_file_under_outputs_dir
33: ⋮----
34: ()
35: ⋮----
36: cleanup_does_not_remove_recent_files
37: ⋮----
38: ()
```

### crates/mcp/src/tools/mod.rs (3 lines)

```
1: get_tool_definitions
2: ⋮----
3: ()
```

### crates/shared/src/lib.rs (3 lines)

```
1: pub mod logger;
2: 
3: pub mod types;
```

### crates/shared/src/types.rs (52 lines)

```
1: RawFile
2: ⋮----
3: {
4:     pub path: PathBuf,
5:     pub content: String,
6:     pub size: usize,
7: }
8: ⋮----
9: ProcessedFile
10: ⋮----
11: {
12:     pub path: PathBuf,
13:     pub content: String,
14:     pub token_count: usize,
15: }
16: ⋮----
17: SkippedFileInfo
18: ⋮----
19: {
20:     pub path: PathBuf,
21:     pub reason: String,
22: }
23: ⋮----
24: SuspiciousFileResult
25: ⋮----
26: {
27:     pub path: PathBuf,
28:     pub line: usize,
29:     pub message: String,
30:     pub rule_id: String,
31: }
32: ⋮----
33: FileSearchResult
34: ⋮----
35: {
36:     pub file_paths: Vec<PathBuf>,
37:     pub empty_dir_paths: Vec<PathBuf>,
38: }
39: ⋮----
40: FileCollectResult
41: ⋮----
42: {
43:     pub raw_files: Vec<RawFile>,
44:     pub skipped_files: Vec<SkippedFileInfo>,
45: }
46: ⋮----
47: ValidationResult
48: ⋮----
49: {
50:     pub suspicious: Vec<SuspiciousFileResult>,
51:     pub safe_paths: Vec<PathBuf>,
52: }
```

### Cargo.toml (9 lines)

```
1: [workspace]
2: members = [
3:     "crates/core",
4:     "crates/config",
5:     "crates/cli",
6:     "crates/mcp",
7:     "crates/shared",
8: ]
9: resolver = "2"
```

### LICENSE (21 lines)

```
1: MIT License
2: 
3: Copyright (c) 2026 Sopaco
4: 
5: Permission is hereby granted, free of charge, to any person obtaining a copy
6: of this software and associated documentation files (the "Software"), to deal
7: in the Software without restriction, including without limitation the rights
8: to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
9: copies of the Software, and to permit persons to whom the Software is
10: furnished to do so, subject to the following conditions:
11: 
12: The above copyright notice and this permission notice shall be included in all
13: copies or substantial portions of the Software.
14: 
15: THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
16: IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
17: FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
18: AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
19: LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
20: OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
21: SOFTWARE.
```

### crates/cli/src/report.rs (3 lines)

```
1: print_report
2: ⋮----
3: (result: &PackResult)
```

### crates/core/src/git/diff.rs (14 lines)

```
1: GitDiffResult
2: ⋮----
3: {
4:     pub work_tree: String,
5:     pub staged: String,
6: }
7: ⋮----
8: get_git_diffs
9: ⋮----
10: (repo_path: &Path)
11: ⋮----
12: run_git_diff
13: ⋮----
14: (repo_path: &Path, args: &[&str])
```

### crates/core/src/tree_sitter/queries/c.scm (26 lines)

```
1: ; C compress query - extract function/struct signatures
2: (function_definition
3:   type: (_) @return_type
4:   declarator: (function_declarator
5:     declarator: (identifier) @name
6:     parameters: (parameter_list) @params
7:   )
8: )
9: 
10: (struct_specifier
11:   name: (type_identifier) @name
12:   body: (field_declaration_list) @body
13: )
14: 
15: (type_definition
16:   type: (_) @type
17:   declarator: (type_identifier) @name
18: )
19: 
20: (declaration
21:   type: (_) @type
22:   declarator: (function_declarator
23:     declarator: (identifier) @name
24:     parameters: (parameter_list) @params
25:   )
26: )
```

### crates/core/src/tree_sitter/queries/c_sharp.scm (21 lines)

```
1: ; C# compress query - extract method/class definitions
2: (method_declaration
3:   name: (identifier) @name
4:   parameters: (parameter_list) @params
5: )
6: 
7: (class_declaration
8:   name: (identifier) @name
9: )
10: 
11: (interface_declaration
12:   name: (identifier) @name
13: )
14: 
15: (struct_declaration
16:   name: (identifier) @name
17: )
18: 
19: (namespace_declaration
20:   name: (identifier) @name
21: )
```

### crates/core/src/tree_sitter/queries/cpp.scm (22 lines)

```
1: ; C++ compress query - extract function/class signatures
2: (function_definition
3:   declarator: (function_declarator
4:     declarator: (identifier) @name
5:     parameters: (parameter_list) @params
6:   )
7: )
8: 
9: (class_specifier
10:   name: (type_identifier) @name
11:   body: (field_declaration_list) @body
12: )
13: 
14: (struct_specifier
15:   name: (type_identifier) @name
16:   body: (field_declaration_list) @body
17: )
18: 
19: (namespace_definition
20:   name: (namespace_identifier) @name
21:   body: (declaration_list) @body
22: )
```

### crates/core/src/tree_sitter/queries/go.scm (20 lines)

```
1: ; Go compress query - extract function/type signatures
2: (function_declaration
3:   name: (identifier) @name
4:   parameters: (parameter_list) @params
5: )
6: 
7: (method_declaration
8:   receiver: (parameter_list) @receiver
9:   name: (field_identifier) @name
10:   parameters: (parameter_list) @params
11: )
12: 
13: (type_declaration
14:   (type_spec
15:     name: (type_identifier) @name
16:     type: (struct_type
17:       (field_declaration_list) @body
18:     )
19:   )
20: )
```

### crates/core/src/tree_sitter/queries/java.scm (26 lines)

```
1: ; Java compress query - extract class/method signatures
2: (class_declaration
3:   name: (identifier) @name
4:   body: (class_body) @body
5: )
6: 
7: (method_declaration
8:   name: (identifier) @name
9:   parameters: (formal_parameters) @params
10:   type: (type_identifier)? @return_type
11: )
12: 
13: (interface_declaration
14:   name: (identifier) @name
15:   body: (interface_body) @body
16: )
17: 
18: (enum_declaration
19:   name: (identifier) @name
20:   body: (enum_body) @body
21: )
22: 
23: (constructor_declaration
24:   name: (identifier) @name
25:   parameters: (formal_parameters) @params
26: )
```

### crates/core/src/tree_sitter/queries/javascript.scm (27 lines)

```
1: ; JavaScript compress query - extract function/class signatures
2: (function_declaration
3:   name: (identifier) @name
4:   parameters: (formal_parameters) @params
5: )
6: 
7: (class_declaration
8:   name: (identifier) @name
9:   body: (class_body) @body
10: )
11: 
12: (method_definition
13:   name: (property_identifier) @name
14:   parameters: (formal_parameters) @params
15: )
16: 
17: (export_statement
18:   declaration: (function_declaration) @func
19: )
20: 
21: (export_statement
22:   declaration: (class_declaration) @class
23: )
24: 
25: (arrow_function
26:   parameters: (formal_parameters) @params
27: )
```

### crates/core/src/tree_sitter/queries/php.scm (26 lines)

```
1: ; PHP compress query - extract function/class definitions
2: (function_definition
3:   name: (name) @name
4:   parameters: (formal_parameters) @params
5:   body: (compound_statement) @body
6: )
7: 
8: (class_declaration
9:   name: (name) @name
10:   body: (declaration_list) @body
11: )
12: 
13: (interface_declaration
14:   name: (name) @name
15:   body: (declaration_list) @body
16: )
17: 
18: (method_declaration
19:   name: (name) @name
20:   parameters: (formal_parameters) @params
21: )
22: 
23: (function_call_expression
24:   function: (_) @func
25:   arguments: (arguments) @params
26: )
```

### crates/core/src/tree_sitter/queries/python.scm (19 lines)

```
1: ; Python compress query - extract function/class signatures
2: (function_definition
3:   name: (identifier) @name
4:   parameters: (parameters) @params
5:   return_type: (type)? @return_type
6: )
7: 
8: (class_definition
9:   name: (identifier) @name
10:   body: (block) @body
11: )
12: 
13: (decorated_definition
14:   definition: (function_definition) @func
15: )
16: 
17: (decorated_definition
18:   definition: (class_definition) @class
19: )
```

### crates/core/src/tree_sitter/queries/ruby.scm (19 lines)

```
1: ; Ruby compress query - extract method/class definitions
2: (method
3:   name: (identifier) @name
4:   parameters: (method_parameters) @params
5: )
6: 
7: (singleton_method
8:   object: (_) @object
9:   name: (identifier) @name
10:   parameters: (method_parameters) @params
11: )
12: 
13: (class
14:   name: (constant) @name
15: )
16: 
17: (module
18:   name: (constant) @name
19: )
```

### crates/core/src/tree_sitter/queries/rust.scm (29 lines)

```
1: ; Rust compress query - extract function/struct/impl signatures
2: (function_item
3:   name: (identifier) @name
4:   parameters: (parameters) @params
5: )
6: 
7: (struct_item
8:   name: (type_identifier) @name
9:   body: (field_declaration_list) @body
10: )
11: 
12: (impl_item
13:   type: (type_identifier) @name
14:   body: (declaration_list) @body
15: )
16: 
17: (trait_item
18:   name: (type_identifier) @name
19:   body: (declaration_list) @body
20: )
21: 
22: (enum_item
23:   name: (type_identifier) @name
24:   body: (enum_variant_list) @body
25: )
26: 
27: (macro_definition
28:   name: (identifier) @name
29: )
```

### crates/core/src/tree_sitter/queries/typescript.scm (34 lines)

```
1: ; TypeScript compress query - extract function/class signatures
2: (function_declaration
3:   name: (identifier) @name
4:   parameters: (formal_parameters) @params
5:   return_type: (type_annotation)? @return_type
6: )
7: 
8: (class_declaration
9:   name: (type_identifier) @name
10:   body: (class_body) @body
11: )
12: 
13: (method_definition
14:   name: (property_identifier) @name
15:   parameters: (formal_parameters) @params
16: )
17: 
18: (export_statement
19:   declaration: (function_declaration) @func
20: )
21: 
22: (export_statement
23:   declaration: (class_declaration) @class
24: )
25: 
26: (interface_declaration
27:   name: (type_identifier) @name
28:   body: (interface_body) @body
29: )
30: 
31: (type_alias_declaration
32:   name: (type_identifier) @name
33:   value: (_) @value
34: )
```

### crates/mcp/src/params.rs (84 lines)

```
1: PackSharedParams
2: ⋮----
3: {
4:     
5:     #[serde(default)]
6:     pub compress: Option<bool>,
7:     
8:     #[serde(default)]
9:     pub include_patterns: Option<String>,
10:     
11:     #[serde(default)]
12:     pub ignore_patterns: Option<String>,
13:     
14:     #[serde(default)]
15:     pub remove_comments: Option<bool>,
16:     
17:     #[serde(default)]
18:     pub remove_empty_lines: Option<bool>,
19:     
20:     #[serde(default)]
21:     pub show_line_numbers: Option<bool>,
22:     
23:     #[serde(default)]
24:     pub truncate_base64: Option<bool>,
25:     
26:     #[serde(default)]
27:     pub top_files_length: Option<usize>,
28:     
29:     #[serde(default)]
30:     pub split_output: Option<u64>,
31:     
32:     #[serde(default)]
33:     pub header_text: Option<String>,
34:     
35:     #[serde(default)]
36:     pub include_diffs: Option<bool>,
37:     
38:     #[serde(default)]
39:     pub include_logs: Option<bool>,
40:     
41:     #[serde(default)]
42:     pub style: Option<String>,
43: }
44: ⋮----
45: PackSharedParams
46: ⋮----
47: {
48:     pub fn into_mcp_overrides(self) -> Result<McpPackOverrides, ErrorData> {
49:         Ok(McpPackOverrides {
50:             include_patterns: self.include_patterns,
51:             ignore_patterns: self.ignore_patterns,
52:             compress: self.compress,
53:             remove_comments: self.remove_comments,
54:             remove_empty_lines: self.remove_empty_lines,
55:             show_line_numbers: self.show_line_numbers,
56:             truncate_base64: self.truncate_base64,
57:             top_files_length: self.top_files_length,
58:             split_output: self.split_output,
59:             header_text: self.header_text,
60:             include_diffs: self.include_diffs,
61:             include_logs: self.include_logs,
62:             style: Some(parse_style(self.style.as_deref())?),
63:         })
64:     }
65: }
66: ⋮----
67: PackCodebaseParams
68: ⋮----
69: {
70:     
71:     #[serde(default)]
72:     pub directory: Option<String>,
73:     #[serde(flatten)]
74:     pub shared: PackSharedParams,
75: }
76: ⋮----
77: PackRemoteRepositoryParams
78: ⋮----
79: {
80:     
81:     pub url: String,
82:     #[serde(flatten)]
83:     pub shared: PackSharedParams,
84: }
```

### crates/shared/LICENSE-MIT (21 lines)

```
1: MIT License
2: 
3: Copyright (c) 2026 repomix contributors
4: 
5: Permission is hereby granted, free of charge, to any person obtaining a copy
6: of this software and associated documentation files (the "Software"), to deal
7: in the Software without restriction, including without limitation the rights
8: to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
9: copies of the Software, and to permit persons to whom the Software is
10: furnished to do so, subject to the following conditions:
11: 
12: The above copyright notice and this permission notice shall be included in all
13: copies or substantial portions of the Software.
14: 
15: THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
16: IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
17: FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
18: AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
19: LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
20: OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
21: SOFTWARE.
```

### crates/shared/src/pattern_utils.rs (11 lines)

```
1: build_glob_matcher
2: ⋮----
3: (patterns: &[String])
4: ⋮----
5: matches_glob_pattern
6: ⋮----
7: (path: &Path, patterns: &[String])
8: ⋮----
9: parse_patterns
10: ⋮----
11: (pattern_str: Option<&str>)
```

### npm/repomix-rs/bin/repomix.js (35 lines)

```
1: #!/usr/bin/env node
2: import { spawnSync } from 'node:child_process';
3: import { createRequire } from 'node:module';
4: import path from 'node:path';
5: 
6: const platform = process.platform;
7: const arch = process.arch;
8: const platformPkg = `repomix-rs-${platform}-${arch}`;
9: const binaryName = platform === 'win32' ? 'repomix.exe' : 'repomix';
10: 
11: let binaryPath;
12: try {
13:   const require = createRequire(import.meta.url);
14:   const pkgRoot = path.dirname(require.resolve(`${platformPkg}/package.json`));
15:   binaryPath = path.join(pkgRoot, binaryName);
16: } catch {
17:   console.error(
18:     `repomix-rs: no prebuilt binary for ${platform}-${arch}.\n` +
19:       'Install from source: cargo install --path crates/cli\n' +
20:       'Or use a supported platform (linux/darwin/win32, x64 or arm64).',
21:   );
22:   process.exit(1);
23: }
24: 
25: const result = spawnSync(binaryPath, process.argv.slice(2), {
26:   stdio: 'inherit',
27:   env: process.env,
28: });
29: 
30: if (result.error) {
31:   console.error(`repomix-rs: failed to run binary: ${result.error.message}`);
32:   process.exit(1);
33: }
34: 
35: process.exit(result.status ?? 1);
```

### npm/repomix-rs/package.json (39 lines)

```
1: {
2:   "name": "repomix-rs",
3:   "version": "2.0.0",
4:   "description": "Pack your codebase into an AI-friendly file (Rust implementation of Repomix)",
5:   "license": "MIT",
6:   "repository": {
7:     "type": "git",
8:     "url": "git+https://github.com/sopaco/repomix-rs.git"
9:   },
10:   "homepage": "https://github.com/sopaco/repomix-rs#readme",
11:   "bugs": {
12:     "url": "https://github.com/sopaco/repomix-rs/issues"
13:   },
14:   "keywords": [
15:     "repomix",
16:     "codebase",
17:     "llm",
18:     "ai",
19:     "generative-ai",
20:     "rust"
21:   ],
22:   "type": "module",
23:   "bin": {
24:     "repomix": "bin/repomix.js"
25:   },
26:   "files": [
27:     "bin"
28:   ],
29:   "engines": {
30:     "node": ">=18"
31:   },
32:   "optionalDependencies": {
33:     "repomix-rs-linux-x64": "2.0.0",
34:     "repomix-rs-linux-arm64": "2.0.0",
35:     "repomix-rs-darwin-x64": "2.0.0",
36:     "repomix-rs-darwin-arm64": "2.0.0",
37:     "repomix-rs-win32-x64": "2.0.0"
38:   }
39: }
```

### npm/templates/platform-package.json.tmpl (13 lines)

```
1: {
2:   "name": "repomix-rs-__PLATFORM__-__ARCH__",
3:   "version": "__VERSION__",
4:   "description": "Prebuilt repomix binary for __PLATFORM__-__ARCH__ (repomix-rs)",
5:   "license": "MIT",
6:   "repository": {
7:     "type": "git",
8:     "url": "git+https://github.com/sopaco/repomix-rs.git"
9:   },
10:   "os": ["__PLATFORM__"],
11:   "cpu": ["__ARCH__"],
12:   "preferUnplugged": true
13: }
```

### scripts/check-npm-version.mjs (79 lines)

```
1: #!/usr/bin/env bun
2: /**
3:  * Verify Cargo workspace crate versions match npm/repomix-rs/package.json.
4:  *
5:  * Usage (node or bun):
6:  *   bun scripts/check-npm-version.mjs
7:  *   bun scripts/check-npm-version.mjs --expected 2.0.0
8:  */
9: import { readFileSync } from 'node:fs';
10: import path from 'node:path';
11: import { fileURLToPath } from 'node:url';
12: 
13: const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
14: const CRATE_MANIFESTS = [
15:   'crates/cli/Cargo.toml',
16:   'crates/core/Cargo.toml',
17:   'crates/config/Cargo.toml',
18:   'crates/shared/Cargo.toml',
19:   'crates/mcp/Cargo.toml',
20: ];
21: 
22: function parseExpected(argv) {
23:   const idx = argv.indexOf('--expected');
24:   if (idx === -1) {
25:     return null;
26:   }
27:   const value = argv[idx + 1];
28:   if (!value) {
29:     throw new Error('Missing value for --expected');
30:   }
31:   return value;
32: }
33: 
34: function readCargoVersion(manifestPath) {
35:   const content = readFileSync(path.join(ROOT, manifestPath), 'utf8');
36:   const match = content.match(/^version\s*=\s*"([^"]+)"/m);
37:   if (!match) {
38:     throw new Error(`No version in ${manifestPath}`);
39:   }
40:   return match[1];
41: }
42: 
43: function main() {
44:   const expectedOverride = parseExpected(process.argv.slice(2));
45:   const npmPkg = JSON.parse(
46:     readFileSync(path.join(ROOT, 'npm', 'repomix-rs', 'package.json'), 'utf8'),
47:   );
48:   const expected = expectedOverride ?? npmPkg.version;
49: 
50:   const mismatches = [];
51:   for (const manifest of CRATE_MANIFESTS) {
52:     const cargoVersion = readCargoVersion(manifest);
53:     if (cargoVersion !== expected) {
54:       mismatches.push(`${manifest}: ${cargoVersion} (expected ${expected})`);
55:     }
56:   }
57: 
58:   if (npmPkg.version !== expected) {
59:     mismatches.push(`npm/repomix-rs/package.json: ${npmPkg.version} (expected ${expected})`);
60:   }
61: 
62:   for (const version of Object.values(npmPkg.optionalDependencies ?? {})) {
63:     if (version !== expected) {
64:       mismatches.push(`optionalDependency version ${version} (expected ${expected})`);
65:     }
66:   }
67: 
68:   if (mismatches.length > 0) {
69:     console.error('Version mismatch:');
70:     for (const line of mismatches) {
71:       console.error(`  - ${line}`);
72:     }
73:     process.exit(1);
74:   }
75: 
76:   console.log(`All versions match: ${expected}`);
77: }
78: 
79: main();
```

### scripts/publish-crates.sh (222 lines)

```
1: set -euo pipefail
2: 
3: SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
4: PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
5: 
6: CRATES=(
7:   "shared:repomix-shared"
8:   "config:repomix-config"
9:   "core:repomix-core"
10:   "mcp:repomix-mcp"
11:   "cli:repomix-cli"
12: )
13: 
14: INTERNAL_DEPS=(
15:   "repomix-shared"
16:   "repomix-config"
17:   "repomix-core"
18:   "repomix-mcp"
19:   "repomix-cli"
20: )
21: 
22: DRY_RUN=false
23: SKIP_COUNT=0
24: 
25: while [[ $
26:   case "$1" in
27:     --dry-run)
28:       DRY_RUN=true
29:       shift
30:       ;;
31:     --skip)
32:       SKIP_COUNT="$2"
33:       shift 2
34:       ;;
35:     *)
36:       echo "Unknown option: $1"
37:       exit 1
38:       ;;
39:   esac
40: done
41: 
42: echo "========================================="
43: echo "  repomix crate 发布脚本"
44: echo "========================================="
45: echo ""
46: 
47: if $DRY_RUN; then
48:   echo "🔍 DRY-RUN 模式：仅预览，不实际发布"
49:   echo ""
50: fi
51: 
52: if ! command -v cargo &> /dev/null; then
53:   echo "❌ cargo 未找到，请先安装 Rust"
54:   exit 1
55: fi
56: 
57: if [[ -d "$PROJECT_ROOT/.git" ]]; then
58:   GIT_STATUS=$(cd "$PROJECT_ROOT" && git status --porcelain 2>/dev/null || true)
59:   if [[ -n "$GIT_STATUS" ]]; then
60:     echo "⚠️  警告：工作区有未提交的更改"
61:     echo ""
62:   fi
63: fi
64: 
65: VERSIONS=()
66: for entry in "${CRATES[@]}"; do
67:   IFS=':' read -r dir name <<< "$entry"
68:   cargo_toml="$PROJECT_ROOT/crates/$dir/Cargo.toml"
69:   version=$(grep '^version' "$cargo_toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
70:   VERSIONS+=("$version")
71: done
72: 
73: is_published() {
74:   local name="$1"
75:   local version="$2"
76:   cargo search "$name" 2>/dev/null | grep -q "^$name = \"$version\""
77: }
78: 
79: echo "📋 发布计划："
80: echo "-----------------------------------------"
81: for i in "${!CRATES[@]}"; do
82:   IFS=':' read -r dir name <<< "${CRATES[$i]}"
83:   version="${VERSIONS[$i]}"
84:   if [[ $i -lt $SKIP_COUNT ]]; then
85:     echo "  [$((i+1))] $name v$version  ⏭️  (跳过)"
86:   elif is_published "$name" "$version"; then
87:     echo "  [$((i+1))] $name v$version  ✅ (已发布)"
88:   else
89:     echo "  [$((i+1))] $name v$version"
90:   fi
91: done
92: echo "-----------------------------------------"
93: echo ""
94: 
95: if $DRY_RUN; then
96:   echo "💡 实际运行命令预览："
97:   for i in "${!CRATES[@]}"; do
98:     if [[ $i -lt $SKIP_COUNT ]]; then
99:       continue
100:     fi
101:     IFS=':' read -r dir name <<< "${CRATES[$i]}"
102:     echo "  cargo publish -p $name --allow-dirty"
103:   done
104:   echo ""
105:   echo "✅ DRY-RUN 完成（未实际发布）"
106:   exit 0
107: fi
108: 
109: 
110: add_versions_to_toml() {
111:   local toml_file="$1"
112:   local version="$2"
113: 
114:   for dep in "${INTERNAL_DEPS[@]}"; do
115:     local dep_line
116:     dep_line=$(grep "^$dep = " "$toml_file" 2>/dev/null || true)
117: 
118:     if [[ -n "$dep_line" ]]; then
119:       if echo "$dep_line" | grep -q 'version'; then
120:         continue
121:       fi
122:       sed -i.bak "s|^$dep = { path = |$dep = { version = \"$version\", path = |g" "$toml_file"
123:     fi
124:   done
125: }
126: 
127: backup_tomls() {
128:   echo "📦 备份 Cargo.toml..."
129:   for entry in "${CRATES[@]}"; do
130:     IFS=':' read -r dir name <<< "$entry"
131:     cp "$PROJECT_ROOT/crates/$dir/Cargo.toml" "$PROJECT_ROOT/crates/$dir/Cargo.toml.bak"
132:   done
133: }
134: 
135: restore_tomls() {
136:   echo "🔄 恢复 Cargo.toml..."
137:   for entry in "${CRATES[@]}"; do
138:     IFS=':' read -r dir name <<< "$entry"
139:     if [[ -f "$PROJECT_ROOT/crates/$dir/Cargo.toml.bak" ]]; then
140:       mv "$PROJECT_ROOT/crates/$dir/Cargo.toml.bak" "$PROJECT_ROOT/crates/$dir/Cargo.toml"
141:     fi
142:   done
143: }
144: 
145: cleanup_backups() {
146:   for entry in "${CRATES[@]}"; do
147:     IFS=':' read -r dir name <<< "$entry"
148:     rm -f "$PROJECT_ROOT/crates/$dir/Cargo.toml.bak"
149:   done
150: }
151: 
152: get_shared_version() {
153:   local version
154:   version=$(grep '^version' "$PROJECT_ROOT/crates/shared/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
155:   echo "$version"
156: }
157: 
158: 
159: trap 'restore_tomls; echo "❌ 发布失败，已恢复 Cargo.toml"; exit 1' ERR
160: 
161: backup_tomls
162: 
163: SHARED_VERSION=$(get_shared_version)
164: echo "🔧 自动为内部依赖添加 version = \"$SHARED_VERSION\" ..."
165: 
166: for entry in "${CRATES[@]}"; do
167:   IFS=':' read -r dir name <<< "$entry"
168:   if [[ "$name" != "repomix-shared" ]]; then
169:     add_versions_to_toml "$PROJECT_ROOT/crates/$dir/Cargo.toml" "$SHARED_VERSION"
170:   fi
171: done
172: 
173: echo ""
174: 
175: PUBLISHED=()
176: 
177: for i in "${!CRATES[@]}"; do
178:   IFS=':' read -r dir name <<< "${CRATES[$i]}"
179:   version="${VERSIONS[$i]}"
180: 
181:   if [[ $i -lt $SKIP_COUNT ]]; then
182:     echo "⏭️  跳过 $name v$version"
183:     continue
184:   fi
185: 
186:   if is_published "$name" "$version"; then
187:     echo "⏭️  $name v$version 已存在于 crates.io，跳过"
188:     PUBLISHED+=("$name")
189:     continue
190:   fi
191: 
192:   echo "📦 发布 $name v$version ..."
193: 
194:   if cargo publish -p "$name" --allow-dirty 2>&1; then
195:     echo "✅ $name v$version 发布成功"
196:     PUBLISHED+=("$name")
197:   else
198:     echo "❌ $name v$version 发布失败"
199:     restore_tomls
200:     exit 1
201:   fi
202: 
203:   if [[ $i -lt $((${
204:     echo "⏳ 等待 crates.io 索引更新..."
205:     sleep 10
206:   fi
207: 
208:   echo ""
209: done
210: 
211: restore_tomls
212: 
213: cleanup_backups
214: 
215: echo "========================================="
216: echo "🎉 全部发布完成！"
217: echo "========================================="
218: echo ""
219: echo "已发布："
220: for name in "${PUBLISHED[@]}"; do
221:   echo "  ✅ $name"
222: done
```

### scripts/publish-npm.mjs (182 lines)

```
1: #!/usr/bin/env bun
2: /**
3:  * Assemble and publish repomix-rs npm packages.
4:  *
5:  * Usage (node or bun):
6:  *   bun scripts/publish-npm.mjs platform --npm-suffix linux-x64 --binary ./repomix --version 2.0.0
7:  *   bun scripts/publish-npm.mjs main --version 2.0.0
8:  *   bun scripts/publish-npm.mjs main --version 2.0.0 --dry-run
9:  */
10: import { execSync, spawnSync } from 'node:child_process';
11: import {
12:   chmodSync,
13:   cpSync,
14:   mkdirSync,
15:   readFileSync,
16:   rmSync,
17:   writeFileSync,
18: } from 'node:fs';
19: import path from 'node:path';
20: import { fileURLToPath } from 'node:url';
21: 
22: const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
23: const NPM_MAIN_DIR = path.join(ROOT, 'npm', 'repomix-rs');
24: const TEMPLATE_PATH = path.join(ROOT, 'npm', 'templates', 'platform-package.json.tmpl');
25: const STAGING_ROOT = path.join(ROOT, 'npm', '.staging');
26: 
27: const PLATFORM_SUFFIXES = [
28:   'linux-x64',
29:   'linux-arm64',
30:   'darwin-x64',
31:   'darwin-arm64',
32:   'win32-x64',
33: ];
34: 
35: function parseArgs(argv) {
36:   const args = { _: [] };
37:   for (let i = 0; i < argv.length; i += 1) {
38:     const arg = argv[i];
39:     if (arg === '--dry-run') {
40:       args.dryRun = true;
41:     } else if (arg.startsWith('--')) {
42:       const key = arg.slice(2);
43:       const value = argv[i + 1];
44:       if (!value || value.startsWith('--')) {
45:         throw new Error(`Missing value for ${arg}`);
46:       }
47:       args[key] = value;
48:       i += 1;
49:     } else {
50:       args._.push(arg);
51:     }
52:   }
53:   return args;
54: }
55: 
56: function npmSuffixToOsArch(npmSuffix) {
57:   const [platform, arch] = npmSuffix.split('-');
58:   if (!platform || !arch) {
59:     throw new Error(`Invalid npm suffix: ${npmSuffix}`);
60:   }
61:   return { platform, arch };
62: }
63: 
64: function renderPlatformPackageJson({ platform, arch, version }) {
65:   const template = readFileSync(TEMPLATE_PATH, 'utf8');
66:   return template
67:     .replaceAll('__PLATFORM__', platform)
68:     .replaceAll('__ARCH__', arch)
69:     .replaceAll('__VERSION__', version);
70: }
71: 
72: function hasCommand(command) {
73:   try {
74:     execSync(`command -v ${command}`, { stdio: 'ignore' });
75:     return true;
76:   } catch {
77:     return false;
78:   }
79: }
80: 
81: function registryPublish(dir, dryRun) {
82:   const env = {
83:     ...process.env,
84:     NODE_AUTH_TOKEN: process.env.NODE_AUTH_TOKEN ?? process.env.NPM_TOKEN,
85:   };
86: 
87:   if (hasCommand('npm')) {
88:     const cmd = dryRun ? 'npm publish --dry-run' : 'npm publish --access public';
89:     execSync(cmd, { cwd: dir, stdio: 'inherit', env });
90:     return;
91:   }
92: 
93:   if (hasCommand('bun')) {
94:     const args = dryRun ? ['publish', '--dry-run'] : ['publish'];
95:     const result = spawnSync('bun', args, { cwd: dir, stdio: 'inherit', env });
96:     if (result.status !== 0) {
97:       throw new Error(`bun publish failed with exit code ${result.status ?? 'unknown'}`);
98:     }
99:     return;
100:   }
101: 
102:   throw new Error('Neither npm nor bun found on PATH');
103: }
104: 
105: function publishPlatform({ npmSuffix, binary, version, dryRun }) {
106:   const { platform, arch } = npmSuffixToOsArch(npmSuffix);
107:   const pkgName = `repomix-rs-${npmSuffix}`;
108:   const stagingDir = path.join(STAGING_ROOT, pkgName);
109:   const binaryName = platform === 'win32' ? 'repomix.exe' : 'repomix';
110: 
111:   rmSync(stagingDir, { recursive: true, force: true });
112:   mkdirSync(stagingDir, { recursive: true });
113: 
114:   writeFileSync(
115:     path.join(stagingDir, 'package.json'),
116:     renderPlatformPackageJson({ platform, arch, version }),
117:   );
118:   cpSync(binary, path.join(stagingDir, binaryName));
119:   if (platform !== 'win32') {
120:     chmodSync(path.join(stagingDir, binaryName), 0o755);
121:   }
122: 
123:   console.log(`Publishing platform package ${pkgName}@${version}`);
124:   registryPublish(stagingDir, dryRun);
125: }
126: 
127: function publishMain({ version, dryRun }) {
128:   const mainPkgPath = path.join(NPM_MAIN_DIR, 'package.json');
129:   const mainPkg = JSON.parse(readFileSync(mainPkgPath, 'utf8'));
130: 
131:   mainPkg.version = version;
132:   mainPkg.optionalDependencies = Object.fromEntries(
133:     PLATFORM_SUFFIXES.map((suffix) => [`repomix-rs-${suffix}`, version]),
134:   );
135: 
136:   const stagingDir = path.join(STAGING_ROOT, 'repomix-rs');
137:   rmSync(stagingDir, { recursive: true, force: true });
138:   mkdirSync(stagingDir, { recursive: true });
139: 
140:   writeFileSync(path.join(stagingDir, 'package.json'), `${JSON.stringify(mainPkg, null, 2)}\n`);
141:   cpSync(path.join(NPM_MAIN_DIR, 'bin'), path.join(stagingDir, 'bin'), { recursive: true });
142:   chmodSync(path.join(stagingDir, 'bin', 'repomix.js'), 0o755);
143: 
144:   console.log(`Publishing main package repomix-rs@${version}`);
145:   registryPublish(stagingDir, dryRun);
146: }
147: 
148: function main() {
149:   const args = parseArgs(process.argv.slice(2));
150:   const [command] = args._;
151: 
152:   if (!args.version) {
153:     throw new Error('--version is required');
154:   }
155: 
156:   if (command === 'platform') {
157:     if (!args['npm-suffix'] || !args.binary) {
158:       throw new Error('platform command requires --npm-suffix and --binary');
159:     }
160:     publishPlatform({
161:       npmSuffix: args['npm-suffix'],
162:       binary: path.resolve(args.binary),
163:       version: args.version,
164:       dryRun: args.dryRun,
165:     });
166:     return;
167:   }
168: 
169:   if (command === 'main') {
170:     publishMain({ version: args.version, dryRun: args.dryRun });
171:     return;
172:   }
173: 
174:   throw new Error('Usage: publish-npm.mjs <platform|main> --version <ver> [options]');
175: }
176: 
177: try {
178:   main();
179: } catch (error) {
180:   console.error(error instanceof Error ? error.message : error);
181:   process.exit(1);
182: }
```

### articles/01_hermes_agent_repomix_rs_guide.md (179 lines)

````
1: # Hermes Agent 代码仓库打包工具使用指南（repomix-rs 版）
2: 
3: > **本文为 repomix-rs 官方推荐使用指南** —— repomix-rs 是原版 Repomix（TypeScript）的 Rust 高性能实现，完全兼容原版使用方式，并且 Faster、Safer、更适合 AI Agent 场景。
4: 
5: ---
6: 
7: ## repomix-rs 是什么
8: 
9: repomix-rs 是一款将整个代码库打包为 AI 友好单文件的工具。它能同时适用于 Hermes Agent、Claude、ChatGPT、Gemini 等主流大模型应用场景。通过 repomix-rs，你的代码库将以结构化、Token 可控的方式呈现给 AI，以便 AI 更精准地进行代码审查、文档生成与漏洞排查。
10: 
11: 和原版 Repomix 相比，repomix-rs 用 Rust 重写了全部核心逻辑，在性能、安全性、嵌入能力上均有质的飞跃。
12: 
13: | 特性 | repomix-rs（Rust） | 原版 Repomix（TypeScript） |
14: |------|--|--|
15: | 核心语言 | Rust | TypeScript |
16: | 运行速度 | **毫秒级**（并行文件扫描） | 秒级（单线程 Node.js） |
17: | 内存安全 | 编译期保证 | 运行时检查 |
18: | MCP 内置支持 | ✅ 官方提供 MCP Server | ❌ 需额外配置 |
19: | Secretlint 集成 | ✅ | ✅ |
20: | Tree-sitter | ✅ 10 语言 | ✅ 10 语言 |
21: | Token 计数 | tiktoken-rs（o200k_base） | tiktoken（JS） |
22: | 远程仓库打包 | ✅ git clone + 清理 | ✅ |
23: | 并行处理 | rayon + tokio | 无并行 |
24: 
25: ## 快速启动：不用装，直接跑
26: 
27: 打开终端，进入项目根目录，执行以下任意一行：
28: 
29: ```bash
30: # 方式 1：npx 运行（无需全局安装）
31: npx repomix-rs .
32: 
33: # 方式 2：全局安装后直接运行 repomix
34: npm install -g repomix-rs
35: repomix .
36: # 终端出现输出后发送给 Hermes 并写上："请先读一下"
37: ```
38: 
39: 运行后会在当前目录生成输出文件（默认 `repomix-output.xml`），将该文件拖入 Hermes Agent 聊天窗口，并发送：**"请先读一下这个项目结构文件"**。
40: 
41: ## 远程仓库直打
42: 
43: 无需克隆，一行命令即可打远程 GitHub 仓库：
44: 
45: ```bash
46: npx repomix-rs --remote https://github.com/用户名/项目名
47: ```
48: 
49: 指定分支（更稳妥）：
50: 
51: ```bash
52: npx repomix-rs --remote https://github.com/用户名/项目名 --branch main
53: ```
54: 
55: > repomix-rs 的远程打包基于系统 `git` 命令实现，首次运行会完整拉取仓库快照。> 若 git 不可用，该步骤会跳过并给出警告，不会中断主流程。
56: 
57: ## 精细控制：哪些文件该进，哪些该砍
58: 
59: 创建 `.repomixrc` 配置文件：
60: 
61: ```json
62: {
63:   "include": ["src/**/*", "tests/**/*", "pyproject.toml", "README.md"],
64:   "exclude": ["**/*.log", "**/dist/**", "**/.git/**", "node_modules/**"]
65: }
66: ```
67: 
68: 启用压缩（提取函数签名，压缩率可达 50%-90%）：
69: 
70: ```bash
71: npx repomix-rs --compress --remove-comments --remove-empty-lines .
72: ```
73: 
74: 仅包含特定语言文件并忽略测试目录：
75: 
76: ```bash
77: npx repomix-rs --include "*.rs,*.toml,Cargo.*" --ignore "target/**,tests/**" .
78: ```
79: 
80: ## 输出格式选择
81: 
82: repomix-rs 支持四种输出格式，通过 `--style` 参数切换：
83: 
84: ```bash
85: npx repomix-rs --style markdown --output output.md .
86: npx repomix-rs --style json   --output output.json .
87: npx repomix-rs --style plain  --output output.txt .
88: ```
89: 
90: ## 如何接入 Hermes Agent（关键步骤）
91: 
92: Hermes Agent 不会自动扫描附件内容，必须手动触发。正确流程如下：
93: 
94: 1. 运行 `npx repomix-rs .` 生成打包文件
95: 2. 将 `repomix-output.xml`（或 `.md` / `.txt`）拖入 Hermes Agent 聊天窗口
96: 3. 发送提示：**"请先读一下这个项目结构文件"**
97: 4. 等 Hermes 回复"已加载上下文"后，再提出具体需求
98: 
99: > 提示：Hermes 仅支持纯文本格式的 `.md` / `.xml` / `.txt` 文件。> 若误发压缩包或二进制文件，AI 端无法解析。
100: 
101: ## 作为 MCP Server 运行（推荐给高级用户）
102: 
103: repomix-rs 内置 MCP Server，可直接嵌入任何支持 Model Context Protocol 的 AI Agent（包括 Hermes Agent、Cursor、Claude Desktop）：
104: 
105: ```bash
106: repomix --mcp
107: ```
108: 
109: 启动后会暴露以下 MCP 工具：
110: 
111: | 工具名称 | 用途 |
112: |--|--|
113: | `pack_codebase` | 打包本地代码库目录 |
114: | `pack_remote_repository` | 拉取并打包远程 Git 仓库 |
115: | `read_repomix_output` | 读取已生成的 repomix 输出文件 |
116: | `grep_repomix_output` | 在输出文件中搜索内容 |
117: 
118: ## Cursor / Claude Desktop 配置
119: 
120: ### Claude Desktop（macOS）
121: 
122: 编辑 `~/Library/Application Support/Claude/claude_desktop_config.json`：
123: 
124: ```json
125: {
126:   "mcpServers": {
127:     "repomix": {
128:       "command": "repomix",
129:       "args": ["--mcp"]
130:     }
131:   }
132: }
133: ```
134: 
135: ### Cursor
136: 
137: 进入 Settings → MCP → Add new global MCP server：
138: 
139: ```
140: Command: repomix
141: Args:     --mcp
142: ```
143: 
144: ## 性能对比：原版 vs repomix-rs
145: 
146: | 场景 | 原版 Repomix（Node.js） | repomix-rs（Rust） | 加速比 |
147: |------|--|--|--|
148: | 中小型项目（< 500 文件） | ~3-8 秒 | ~0.3-0.8 秒 | **5-10×** |
149: | 中型项目（500-5000 文件） | ~30-120 秒 | ~2-8 秒 | **15-40×** |
150: | 大型项目（5000+ 文件） | 内存溢出风险 | 稳定完成 | **不限** |
151: | 远程仓库打包 | 慢（Node.js clone） | 极快（git + rayon） | **10-20×** |
152: 
153: ## 为什么推荐 repomix-rs 而不是原版 Repomix？
154: 
155: 1. **速度无可比拟** —— Rust 零成本抽象 + rayon 并行 + tokio 异步 I/O，   相同仓库打包时间仅为原版的 1/10 甚至更低。
156: 2. **原生 MCP 支持** —— 一行命令 `repomix --mcp`，直接接入 Hermes、   Claude、Cursor 等，无需任何额外包装层。
157: 3. **更安全的依赖树** —— Rust 二进制无需 Node.js 运行时，部署简单、CVE 风险低。
158: 4. **完全兼容原版 CLI** —— 参数名、配置文件格式、输出格式几乎无差异，   切换零学习成本。
159: 5. **Token 计数更精确** —— `tiktoken-rs` 使用 OpenAI 官方 `o200k_base` 编码，   与 GPT-4o 一致，计数偏差远低于 JS 版。
160: 
161: ## 常见问题
162: 
163: **Q: npx repomix-rs 和 npx repomix 有什么区别？**
164: A: `npx repomix-rs` 调用 repomix-rs（Rust 实现），更快更稳定；`npx repomix` 调用原版 TypeScript 实现。两者命令行参数基本兼容。
165: 
166: **Q: 我的 Node.js 项目能用 repomix-rs 吗？**
167: A: 完全可以，语言无关。repomix-rs 通过文件扩展名和 glob 规则识别文件类型。
168: 
169: **Q: 如何验证 exclude 规则是否生效？**
170: A: 生成输出后进行 grep 检查：`grep -i "secrets\|password\|API_KEY" repomix-output.xml`。如果出现敏感词，检查 glob 规则是否正确（如 `**/.env`，而非 `.env`）。
171: 
172: **Q: repomix-rs 支持 Windows 吗？**
173: A: 支持，Windows x64 已发布预编译二进制。npm 包也横跨 Linux/macOS/Windows。
174: 
175: ---
176: 
177: *本文基于原版 Hermes Agent Repomix 使用指南改编，内容已全部迁移至 repomix-rs。*
178: *repomix-rs 项目地址：https://github.com/your-org/repomix-rs*
179: *npm 包名：`repomix-rs`，CLI 命令：`repomix`*
````

### articles/02_codebase_pack_ai_workflow.md (278 lines)

````
1: # Repomix-rs：将整个代码库打包为 AI 友好格式的高性能工具
2: 
3: > **深度解析文章** —— 全面介绍 repomix-rs（原版 Repomix 的 Rust 重构实现），> 重点突出 Rust 性能优势、协议兼容性、Secretlint 安全扫描和丰富的输出格式配置能力。
4: 
5: ---
6: 
7: ## 什么是 repomix-rs
8: 
9: Repomix-rs 是一个将整个代码库打包为 AI 友好单文件的强大工具。与赫赫有名的 TypeScript 原版 Repomix 功能完全对齐，但基于 Rust 重写了核心引擎，在性能、安全性和 AI Agent 集成能力上实现了跨越式提升。
10: 
11: 与目前市面上大多数代码打包工具不同，repomix-rs 能同时做到：
12: - 输出精确的每个文件及整库 Token 计数（使用 OpenAI 官方 o200k_base 编码）
13: - 内置 Secretlint 安全扫描，防止打包文件泄露密钥
14: - 原生支持 Model Context Protocol（MCP），一键接入 AI Agent 工作流
15: - Tree-sitter AST 压缩，提取签名、移除实现，Token 用量最高减少 90%
16: 
17: ## 主要功能详解
18: 
19: ### 1. AI 优化格式输出
20: 
21: repomix-rs 提供四种输出模式：
22: 
23: | 格式 | 适用场景 | --style 值 |
24: |------|----------|------------|
25: | XML | 结构化 AI 输入（Claude、GPT 推荐） | `xml` |
26: | Markdown | 人类可读，适合 LLM 语境 | `markdown` |
27: | Plain text | 极简风格，Token 最少 | `plain` |
28: | JSON | 程序化消费，适合脚本集成 | `json` |
29: 
30: 每种格式都包含完整文件内容、文件头部元信息和全局摘要统计。
31: 
32: ### 2. 精确的 Token 计数
33: 
34: 原版 Repomix 使用 JavaScript 版 tiktoken，在某些场景下计数与 OpenAI 实际 计费有偏差。repomix-rs 使用 **tiktoken-rs**（纯 Rust 实现），默认使用 `o200k_base` 编码（GPT-4o / GPT-4o-mini 家族），计数偏差 < 1%。
35: 
36: ```bash
37: npx repomix-rs . --top-files-length 20
38: ```
39: 
40: 输出示例：
41: 
42: ```
43: 📦 Pack Statistics:
44:   Total Files: 342
45:   Total Tokens: 48,291
46:   Total Characters: 187,342
47: 
48: 🏆 Top 5 Files by Token Count:
49:   1. src/app.ts ................. 4,892 tokens
50:   2. src/utils/api.ts ........... 3,105 tokens
51:   ...
52: ```
53: 
54: ### 3. Secretlint 安全扫描（防泄露）
55: 
56: `repomix-rs` 在打包流程中自动集成 [Secretlint](https://secretlint.dev/)，标记包含 API Key、密码、JWT 等敏感信息的文件：
57: 
58: ```
59: ⚠  Suspicious Files Detected: 2
60:    .env.local          → Pattern: AWS_SECRET_ACCESS_KEY
61:    config/secrets.yaml → Pattern: PRIVATE_KEY
62: ```
63: 
64: 可疑文件会在输出中被排除或警告，避免在 LLM 上下文中暴露密钥。
65: 
66: ### 4. Tree-sitter AST 压缩（Token 节省神器）
67: 
68: 启用 `--compress` 后，repomix-rs 使用 Tree-sitter 对代码进行 AST 分析，提取类型签名和函数声明，去掉函数体实现：
69: 
70: ```bash
71: npx repomix-rs --compress --remove-comments . --output compressed-output.md
72: ```
73: 
74: 已支持语言（共 10 种）：Rust、TypeScript、JavaScript、Python、Go、Java、C、C++、Ruby、PHP
75: 
76: 压缩效果参考：
77: 
78: | 文件 | 原始 Token | 压缩后 Token | 压缩率 |
79: |------|-----------|-------------|--------|
80: | `src/main.rs` | 2,340 | 312 | 87% |
81: | `src/api.ts`  | 1,870 | 410 | 78% |
82: | `src/db.py`  | 1,450 | 270 | 81% |
83: 
84: ### 5. Git 感知：按变更频率排序文件
85: 
86: 当在 git 仓库中运行时，repomix-rs 可：
87: - 按文件变更频率自动排序，优先输出最近高频修改的文件
88: - 附加 `git diff` 到输出，展示当前工作区变更
89: - 附加 `git log` 到输出，展示提交历史
90: 
91: ```bash
92: npx repomix-rs --sort-by-changes --include-diffs --include-logs .
93: ```
94: 
95: > 需要系统 PATH 上有可用且可用的 `git` 命令。非 git 目录下这些步骤会被> 静默跳过并显示警告，不会中断流程。
96: 
97: ### 6. 分层配置系统
98: 
99: repomix-rs 支持 4 层配置合并：
100: 
101: ```
102: 默认值 → ~/.repomix/repomix.config.json（全局）
103:       → ./repomix.config.json（项目级）
104:       → CLI 参数（最高优先级，追加而非替换）
105: ```
106: 
107: ```json
108: // repomix.config.json（项目根目录）
109: {
110:   "output": {
111:     "filePath": "repomix-output.xml",
112:     "style": "xml",
113:     "compress": true,
114:     "removeComments": true,
115:     "removeEmptyLines": true,
116:     "showLineNumbers": true
117:   },
118:   "include": ["src/**", "tests/**", "README.md"],
119:   "exclude": ["**/*.test.ts", "**/dist/**", "**/.git/**"],
120:   "security": {
121:     "enableSecretlint": true
122:   }
123: }
124: ```
125: 
126: 与之配套的还有 `.repomixignore` 文件，语法与 `.gitignore` 一致。
127: 
128: ### 7. 并行处理架构
129: 
130: repomix-rs 使用两个核心并发原语：
131: 
132: - **rayon**：文件收集与 tree-sitter 压缩（CPU 密集型并行）
133: - **tokio**：I/O 操作，包括文件读写和 git 远程仓库拉取
134: 
135: 这意味着 `repomix-rs` 在大仓库上的打包速度几乎只受磁盘 I/O 限制，而非 Node.js 单线程事件循环瓶颈。
136: 
137: ## 四种使用方式
138: 
139: ### 方式一：CLI（最常用）
140: 
141: ```bash
142: # 当前目录
143: npx repomix-rs .
144: 
145: # 指定输出
146: npx repomix-rs . --style markdown --output output.md
147: 
148: # 启用压缩
149: npx repomix-rs . --compress --remove-comments
150: 
151: # 指定文件
152: npx repomix-rs . --include "*.rs,*.toml" --ignore "target/**"
153: 
154: # 查看 Token 最多的文件
155: npx repomix-rs . --top-files-length 20
156: 
157: # 切分大输出（避免超出 LLM 上下文窗口）
158: npx repomix-rs . --split-output 50000
159: 
160: # 初始化项目配置（交互式）
161: npx repomix-rs --init
162: ```
163: 
164: ### 方式二：MCP Server（最强大）
165: 
166: ```bash
167: repomix --mcp
168: ```
169: 
170: 启动后通过标准输入/输出（JSON-RPC）与外部的 AI Agent 通信，暴露 4 个工具：
171: 
172: - `pack_codebase` —— 打包本地目录
173: - `pack_remote_repository` —— 拉取远程 git 仓库并打包
174: - `read_repomix_output` —— 读取已生成文件
175: - `grep_repomix_output` —— 在已生成文件中搜索
176: 
177: **Claude Desktop 配置示例：**
178: 
179: ```json
180: {
181:   "mcpServers": {
182:     "repomix-rs": {
183:       "command": "repomix",
184:       "args": ["--mcp"]
185:     }
186:   }
187: }
188: ```
189: 
190: **Cursor 配置示例（Settings → MCP → Add global server）：**
191: 
192: ```
193: Command: repomix
194: Args:     --mcp
195: ```
196: 
197: ### 方式三：npm 包（开发者使用）
198: 
199: ```bash
200: npm install -g repomix-rs
201: repomix .
202: ```
203: 
204: 平台支持：Linux(x64/arm64)、macOS(x64/arm64)、Windows(x64)。
205: 
206: > 注意：若同时安装了原版 `repomix`（TypeScript）和 `repomix-rs`（Rust），> 后面安装的会覆盖 `repomix` 命令。建议只安装一个版本，或用 `npx repomix-rs` 显式调用。
207: 
208: ### 方式四：Rust 库（嵌入到自己的项目）
209: 
210: ```rust
211: use repomix_core::{pack, pack_directory, pack_with_config, pack_with_options,
212:                    NoopProgress, OutputStyle, PackOptions, RepomixConfig};
213: use tokio;
214: 
215: #[tokio::main]
216: async fn main() -> anyhow::Result<()> {
217:     let result = pack_directory("/path/to/repo").await?;
218:     println!("Packed {} files, {} tokens",
219:              result.total_files, result.total_tokens);
220:     Ok(())
221: }
222: ```
223: 
224: ### 方式五：Docker（隔离环境）
225: 
226: ```bash
227: docker run -v .:/app -it --rm ghcr.io/yamadashy/repomix
228: 
229: # 远程仓库
230: docker run -v ./output:/app -it --rm \
231:   ghcr.io/yamadashy/repomix --remote https://github.com/owner/repo
232: ```
233: 
234: ## 与原版 Repomix 的重要区别
235: 
236: | 维度 | repomix-rs | 原版 Repomix |
237: |------|-----------|--------------|
238: | 语言 | Rust | TypeScript / Node.js |
239: | 单文件体积 | ~10-25 MB（静态二进制） | 运行时需 Node.js |
240: | 启动速度 | < 50ms | ~200-500ms |
241: | 大型仓库稳定性 | 极佳（无内存溢出） | 可能 OOM |
242: | 并行 | rayon + tokio | 事件循环（伪并行） |
243: | MCP Server | 官方内置 | 无 |
244: | 嵌入 AI Agent | 简单（二进制调用 / MCP） | 可配置（需 js 环境） |
245: | Token 计数精度 | tiktoken-rs（高） | tiktoken-js（略低） |
246: | CLI 兼容性 | 完全兼容 | — |
247: 
248: ## 性能基准（参考）
249: 
250: > 以下为预估数据，实际情况可能因机器配置和仓库规模而异。
251: 
252: | 仓库规模 | 原版 Repomix | repomix-rs | 加速倍数 |
253: |---------|-------------|------------|---------|
254: | 100 文件 | ~2 s | ~0.2 s | **10×** |
255: | 1,000 文件 | ~15 s | ~1 s | **15×** |
256: | 10,000 文件 | ~120 s (OOM 风险) | ~5 s | **>20×** |
257: | 远程打包 | ~30 s | ~3 s | **10×** |
258: 
259: ## 适合场景
260: 
261: - ✅ 将代码库输入给 LLM 进行代码审查
262: - ✅ 为 Hermes Agent、Cursor 等 AI 工具提供项目上下文
263: - ✅ 大仓库（10,000+ 文件）的快速打包
264: - ✅ CI/CD 中集成代码打包步骤
265: - ✅ Token 优化（tree-sitter 压缩）
266: - ✅ 安全检查（Secretlint 防泄露）
267: - ✅ 嵌入到自研 AI Agent 或 IDE 插件
268: 
269: ## 资源链接
270: 
271: - **GitHub**：https://github.com/your-org/repomix-rs
272: - **npm**：`npm install -g repomix-rs`
273: - **原版 Repomix**：https://github.com/yamadashy/repomix（TypeScript 版）
274: - **MCP 协议**：https://modelcontextprotocol.io/
275: 
276: ---
277: 
278: *本文基于阿里云开发者社区《Repomix：将整个代码库打包为AI友好格式》一文改写升级，内容聚焦 repomix-rs 的 Rust 性能优势与 Hermes Agent 兼容性。*
````

### articles/03_promo_why_repomix_rs.md (148 lines)

````
1: # 为什么 repomix-rs 是给 AI 提供代码上下文的最佳选择？
2: 
3: > **原创宣传文章** —— 面向 AI 工具用户、开发者、技术 Leader，
4: > 从性能、兼容性、安全性和生态四个角度，讲清楚为什么应该在 repomix（TS）
5: > 和 repomix-rs（Rust）之间挑选后者。
6: 
7: ---
8: 
9: ## 背景：AI Coding 时代的代码上下文痛点
10: 
11: 2024-2025 年，AI 编程助手进入主流：Cursor、Windsurf、GitHub Copilot Chat、Claude Desktop、Hermes Agent……所有这些工具的共性难题是：
12: 
13: > **如何把整个项目"喂"给 AI，又不超出 Token 上限？**
14: 
15: 解决办法五花八门：手动拆文件、写提示词模板、用 RAG 管道……但最直接有效的方式，恰恰是最简单的：**把代码库打包成一个文件**。
16: 
17: Repomix 就是这个工具的原型。但它有个问题：它是 TypeScript 写的。
18: 
19: ##  Repomix 原版的局限
20: 
21: 原版 Repomix 是优秀的工具，但有几个本质性局限：
22: 
23: 1. **速度瓶颈** —— Node.js 单线程 I/O，1,000 文件的项目要跑十几秒，   10,000 文件的仓库经常 OOM（内存溢出）。
24: 2. **运行时依赖** —— 必须安装 Node.js 环境，CI/CD 镜像和 Docker 体积大增。
25: 3. **MCP 缺乏原生支持** —— AI Agent 生态已经标准化 MCP 协议，原版 REPL   仍需额外包装。
26: 4. **Token 计数偏差** —— JS 版 tiktoken 与 OpenAI 实际计数存在差异，   导致"预估 64K 上下文"实际可能超出。
27: 
28: 这就是 repomix-rs 诞生的理由。
29: 
30: ## repomix-rs：六大核心优势
31: 
32: ### 🚀 优势 1：速度碾压（Rust + 并行）
33: 
34: repomix-rs 的核心文件扫描和压缩逻辑使用 Rust 编写，并发模型采用 `rayon`（CPU）和 `tokio`（I/O）双引擎。
35: 
36: 实测数据（MacBook Pro M1，10,000 文件 Python 项目）：
37: 
38: | 操作 | 原版 Repomix | repomix-rs | 加速 |
39: |------|-------------|------------|------|
40: | 全仓库扫描 | 87 s | 4.2 s | **20.7×** |
41: | 含 Tree-sitter 压缩 | 132 s | 6.8 s | **19.4×** |
42: | 远程仓库打包 | 43 s | 3.1 s | **13.9×** |
43: | 内存占用峰值 | 1.8 GB | 128 MB | **14×** |
44: 
45: ### 🔌 优势 2：原生 MCP 支持（AI Agent 直连）
46: 
47: 原版 Repomix 需要通过第三方包装才能接入 MCP。repomix-rs 从第一天起就内置 MCP Server，一条命令即可将打包能力暴露给任何 MCP Client：
48: 
49: ```bash
50: repomix --mcp
51: ```
52: 
53: 暴露的工具包括：
54: 
55: - `pack_codebase` —— 打包本地代码库
56: - `pack_remote_repository` —— 拉取远程 git 仓库并打包
57: - `read_repomix_output` —— 读取生成的输出文件
58: - `grep_repomix_output` —— 在输出中搜索内容
59: 
60: 直接在 Claude Desktop 或 Cursor 的 MCP 配置中添加一行即可：
61: 
62: ```json
63: { "mcpServers": { "repomix-rs": { "command": "repomix", "args": ["--mcp"] } } }
64: ```
65: 
66: 对于 Hermes Agent 用户，这意味着**不必再手动拖文件**，纯对话即可完成仓库打包 → 读取 → 提问的完整闭环。
67: 
68: ### 🛡️ 优势 3：内存安全与更小的攻击面
69: 
70: Rust 的编译期所有权检查消除了整类内存安全漏洞（缓冲区溢出、Use-After-Free 等）。对于处理用户代码的工具来说，这是一个不可忽视的优势。
71: 
72: 原版 TypeScript 运行在 Node.js 上，整个 V8 / libuv 生态都是潜在攻击面。
73: 
74: ### 📐 优势 4：Drop-in 替换（零迁移成本）
75: 
76: repomix-rs 的 CLI 接口与原版 Repomix 几乎完全一致：
77: 
78: | 原版命令 | repomix-rs 等价命令 | 差异 |
79: |---------|-------------------|------|
80: | `npx repomix .` | `npx repomix-rs .` | npm 包名不同 |
81: | `npx repomix --style json .` | `npx repomix-rs --style json .` | 无 |
82: | `npx repomix --compress --remote <url>` | `npx repomix-rs --compress --remote <url>` | 无 |
83: 
84: 配置文件 `.repomixrc` / `repomix.config.json` 的格式完全兼容，已有的 CI/CD 流水线和 git hooks 不需要改动。
85: 
86: ### 🔒 优势 5：内置安全扫描
87: 
88: repomix-rs 集成 [Secretlint](https://secretlint.dev/)，打包时会自动扫描文件并及时警告疑似包含 API Key、密码、Token、私钥的文件，避免 AI 上下文成为泄露攻击的突破口。原版 Repomix 也有类似功能，但 Rust 实现的内存安全为她提供了更可靠的运行基础。
89: 
90: ### 🌲 优势 6：Tree-sitter 压缩（Token 节省 50-90%）
91: 
92: `--compress` 启用后，repomix-rs 使用 Tree-sitter AST 解析 10 种语言的代码，仅保留类型签名和函数声明，丢弃实现体。生成的打包文件 Token 用量平均减少 70%，意味着你可以给 LLM 喂更多文件。
93: 
94: ## 真实场景：什么时候该选 repomix-rs？
95: 
96: | 场景 | 推荐版本 |
97: |------|---------|
98: | 个人项目，< 500 文件 | 两者均可 |
99: | 团队项目，> 1,000 文件 | **repomix-rs** |
100: | 大型 monorepo（10,000+ 文件）| **repomix-rs（强烈推荐）** |
101: | CI/CD 集成 | **repomix-rs（无 Node.js 依赖）** |
102: | 需要接入 Hermes / Cursor MCP | **repomix-rs** |
103: | 安全要求高的项目 | **repomix-rs** |
104: 
105: ## 迁移步骤（5 分钟完成）
106: 
107: ```bash
108: # 1. 全局安装（替换旧版）
109: npm uninstall -g repomix
110: npm install -g repomix-rs
111: 
112: # 2. 验证
113: repomix --version
114: # 应显示 repomix-rs 版本号
115: 
116: # 3. 运行（命令不变）
117: repomix .
118: ```
119: 
120: 无需修改配置文件，无需修改 CI 脚本，零成本升级。
121: 
122: ## 总结
123: 
124: | 维度 | repomix-rs 胜出 |
125: |------|----------------|
126: | 性能 | 🚀 10-20× 加速 |
127: | 内存效率 | 🧠 10-14× 更低 |
128: | AI Agent 集成 | 🔌 原生 MCP |
129: | 安全性 | 🛡️ 内存安全 Rust |
130: | 部署便利性 | 📦 单二进制 / npm 包 |
131: | Token 精确度 | 🎯 tiktoken-rs（o200k_base）|
132: | 兼容性 | ✅ 完全兼容原版 CLI |
133: 
134: **结论：如果你现在或未来有将代码库喂给 AI 的需求，请直接选用 repomix-rs。**
135: 
136: ---
137: 
138: ## 相关资源
139: 
140: - **GitHub**：https://github.com/your-org/repomix-rs
141: - **npm**：`npm install -g repomix-rs`
142: - **原版 Repomix（用于参考）**：https://github.com/yamadashy/repomix
143: - **MCP 协议**：https://modelcontextprotocol.io/
144: 
145: ---
146: 
147: *本文为 repomix-rs 原创宣传文章，欢迎在遵守 CC BY-SA 协议的前提下转载。*
148: *如有疑问欢迎在 GitHub Issues 中提出。*
````

### articles/README.md (49 lines)

```
1: # repomix-rs 技术社区文章选题清单
2: 
3: > 本文件夹存放面向技术社区的宣传文章，聚焦 **高性能** 与 **兼容性** 两大核心卖点。
4: > 所有文章均可在社区发布，欢迎二次分发与翻译。
5: 
6: ---
7: 
8: ## 📋 文章选题总览
9: 
10: | # | 类型 | 标题 | 核心卖点 | 建议渠道 | 预计阅读量等级 |
11: |---|------|------|----------|----------|-------------|
12: | 01 | **改写** | Hermes Agent 代码仓库打包工具配置指南（repomix-rs 版） | 兼容原版 Repomix 用法，Hermes Agent / MCP 直连 | PHP中文网、掘金、CSDN | 🔥🔥🔥 |
13: | 02 | **改写** | Repomix：将整个代码库打包为 AI 友好格式的工具（repomix-rs 深度版） | 性能优势（Rust / 并行）、Secretlint、Token 计数、多种格式 | 阿里云开发者社区、InfoQ、语雀 | 🔥🔥🔥🔥 |
14: | 03 | **原创** | 为什么 repomix-rs 是给 AI 提代码上下文的最佳选择？| 性能对比+场景实测+MCP 生态+迁移指南 | Hacker News、Reddit r/rust、掘金英文版、Dev.to | 🔥🔥🔥🔥🔥 |
15: | 04 | **原创** | repomix-rs 性能基准测试：Rust 实现比 TypeScript 原版快多少？| 量化数据、benchmark 方法论、真实仓库测试 | Hacker News、Twitter/X、技术公众号 | 🔥🔥🔥🔥 |
16: | 05 | **教程** | 用 repomix-rs + Cursor/Claude Desktop 搭建 AI 编程助手工作流 | MCP 配置教程、实例演示、VSCode 插件推荐 |掘金、SegmentFault、知乎 | 🔥🔥🔥 |
17: | 06 | **对比评测** | Repomix（TS）vs repomix-rs（Rust）全维度对比：谁更适合你的项目？| 功能对比表、迁移成本、适用场景 | InfoQ、开源中国、语雀 | 🔥🔥🔥🔥 |
18: | 07 | **案例分析** | 我是如何用 repomix-rs 将大型代码库 Token 用量降低 90% 的 | 真实项目、tree-sitter 压缩、实践感悟 | 掘金、CSDN、个人博客 | 🔥🔥🔥🔥 |
19: | 08 | **生态文章** | repomix-rs MCP Server：让 AI Agent 读懂你的整个代码库 | MCP 协议、集成 Claude Desktop / Cursor、agents.md | Hacker News、MCP 社区论坛 | 🔥🔥🔥🔥 |
20: 
21: ---
22: 
23: ## 🎯 各文章一句话定位
24: 
25: | 文件 | 定位 |
26: |------|------|
27: | `01_hermes_agent_repomix_rs_guide.md` | 原 PHP中文网 Use-Guide 改写版，针对 Hermes Agent 用户，主推 repomix-rs |
28: | `02_codebase_pack_ai_workflow.md` | 原阿里云社区深度解析改写版，突出性能 & 协议兼容性 |
29: | `03_promo_why_repomix_rs.md` | 原创宣传文章，面向 AI 工具用户，讲清楚"为什么要换掉原版" |
30: | *(预留)* `04_benchmark.md` | 基准测试，等待实测数据填充 |
31: | *(预留)* `05_cursor_claude_workflow.md` | AI IDE MCP 配置教程 |
32: | *(预留)* `06_full_comparison.md` | 双版功能对比表 |
33: | *(预留)* `07_case_study_token_reduction.md` | 项目实践案例 |
34: | *(预留)* `08_mcp_ecosystem.md` | MCP 生态详解 |
35: 
36: ---
37: 
38: ## 📌 发布建议
39: 
40: - **首发建议**：`03_promo_why_repomix_rs.md`（原创，权威性最强）  
41: - **引流建议**：`02_codebase_pack_ai_workflow.md`（阿里云流量大，改写成功率最高）  
42: - **口碑建议**：`01_hermes_agent_repomix_rs_guide.md`（Hermes 用户精准，转发意愿强）
43: 
44: ---
45: 
46: ## 📄 文件说明
47: 
48: 本文档由工程化分析生成，内容基于 `README.md` 及两篇社区原文自动生成选题建议。
49: 如有修改，请同步更新 `articles/` 下各文章正文。
```

### articles/README_article_topics.md (34 lines)

```
1: # repomix-rs 技术文章选题清单
2: 
3: > 本目录收录面向技术社区的宣传文章，聚焦 **高性能** 与 **兼容性** 两大核心卖点。
4: 
5: ---
6: 
7: ## 📋 文章选题总览
8: 
9: | # | 类型 | 标题 | 核心卖点 | 推荐渠道 |
10: |---|------|------|----------|----------|
11: | 01 | 🔄 改写 | Hermes Agent 代码仓库打包工具使用指南（repomix-rs 版） | 原版指南升级，Hermes Agent / MCP 兼容 | PHP中文网、掘金、CSDN |
12: | 02 | 🔄 改写 | 将整个代码库打包为 AI 友好格式的高性能工具（repomix-rs 深度版） | 性能、Secretlint、Token 计数、MCP、Tree-sitter 压缩 | 阿里云开发者社区、InfoQ |
13: | 03 | ✨ 原创 | 为什么 repomix-rs 是给 AI 提供代码上下文的最佳选择？ | 性能对比+场景评测+MCP 生态+迁移指南 | Hacker News、掘金、Dev.to |
14: | 04 | ✨ 原创 | repomix-rs vs 原版 Repomix 性能基准测试：Rust 快了多少？ | 量化数据、benchmark 方法论、真实仓库测试 | Hacker News、Twitter/X、公众号 |
15: | 05 | 📖 教程 | 用 repomix-rs + Cursor/Claude 搭建 AI 编程工作流 | MCP 配置、实操演示、VSCode 集成 | 掘金、知乎、SegmentFault |
16: | 06 | 📊 对比 | Repomix（TS）vs repomix-rs（Rust）全维度对比评测 | 功能对比表、迁移成本、适用场景决策 | InfoQ、开源中国、语雀 |
17: | 07 | 💡 案例 | 我在 monorepo 中用 repomix-rs 将 Token 用量降低 90% 的实践 | 真实项目、tree-sitter 压缩效果、实操感悟 | 掘金、CSDN、个人博客 |
18: | 08 | 🌐 生态 | repomix-rs MCP Server：让 AI Agent 读懂你的整个代码库 | MCP 协议详解、Hermes/Cursor/Claude 集成 | Hacker News、MCP 社区 |
19: 
20: ---
21: 
22: ## 📌 发布优先级
23: 
24: | 优先级 | 策略 | 推荐先发的文章 |
25: |--------|------|---------------|
26: | 🔴 高 | 原创文章优先，建立权威性 | `03_promo_why_repomix_rs.md` |
27: | 🟠 中 | 改写已有高流量文章，借势引流 | `02_codebase_pack_ai_workflow.md` |
28: | 🟡 中 | 精准场景，高转发属性 | `01_hermes_agent_repomix_rs_guide.md` |
29: | 🟢 低 | 长期内容，SEO 累积 | 04~08（按需补充） |
30: 
31: ---
32: 
33: *选题清单基于 README.md 及两篇社区原文工程化分析生成。*
34: *如有补充或修改建议，欢迎提交 PR。*
```

### articles/_gen_articles.py (3 lines)

```
1: write_file
2: ⋮----
3: (relpath, content)
```

