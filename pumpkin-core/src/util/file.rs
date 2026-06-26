use std::fs;
use std::path::{Path, PathBuf};

use crate::error::Result;

pub fn read_text(path: &Path) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

pub fn write_text(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    fs::write(path, contents)?;
    Ok(())
}

pub fn resolve_path(base: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        return path.to_path_buf();
    }

    base.join(path)
}
