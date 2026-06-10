use tracing::Level;
use tracing_subscriber::{fmt, EnvFilter};

/// 初始化全局 tracing subscriber
///
/// P3 修复（Bug #15）：用 try_init() 替代 init()，
/// 当此 crate 被嵌入已有 subscriber 的 host 应用时（如集成测试、库 API 场景），
/// 不会 panic，只是静默跳过。
pub fn init_logger(verbose: bool) {
    let level = if verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };

    let _ = fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(level.to_string())),
        )
        .try_init();
}
