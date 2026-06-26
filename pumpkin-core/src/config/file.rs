use std::path::{Path, PathBuf};

use crate::error::{PumpkinError, Result};
use crate::model::PumpkinConfig;
use crate::util::{read_text, write_text};

pub const CONFIG_FILE_NAME: &str = "pumpkin-ui.toml";

pub fn config_path(base_directory: impl AsRef<Path>) -> PathBuf {
    base_directory.as_ref().join(CONFIG_FILE_NAME)
}

pub fn load_config(path: &Path) -> Result<PumpkinConfig> {
    if !path.exists() {
        return Err(PumpkinError::ConfigNotFound(path.to_path_buf()));
    }

    let contents = read_text(path)?;
    Ok(toml::from_str(&contents)?)
}

pub fn load_or_default(path: &Path) -> Result<PumpkinConfig> {
    if path.exists() {
        return load_config(path);
    }

    Ok(PumpkinConfig::default())
}

pub fn save_config(path: &Path, config: &PumpkinConfig) -> Result<()> {
    let contents = toml::to_string_pretty(config)?;
    write_text(path, &contents)?;
    Ok(())
}
