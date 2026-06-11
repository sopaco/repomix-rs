use console::style;
use dialoguer::{Confirm, Select};
use repomix_config::schema::{OutputStyle, RepomixConfig};
use std::fs;
use std::path::Path;

/// 交互式配置提示
pub fn prompt_for_config(_root_dir: &Path) -> RepomixConfig {
    println!();
    println!("{}", style("Welcome to Repomix Configuration!").bold());
    println!();

    let mut config = RepomixConfig::default();

    // 选择输出格式
    let output_styles = vec!["XML", "Markdown", "JSON", "Plain"];
    let output_style_index = Select::new()
        .with_prompt("Select output style")
        .items(&output_styles)
        .default(0)
        .interact()
        .unwrap_or(0);

    config.output.style = match output_style_index {
        0 => OutputStyle::Xml,
        1 => OutputStyle::Markdown,
        2 => OutputStyle::Json,
        3 => OutputStyle::Plain,
        _ => OutputStyle::Xml,
    };

    // 设置输出文件路径
    let default_path = match config.output.style {
        OutputStyle::Xml => "repomix-output.xml",
        OutputStyle::Markdown => "repomix-output.md",
        OutputStyle::Json => "repomix-output.json",
        OutputStyle::Plain => "repomix-output.txt",
    };
    config.output.file_path = default_path.to_string();

    // 询问是否启用目录结构
    config.output.directory_structure = Confirm::new()
        .with_prompt("Include directory structure in output?")
        .default(true)
        .interact()
        .unwrap_or(true);

    // 询问是否启用文件内容
    config.output.files = Confirm::new()
        .with_prompt("Include file contents in output?")
        .default(true)
        .interact()
        .unwrap_or(true);

    // 询问是否移除注释
    config.output.remove_comments = Confirm::new()
        .with_prompt("Remove comments from code?")
        .default(false)
        .interact()
        .unwrap_or(false);

    // 询问是否显示行号
    config.output.show_line_numbers = Confirm::new()
        .with_prompt("Show line numbers?")
        .default(false)
        .interact()
        .unwrap_or(false);

    // 询问是否启用压缩
    config.output.compress = Confirm::new()
        .with_prompt("Enable Tree-sitter compression?")
        .default(false)
        .interact()
        .unwrap_or(false);

    config
}

/// 创建配置文件
pub fn create_config_file(root_dir: &Path) -> bool {
    let config_path = root_dir.join("repomix.config.json");

    if config_path.exists() {
        let overwrite = Confirm::new()
            .with_prompt(format!(
                "{} already exists. Overwrite?",
                style("repomix.config.json").green()
            ))
            .default(false)
            .interact()
            .unwrap_or(false);

        if !overwrite {
            println!("{}", style("Skipping repomix.config.json creation.").dim());
            return false;
        }
    }

    let config = prompt_for_config(root_dir);
    let config_json = serde_json::to_string_pretty(&config).unwrap_or_default();

    match fs::write(&config_path, &config_json) {
        Ok(_) => {
            println!(
                "{}\n{}",
                style("Config file created!").green(),
                style(format!("Path: {}", config_path.display())).dim()
            );
            true
        }
        Err(e) => {
            println!(
                "{} Failed to create config file: {}",
                style("Error:").red(),
                e
            );
            false
        }
    }
}

/// 创建 .repomixignore 文件
pub fn create_ignore_file(root_dir: &Path) -> bool {
    let ignore_path = root_dir.join(".repomixignore");

    if ignore_path.exists() {
        let overwrite = Confirm::new()
            .with_prompt(format!(
                "{} already exists. Overwrite?",
                style(".repomixignore").green()
            ))
            .default(false)
            .interact()
            .unwrap_or(false);

        if !overwrite {
            println!("{}", style("Skipping .repomixignore creation.").dim());
            return false;
        }
    }

    let default_content = r#"# Add patterns to ignore here, one per line
# Example:
# *.log
# tmp/
"#;

    match fs::write(&ignore_path, default_content) {
        Ok(_) => {
            println!(
                "{}\n{}",
                style("Created .repomixignore file!").green(),
                style(format!("Path: {}", ignore_path.display())).dim()
            );
            true
        }
        Err(e) => {
            println!(
                "{} Failed to create .repomixignore file: {}",
                style("Error:").red(),
                e
            );
            false
        }
    }
}
