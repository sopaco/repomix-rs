use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, Content, ErrorData, Implementation, ProtocolVersion, ServerCapabilities,
    ServerInfo,
};
use rmcp::schemars;
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use repomix_config::schema::{OutputStyle, RepomixConfig};
use repomix_core::packager::{pack, NoopProgress, PackResult};

// ===== Result / metrics structs =====

#[derive(Debug, Serialize, Deserialize)]
pub struct PackToolResult {
    pub description: String,
    pub result: String,
    pub directory_structure: String,
    pub output_id: String,
    pub output_file_path: String,
    pub total_files: usize,
    pub total_tokens: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackMetrics {
    pub total_files: usize,
    pub total_tokens: usize,
    pub total_characters: usize,
    pub file_token_counts: std::collections::HashMap<String, usize>,
    pub file_char_counts: std::collections::HashMap<String, usize>,
    /// 按 token 数降序的前 N 个文件（路径, token）。
    pub top_files_by_tokens: Vec<(String, usize)>,
}

impl From<&PackResult> for PackMetrics {
    fn from(r: &PackResult) -> Self {
        Self {
            total_files: r.total_files,
            total_tokens: r.total_tokens,
            total_characters: r.total_characters,
            file_token_counts: r.file_token_counts.clone(),
            file_char_counts: r.file_char_counts.clone(),
            top_files_by_tokens: r.top_files_by_tokens.clone(),
        }
    }
}

// ===== Tool parameter types =====

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PackCodebaseParams {
    /// Directory to pack (absolute path). Defaults to current working directory.
    #[serde(default)]
    pub directory: Option<String>,
    /// Enable Tree-sitter compression to extract code signatures.
    #[serde(default)]
    pub compress: Option<bool>,
    /// Comma-separated include patterns, e.g. "*.rs,*.toml".
    #[serde(default)]
    pub include_patterns: Option<String>,
    /// Comma-separated ignore patterns, e.g. "target/**,tests/**".
    #[serde(default)]
    pub ignore_patterns: Option<String>,
    /// Number of top files to include in metrics breakdown.
    #[serde(default)]
    pub top_files_length: Option<usize>,
    /// Output style: xml | markdown | plain | json. Default: xml.
    #[serde(default)]
    pub style: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PackRemoteRepositoryParams {
    /// Git remote URL (https://... or git@...).
    pub url: String,
    /// Output style: xml | markdown | plain | json. Default: xml.
    #[serde(default)]
    pub style: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReadRepomixOutputParams {
    /// Path to the repomix output file.
    pub file_path: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GrepRepomixOutputParams {
    /// Path to the repomix output file.
    pub file_path: String,
    /// Regular expression to search for.
    pub pattern: String,
    /// Number of context lines before and after each match. Default: 0.
    #[serde(default)]
    pub context: Option<usize>,
}

// ===== helpers =====

/// 解析输出风格；未知值返回 `invalid_params` 错误，空值默认 XML。
fn parse_style(s: Option<&str>) -> Result<OutputStyle, ErrorData> {
    match s {
        None | Some("") | Some("xml") => Ok(OutputStyle::Xml),
        Some("markdown") => Ok(OutputStyle::Markdown),
        Some("plain") => Ok(OutputStyle::Plain),
        Some("json") => Ok(OutputStyle::Json),
        Some(other) => Err(ErrorData::invalid_params(
            format!(
                "invalid style '{}': expected one of xml, markdown, plain, json",
                other
            ),
            None,
        )),
    }
}

/// 验证远程仓库 URL 的基本合法性。
fn validate_remote_url(url: &str) -> Result<(), ErrorData> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return Err(ErrorData::invalid_params(
            "remote url is empty",
            None,
        ));
    }
    // 支持 https://, http://, git://, ssh (user@host:path) 协议
    let ok = trimmed.starts_with("https://")
        || trimmed.starts_with("http://")
        || trimmed.starts_with("git://")
        || trimmed.starts_with("ssh://")
        || (trimmed.contains('@') && trimmed.contains(':') && !trimmed.contains(' '));
    if !ok {
        return Err(ErrorData::invalid_params(
            format!(
                "remote url '{}' is not a recognized git url \
                 (expected https://, http://, git://, ssh:// or user@host:path)",
                trimmed
            ),
            None,
        ));
    }
    Ok(())
}

fn split_csv(s: Option<&str>) -> Vec<String> {
    s.map(|v| {
        v.split(',')
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect()
    })
    .unwrap_or_default()
}

/// 与 CLI 一致：从 CWD 加载分层配置，再应用 MCP 工具参数。
fn load_mcp_config(partial: repomix_config::load::PartialConfig) -> Result<RepomixConfig, ErrorData> {
    let config_root = std::env::current_dir().map_err(|e| {
        ErrorData::internal_error(format!("cannot resolve config root: {}", e), None)
    })?;
    RepomixConfig::load(Some(partial), &config_root).map_err(|e| {
        ErrorData::internal_error(format!("load config: {}", e), None)
    })
}

/// 创建唯一临时目录（PID + 时间戳 + 随机后缀）
fn make_temp_dir(prefix: &str) -> Result<PathBuf> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;

    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let mut h = DefaultHasher::new();
    SystemTime::now().hash(&mut h);
    let rand = h.finish();
    let dir = std::env::temp_dir().join(format!(
        "{}_{}_{}_{:x}",
        prefix,
        std::process::id(),
        nanos,
        rand
    ));
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// RAII 临时目录守卫；drop 时 best-effort 清理目录。
struct TempDirGuard {
    path: PathBuf,
}

impl TempDirGuard {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_dir_all(&self.path) {
            tracing::warn!(
                "Failed to clean up temp dir '{}': {}. \
                 This may be a permission issue or the directory is in use.",
                self.path.display(),
                e
            );
        }
    }
}

fn style_extension(s: &OutputStyle) -> &'static str {
    match s {
        OutputStyle::Xml => "xml",
        OutputStyle::Markdown => "md",
        OutputStyle::Json => "json",
        OutputStyle::Plain => "txt",
    }
}

