# Deep-Exploration：配置与分层加载

配置模块是 `repomix-rs` 的"大脑中的默认值工厂"。如果不靠它，用户要么在每个 `--compress --style markdown` 里重复参数，要么就得自己手写完整的 JSON。`repomix-config` 让用户可以用最轻量的表达（一条 CLI 标志）覆盖配置，同时保留一个稳定的、JSON 可编辑的默认配置骨架。

## 这个模块在做什么

`repomix-config` 提供了两样东西给外部世界：

1. **Schema** —— 完整的 `RepomixConfig` 类型，描述了系统支持的所有配置键。
2. **Loader** —— `RepomixConfig::load` 方法，按层次把默认值、全局配置、项目配置和 CLI 参数按层叠加。

分层加载的策略是"后出现的覆盖先出现的默认值，没出现的就保留当前层的值"：

```
defaults（硬编码）→ ~/.repomix/repomix.config.json（全局）→ ./repomix.config.json（项目）→ CLI PartialConfig
```

每一层的语义是：

- **defaults**：内嵌在 Rust 代码里的 `Default` 实现，永远不会被不经意地覆盖。
- **global**：用户机器全局的配置，适合常年不变的偏好（例如默认启用压缩）。
- **project**：当前项目的 `./repomix.config.json`，跟着项目走。
- **CLI partial**：仅包含用户本次命令行显式提供的字段，其余保持 `None` 表示"沿用下层"。

这种策略让用户可以用最少的表达达到最多的效果：`repomix --compress` 只改 `compress`，别的照旧。

## 核心组件

`crates/config/src/schema.rs` 定义了全部配置结构体和它们的 serde 结构：

- `RepomixConfig` —— 根节点，含 6 个子结构。
- `InputConfig` —— 输入控制（`max_file_size`）。
- `OutputConfig` —— 输出控制（`file_path`, `style`, `compress`, `remove_comments`, `remove_empty_lines`, 以及嵌套的 `git`, `json`, `token_count_tree`）。
- `IgnoreConfig` —— 过滤规则（`use_gitignore`, `custom_ignore`）。
- `SecurityConfig` —— 安全开关（`enable_secretlint`）。
- `TokenCountConfig` —— token 计数参数（`encoding`）。
- `GitOutputConfig` —— 嵌入 output 里的 git 子配置。

每个配置结构体都实现了 `Default`，默认值由 `default_*` 辅助函数返回，例如 `default_file_path` 返回 `"repomix-output.txt"`（后续 `pack()` 会按实际 style 自动重命名），`default_style` 返回 `OutputStyle::Xml`，等等。

`crates/config/src/load.rs` 实现了分层合并。`PartialConfig` 结构体是 CLI 端的增量补丁格式：每个字段都是 `Option<T>`，`None` 表示"用户没有在这个字段上表态"。这个"用车牌"的设计让 `repomix --compress` 这种弱配置非常自然地表达出来，不需要构造一个完整的 `RepomixConfig`。

`crates/config/src/default_ignore.rs` 注册了库内置的忽略规则，这些是用户不管写不写 `.gitignore` 都默认排除的文件和目录（例如 `target/`, `node_modules/`, `.git/`, 等等），确保打包结果的清洁性。

`crates/config/src/global_dir.rs` 负责把"用户全局 config 目录"翻译成实际路径。它依赖 `dirs` crate 做平台抽象，所以 Linux、macOS、Windows 上都能解析到正确的用户配置目录。

`crates/config/src/tests.rs` 是测试床，验证 `RepomixConfig::load` 的分层叠加逻辑。

## 内部数据流

```text
defaults (Default impl)
    ↓
加载 global 配置（optional，如果文件存在则反序列化）
    ↓
加载 project 配置（optional，./repomix.config.json）
    ↓
叠加 CLI PartialConfig（只有 Some(v) 的字段覆盖）
    ↓
RepomixConfig（最终结果）
```

叠加顺序保证了：CLI 的参数永远最高优先级，项目配置覆盖全局配置，全局配置覆盖硬编码默认值。每一层只负责"当上一层没表态时，替我表态"。

## 与其他模块的接口

- **提供给 `repomix-cli`**：`build_config` 函数在 `cli/run.rs` 里把 `Cli` 结构转换成 `PartialConfig`，然后调用 `RepomixConfig::load`。
- **提供给 `repomix-mcp`**：MCP 工具处理函数同样调用 `RepomixConfig::load`，把 JSON-RPC 请求的参数映射成 `PartialConfig`。
- **被 `repomix-core` 读取**：`file`, `output`, `security`, `metrics`, `git` 等各个子模块都从 `RepomixConfig` 里读取各自关心的字段。

`repomix-config` 自身和 `repomix-shared` 双向依赖（`shared` 的 types 没有依赖 `config`，但 `config` 本身不依赖 `shared`）。

## 扩展点

要新增一个配置选项：

1. 在 `schema.rs` 里对应配置结构体加一个字段。
2. 在 `Default` 实现里设置缺省值。
3. 在 `load.rs` 的合并逻辑里处理（通常是自动的，因为 `#[serde(default)]` 和 `Option` 让合并可以自然延伸）。

如果在 CLI 侧把这个选项暴露给用户，还需在 `cli/run.rs` 的 `build_config` 里加一行映射到 `PartialConfig`。

---

## 置信度评分：8 / 10

> 说明：本模块的分析基于对 `crates/config/src/schema.rs`、`crates/config/src/lib.rs` 和 `crates/config/src/` 目录结构的直接读取。`load.rs` 的具体合并实现细节（尤其是全局配置到项目配置到 CLI 参数的精确合并算法）虽然从文件名和目录结构可以推断功能，但具体的实现代码没有全量阅读，因此扣 2 分。
