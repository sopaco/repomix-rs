// Integration test for new language support
use repomix_core::tree_sitter::compress::compress_file;
use repomix_core::tree_sitter::languages::get_language_config;
use std::path::Path;

fn test_compression(lang: &str, ext: &str, code: &str) -> bool {
    let filename = format!("test.{}", ext);
    let path = Path::new(&filename);
    let config = match get_language_config(path) {
        Some(cfg) => cfg,
        None => {
            println!("✗ {} not registered in language config", lang);
            return false;
        }
    };

    match compress_file(code, path, config) {
        Ok(Some(result)) => {
            println!(
                "✓ {} compression succeeded ({} bytes)",
                lang,
                result.len()
            );
            if result.len() < 200 {
                println!("  Output: {}", result);
            } else {
                println!("  Output preview: {}...", &result[..200]);
            }
            true
        }
        Ok(None) => {
            println!("⚠ {} compression returned None", lang);
            false
        }
        Err(e) => {
            println!("✗ {} compression failed: {}", lang, e);
            false
        }
    }
}

#[test]
fn test_swift_compression() {
    let swift_code = r#"
import Foundation

class ViewController: UIViewController {
    override func viewDidLoad() {
        super.viewDidLoad()
        print("Hello")
    }

    func fetchData() async throws -> Data {
        let url = URL(string: "https://api.example.com")!
        let (data, _) = try await URLSession.shared.data(from: url)
        return data
    }
}

struct User {
    let id: Int
    let name: String
}
"#;
    assert!(test_compression("Swift", "swift", swift_code));
}

#[test]
fn test_vue_compression() {
    let vue_code = r#"
<template>
  <div class="app">
    <h1>{{ title }}</h1>
    <button @click="increment">Count: {{ count }}</button>
  </div>
</template>

<script>
import { ref } from 'vue';

export default {
  setup() {
    const count = ref(0);
    const title = 'Vue App';
    
    function increment() {
      count.value++;
    }
    
    return { count, title, increment };
  }
}
</script>

<style scoped>
.app {
  padding: 20px;
}
</style>
"#;
    assert!(test_compression("Vue", "vue", vue_code));
}

#[test]
fn test_kotlin_compression() {
    // Use top-level declarations only (compress.rs skips nested captures)
    let kotlin_code = r#"
class MainActivity

interface UserRepository

object AppContext

fun topLevelFunction(): String {
    return "Hello"
}
"#;
    assert!(test_compression("Kotlin", "kt", kotlin_code));
}

#[test]
fn test_kotlin_query_compiles() {
    use tree_sitter::{Language, Query};
    let lang: Language = tree_sitter_kotlin_ng::LANGUAGE.into();
    let query_src = include_str!("../src/tree_sitter/queries/kotlin.scm");
    
    match Query::new(&lang, query_src) {
        Ok(_) => println!("✓ Kotlin query compiled successfully"),
        Err(e) => {
            println!("✗ Kotlin query compilation failed: {:?}", e);
            panic!("Query compilation failed");
        }
    }
}

#[test]
fn test_dart_compression() {
    let dart_code = r#"
import 'package:flutter/material.dart';

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        body: Center(child: Text('Hello')),
      ),
    );
  }
}

class User {
  final int id;
  final String name;
  User(this.id, this.name);
}
"#;
    assert!(test_compression("Dart", "dart", dart_code));
}

#[test]
fn test_svelte_compression() {
    let svelte_code = r#"
<script>
  import { onMount } from 'svelte';

  let count = 0;

  function increment() {
    count += 1;
  }

  onMount(() => {
    console.log('Component mounted');
  });
</script>

<button on:click={increment}>
  Clicks: {count}
</button>
"#;
    assert!(test_compression("Svelte", "svelte", svelte_code));
}
