use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[allow(deprecated)]
fn runz() -> Command {
    Command::cargo_bin("runz").unwrap()
}

#[test]
fn init_creates_file() {
    let dir = TempDir::new().unwrap();
    runz()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created runz.toml"));

    assert!(dir.path().join("runz.toml").exists());
}

#[test]
fn init_already_exists() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("runz.toml"), "[tasks]").unwrap();

    runz()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("runz.toml already exists"));
}
