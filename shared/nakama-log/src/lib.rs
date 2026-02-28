//! Structured logging for the Nakama CLI Suite.
//!
//! Provides a single `init_logging` entry point that configures a tracing
//! subscriber with JSON formatting, daily-rotated per-tool log files, and
//! an environment filter derived from [`nakama_core::config::LoggingConfig`].

use nakama_core::config::LoggingConfig;
use nakama_core::error::{NakamaError, NakamaResult};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

/// Guard that keeps the background log-writer alive.
///
/// When this value is dropped the internal worker flushes all buffered log
/// records to disk, so callers should hold onto it for the lifetime of the
/// application (typically by binding it in `main`).
pub struct LogGuard {
    _guard: WorkerGuard,
}

/// Initialise the tracing subscriber for a specific Nakama tool.
///
/// * `tool_name` -- used to derive the log file prefix (e.g. `zangetsu.2024-03-15.log`).
/// * `config`    -- the [`LoggingConfig`] section of the global Nakama config.
///
/// Returns a [`LogGuard`] whose drop implementation flushes pending log records.
pub fn init_logging(tool_name: &str, config: &LoggingConfig) -> NakamaResult<LogGuard> {
    // Resolve the log directory -- expand `~` if present.
    let log_dir = shellexpand_tilde(&config.directory);

    // Ensure the directory exists.
    std::fs::create_dir_all(&log_dir).map_err(|e| NakamaError::Config {
        message: format!("Failed to create log directory {log_dir}"),
        source: Some(Box::new(e)),
    })?;

    // Build a daily-rotating file appender.
    let file_appender = tracing_appender::rolling::daily(&log_dir, format!("{tool_name}.log"));

    // Wrap the appender in a non-blocking writer so logging never blocks the
    // caller.  The returned guard must be kept alive.
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Build the env filter from the configured level string (e.g. "info",
    // "debug", "nakama_core=trace,warn").
    let env_filter = EnvFilter::try_new(&config.level).unwrap_or_else(|_| {
        tracing::warn!(
            "Invalid log level '{}', falling back to 'info'",
            config.level
        );
        EnvFilter::new("info")
    });

    // Assemble the subscriber with JSON formatting.
    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_env_filter(env_filter)
        .with_writer(non_blocking)
        .with_target(true)
        .with_thread_ids(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_current_span(true)
        .finish();

    // Install the subscriber globally.  If a subscriber has already been set
    // (e.g. in tests) we silently ignore the error.
    tracing::subscriber::set_global_default(subscriber).ok();

    Ok(LogGuard { _guard: guard })
}

/// Minimal tilde expansion (handles `~/...` only).
fn shellexpand_tilde(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = dirs_home() {
            return format!("{}/{rest}", home.display());
        }
    } else if path == "~" {
        if let Some(home) = dirs_home() {
            return home.display().to_string();
        }
    }
    path.to_string()
}

fn dirs_home() -> Option<std::path::PathBuf> {
    #[cfg(unix)]
    {
        std::env::var("HOME").ok().map(std::path::PathBuf::from)
    }
    #[cfg(not(unix))]
    {
        std::env::var("USERPROFILE")
            .ok()
            .map(std::path::PathBuf::from)
    }
}

// ---------------------------------------------------------------------------
// Convenience re-exports so downstream crates can `use nakama_log::{info, warn, ...}`
// without depending on `tracing` directly.
// ---------------------------------------------------------------------------

pub use tracing::{debug, error, info, trace, warn};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shellexpand_tilde_plain() {
        let result = shellexpand_tilde("/tmp/logs");
        assert_eq!(result, "/tmp/logs");
    }

    #[test]
    fn test_shellexpand_tilde_with_home() {
        let result = shellexpand_tilde("~/logs");
        // Should not start with ~ anymore (assuming HOME is set)
        assert!(!result.starts_with('~'));
    }
}
