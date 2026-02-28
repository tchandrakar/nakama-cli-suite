//! Nakama Update Checker
//!
//! Non-blocking auto-update checker for Nakama CLI tools.
//! Spawns a background task on startup; after the command finishes,
//! displays a panel if a newer version is available.

pub mod cache;
pub mod checker;

pub use checker::UpdateInfo;

use nakama_core::config::UpdatesConfig;
use nakama_ui::NakamaUI;
use tokio::sync::oneshot;

/// Convenience wrapper: spawn a background update check using config values.
///
/// Call this early in `main()` (after loading config, before running the command).
pub fn spawn_check(
    updates_config: &UpdatesConfig,
    current_version: &str,
) -> oneshot::Receiver<Option<UpdateInfo>> {
    checker::spawn_check(
        current_version,
        updates_config.check_interval_hours,
        updates_config.enabled,
    )
}

/// After the command finishes, call this to (maybe) show an update notice.
///
/// Uses `try_recv()` so it never blocks. If the background check hasn't
/// finished yet, it silently does nothing.
pub fn maybe_show_update(ui: &NakamaUI, mut rx: oneshot::Receiver<Option<UpdateInfo>>) {
    if let Ok(Some(info)) = rx.try_recv() {
        ui.panel(
            "Update Available",
            &format!(
                "A new version of Nakama CLI Suite is available: v{} (current: v{})\n\
                 \n\
                 Update with:\n\
                 \n\
                 curl -fsSL https://raw.githubusercontent.com/tchandrakar/nakama-cli-suite/main/install-release.sh | bash\n\
                 \n\
                 Or build from source:\n\
                 git pull && ./install.sh\n\
                 \n\
                 Release notes: {}",
                info.latest, info.current, info.url,
            ),
        );
    }
}
