use std::path::Path;

use crate::error::RunzError;

const TEMPLATE: &str = r#"[tasks.hello]
cmd = "echo hello"
description = "Say hello"
"#;

pub fn init(dir: &Path) -> Result<(), RunzError> {
    let path = dir.join("runz.toml");
    if path.exists() {
        return Err(RunzError::AlreadyExists);
    }
    std::fs::write(&path, TEMPLATE)?;
    println!("Created runz.toml");
    Ok(())
}
