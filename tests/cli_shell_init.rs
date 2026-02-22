use assert_cmd::Command;
use predicates::prelude::*;

#[allow(deprecated)]
fn runz() -> Command {
    Command::cargo_bin("runz").unwrap()
}

#[test]
fn shell_init_zsh_outputs_runz_function() {
    runz()
        .args(["shell-init", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("runz()"))
        .stdout(predicate::str::contains("command runz"))
        .stdout(predicate::str::contains("print -z"))
        .stdout(predicate::str::contains("2>/dev/tty"));
}

#[test]
fn shell_init_unsupported_shell() {
    runz()
        .args(["shell-init", "fish"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unsupported shell 'fish'"));
}
