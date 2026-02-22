mod cli;
mod commands;
mod config;
mod error;
mod runner;
mod task;

use std::process;

use clap::Parser;

use cli::{Cli, Command};
use error::MuuError;

fn main() {
    let cli = Cli::parse();
    let code = match run(cli) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("\x1b[31mError: {e}\x1b[0m");
            1
        }
    };
    process::exit(code);
}

fn run(cli: Cli) -> Result<i32, MuuError> {
    let cwd = std::env::current_dir()?;

    match cli.command {
        Some(Command::Init) => {
            commands::init::init(&cwd)?;
            Ok(0)
        }
        Some(Command::List) => {
            let tasks = config::load_tasks(&cwd, cli.local_only, cli.global_only)?;
            commands::list::list(&tasks);
            Ok(0)
        }
        Some(Command::External(ref args)) if !args.is_empty() => {
            let task_name = &args[0];
            let task_args = &args[1..];
            let tasks = config::load_tasks(&cwd, cli.local_only, cli.global_only)?;
            commands::run::run(task_name, task_args, &tasks)
        }
        _ => {
            let tasks = config::load_tasks(&cwd, cli.local_only, cli.global_only)?;
            commands::select::select(&tasks)
        }
    }
}
