use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MuuError {
    #[error("missing required argument '{name}'")]
    MissingRequiredArg { name: String },

    #[error("task '{name}' not found")]
    TaskNotFound { name: String },

    #[error("no muu.toml or global config found")]
    NoConfigFound,

    #[error("duplicate task '{name}' in {path}")]
    DuplicateTask { name: String, path: PathBuf },

    #[error("muu.toml already exists")]
    AlreadyExists,

    #[error("unknown argument '{name}'")]
    UnknownArg { name: String },

    #[error("cannot mix positional and named arguments")]
    MixedArgStyles,

    #[error("failed to parse {path}: {reason}")]
    ConfigParse { path: PathBuf, reason: String },

    #[error("no tasks defined")]
    NoTasksDefined,

    #[error("{0}")]
    Io(#[from] std::io::Error),
}
