use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "runz",
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
    /// Initialize a new runz.toml
    Init,
    /// Run a task (catch-all for dynamic task names)
    #[command(external_subcommand)]
    External(Vec<String>),
}
