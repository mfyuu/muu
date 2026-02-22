use std::fmt;

use inquire::InquireError;
use inquire::Select;

use crate::config::ResolvedTask;
use crate::error::RunzError;

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
        Ok(selected) => {
            println!("runz {} ", selected.task.name);
            Ok(0)
        }
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => Ok(1),
        Err(e) => {
            eprintln!("error: {e}");
            Ok(1)
        }
    }
}
