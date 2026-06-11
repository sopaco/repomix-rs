use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn repomix() -> Command {
    Command::new(env!("CARGO_BIN_EXE_repomix"))
}

fn smoke_tmpdir() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/cli-smoke-tests")
        .join(format!("run_{nanos}"));
    fs::create_dir_all(&dir).expect("create cli smoke tmpdir");
    dir
}

#[test]
fn cli_help_succeeds() {
    repomix()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pack your codebase"));
}

#[test]
fn cli_pack_single_file_writes_output() {
    let tmp = smoke_tmpdir();
    let input = tmp.join("main.rs");
    fs::write(&input, "fn main() {}\n").unwrap();
    let output = tmp.join("packed.xml");

    repomix()
        .args([
            "--style",
            "xml",
            "--output",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("总文件数"));

    assert!(output.exists());
    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("fn main()"));

    let _ = fs::remove_dir_all(&tmp);
}
