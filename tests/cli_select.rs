use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[allow(deprecated)]
fn runz() -> Command {
    Command::cargo_bin("runz").unwrap()
}

#[test]
fn select_no_config() {
    let dir = TempDir::new().unwrap();
    runz()
        .arg("-l")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("no runz.toml or global config found"));
}

#[test]
fn select_no_tasks_defined() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("runz.toml"), "[tasks]\n").unwrap();

    runz()
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("no tasks defined"));
}

#[test]
fn select_non_tty_does_not_panic() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("runz.toml"),
        r#"
[tasks.hello]
cmd = "echo hello"
"#,
    )
    .unwrap();

    // In a non-TTY environment (CI, piped stdin), inquire will fail gracefully
    // We just verify it doesn't panic and exits with a non-zero code
    runz()
        .current_dir(dir.path())
        .assert()
        .failure();
}
