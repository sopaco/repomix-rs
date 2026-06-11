use tracing::Level;
use tracing_subscriber::{fmt, EnvFilter};

/// 初始化全局 tracing subscriber
///
/// 使用 `try_init()` 而非 `init()`，避免在已有 subscriber 的 host 应用中 panic。
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
