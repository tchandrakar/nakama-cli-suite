use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Cached result of the last update check, stored in `~/.nakama/update_check.json`.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCache {
    /// When the last check was performed (UTC).
    pub last_check: DateTime<Utc>,
    /// The latest version found, if any.
    pub latest_version: Option<String>,
}

/// Return the path to the cache file (`~/.nakama/update_check.json`).
pub fn cache_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".nakama").join("update_check.json"))
}

/// Read the cached update check, returning `None` if missing or corrupt.
pub fn read_cache() -> Option<UpdateCache> {
    let path = cache_path()?;
    let data = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

/// Write an update cache entry to disk. Errors are silently ignored.
pub fn write_cache(cache: &UpdateCache) {
    if let Some(path) = cache_path() {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(cache) {
            let _ = std::fs::write(path, json);
        }
    }
}

/// Returns `true` if enough time has elapsed since the last check.
pub fn should_check(interval_hours: u64) -> bool {
    match read_cache() {
        Some(cache) => {
            let elapsed = Utc::now().signed_duration_since(cache.last_check);
            elapsed.num_hours() >= interval_hours as i64
        }
        None => true, // No cache → always check
    }
}
