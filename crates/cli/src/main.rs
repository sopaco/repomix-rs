mod run;
mod report;
mod spinner;
mod prompts;

use clap::{Parser, ValueEnum};
use repomix_config::schema::OutputStyle;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "repomix-rs", version, about = "Pack your codebase into AI-friendly formats")]
struct Cli {
    /// Directory to pack
    root: Option<PathBuf>,

    /// Remote repository URL
    #[arg(long)]
    remote: Option<String>,

    /// Include patterns (comma-separated)
    #[arg(long)]
    include: Option<String>,

    /// Ignore patterns (comma-separated)
    #[arg(long)]
    ignore: Option<String>,

    /// Output style
    #[arg(long, default_value = "xml", value_enum)]
    style: CliOutputStyle,

    /// Enable Tree-sitter compression
    #[arg(long)]
    compress: bool,

    /// Remove comments
    #[arg(long)]
    remove_comments: bool,

    /// Remove empty lines
    #[arg(long)]
    remove_empty_lines: bool,

    /// Show line numbers
    #[arg(long)]
    line_numbers: bool,

    /// Truncate base64 data
    #[arg(long)]
    truncate_base64: bool,

    /// Copy to clipboard
    #[arg(long)]
    copy: bool,

    /// Initialize config file
    #[arg(long)]
    init: bool,

    /// MCP server mode
    #[arg(long)]
    mcp: bool,

    /// Output file path
    #[arg(long)]
    output: Option<String>,

    /// Include empty directories
    #[arg(long)]
    include_empty_directories: bool,

    /// Top files length for metrics
    #[arg(long)]
    top_files_length: Option<usize>,

    /// Split output when total tokens exceed this threshold (per-part token budget)
    #[arg(long)]
    split_output: Option<u64>,

    /// Header text to include in output
    #[arg(long)]
    header_text: Option<String>,

    /// Instruction file path
    #[arg(long)]
    instruction_file: Option<String>,

    /// Include git diffs
    #[arg(long)]
    include_diffs: bool,

    /// Include git logs
    #[arg(long)]
    include_logs: bool,

    /// Verbose logging
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

/// CLI 输出风格；无效值由 clap 拒绝并打印 usage。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum CliOutputStyle {
    Xml,
    Markdown,
    Plain,
    Json,
}

impl From<CliOutputStyle> for OutputStyle {
    fn from(s: CliOutputStyle) -> Self {
        match s {
            CliOutputStyle::Xml => OutputStyle::Xml,
            CliOutputStyle::Markdown => OutputStyle::Markdown,
            CliOutputStyle::Plain => OutputStyle::Plain,
            CliOutputStyle::Json => OutputStyle::Json,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // 初始化日志
    repomix_shared::logger::init_logger(cli.verbose > 0);
    
    // 处理初始化命令
    if cli.init {
        run::init_config().await?;
        return Ok(());
    }
    
    // 处理MCP服务器模式
    if cli.mcp {
        run::run_mcp_server().await?;
        return Ok(());
    }
    
    // 正常打包模式
    run::run_pack(cli).await?;
    
    Ok(())
}