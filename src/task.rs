use indexmap::IndexMap;

use crate::error::RunzError;

/// Classify raw CLI args as positional or named.
enum ArgStyle {
    Positional(Vec<String>),
    Named(Vec<(String, String)>),
    None,
}

fn classify_args(raw: &[String]) -> Result<ArgStyle, RunzError> {
    if raw.is_empty() {
        return Ok(ArgStyle::None);
    }

    let has_named = raw.iter().any(|a| a.starts_with("--"));
    let has_positional = raw.iter().any(|a| !a.starts_with("--"));

    if has_named && has_positional {
        return Err(RunzError::MixedArgStyles);
    }

    if has_named {
        let mut pairs = Vec::new();
        for arg in raw {
            let stripped = arg.strip_prefix("--").unwrap();
            if let Some((key, value)) = stripped.split_once('=') {
                pairs.push((key.to_string(), value.to_string()));
            } else {
                pairs.push((stripped.to_string(), String::new()));
            }
        }
        Ok(ArgStyle::Named(pairs))
    } else {
        Ok(ArgStyle::Positional(raw.to_vec()))
    }
}

/// Resolve raw args against the defined args for a task.
/// Returns a map of arg name → resolved value.
pub fn resolve_args(
    defined: &IndexMap<String, String>,
    raw: &[String],
) -> Result<IndexMap<String, String>, RunzError> {
    let style = classify_args(raw)?;
    let mut resolved: IndexMap<String, String> = IndexMap::new();

    match style {
        ArgStyle::None => {
            for (name, default) in defined {
                if default.is_empty() {
                    return Err(RunzError::MissingRequiredArg {
                        name: name.clone(),
                    });
                }
                resolved.insert(name.clone(), default.clone());
            }
        }
        ArgStyle::Positional(values) => {
            for (i, (name, default)) in defined.iter().enumerate() {
                if let Some(val) = values.get(i) {
                    resolved.insert(name.clone(), val.clone());
                } else if default.is_empty() {
                    return Err(RunzError::MissingRequiredArg {
                        name: name.clone(),
                    });
                } else {
                    resolved.insert(name.clone(), default.clone());
                }
            }
        }
        ArgStyle::Named(pairs) => {
            // Start with defaults
            for (name, default) in defined {
                resolved.insert(name.clone(), default.clone());
            }
            // Override with provided values
            for (key, value) in &pairs {
                if resolved.contains_key(key) {
                    resolved.insert(key.clone(), value.clone());
                }
                // Unknown named args are silently ignored
            }
            // Check required args
            for (name, value) in &resolved {
                if value.is_empty() {
                    return Err(RunzError::MissingRequiredArg {
                        name: name.clone(),
                    });
                }
            }
        }
    }

    Ok(resolved)
}

/// Expand `$name` placeholders in a command string.
/// Uses longest-match-first to avoid partial substitutions.
pub fn expand_command(cmd: &str, resolved: &IndexMap<String, String>) -> String {
    // Sort keys by length descending for longest-match-first
    let mut keys: Vec<&String> = resolved.keys().collect();
    keys.sort_by_key(|k| std::cmp::Reverse(k.len()));

    let mut result = cmd.to_string();
    for key in keys {
        let placeholder = format!("${key}");
        result = result.replace(&placeholder, &resolved[key]);
    }
    result
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    fn idx(pairs: &[(&str, &str)]) -> IndexMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    fn strs(s: &[&str]) -> Vec<String> {
        s.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn positional_all_provided() {
        let defined = idx(&[("dir", "."), ("bucket", "")]);
        let resolved = resolve_args(&defined, &strs(&["./dist", "my-bucket"])).unwrap();
        assert_eq!(resolved["dir"], "./dist");
        assert_eq!(resolved["bucket"], "my-bucket");
    }

    #[test]
    fn positional_with_default() {
        let defined = idx(&[("dir", "."), ("count", "10")]);
        let resolved = resolve_args(&defined, &strs(&["./src"])).unwrap();
        assert_eq!(resolved["dir"], "./src");
        assert_eq!(resolved["count"], "10");
    }

    #[test]
    fn positional_missing_required() {
        let defined = idx(&[("dir", "."), ("bucket", "")]);
        let err = resolve_args(&defined, &strs(&["./dist"])).unwrap_err();
        assert!(matches!(err, RunzError::MissingRequiredArg { name } if name == "bucket"));
    }

    #[test]
    fn named_args() {
        let defined = idx(&[("dir", "."), ("bucket", "")]);
        let resolved =
            resolve_args(&defined, &strs(&["--bucket=my-bucket"])).unwrap();
        assert_eq!(resolved["dir"], ".");
        assert_eq!(resolved["bucket"], "my-bucket");
    }

    #[test]
    fn named_missing_required() {
        let defined = idx(&[("dir", "."), ("bucket", "")]);
        let err = resolve_args(&defined, &strs(&["--dir=./dist"])).unwrap_err();
        assert!(matches!(err, RunzError::MissingRequiredArg { name } if name == "bucket"));
    }

    #[test]
    fn mixed_args_error() {
        let defined = idx(&[("dir", "."), ("bucket", "")]);
        let err =
            resolve_args(&defined, &strs(&["./dist", "--bucket=my-bucket"])).unwrap_err();
        assert!(matches!(err, RunzError::MixedArgStyles));
    }

    #[test]
    fn no_args_with_defaults() {
        let defined = idx(&[("dir", "."), ("count", "10")]);
        let resolved = resolve_args(&defined, &[]).unwrap();
        assert_eq!(resolved["dir"], ".");
        assert_eq!(resolved["count"], "10");
    }

    #[test]
    fn no_args_missing_required() {
        let defined = idx(&[("bucket", "")]);
        let err = resolve_args(&defined, &[]).unwrap_err();
        assert!(matches!(err, RunzError::MissingRequiredArg { name } if name == "bucket"));
    }

    #[test]
    fn expand_simple() {
        let resolved = idx(&[("dir", "./dist"), ("bucket", "my-bucket")]);
        let cmd = "aws s3 sync $dir s3://$bucket";
        assert_eq!(
            expand_command(cmd, &resolved),
            "aws s3 sync ./dist s3://my-bucket"
        );
    }

    #[test]
    fn expand_longest_match_first() {
        let resolved = idx(&[("a", "short"), ("ab", "long")]);
        let cmd = "$ab $a";
        assert_eq!(expand_command(cmd, &resolved), "long short");
    }

    #[test]
    fn no_defined_args_no_raw() {
        let defined: IndexMap<String, String> = IndexMap::new();
        let resolved = resolve_args(&defined, &[]).unwrap();
        assert!(resolved.is_empty());
    }
}
