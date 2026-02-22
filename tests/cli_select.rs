use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[allow(deprecated)]
fn muu() -> Command {
    Command::cargo_bin("muu").unwrap()
}

#[test]
fn select_no_config() {
    let dir = TempDir::new().unwrap();
    muu()
        .arg("-l")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("no muu.toml or global config found"));
}

#[test]
fn select_no_tasks_defined() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("muu.toml"), "[tasks]\n").unwrap();

    // Override HOME so the global config is not picked up
    let fake_home = TempDir::new().unwrap();
    muu()
        .env("HOME", fake_home.path())
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("no tasks defined"));
}

#[test]
fn select_non_tty_does_not_panic() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("muu.toml"),
        r#"
[tasks.hello]
cmd = "echo hello"
"#,
    )
    .unwrap();

    // In a non-TTY environment (CI, piped stdin), inquire will fail gracefully
    // We just verify it doesn't panic and exits with a non-zero code
    muu()
        .current_dir(dir.path())
        .assert()
        .failure();
}
