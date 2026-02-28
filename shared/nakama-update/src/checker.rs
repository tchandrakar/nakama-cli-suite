use crate::cache::{self, UpdateCache};
use chrono::Utc;
use semver::Version;
use serde::Deserialize;
use std::time::Duration;
use tokio::sync::oneshot;

/// Information about an available update.
#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub current: String,
    pub latest: String,
    pub url: String,
}

/// The GitHub API response shape for `/repos/:owner/:repo/releases/latest`.
#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

const GITHUB_API_URL: &str =
    "https://api.github.com/repos/tchandrakar/nakama-cli-suite/releases/latest";

/// Spawn a background task that checks GitHub for a newer release.
///
/// Returns a `oneshot::Receiver` that will eventually contain `Some(UpdateInfo)`
/// if a newer version exists, or `None` if the current version is up-to-date
/// (or if the check fails/times out).
pub fn spawn_check(
    current_version: &str,
    interval_hours: u64,
    enabled: bool,
) -> oneshot::Receiver<Option<UpdateInfo>> {
    let (tx, rx) = oneshot::channel();
    let current = current_version.to_string();

    if !enabled {
        let _ = tx.send(None);
        return rx;
    }

    // Throttle: skip if we checked recently
    if !cache::should_check(interval_hours) {
        // Even if throttled, check if the cache already knows about a newer version
        if let Some(cached) = cache::read_cache() {
            if let Some(ref latest_str) = cached.latest_version {
                if is_newer(latest_str, &current) {
                    let _ = tx.send(Some(UpdateInfo {
                        current: current.clone(),
                        latest: latest_str.clone(),
                        url: format!(
                            "https://github.com/tchandrakar/nakama-cli-suite/releases/tag/v{}",
                            latest_str
                        ),
                    }));
                    return rx;
                }
            }
        }
        let _ = tx.send(None);
        return rx;
    }

    tokio::spawn(async move {
        let result = check_github(&current).await;

        // Update cache regardless of outcome
        let latest_version = result.as_ref().map(|info| info.latest.clone());
        cache::write_cache(&UpdateCache {
            last_check: Utc::now(),
            latest_version,
        });

        // Send the result; ignore error (receiver may have been dropped)
        let _ = tx.send(result);
    });

    rx
}

/// Hit the GitHub API and compare versions.
async fn check_github(current_version: &str) -> Option<UpdateInfo> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .user_agent("nakama-cli-suite")
        .build()
        .ok()?;

    let release: GitHubRelease = client
        .get(GITHUB_API_URL)
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;

    let latest_str = release.tag_name.strip_prefix('v').unwrap_or(&release.tag_name);

    if is_newer(latest_str, current_version) {
        Some(UpdateInfo {
            current: current_version.to_string(),
            latest: latest_str.to_string(),
            url: release.html_url,
        })
    } else {
        None
    }
}

/// Returns `true` if `latest` is strictly newer than `current` by semver.
fn is_newer(latest: &str, current: &str) -> bool {
    match (Version::parse(latest), Version::parse(current)) {
        (Ok(l), Ok(c)) => l > c,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_newer() {
        assert!(is_newer("0.2.0", "0.1.0"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(!is_newer("0.1.0", "0.1.0"));
        assert!(!is_newer("0.0.9", "0.1.0"));
    }

    #[test]
    fn test_is_newer_invalid() {
        assert!(!is_newer("not-a-version", "0.1.0"));
        assert!(!is_newer("0.1.0", "not-a-version"));
    }
}
