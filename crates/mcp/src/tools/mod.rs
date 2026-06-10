//! MCP tool registry
//!
//! 在 `server.rs` 中通过 `#[tool_router]` 宏声明工具，
//! 此模块保留为对外暴露的「工具列表」常量，供文档/客户端发现使用。

/// 支持的 MCP 工具名称常量
pub const TOOL_PACK_CODEBASE: &str = "pack_codebase";
pub const TOOL_PACK_REMOTE: &str = "pack_remote_repository";
pub const TOOL_READ_OUTPUT: &str = "read_repomix_output";
pub const TOOL_GREP_OUTPUT: &str = "grep_repomix_output";

/// 获取所有工具 (name, description) 元数据
pub fn get_tool_definitions() -> Vec<(&'static str, &'static str)> {
    vec![
        (TOOL_PACK_CODEBASE, "Pack a local directory into AI-friendly format"),
        (TOOL_PACK_REMOTE, "Clone and pack a remote git repository"),
        (TOOL_READ_OUTPUT, "Read the contents of a repomix output file"),
        (TOOL_GREP_OUTPUT, "Search a repomix output file using a regex pattern"),
    ]
}
