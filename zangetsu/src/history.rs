//! The `history` subcommand — show zangetsu AI interaction history from the audit log.

use anyhow::Result;
use nakama_audit::{AuditFilter, AuditLog, Category};
use nakama_core::config::Config;
use nakama_ui::NakamaUI;

/// Execute the `history` subcommand.
pub async fn run(config: &Config, ui: &NakamaUI) -> Result<()> {
    if !config.audit.enabled {
        ui.warn("Audit logging is disabled. No history available.");
        ui.warn("Enable it in ~/.nakama/config.toml with [audit] enabled = true");
        return Ok(());
    }

    let audit = match AuditLog::new(&config.audit) {
        Ok(a) => a,
        Err(e) => {
            ui.step_fail(&format!("Could not open audit database: {}", e));
            return Err(anyhow::anyhow!("Failed to open audit database: {}", e));
        }
    };

    // Query for zangetsu AI interaction entries, most recent first
    let filter = AuditFilter::new()
        .with_tool("zangetsu")
        .with_category(Category::AiInteraction)
        .with_limit(25);

    let entries = match audit.query(&filter) {
        Ok(e) => e,
        Err(e) => {
            ui.step_fail(&format!("Failed to query audit log: {}", e));
            return Err(anyhow::anyhow!("Failed to query audit log: {}", e));
        }
    };

    if entries.is_empty() {
        ui.panel(
            "Zangetsu History",
            "No interactions recorded yet.\nStart using `zangetsu ask`, `explain`, `fix`, or `chain` to build history.",
        );
        return Ok(());
    }

    // Build table rows
    let rows: Vec<Vec<String>> = entries
        .iter()
        .map(|entry| {
            let timestamp = entry.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
            let command = entry.command.clone();
            let action = truncate_string(&entry.action, 60);
            let outcome = entry.outcome.to_string();
            let duration = if entry.duration_ms > 0 {
                format!("{}ms", entry.duration_ms)
            } else {
                "-".to_string()
            };

            vec![timestamp, command, action, outcome, duration]
        })
        .collect();

    ui.panel(
        "Zangetsu History",
        &format!("Showing last {} interaction(s):", entries.len()),
    );

    ui.table(
        &["Timestamp", "Command", "Action", "Outcome", "Duration"],
        rows,
    );

    Ok(())
}

/// Truncate a string to a maximum length, adding ellipsis if needed.
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
