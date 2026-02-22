use std::path::Path;

use crate::error::MuuError;

const TEMPLATE: &str = r#"[tasks.hello]
cmd = "echo hello"
description = "Say hello"
"#;

pub fn init(dir: &Path) -> Result<(), MuuError> {
    let path = dir.join("muu.toml");
    if path.exists() {
        return Err(MuuError::AlreadyExists);
    }
    std::fs::write(&path, TEMPLATE)?;
    println!("Created muu.toml");
    Ok(())
}
