use std::fs;
use std::path::Path;

use reqwest::blocking::Client;

use crate::error::{PumpkinError, Result};

const NIGHTLY_BASE_URL: &str = "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly";

pub fn ensure_executable(path: &Path) -> Result<()> {
    if path.exists() {
        return Ok(());
    }

    let Some(url) = executable_url(path) else {
        return Err(PumpkinError::MissingExecutable(path.to_path_buf()));
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let response = Client::new()
        .get(url)
        .send()
        .map_err(|error| PumpkinError::StartFailed(error.to_string()))?
        .error_for_status()
        .map_err(|error| PumpkinError::StartFailed(error.to_string()))?;

    let bytes = response
        .bytes()
        .map_err(|error| PumpkinError::StartFailed(error.to_string()))?;

    if bytes.is_empty() {
        return Err(PumpkinError::StartFailed(
            "downloaded executable was empty".to_string(),
        ));
    }

    fs::write(path, &bytes)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
    }

    Ok(())
}

fn executable_url(path: &Path) -> Option<&'static str> {
    let file_name = path.file_name()?.to_str()?;
    match (std::env::consts::OS, std::env::consts::ARCH, file_name) {
        ("windows", "x86_64", "pumpkin-X64-Windows.exe") => Some(
            "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-X64-Windows.exe",
        ),
        ("windows", "aarch64", "pumpkin-ARM64-Windows.exe") => Some(
            "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-ARM64-Windows.exe",
        ),
        ("macos", "x86_64", "pumpkin-X64-macOS") => Some(
            "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-X64-macOS",
        ),
        ("macos", "aarch64", "pumpkin-ARM64-macOS") => Some(
            "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-ARM64-macOS",
        ),
        ("linux", "x86_64", "pumpkin-X64-Linux") => Some(
            "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-X64-Linux",
        ),
        ("linux", "aarch64", "pumpkin-ARM64-Linux") => Some(
            "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-ARM64-Linux",
        ),
        _ => None,
    }
}
