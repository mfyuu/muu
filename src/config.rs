use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use serde::Deserialize;

use crate::error::MuuError;

// ---------- TOML deserialization types ----------

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    #[serde(default)]
    pub tasks: IndexMap<String, TaskDef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskDef {
    pub cmd: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub args: IndexMap<String, String>,
}

// ---------- Resolved types ----------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskSource {
    Local,
    Global,
}

impl std::fmt::Display for TaskSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskSource::Local => write!(f, "local"),
            TaskSource::Global => write!(f, "global"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedTask {
    pub name: String,
    pub def: TaskDef,
    pub source: TaskSource,
}

// ---------- Parsing ----------

pub fn parse_config(path: &Path) -> Result<ConfigFile, MuuError> {
    let content = std::fs::read_to_string(path).map_err(MuuError::Io)?;
    toml::from_str::<ConfigFile>(&content).map_err(|e| {
        let msg = e.to_string();
        if let Some(name) = extract_duplicate_key(&msg) {
            MuuError::DuplicateTask {
                name,
                path: path.to_path_buf(),
            }
        } else {
            MuuError::ConfigParse {
                path: path.to_path_buf(),
                reason: msg,
            }
        }
    })
}

fn extract_duplicate_key(msg: &str) -> Option<String> {
    let marker = "duplicate key `";
    let start = msg.find(marker)? + marker.len();
    let end = msg[start..].find('`')? + start;
    Some(msg[start..end].to_string())
}

// ---------- File discovery ----------

pub fn find_local_config(start: &Path) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();
    loop {
        let candidate = dir.join("muu.toml");
        if candidate.is_file() {
            return Some(candidate);
        }
        if !dir.pop() {
            return None;
        }
    }
}

pub fn global_config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|d| d.join(".config").join("muu").join("config.toml"))
}

// ---------- Loading & merging ----------

