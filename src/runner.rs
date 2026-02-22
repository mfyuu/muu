use std::process::Command;

/// Execute a command string via `zsh -c` with `set -e` prepended.
/// Returns the exit code (0 on success).
pub fn execute(cmd: &str) -> i32 {
    let script = format!("set -e\n{cmd}");
    let status = Command::new("zsh").arg("-c").arg(&script).status();

    match status {
        Ok(s) => s.code().unwrap_or(1),
        Err(e) => {
            eprintln!("error: failed to execute zsh: {e}");
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        assert_eq!(execute("true"), 0);
    }

    #[test]
    fn failure() {
        assert_ne!(execute("false"), 0);
    }

    #[test]
    fn multiline_stops_on_error() {
        // Second line fails, third line should not run
        let code = execute("true\nfalse\necho should_not_reach");
        assert_ne!(code, 0);
    }

    #[test]
    fn multiline_success() {
        assert_eq!(execute("echo a\necho b\ntrue"), 0);
    }
}
