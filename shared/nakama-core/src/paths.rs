use crate::error::{NakamaError, NakamaResult};
use std::path::PathBuf;

/// Get the root Nakama config directory (~/.nakama/).
pub fn nakama_home() -> NakamaResult<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| NakamaError::Config {
        message: "Could not determine home directory".to_string(),
        source: None,
    })?;
    Ok(home.join(".nakama"))
}

/// Get the global config file path (~/.nakama/config.toml).
pub fn global_config_path() -> NakamaResult<PathBuf> {
    Ok(nakama_home()?.join("config.toml"))
}

/// Get the logs directory (~/.nakama/logs/).
pub fn logs_dir() -> NakamaResult<PathBuf> {
    Ok(nakama_home()?.join("logs"))
}

/// Get the audit directory (~/.nakama/audit/).
pub fn audit_dir() -> NakamaResult<PathBuf> {
    Ok(nakama_home()?.join("audit"))
}

/// Get the vault directory (~/.nakama/vault/).
pub fn vault_dir() -> NakamaResult<PathBuf> {
    Ok(nakama_home()?.join("vault"))
}

/// Get a tool-specific config directory (~/.nakama/<tool>/ or ~/.<tool>/).
pub fn tool_config_dir(tool: &str) -> NakamaResult<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| NakamaError::Config {
        message: "Could not determine home directory".to_string(),
        source: None,
    })?;
    Ok(home.join(format!(".{tool}")))
}

/// Ensure a directory exists with secure permissions (0700).
pub fn ensure_dir(path: &PathBuf) -> NakamaResult<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
        crate::permissions::set_dir_permissions(path)?;
    }
    Ok(())
}

/// Ensure the entire ~/.nakama/ directory tree exists.
pub fn ensure_nakama_dirs() -> NakamaResult<()> {
    ensure_dir(&nakama_home()?)?;
    ensure_dir(&logs_dir()?)?;
    ensure_dir(&audit_dir()?)?;
    ensure_dir(&vault_dir()?)?;
    Ok(())
}