pub fn load_tasks(
    start_dir: &Path,
    local_only: bool,
    global_only: bool,
) -> Result<Vec<ResolvedTask>, MuuError> {
    let local_path = if !global_only {
        find_local_config(start_dir)
    } else {
        None
    };
    let global_path = if !local_only {
        global_config_path().filter(|p| p.is_file())
    } else {
        None
    };

    if local_path.is_none() && global_path.is_none() {
        return Err(MuuError::NoConfigFound);
    }

    let mut tasks: IndexMap<String, ResolvedTask> = IndexMap::new();

    // Global first (will be overridden by local)
    if let Some(ref gp) = global_path {
        let cfg = parse_config(gp)?;
        for (name, def) in cfg.tasks {
            if tasks.contains_key(&name) {
                return Err(MuuError::DuplicateTask {
                    name,
                    path: gp.clone(),
                });
            }
            tasks.insert(
                name.clone(),
                ResolvedTask {
                    name,
                    def,
                    source: TaskSource::Global,
                },
            );
        }
    }

    // Local overrides global
    if let Some(ref lp) = local_path {
        let cfg = parse_config(lp)?;
        let mut seen_local: IndexMap<String, ()> = IndexMap::new();
        for (name, def) in cfg.tasks {
            if seen_local.contains_key(&name) {
                return Err(MuuError::DuplicateTask {
                    name,
                    path: lp.clone(),
                });
            }
            seen_local.insert(name.clone(), ());
            tasks.insert(
                name.clone(),
                ResolvedTask {
                    name,
                    def,
                    source: TaskSource::Local,
                },
            );
        }
    }

    Ok(tasks.into_values().collect())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;

    use super::*;

    fn write_file(dir: &Path, name: &str, content: &str) -> PathBuf {
        let path = dir.join(name);
        std::fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn parse_valid_config() {
        let dir = TempDir::new().unwrap();
        let path = write_file(
            dir.path(),
            "muu.toml",
            r#"
[tasks.hello]
cmd = "echo hello"
description = "Say hello"

[tasks.deploy]
cmd = "aws s3 sync $dir s3://$bucket"
args = { dir = ".", bucket = "" }
"#,
        );
        let cfg = parse_config(&path).unwrap();
        assert_eq!(cfg.tasks.len(), 2);
        assert_eq!(cfg.tasks["hello"].cmd, "echo hello");
        assert_eq!(
            cfg.tasks["hello"].description.as_deref(),
            Some("Say hello")
        );
        assert!(cfg.tasks["hello"].args.is_empty());

        let deploy = &cfg.tasks["deploy"];
        assert_eq!(deploy.args.len(), 2);
        // Verify key order is preserved
        let keys: Vec<&String> = deploy.args.keys().collect();
        assert_eq!(keys, vec!["dir", "bucket"]);
        assert_eq!(deploy.args["dir"], ".");
        assert_eq!(deploy.args["bucket"], "");
    }

    #[test]
    fn parse_invalid_toml() {
        let dir = TempDir::new().unwrap();
        let path = write_file(dir.path(), "muu.toml", "not valid toml {{{}");
        let err = parse_config(&path).unwrap_err();
        assert!(matches!(err, MuuError::ConfigParse { .. }));
    }

    #[test]
    fn find_local_config_upward() {
        let root = TempDir::new().unwrap();
        write_file(root.path(), "muu.toml", "[tasks]");
        let child = root.path().join("a").join("b").join("c");
        std::fs::create_dir_all(&child).unwrap();
        let found = find_local_config(&child).unwrap();
        assert_eq!(found, root.path().join("muu.toml"));
    }

    #[test]
    fn find_local_config_not_found() {
        let dir = TempDir::new().unwrap();
        assert!(find_local_config(dir.path()).is_none());
    }

    #[test]
    fn merge_local_overrides_global() {
        let local_dir = TempDir::new().unwrap();
        write_file(
            local_dir.path(),
            "muu.toml",
            r#"
[tasks.hello]
cmd = "echo local"
"#,
        );

        // Simulate global config
        let global_dir = TempDir::new().unwrap();
        let global_cfg_dir = global_dir.path().join("muu");
        std::fs::create_dir_all(&global_cfg_dir).unwrap();
        write_file(
            &global_cfg_dir,
            "config.toml",
            r#"
[tasks.hello]
cmd = "echo global"

[tasks.global_only]
cmd = "echo g"
"#,
        );

        // We can't easily override global_config_path in a unit test,
        // so we test the merge logic directly.
        let global_cfg = parse_config(&global_cfg_dir.join("config.toml")).unwrap();
        let local_cfg = parse_config(&local_dir.path().join("muu.toml")).unwrap();

        let mut tasks: IndexMap<String, ResolvedTask> = IndexMap::new();
        for (name, def) in global_cfg.tasks {
            tasks.insert(
                name.clone(),
                ResolvedTask {
                    name,
                    def,
                    source: TaskSource::Global,
                },
            );
        }
        for (name, def) in local_cfg.tasks {
            tasks.insert(
                name.clone(),
                ResolvedTask {
                    name,
                    def,
                    source: TaskSource::Local,
                },
            );
        }

        let result: Vec<ResolvedTask> = tasks.into_values().collect();
        assert_eq!(result.len(), 2);
        let hello = result.iter().find(|t| t.name == "hello").unwrap();
        assert_eq!(hello.def.cmd, "echo local");
        assert_eq!(hello.source, TaskSource::Local);
    }

    #[test]
    fn no_config_found() {
        let dir = TempDir::new().unwrap();
        let err = load_tasks(dir.path(), true, false).unwrap_err();
        assert!(matches!(err, MuuError::NoConfigFound));
    }

    #[test]
    fn args_order_preserved() {
        let dir = TempDir::new().unwrap();
        let path = write_file(
            dir.path(),
            "muu.toml",
            r#"
[tasks.deploy]
cmd = "deploy $env $region $count"
args = { env = "", region = "us-east-1", count = "1" }
"#,
        );
        let cfg = parse_config(&path).unwrap();
        let keys: Vec<&String> = cfg.tasks["deploy"].args.keys().collect();
        assert_eq!(keys, vec!["env", "region", "count"]);
    }
}
