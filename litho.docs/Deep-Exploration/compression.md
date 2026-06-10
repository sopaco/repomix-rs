# Deep-Exploration：Tree-sitter 代码压缩

代码压缩是 `repomix-rs` 最独特的卖点，也是区分它和"只是把一个目录拼成一个文本文件"的工具的分水岭。它用 tree-sitter 这一跨语言 AST 解析库，在保持代码调用签名和模块结构的前提下，批量删除函数和方法的实现体，把实现换成一行 "..."。结果是同样的代码体积，信息密度提升了数倍到数十倍，整个文件对 LLM 的理解负担大幅降低。

## 这个模块在做什么

`crates/core/src/tree_sitter/compress.rs` 封装了整个压缩操作。流程是：

1. 根据文件扩展名匹配到对应的 tree-sitter language grammar。
2. 把文件内容解析成一棵 AST。
3. 用预定义的 **tree-sitter query**（`.scm` 文件）找出所有函数签名、类定义、接口声明等节点。
4. 把这些节点的文本替换成摘要形式（通常是"签名 + `...`"），其余实现节点的内容直接移除或替换。
5. 将处理后的 AST 重新输出成源码文本。

压缩是可选的——只有用户传了 `--compress` 才触发。`crates/core/src/packager.rs` 在 step 7 的 `file::process::process_files` 里检查这个选项，再调用压缩逻辑。

## 核心组件

`crates/core/src/tree_sitter/languages.rs` 负责语言映射。它维护了一张从文件扩展名（如 `.rs`、`.py`、`.ts`）到 tree-sitter language 对象的查找表。当前支持 10 种语言， crate 依赖声明里可以清晰看到：`tree-sitter-typescript`, `tree-sitter-javascript`, `tree-sitter-python`, `tree-sitter-rust`, `tree-sitter-go`, `tree-sitter-java`, `tree-sitter-c`, `tree-sitter-cpp`, `tree-sitter-ruby`, `tree-sitter-php`。

`crates/core/src/tree_sitter/mod.rs` 是公共出口，把 `compress` 和 `languages` 两个子模块暴露给 `core` 的其他模块。

`.scm` 查询文件（`crates/core/src/tree_sitter/queries/`）不是一个 Rust 文件，而是 tree-sitter 自己的 S-expression 查询语言写成的模式匹配规则。每条规则瞄准一种代码结构：
- **函数签名**：`(function_declaration ...)` 或对应语言的等价节点。
- **类/类型定义**：`(class_declaration ...)`, `(struct_item ...)`, `(interface_item ...)` 等。
- **导出符号**：把导出的 API 保留，把内部实现删掉。

查询结果最终决定哪些源码片段被保留、哪些被替换成 "..."。

## 内部数据流

```text
RawFile { path, content }
        ↓
语言解析：match 扩展名 → language::parse(content) → SyntaxTree
        ↓
Query 执行：run 所有 .scm 查询 → matches
        ↓
文本替换：对命中节点保留"签名"，其余替换为 "..."
        ↓
ProcessedFile { path, content (压缩后), token_count }
```

压缩是**纯函数**变换：相同的输入文件和相同的选项总会产生相同的输出。没有缓存，没有全局可变状态，这意味着它可以被 rayon 的 `par_iter` 安全地并行调用。

## 扩展点

要新增一种语言的支持（例如 Kotlin、Scala、C# 等）：
1. 在 `core/Cargo.toml` 里加对应的 `tree-sitter-xxx` 依赖。
2. 在 `tree_sitter/languages.rs` 的映射表里注册扩展名。
3. 在 `tree_sitter/queries/` 里添加一个新的 `.scm` 查询文件，定义该语言的签名提取规则。

当前 C# 支持被注释掉了，是因为 `tree-sitter-c-sharp` 0.23 的 ABI 和仓库自己的 `queries/c_sharp.scm` 文件不兼容。注释里明确引用了 Bug #2 的修复编号，这是计划内的临时停用。

## 性能考量

- **tree-sitter 解析是 CPU 密集型的**，尤其是对于大文件。把数百 KB 的文件解析成 AST 然后遍历，单任务的耗时并不能忽略。得益于 rayon 的 `par_iter`，多个大文件可以并行解析，所以实际瓶颈往往是从磁盘读文件而非 AST 遍历本身。
- **查询语句（.scm 文件）的设计直接影响压缩率和语义保留**。过于激进的规则会把实现细节删过头，过于保守的规则会保留太多无用代码。当前仓库的 10 个查询文件都是手工调校过的，针对每种语言的最常见 API 结构。
- **tree-sitter 解析有最小粒度**：如果文件不包含任何函数或类定义（比如纯配置、纯 JSON），压缩步骤实际上什么都不做，直接把原始内容原样返回。这种情况下 token 数不会减少。

## 关键设计决策

为什么用 tree-sitter 而不是正则或者简单的行过滤？tree-sitter 的优势在于 **语言感知的结构解析**：它知道什么是函数、什么是类、什么是注释，而不是简单地"把 `fn` 到 `}` 之间的行删掉"。正则对缩进敏感，对嵌套结构脆弱，tree-sitter 则是基于 AST 的，基本不受缩进风格影响。只要 grammar 正确，解析结果就是可靠的。

另一个有意识的决定是**压缩和后续处理（注释删除、空行删除）是串行的管道**，而不是先把结果"压缩完"再统一做注释删除。原因是：tree-sitter 的注释删除基于 AST 节点，可以和签名提取在同一轮遍历里完成，减少一次全文件的字符串扫描。

---

## 置信度评分：7 / 10

> 说明：本模块的分析依赖对 `crates/core/src/tree_sitter/` 目录结构和 Cargo.toml 里的 tree-sitter 依赖声明的直接读取。`compress.rs` 和 `languages.rs` 的实际实现细节（处理函数签名、语言匹配逻辑）没有全量阅读，`.scm` 查询文件的具体规则也没有枚举。因此压缩粒度和语言覆盖保证细节存在一定的不确定性，扣 3 分。建议对 `compress.rs` 和至少一个 `.scm` 查询文件做针对性阅读，能修复大部分猜测。
