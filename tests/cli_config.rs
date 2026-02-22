use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[allow(deprecated)]
fn runz() -> Command {
    Command::cargo_bin("runz").unwrap()
}

#[test]
fn config_not_found() {
    let dir = TempDir::new().unwrap();
    runz()
        .arg("-l")
        .arg("list")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("no runz.toml or global config found"));
}

#[test]
fn config_upward_search() {
    let root = TempDir::new().unwrap();
    std::fs::write(
        root.path().join("runz.toml"),
        r#"
[tasks.hello]
cmd = "echo found"
description = "Found via upward search"
"#,
    )
    .unwrap();

    let child = root.path().join("a").join("b");
    std::fs::create_dir_all(&child).unwrap();

    runz()
        .arg("hello")
        .current_dir(&child)
        .assert()
        .success()
        .stdout(predicate::str::contains("found"));
}

#[test]
fn config_parse_error() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("runz.toml"), "invalid toml {{{").unwrap();

    runz()
        .arg("list")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to parse"));
}

#[test]
fn mixed_args_error() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("runz.toml"),
        r#"
[tasks.deploy]
cmd = "echo $dir $bucket"
args = { dir = ".", bucket = "" }
"#,
    )
    .unwrap();

    runz()
        .args(["deploy", "./dist", "--bucket=my-bucket"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot mix positional and named arguments"));
}