/// 在 `~/.repomix/outputs/` 下创建唯一输出路径，供 MCP 持久化写入。
fn make_mcp_output_path(style: &OutputStyle) -> Result<PathBuf, ErrorData> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let dir = repomix_config::global_dir::mcp_outputs_dir().map_err(|e| {
        ErrorData::internal_error(format!("create mcp outputs dir: {}", e), None)
    })?;
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    Ok(dir.join(format!(
        "pack_{}_{}.{}",
        std::process::id(),
        nanos,
        style_extension(style)
    )))
}

fn pack_tool_result(result: &PackResult, description: &str) -> PackToolResult {
    PackToolResult {
        description: description.to_string(),
        result: serde_json::to_string_pretty(&PackMetrics::from(result)).unwrap_or_default(),
        directory_structure: result.directory_structure.clone(),
        output_id: "packed_output".to_string(),
        output_file_path: result
            .output_paths
            .first()
            .cloned()
            .unwrap_or_default(),
        total_files: result.total_files,
        total_tokens: result.total_tokens,
    }
}

fn ok_result(value: serde_json::Value) -> Result<CallToolResult, ErrorData> {
    let text = serde_json::to_string_pretty(&value)
        .map_err(|e| ErrorData::internal_error(format!("serialize result: {}", e), None))?;
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

// ===== MCP server =====

#[derive(Clone)]
pub struct RepomixMcpServer {
    tool_router: ToolRouter<Self>,
    /// 防止并发 pack 同一仓库（rayon + git 子进程，串行化更安全）
    lock: Arc<Mutex<()>>,
}

impl Default for RepomixMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl RepomixMcpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            lock: Arc::new(Mutex::new(())),
        }
    }

    #[tool(
        name = "pack_codebase",
        description = "Pack a local directory into an AI-friendly format (XML/Markdown/Plain/JSON). Returns a JSON object with total_files, total_tokens, output_file_path and metrics breakdown. Use this when the user wants to feed a codebase to an LLM."
    )]
    async fn pack_codebase(
        &self,
        Parameters(p): Parameters<PackCodebaseParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let _guard = self.lock.lock().await;

        let root_dir: PathBuf = p
            .directory
            .as_deref()
            .map(PathBuf::from)
            .or_else(|| std::env::current_dir().ok())
            .ok_or_else(|| {
                ErrorData::invalid_params("directory not provided and CWD unavailable", None)
            })?;

        let style = parse_style(p.style.as_deref())?;
        let partial = repomix_config::load::PartialConfig {
            include: p.include_patterns.as_deref().map(|s| split_csv(Some(s))),
            ignore: p.ignore_patterns.as_deref().map(|s| split_csv(Some(s))),
            compress: p.compress,
            top_files_length: p.top_files_length,
            style: Some(style),
            ..Default::default()
        };
        let mut config = load_mcp_config(partial)?;

        let output_path = make_mcp_output_path(&config.output.style)?;
        config.output.file_path = output_path.to_string_lossy().to_string();

        let result = pack(vec![root_dir], config, Box::new(NoopProgress))
            .await
            .map_err(|e| ErrorData::internal_error(format!("pack failed: {}", e), None))?;

        let tool_result = pack_tool_result(
            &result,
            &format!(
                "Successfully packed {} files ({} tokens) from repository",
                result.total_files, result.total_tokens
            ),
        );
        ok_result(serde_json::to_value(&tool_result).unwrap_or_default())
    }

    #[tool(
        name = "pack_remote_repository",
        description = "Clone a remote git repository to a temporary directory and pack it. Returns the same structure as pack_codebase."
    )]
    async fn pack_remote_repository(
        &self,
        Parameters(p): Parameters<PackRemoteRepositoryParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let _guard = self.lock.lock().await;

        validate_remote_url(&p.url)?;

        let temp_dir = make_temp_dir("repomix_mcp_remote")
            .map_err(|e| ErrorData::internal_error(format!("create temp dir: {}", e), None))?;
        let _temp_guard = TempDirGuard::new(temp_dir.clone());

        repomix_core::git::remote::clone_remote_repo(&p.url, &temp_dir)
            .map_err(|e| ErrorData::internal_error(format!("git clone failed: {}", e), None))?;

        let partial = repomix_config::load::PartialConfig {
            style: Some(parse_style(p.style.as_deref())?),
            ..Default::default()
        };
        let mut config = load_mcp_config(partial)?;
        let output_path = make_mcp_output_path(&config.output.style)?;
        config.output.file_path = output_path.to_string_lossy().to_string();

        let result = pack(vec![temp_dir.clone()], config, Box::new(NoopProgress))
            .await
            .map_err(|e| ErrorData::internal_error(format!("pack failed: {}", e), None))?;

        let tool_result = pack_tool_result(
            &result,
            &format!(
                "Successfully packed {} files ({} tokens) from remote repository",
                result.total_files, result.total_tokens
            ),
        );
        ok_result(serde_json::to_value(&tool_result).unwrap_or_default())
    }

    #[tool(
        name = "read_repomix_output",
        description = "Read the contents of a previously generated repomix output file. Returns the raw text content."
    )]
    async fn read_repomix_output(
        &self,
        Parameters(p): Parameters<ReadRepomixOutputParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let content = std::fs::read_to_string(&p.file_path)
            .map_err(|e| ErrorData::internal_error(format!("read failed: {}", e), None))?;
        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(
        name = "grep_repomix_output",
        description = "Search a repomix output file for lines matching a regular expression. Returns a JSON object with match_count and a matches array."
    )]
    async fn grep_repomix_output(
        &self,
        Parameters(p): Parameters<GrepRepomixOutputParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let content = std::fs::read_to_string(&p.file_path)
            .map_err(|e| ErrorData::internal_error(format!("read failed: {}", e), None))?;
        let regex = regex::Regex::new(&p.pattern)
            .map_err(|e| ErrorData::invalid_params(format!("invalid regex: {}", e), None))?;

        let context = p.context.unwrap_or(0);
        let lines: Vec<&str> = content.lines().collect();
        let mut matches: Vec<serde_json::Value> = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            if regex.is_match(line) {
                let mut entry = serde_json::Map::new();
                entry.insert("line_number".into(), serde_json::json!(i + 1));
                entry.insert("text".into(), serde_json::json!(line));
                if context > 0 {
                    let start = i.saturating_sub(context);
                    let end = (i + context + 1).min(lines.len());
                    entry.insert(
                        "context_before".into(),
                        serde_json::json!(lines[start..i].join("\n")),
                    );
                    entry.insert(
                        "context_after".into(),
                        serde_json::json!(lines[i + 1..end].join("\n")),
                    );
                }
                matches.push(serde_json::Value::Object(entry));
            }
        }

        ok_result(serde_json::json!({
            "file": p.file_path,
            "pattern": p.pattern,
            "match_count": matches.len(),
            "matches": matches,
        }))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for RepomixMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(ProtocolVersion::V_2024_11_05)
            .with_instructions(
                "Pack codebases into AI-friendly formats. Tools: pack_codebase, \
                 pack_remote_repository, read_repomix_output, grep_repomix_output.",
            )
            .with_server_info(Implementation::new("repomix", env!("CARGO_PKG_VERSION")))
    }
}

/// 真正以 MCP 协议启动 stdio 服务器
pub async fn run_stdio_server() -> Result<()> {
    use rmcp::transport::stdio;
    use rmcp::ServiceExt;

    let server = RepomixMcpServer::new();
    let service = server
        .serve(stdio())
        .await
        .map_err(|e| anyhow::anyhow!("mcp stdio serve failed: {}", e))?;
    service
        .waiting()
        .await
        .map_err(|e| anyhow::anyhow!("mcp server stopped: {}", e))?;
    Ok(())
}
