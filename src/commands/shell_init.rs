use crate::error::RunzError;

const ZSH_INIT: &str = r#"runz() {
  local result
  result=$(command runz "$@" 2>/dev/tty)
  if [[ -n "$result" ]]; then
    print -z "$result"
  fi
}
"#;

pub fn print_init(shell: &str) -> Result<(), RunzError> {
    match shell {
        "zsh" => {
            print!("{ZSH_INIT}");
            Ok(())
        }
        _ => Err(RunzError::UnsupportedShell {
            name: shell.to_string(),
        }),
    }
}
