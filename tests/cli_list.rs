use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[allow(deprecated)]
fn muu() -> Command {
    Command::cargo_bin("muu").unwrap()
}

#[test]
fn list_shows_tasks() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("muu.toml"),
        r#"
[tasks.hello]
cmd = "echo hello"
description = "Say hello"

[tasks.build]
cmd = "cargo build"
description = "Build the project"
"#,
    )
    .unwrap();

    muu()
        .arg("list")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"))
        .stdout(predicate::str::contains("Say hello"))
        .stdout(predicate::str::contains("[local]"))
        .stdout(predicate::str::contains("build"))
        .stdout(predicate::str::contains("Build the project"));
}

#[test]
fn list_no_config() {
    let dir = TempDir::new().unwrap();
    muu()
        .arg("list")
        .arg("-l")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("no muu.toml or global config found"));
}

#[test]
fn list_local_only_flag() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("muu.toml"),
        r#"
[tasks.local_task]
cmd = "echo local"
description = "Local task"
"#,
    )
    .unwrap();

    muu()
        .arg("-l")
        .arg("list")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("local_task"))
        .stdout(predicate::str::contains("[local]"));
}
