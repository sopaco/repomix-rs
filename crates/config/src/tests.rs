//! 配置层回归测试。

use std::path::Path;

use crate::load::PartialConfig;
use crate::schema::RepomixConfig;

/// P0 修复（Bug #1）回归测试：CLI `--include` / `--ignore` 应追加
/// 到已有配置上，而不是覆盖全局/项目配置中已有的 include / ignore。
#[test]
fn test_merge_cli_appends_includes_bug1() {
    let mut config = RepomixConfig::default();
    config.include = vec!["*.rs".to_string(), "*.toml".to_string()];
    config.ignore.custom_ignore = vec!["target/**".to_string()];

    let overrides = PartialConfig {
        include: Some(vec!["Cargo.toml".to_string()]),
        ignore: Some(vec!["**/*.bak".to_string()]),
        ..Default::default()
    };

    config.merge_cli(overrides);

    // include 应是原有的 2 项 + CLI 新增 1 项
    assert_eq!(config.include.len(), 3, "include should be appended, not replaced");
    assert!(config.include.contains(&"*.rs".to_string()));
    assert!(config.include.contains(&"*.toml".to_string()));
    assert!(config.include.contains(&"Cargo.toml".to_string()));

    // ignore 同理
    assert_eq!(config.ignore.custom_ignore.len(), 2);
    assert!(config.ignore.custom_ignore.contains(&"target/**".to_string()));
    assert!(config.ignore.custom_ignore.contains(&"**/*.bak".to_string()));
}

/// P0 修复（Bug #1）回归测试：CLI 未传 include / ignore 时不污染配置。
#[test]
fn test_merge_cli_none_preserves_existing_bug1() {
    let mut config = RepomixConfig::default();
    config.include = vec!["*.rs".to_string()];
    config.ignore.custom_ignore = vec!["target/**".to_string()];

    let overrides = PartialConfig::default();
    config.merge_cli(overrides);

    assert_eq!(config.include, vec!["*.rs".to_string()]);
    assert_eq!(config.ignore.custom_ignore, vec!["target/**".to_string()]);
}

/// 验证默认 file_path 是 schema 默认值（用于 B8 回归测试前置）。
#[test]
fn test_default_file_path_is_bug8_baseline() {
    let config = RepomixConfig::default();
    assert_eq!(config.output.file_path, "repomix-output.txt");
}

/// 验证 `load` 在空 CWD 下不会 panic（用于回归测试 config 加载入口）。
#[test]
fn test_load_with_empty_cwd_does_not_panic() {
    let result = RepomixConfig::load(None, Path::new("/tmp"));
    // /tmp 存在即可；load 内部会尝试读 /tmp/repomix.config.json，
    // 没有就 Ok(None) → 用默认配置。
    assert!(result.is_ok());
}
