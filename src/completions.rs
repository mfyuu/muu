use std::env;

use clap_complete::engine::{CompletionCandidate, ValueCandidates};

use crate::config;

#[derive(Clone, Debug)]
pub struct TaskCandidates;

impl ValueCandidates for TaskCandidates {
    fn candidates(&self) -> Vec<CompletionCandidate> {
        let cwd = env::current_dir().unwrap_or_default();

        let tasks = match config::load_tasks(&cwd, false, false) {
            Ok(tasks) => tasks,
            Err(_) => return Vec::new(),
        };

        tasks
            .into_iter()
            .map(|task| {
                let candidate = CompletionCandidate::new(task.name);
                if let Some(desc) = task.def.description {
                    candidate.help(Some(desc.into()))
                } else {
                    candidate
                }
            })
            .collect()
    }
}
