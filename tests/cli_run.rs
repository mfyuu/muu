use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[allow(deprecated)]
fn runz() -> Command {
    Command::cargo_bin("runz").unwrap()
}

#[test]
fn run_simple_task() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("runz.toml"),
        r#"
[tasks.hello]
cmd = "echo hello"
"#,
    )
    .unwrap();

    runz()
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
        dir.path().join("runz.toml"),
        r#"
[tasks.greet]
cmd = "echo $name $greeting"
args = { name = "", greeting = "hello" }
"#,
    )
    .unwrap();

    runz()
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
        dir.path().join("runz.toml"),
        r#"
[tasks.greet]
cmd = "echo $name $greeting"
args = { name = "", greeting = "hello" }
"#,
    )
    .unwrap();

    runz()
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
        dir.path().join("runz.toml"),
        r#"
[tasks.greet]
cmd = "echo $name"
args = { name = "" }
"#,
    )
    .unwrap();

    runz()
        .arg("greet")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing required argument 'name'"));
}

#[test]
fn run_task_not_found() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("runz.toml"), "[tasks]\n").unwrap();

    runz()
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
        dir.path().join("runz.toml"),
        "[tasks.multi]\ncmd = \"\"\"\necho line1\necho line2\n\"\"\"\n",
    )
    .unwrap();

    runz()
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
        dir.path().join("runz.toml"),
        "[tasks.fail]\ncmd = \"\"\"\necho before\nfalse\necho after\n\"\"\"\n",
    )
    .unwrap();

    runz()
        .arg("fail")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("before"))
        .stdout(predicate::str::contains("after").not());
}

#[test]
fn run_no_args_shows_phase2_message() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("runz.toml"), "[tasks]\n").unwrap();

    runz()
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("not yet implemented"));
}

#[test]
fn version_flag() {
    runz()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("runz"));
}
