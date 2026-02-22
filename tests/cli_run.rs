use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[allow(deprecated)]
fn muu() -> Command {
    Command::cargo_bin("muu").unwrap()
}

#[test]
fn run_simple_task() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("muu.toml"),
        r#"
[tasks.hello]
cmd = "echo hello"
"#,
    )
    .unwrap();

    muu()
        .arg("hello")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn run_with_positional_args() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("muu.toml"),
        r#"
[tasks.greet]
cmd = "echo $name $greeting"
args = { name = "", greeting = "hello" }
"#,
    )
    .unwrap();

    muu()
        .args(["greet", "Alice"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice hello"));
}

#[test]
fn run_with_named_args() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("muu.toml"),
        r#"
[tasks.greet]
cmd = "echo $name $greeting"
args = { name = "", greeting = "hello" }
"#,
    )
    .unwrap();

    muu()
        .args(["greet", "--name=Bob", "--greeting=hi"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Bob hi"));
}

#[test]
fn run_missing_required_arg() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("muu.toml"),
        r#"
[tasks.greet]
cmd = "echo $name"
args = { name = "" }
"#,
    )
    .unwrap();

    muu()
        .arg("greet")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing required argument 'name'"));
}

#[test]
fn run_unknown_named_arg() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("muu.toml"),
        r#"
[tasks.greet]
cmd = "echo $name"
args = { name = "" }
"#,
    )
    .unwrap();

    muu()
        .args(["greet", "--typo=value"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown argument 'typo'"));
}

#[test]
fn run_task_not_found() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("muu.toml"), "[tasks]\n").unwrap();

    muu()
        .arg("nonexistent")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("task 'nonexistent' not found"));
}

#[test]
fn run_multiline_command() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("muu.toml"),
        "[tasks.multi]\ncmd = \"\"\"\necho line1\necho line2\n\"\"\"\n",
    )
    .unwrap();

    muu()
        .arg("multi")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("line1"))
        .stdout(predicate::str::contains("line2"));
}

#[test]
fn run_multiline_stops_on_error() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("muu.toml"),
        "[tasks.fail]\ncmd = \"\"\"\necho before\nfalse\necho after\n\"\"\"\n",
    )
    .unwrap();

    muu()
        .arg("fail")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("before"))
        .stdout(predicate::str::contains("after").not());
}

#[test]
fn version_flag() {
    muu()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("muu"));
}
