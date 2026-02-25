use std::fmt;

use indexmap::IndexMap;
use inquire::validator::Validation;
use inquire::{InquireError, Select, Text};

use crate::config::{ArgDef, ResolvedTask};
use crate::error::MuuError;
use crate::runner;
use crate::task::expand_command;

struct TaskOption<'a> {
    task: &'a ResolvedTask,
    max_name: usize,
}

const DIM: &str = "\x1b[2m";
const RESET_DIM: &str = "\x1b[22m";

impl fmt::Display for TaskOption<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = self.task.def.description.as_deref().unwrap_or("");
        if desc.is_empty() {
            write!(f, "{}", self.task.name)
        } else {
            write!(
                f,
                "{:<name_w$}  {DIM}{desc}{RESET_DIM}",
                self.task.name,
                name_w = self.max_name,
            )
        }
    }
}

pub fn select(tasks: &[ResolvedTask]) -> Result<i32, MuuError> {
    if tasks.is_empty() {
        return Err(MuuError::NoTasksDefined);
    }

    let max_name = tasks.iter().map(|t| t.name.len()).max().unwrap_or(0);

    let options: Vec<TaskOption> = tasks
        .iter()
        .map(|task| TaskOption { task, max_name })
        .collect();

    let result = Select::new("Select a task:", options)
        .with_page_size(10)
        .prompt();

    match result {
        Ok(selected) => execute_selected(selected.task),
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => Ok(1),
        Err(e) => {
            eprintln!("error: {e}");
            Ok(1)
        }
    }
}

enum PromptResult {
    Resolved(IndexMap<String, String>),
    Cancelled,
}

fn execute_selected(task: &ResolvedTask) -> Result<i32, MuuError> {
    if task.def.args.is_empty() {
        eprintln!("\x1b[36m$ {}\x1b[0m", task.def.cmd);
        return Ok(runner::execute(&task.def.cmd));
    }

    eprintln!("\x1b[36m$ {}\x1b[0m", task.def.cmd);

    match prompt_args(&task.def.args)? {
        PromptResult::Resolved(resolved) => {
            let cmd = expand_command(&task.def.cmd, &resolved);
            Ok(runner::execute(&cmd))
        }
        PromptResult::Cancelled => Ok(1),
    }
}

fn prompt_args(defined: &IndexMap<String, ArgDef>) -> Result<PromptResult, MuuError> {
    let mut resolved: IndexMap<String, String> = IndexMap::new();

    for (name, arg) in defined {
        let is_required = arg.default.is_empty() && !arg.optional;
        let prompt_message = if arg.default.is_empty() {
            format!("{name}:")
        } else {
            format!("{name}[{}]:", arg.default)
        };

        let mut text_prompt = Text::new(&prompt_message);
        if is_required {
            text_prompt = text_prompt.with_validator(|input: &str| {
                if input.is_empty() {
                    Ok(Validation::Invalid("This argument is required".into()))
                } else {
                    Ok(Validation::Valid)
                }
            });
        }

        match text_prompt.prompt() {
            Ok(value) => {
                let value = if value.is_empty() && !arg.default.is_empty() {
                    arg.default.clone()
                } else {
                    value
                };
                resolved.insert(name.clone(), value);
            }
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
                return Ok(PromptResult::Cancelled);
            }
            Err(e) => {
                eprintln!("error: {e}");
                return Ok(PromptResult::Cancelled);
            }
        }
    }

    Ok(PromptResult::Resolved(resolved))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{TaskDef, TaskSource};

    #[test]
    fn execute_selected_no_args_task() {
        let task = ResolvedTask {
            name: "hello".to_string(),
            def: TaskDef {
                cmd: "echo hello".to_string(),
                description: Some("Say hello".to_string()),
                args: IndexMap::new(),
            },
            source: TaskSource::Local,
        };
        let result = execute_selected(&task).unwrap();
        assert_eq!(result, 0);
    }
}
