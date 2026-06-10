use repomix_core::packager::PackResult;

/// 打印打包报告
pub fn print_report(result: &PackResult) {
    println!("\n打包统计:");
    println!("  总文件数: {}", result.total_files);
    println!("  总字符数: {}", result.total_characters);
    println!("  总Token数: {}", result.total_tokens);

    if !result.top_files_by_tokens.is_empty() {
        println!("\nToken 数 Top {} 文件:", result.top_files_by_tokens.len());
        for (i, (path, tokens)) in result.top_files_by_tokens.iter().enumerate() {
            println!("  {:>2}. {} ({} tokens)", i + 1, path, tokens);
        }
    }

    if !result.suspicious_files.is_empty() {
        println!("\n安全警告:");
        for suspicious in &result.suspicious_files {
            println!("  - {}: {}", suspicious.path.display(), suspicious.message);
        }
    }

    if !result.skipped_files.is_empty() {
        println!("\n跳过的文件:");
        for skipped in &result.skipped_files {
            println!("  - {}: {}", skipped.path.display(), skipped.reason);
        }
    }

    if !result.output_paths.is_empty() {
        println!("\n输出文件:");
        for path in &result.output_paths {
            let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
            println!("  - {} ({} bytes)", path, size);
        }
    }
}
