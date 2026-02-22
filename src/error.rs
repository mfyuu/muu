use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunzError {
    #[error("error: missing required argument '{name}'")]
    MissingRequiredArg { name: String },

    #[error("error: task '{name}' not found")]
    TaskNotFound { name: String },

    #[error("error: no runz.toml or global config found")]
    NoConfigFound,

    #[error("error: duplicate task '{name}' in {path}")]
    DuplicateTask { name: String, path: PathBuf },

    #[error("error: runz.toml already exists")]
    AlreadyExists,

    #[error("error: unknown argument '{name}'")]
    UnknownArg { name: String },

    #[error("error: cannot mix positional and named arguments")]
    MixedArgStyles,

    #[error("error: failed to parse {path}: {reason}")]
    ConfigParse { path: PathBuf, reason: String },

    #[error("error: no tasks defined")]
    NoTasksDefined,

    #[error("error: unsupported shell '{name}'")]
    UnsupportedShell { name: String },

    #[error("{0}")]
    Io(#[from] std::io::Error),
}
