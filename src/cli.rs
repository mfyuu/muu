use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::engine::SubcommandCandidates;

use crate::completions::TaskCandidates;

#[derive(Parser, Debug)]
#[command(
    name = "muu",
    version,
    about = "A minimal task runner"
)]
pub struct Cli {
    /// Show only global tasks
    #[arg(short = 'g', long = "global", global = true)]
    pub global_only: bool,

    /// Show only local tasks
    #[arg(short = 'l', long = "local", global = true)]
    pub local_only: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// List all tasks
    List,
    /// Initialize a new muu.toml
    Init,
    /// Run a task (catch-all for dynamic task names)
    #[command(external_subcommand)]
    External(Vec<String>),
}

pub fn build_cli() -> clap::Command {
    Cli::command()
        .add(SubcommandCandidates::new(TaskCandidates))
        .mut_arg("global_only", |a| a.hide(true))
        .mut_arg("local_only", |a| a.hide(true))
        .mut_subcommand("init", |cmd| cmd.hide(true))
        .mut_subcommand("list", |cmd| cmd.hide(true))
        .disable_help_flag(true)
        .disable_version_flag(true)
        .disable_help_subcommand(true)
}
