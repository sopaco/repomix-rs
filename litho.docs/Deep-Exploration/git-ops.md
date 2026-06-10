# Deep-Exploration：Git 操作

Git 操作模块是 `repomix-rs` 的「时间维度」入口。它让打包结果不只是某个时间点的静态快照，还能承载「文件在最近被改了哪些地方」和「这个改动在 git 历史里是什么」的上下文。

实现上**不依赖 libgit2 或 `git2` crate**，而是通过 `std::process::Command` 调用系统 `git` 可执行文件。无需任何 Cargo feature；只要 `PATH` 上有 `git` 且目标目录是 git 仓库（远程克隆除外），相关功能即可用。

## 这个模块在做什么

`crates/core/src/git/` 包含四个子模块：

| 模块 | 职责 |
|---|---|
| `remote` | `git clone` 克隆远程仓库；`git rev-parse` 判断是否为 git 仓库 |
| `sort` | 根据 `git log --name-only` 统计变更频率，重排 `Vec<ProcessedFile>` |
| `diff` | 获取工作区 diff 与暂存区 diff |
| `log` | 获取最近 N 条 `git log --oneline` |

路径匹配辅助逻辑在 `crates/core/src/path_util.rs`：`git_repo_root()` 解析仓库根目录，`git_relative_path()` 将绝对路径转为与 `git log --name-only` 一致的相对路径键（含 macOS `/var` vs `/private/var` 的 `canonicalize` 处理）。

## 核心组件与流程

### 远程仓库克隆（`git::remote`）

CLI `--remote` 与 MCP `pack_remote_repository` 共用此逻辑：

1. 用 `PID + 纳秒 + 哈希` 生成唯一临时目录。
2. 执行 `git clone <url> <temp_dir>`（完整克隆，非浅克隆）。
3. `pack()` 以临时目录为根目录打包。
4. `TempDirGuard` 在退出时 `remove_dir_all` 清理临时目录。

克隆失败（网络、认证、仓库不存在）会返回 `anyhow::Error`，整个打包中止。

### 按变更排序（`git::sort::sort_by_git_changes`）

1. 执行 `git log -N --pretty=format: --name-only`，统计每个相对路径在最近 N 次提交中出现的次数。
2. 用 `git_repo_root` + `git_relative_path` 将 `ProcessedFile.path`（绝对路径）映射为 git 相对路径。
3. 按出现次数降序排列（变更越频繁的文件越靠前）。

默认 `sort_by_changes_max_commits = 100`。`sort_by_changes` 默认为 `true`，但仅在 `is_git_repo()` 为真时执行。失败时 `packager` 打印 `tracing::warn` 并继续。

### Git diff（`git::diff::get_git_diffs`）

分别执行：

- `git diff` → `work_tree`（工作区未暂存改动）
- `git diff --cached` → `staged`（已暂存改动）

任一条命令非零退出码会返回 `Err`；`packager` 捕获后打警告，diff 段落跳过。

### Git log（`git::log::get_git_logs`）

执行 `git log -N --oneline`，每行一条记录。由 `include_logs_count`（默认 50）控制条数。

## 内部数据流

```text
（可选）pack_remote_repository 或 --remote
    ↓
git::remote::clone_remote_repo(url, temp_dir)
    ↓
core::pack(root_dir)

可选分支（pack() 内部，需 is_git_repo）：
    sort_by_changes  → git::sort::sort_by_git_changes
    include_diffs    → git::diff::get_git_diffs
    include_logs     → git::log::get_git_logs
    ↓
output::generate::produce_output(..., git_diff, git_log)
```

`sort` 在 `process_files` 之后、`filter_suspicious` 之前执行。`diff` / `log` 在生成输出之前获取，以便写入最终文件。

## 配置入口

`config.output.git`（`RepomixConfig`）：

| 字段 | 默认 | 说明 |
|---|---|---|
| `sort_by_changes` | `true` | 按 git 变更频率排序文件 |
| `sort_by_changes_max_commits` | `100` | 参与统计的最近提交数 |
| `include_diffs` | `false` | 在输出中附加 diff |
| `include_logs` | `false` | 在输出中附加 log |
| `include_logs_count` | `50` | log 条数上限 |

CLI：`--include-diffs`、`--include-logs`（需系统 `git` 与 `.git` 目录）。

## 设计考量

**为什么用系统 `git` 而不是 libgit2？** 早期设计曾考虑 optional `git2` crate，但实际实现始终通过 shell 调用 `git`，且 `git2` 从未被代码引用。移除该依赖后：二进制更小、无 libgit2 ABI 问题、用户只需安装 Git 即可使用全部 git 相关功能。

**为什么 diff/log 在 output 生成之前？** 产物需要计入 token 指标并写入各输出格式（XML / Markdown / JSON 等）的专用段落。

**为什么 sort 在 security 过滤之前？** 当前实现在 `filter_suspicious` 之前排序；被 Secretlint 标记的文件随后从列表中剔除，排序结果对最终输出中保留的文件仍然有效。

## 前置条件

- 安装 [Git](https://git-scm.com/)，确保 `git` 在 `PATH` 中。
- 本地打包的 git 功能要求目标目录（或其父目录）为 git 仓库。
- `--remote` / `pack_remote_repository` 需要网络可达的远程 URL 与有效的 clone 权限。
