use once_cell::sync::Lazy;
use std::collections::HashMap;
use tree_sitter::{Language, Query};

pub struct LanguageConfig {
    pub language: Language,
    pub compress_query: Option<Query>,
    pub extensions: Vec<&'static str>,
}

fn load_query(language: &Language, query_source: &str, lang_name: &str) -> Option<Query> {
    match Query::new(language, query_source) {
        Ok(query) => Some(query),
        Err(e) => {
            tracing::warn!(
                "Failed to load compress query for {}: {}. Compress will be disabled for this language.",
                lang_name,
                e
            );
            None
        }
    }
}

static LANGUAGE_REGISTRY: Lazy<HashMap<&'static str, LanguageConfig>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // TypeScript
    let ts_lang: Language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
    map.insert(
        "typescript",
        LanguageConfig {
            compress_query: load_query(
                &ts_lang,
                include_str!("queries/typescript.scm"),
                "typescript",
            ),
            language: ts_lang,
            extensions: vec!["ts", "tsx"],
        },
    );

    // JavaScript
    let js_lang: Language = tree_sitter_javascript::LANGUAGE.into();
    map.insert(
        "javascript",
        LanguageConfig {
            compress_query: load_query(
                &js_lang,
                include_str!("queries/javascript.scm"),
                "javascript",
            ),
            language: js_lang,
            extensions: vec!["js", "jsx"],
        },
    );

    // Python
    let py_lang: Language = tree_sitter_python::LANGUAGE.into();
    map.insert(
        "python",
        LanguageConfig {
            compress_query: load_query(&py_lang, include_str!("queries/python.scm"), "python"),
            language: py_lang,
            extensions: vec!["py"],
        },
    );

    // Rust
    let rust_lang: Language = tree_sitter_rust::LANGUAGE.into();
    map.insert(
        "rust",
        LanguageConfig {
            compress_query: load_query(&rust_lang, include_str!("queries/rust.scm"), "rust"),
            language: rust_lang,
            extensions: vec!["rs"],
        },
    );

    // Go
    let go_lang: Language = tree_sitter_go::LANGUAGE.into();
    map.insert(
        "go",
        LanguageConfig {
            compress_query: load_query(&go_lang, include_str!("queries/go.scm"), "go"),
            language: go_lang,
            extensions: vec!["go"],
        },
    );

    // Java
    let java_lang: Language = tree_sitter_java::LANGUAGE.into();
    map.insert(
        "java",
        LanguageConfig {
            compress_query: load_query(&java_lang, include_str!("queries/java.scm"), "java"),
            language: java_lang,
            extensions: vec!["java"],
        },
    );

    // C
    let c_lang: Language = tree_sitter_c::LANGUAGE.into();
    map.insert(
        "c",
        LanguageConfig {
            compress_query: load_query(&c_lang, include_str!("queries/c.scm"), "c"),
            language: c_lang,
            extensions: vec!["c", "h"],
        },
    );

    // C++
    let cpp_lang: Language = tree_sitter_cpp::LANGUAGE.into();
    map.insert(
        "cpp",
        LanguageConfig {
            compress_query: load_query(&cpp_lang, include_str!("queries/cpp.scm"), "cpp"),
            language: cpp_lang,
            extensions: vec!["cpp", "cxx", "cc", "hpp", "hxx"],
        },
    );

    // Ruby
    let ruby_lang: Language = tree_sitter_ruby::LANGUAGE.into();
    map.insert(
        "ruby",
        LanguageConfig {
            compress_query: load_query(&ruby_lang, include_str!("queries/ruby.scm"), "ruby"),
            language: ruby_lang,
            extensions: vec!["rb"],
        },
    );

    // PHP
    let php_lang: Language = tree_sitter_php::LANGUAGE_PHP.into();
    map.insert(
        "php",
        LanguageConfig {
            compress_query: load_query(&php_lang, include_str!("queries/php.scm"), "php"),
            language: php_lang,
            extensions: vec!["php"],
        },
    );

    // Swift
    let swift_lang: Language = tree_sitter_swift::LANGUAGE.into();
    map.insert(
        "swift",
        LanguageConfig {
            compress_query: load_query(
                &swift_lang,
                include_str!("queries/swift.scm"),
                "swift",
            ),
            language: swift_lang,
            extensions: vec!["swift"],
        },
    );

    // Kotlin
    let kotlin_lang: Language = tree_sitter_kotlin_ng::LANGUAGE.into();
    map.insert(
        "kotlin",
        LanguageConfig {
            compress_query: load_query(
                &kotlin_lang,
                include_str!("queries/kotlin.scm"),
                "kotlin",
            ),
            language: kotlin_lang,
            extensions: vec!["kt", "kts"],
        },
    );

    // Dart
    let dart_lang: Language = tree_sitter_dart_orchard::LANGUAGE.into();
    map.insert(
        "dart",
        LanguageConfig {
            compress_query: load_query(&dart_lang, include_str!("queries/dart.scm"), "dart"),
            language: dart_lang,
            extensions: vec!["dart"],
        },
    );

    // Vue (SFC - uses JavaScript/TypeScript for script sections)
    let vue_lang: Language = tree_sitter_vue_updated::language();
    map.insert(
        "vue",
        LanguageConfig {
            compress_query: load_query(&vue_lang, include_str!("queries/vue.scm"), "vue"),
            language: vue_lang,
            extensions: vec!["vue"],
        },
    );
    // TODO: Find a compatible Vue parser or upgrade tree-sitter version

    // Svelte (uses JavaScript/TypeScript for script sections)
    let svelte_lang: Language = tree_sitter_svelte_ng::LANGUAGE.into();
    map.insert(
        "svelte",
        LanguageConfig {
            compress_query: load_query(
                &svelte_lang,
                include_str!("queries/svelte.scm"),
                "svelte",
            ),
            language: svelte_lang,
            extensions: vec!["svelte"],
        },
    );

    map
});

pub fn get_language_config(file_path: &std::path::Path) -> Option<&'static LanguageConfig> {
    let extension = file_path.extension()?.to_str()?;

    LANGUAGE_REGISTRY
        .values()
        .find(|config| config.extensions.contains(&extension))
}

/// 获取所有支持的语言
pub fn get_supported_languages() -> Vec<&'static str> {
    LANGUAGE_REGISTRY.keys().copied().collect()
}
