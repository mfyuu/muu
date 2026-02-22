use crate::config::ResolvedTask;
use crate::error::MuuError;
use crate::runner;
use crate::task::{expand_command, resolve_args};

pub fn run(name: &str, raw_args: &[String], tasks: &[ResolvedTask]) -> Result<i32, MuuError> {
    let task = tasks
        .iter()
        .find(|t| t.name == name)
        .ok_or_else(|| MuuError::TaskNotFound {
            name: name.to_string(),
        })?;

    let resolved = resolve_args(&task.def.args, raw_args)?;
    let cmd = expand_command(&task.def.cmd, &resolved);
    Ok(runner::execute(&cmd))
}
