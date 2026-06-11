use std::collections::HashMap;

/// 树节点结构
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub children: Vec<TreeNode>,
    pub is_directory: bool,
}

impl TreeNode {
    fn new(name: &str, is_directory: bool) -> Self {
        Self {
            name: name.to_string(),
            children: Vec::new(),
            is_directory,
        }
    }
}

/// 从文件路径列表生成目录树
pub fn generate_file_tree(file_paths: &[String], empty_dir_paths: &[String]) -> TreeNode {
    let mut root = TreeNode::new("root", true);

    for path in file_paths {
        add_path_to_tree(&mut root, path, false);
    }

    for dir in empty_dir_paths {
        add_path_to_tree(&mut root, dir, true);
    }

    root
}

/// 添加路径到树中
fn add_path_to_tree(root: &mut TreeNode, path: &str, is_directory: bool) {
    let separator = if path.contains('\\') { '\\' } else { '/' };
    let parts: Vec<&str> = path.split(separator).collect();
    let mut current_node = root;

    for (i, part) in parts.iter().enumerate() {
        let is_last_part = i == parts.len() - 1;
        let child_is_dir = !is_last_part || is_directory;

        // 查找或创建子节点
        let child_index = current_node.children.iter().position(|c| c.name == *part);

        if let Some(index) = child_index {
            current_node = &mut current_node.children[index];
        } else {
            let new_child = TreeNode::new(part, child_is_dir);
            current_node.children.push(new_child);
            let last_index = current_node.children.len() - 1;
            current_node = &mut current_node.children[last_index];
        }
    }
}

/// 对树节点进行排序（目录在前，按字母顺序）
fn sort_tree_nodes(node: &mut TreeNode) {
    node.children.sort_by(|a, b| {
        if a.is_directory == b.is_directory {
            a.name.cmp(&b.name)
        } else if a.is_directory {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    for child in &mut node.children {
        sort_tree_nodes(child);
    }
}

/// 将树转换为字符串
pub fn tree_to_string(node: &TreeNode, prefix: &str, is_root: bool) -> String {
    if is_root {
        let mut sorted_node = node.clone();
        sort_tree_nodes(&mut sorted_node);
        return tree_to_string_inner(&sorted_node, "", false);
    }
    tree_to_string_inner(node, prefix, false)
}

fn tree_to_string_inner(node: &TreeNode, prefix: &str, _is_root: bool) -> String {
    let mut result = String::new();

    for child in &node.children {
        result.push_str(&format!(
            "{}{}{}\n",
            prefix,
            child.name,
            if child.is_directory { "/" } else { "" }
        ));
        if child.is_directory {
            let new_prefix = format!("{}  ", prefix);
            result.push_str(&tree_to_string_inner(child, &new_prefix, false));
        }
    }

    result
}

/// 将树转换为带行数的字符串
pub fn tree_to_string_with_line_counts(
    node: &TreeNode,
    line_counts: &HashMap<String, usize>,
    prefix: &str,
    current_path: &str,
    is_root: bool,
) -> String {
    if is_root {
        let mut sorted_node = node.clone();
        sort_tree_nodes(&mut sorted_node);
        return tree_to_string_with_line_counts_inner(&sorted_node, line_counts, "", "", false);
    }
    tree_to_string_with_line_counts_inner(node, line_counts, prefix, current_path, false)
}

fn tree_to_string_with_line_counts_inner(
    node: &TreeNode,
    line_counts: &HashMap<String, usize>,
    prefix: &str,
    current_path: &str,
    _is_root: bool,
) -> String {
    let mut result = String::new();

    for child in &node.children {
        let child_path = if current_path.is_empty() {
            child.name.clone()
        } else {
            format!("{}/{}", current_path, child.name)
        };

        if child.is_directory {
            result.push_str(&format!("{}/{}/\n", prefix, child.name));
            let new_prefix = format!("{}  ", prefix);
            result.push_str(&tree_to_string_with_line_counts_inner(
                child,
                line_counts,
                &new_prefix,
                &child_path,
                false,
            ));
        } else {
            let line_count = line_counts.get(&child_path);
            let line_count_suffix = match line_count {
                Some(count) => format!(" ({} lines)", count),
                None => String::new(),
            };
            result.push_str(&format!("{}{}{}\n", prefix, child.name, line_count_suffix));
        }
    }

    result
}

/// 生成目录树字符串
pub fn generate_tree_string(file_paths: &[String], empty_dir_paths: &[String]) -> String {
    let tree = generate_file_tree(file_paths, empty_dir_paths);
    tree_to_string(&tree, "", true).trim().to_string()
}

/// 生成带行数的目录树字符串
pub fn generate_tree_string_with_line_counts(
    file_paths: &[String],
    line_counts: &HashMap<String, usize>,
    empty_dir_paths: &[String],
) -> String {
    let tree = generate_file_tree(file_paths, empty_dir_paths);
    tree_to_string_with_line_counts(&tree, line_counts, "", "", true)
        .trim()
        .to_string()
}

/// 计算文件行数
pub fn calculate_file_line_counts(
    file_paths: &[String],
    contents: &[String],
) -> HashMap<String, usize> {
    let mut line_counts = HashMap::new();

    for (path, content) in file_paths.iter().zip(contents.iter()) {
        let count = if content.is_empty() {
            0
        } else {
            let newline_count = content.matches('\n').count();
            if content.ends_with('\n') {
                newline_count
            } else {
                newline_count + 1
            }
        };
        line_counts.insert(path.clone(), count);
    }

    line_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tree() {
        let file_paths = vec![
            "src/main.rs".to_string(),
            "src/lib.rs".to_string(),
            "Cargo.toml".to_string(),
        ];
        let empty_dirs = vec![];

        let tree = generate_file_tree(&file_paths, &empty_dirs);
        assert_eq!(tree.children.len(), 2); // src/ and Cargo.toml
    }

    #[test]
    fn test_tree_to_string() {
        let file_paths = vec![
            "src/main.rs".to_string(),
            "src/lib.rs".to_string(),
            "Cargo.toml".to_string(),
        ];
        let empty_dirs = vec![];

        let tree = generate_file_tree(&file_paths, &empty_dirs);
        let result = tree_to_string(&tree, "", true);

        assert!(result.contains("src/"));
        assert!(result.contains("Cargo.toml"));
    }

    #[test]
    fn test_generate_tree_string() {
        let file_paths = vec![
            "src/main.rs".to_string(),
            "src/lib.rs".to_string(),
            "Cargo.toml".to_string(),
        ];
        let empty_dirs = vec![];

        let result = generate_tree_string(&file_paths, &empty_dirs);
        assert!(result.contains("src/"));
        assert!(result.contains("Cargo.toml"));
    }
}
