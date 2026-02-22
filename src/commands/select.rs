use std::fmt;

use indexmap::IndexMap;
use inquire::validator::Validation;
use inquire::{InquireError, Select, Text};

use crate::config::ResolvedTask;
use crate::error::RunzError;
use crate::runner;
use crate::task::expand_command;

struct TaskOption<'a> {
    task: &'a ResolvedTask,
    max_name: usize,
}

impl fmt::Display for TaskOption<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = self.task.def.description.as_deref().unwrap_or("");
        let source_label = format!("[{}]", self.task.source);
        if desc.is_empty() {
            write!(f, "{:<w$}   {source_label}", self.task.name, w = self.max_name)
        } else {
            write!(
                f,
                "{:<w$} - {desc} {source_label}",
                self.task.name,
                w = self.max_name,
            )
        }
    }
}

pub fn select(tasks: &[ResolvedTask]) -> Result<i32, RunzError> {
    if tasks.is_empty() {
        return Err(RunzError::NoTasksDefined);
    }

    let max_name = tasks.iter().map(|t| t.name.len()).max().unwrap_or(0);

    let options: Vec<TaskOption> = tasks
        .iter()
        .map(|task| TaskOption { task, max_name })
        .collect();

    let result = Select::new("Select a task:", options).prompt();

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

fn execute_selected(task: &ResolvedTask) -> Result<i32, RunzError> {
    if task.def.args.is_empty() {
        return Ok(runner::execute(&task.def.cmd));
    }

    eprintln!("  \x1b[32m{}\x1b[0m", task.def.cmd);

    match prompt_args(&task.def.args)? {
        PromptResult::Resolved(resolved) => {
            let cmd = expand_command(&task.def.cmd, &resolved);
            Ok(runner::execute(&cmd))
        }
        PromptResult::Cancelled => Ok(1),
    }
}

fn prompt_args(defined: &IndexMap<String, String>) -> Result<PromptResult, RunzError> {
    let mut resolved: IndexMap<String, String> = IndexMap::new();

    for (name, default) in defined {
        let is_required = default.is_empty();
        let prompt_message = if is_required {
            format!("{name}:")
        } else {
            format!("{name}[{default}]:")
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
                let value = if value.is_empty() && !is_required {
                    default.clone()
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
