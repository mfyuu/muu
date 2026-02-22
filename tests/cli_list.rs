use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[allow(deprecated)]
fn runz() -> Command {
    Command::cargo_bin("runz").unwrap()
}

#[test]
fn list_shows_tasks() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("runz.toml"),
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

    runz()
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
    runz()
        .arg("list")
        .arg("-l")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("no runz.toml or global config found"));
}

#[test]
fn list_local_only_flag() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("runz.toml"),
        r#"
[tasks.local_task]
cmd = "echo local"
description = "Local task"
"#,
    )
    .unwrap();

    runz()
        .arg("-l")
        .arg("list")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("local_task"))
        .stdout(predicate::str::contains("[local]"));
}
