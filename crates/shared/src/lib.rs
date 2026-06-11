// 死代码清理：concurrency 模块（3个函数从未调用）和 error 模块（RepomixError 从未使用，全用 anyhow）已移除
pub mod logger;
// M12 清理：pattern_utils 为死代码，移除
pub mod types;
