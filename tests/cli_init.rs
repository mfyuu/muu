use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[allow(deprecated)]
fn muu() -> Command {
    Command::cargo_bin("muu").unwrap()
}

#[test]
fn init_creates_file() {
    let dir = TempDir::new().unwrap();
    muu()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created muu.toml"));

    assert!(dir.path().join("muu.toml").exists());
}

#[test]
fn init_already_exists() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("muu.toml"), "[tasks]").unwrap();

    muu()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("muu.toml already exists"));
}
