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

## Directory Structure

```
lib.rs
```

## Files

### lib.rs (34 lines)

```
pub mod packager;
pub mod file;
pub mod security;
pub mod output;
pub mod metrics;
pub mod git;
pub mod tree_sitter;
pub mod path_util;

// Re-export config crate for convenience
pub use repomix_config as config;

// Re-export public API
pub use packager::{pack, PackResult, PackOptions, NoopProgress, ProgressCallback};
pub use repomix_config::schema::{RepomixConfig, OutputStyle};
pub use repomix_config::load::PartialConfig;
pub use repomix_shared::types::*;

/// 便捷函数：一行代码打包仓库
pub async fn pack_directory(dir: &str) -> anyhow::Result<PackResult> {
    let options = PackOptions::new(std::path::PathBuf::from(dir));
    pack(options.root_dirs, options.config, Box::new(NoopProgress)).await
}

/// 便捷函数：自定义配置打包
pub async fn pack_with_config(dir: &str, config: RepomixConfig) -> anyhow::Result<PackResult> {
    let options = PackOptions::new(std::path::PathBuf::from(dir)).with_config(config);
    pack(options.root_dirs, options.config, Box::new(NoopProgress)).await
}

/// 便捷函数：使用 PackOptions 打包
pub async fn pack_with_options(options: PackOptions) -> anyhow::Result<PackResult> {
    pack(options.root_dirs, options.config, Box::new(NoopProgress)).await
}
```

