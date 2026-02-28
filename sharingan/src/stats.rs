//! Count log levels, calculate error rate, find top error messages, display in table.

use crate::parser::{detect_format, parse_line, LogLevel};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_ui::NakamaUI;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

/// Generate statistics for a log file: level counts, error rate, top error messages.
pub async fn run(config: &Config, ui: &NakamaUI, source: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("sharingan", "summary");
    let start = Instant::now();

    let spinner = ui.step_start(&format!("Reading log file: {}", source));
    let content = fs::read_to_string(source).map_err(|e| {
        nakama_core::error::NakamaError::Tool {
            tool: "sharingan".to_string(),
            message: format!("Failed to read log file '{}': {}", source, e),
        }
    })?;
    spinner.finish_with_success("Log file loaded");

    let lines: Vec<&str> = content.lines().collect();
    let format = detect_format(&lines);

    let spinner = ui.step_start("Parsing and collecting statistics...");
    let parsed: Vec<_> = lines.iter().map(|l| parse_line(l, format)).collect();

    let mut level_counts: HashMap<String, usize> = HashMap::new();
    let mut error_messages: HashMap<String, usize> = HashMap::new();

    for entry in &parsed {
        let level_name = entry
            .level
            .map(|l| l.to_string())
            .unwrap_or_else(|| "UNKNOWN".to_string());
        *level_counts.entry(level_name).or_insert(0) += 1;

        if entry.level == Some(LogLevel::Error) {
            let msg = if entry.message.len() > 100 {
                format!("{}...", &entry.message[..100])
            } else {
                entry.message.clone()
            };
            *error_messages.entry(msg).or_insert(0) += 1;
        }
    }
    spinner.finish_with_success("Statistics collected");

    let total = parsed.len();
    let error_count = level_counts.get("ERROR").copied().unwrap_or(0);
    let warn_count = level_counts.get("WARN").copied().unwrap_or(0);
    let info_count = level_counts.get("INFO").copied().unwrap_or(0);
    let debug_count = level_counts.get("DEBUG").copied().unwrap_or(0);
    let trace_count = level_counts.get("TRACE").copied().unwrap_or(0);
    let unknown_count = level_counts.get("UNKNOWN").copied().unwrap_or(0);

    let error_rate = if total > 0 {
        format!("{:.2}%", (error_count as f64 / total as f64) * 100.0)
    } else {
        "N/A".to_string()
    };

    // Level distribution table
    let level_rows = vec![
        vec!["TRACE".to_string(), trace_count.to_string()],
        vec!["DEBUG".to_string(), debug_count.to_string()],
        vec!["INFO".to_string(), info_count.to_string()],
        vec!["WARN".to_string(), warn_count.to_string()],
        vec!["ERROR".to_string(), error_count.to_string()],
        vec!["UNKNOWN".to_string(), unknown_count.to_string()],
        vec!["---".to_string(), "---".to_string()],
        vec!["Total".to_string(), total.to_string()],
        vec!["Error Rate".to_string(), error_rate.clone()],
    ];
    ui.table(&["Level", "Count"], level_rows);

    // Top error messages
    let mut sorted_errors: Vec<(&String, &usize)> = error_messages.iter().collect();
    sorted_errors.sort_by(|a, b| b.1.cmp(a.1));

    if !sorted_errors.is_empty() {
        let top_error_rows: Vec<Vec<String>> = sorted_errors
            .iter()
            .take(10)
            .map(|(msg, count)| vec![count.to_string(), msg.to_string()])
            .collect();
        ui.table(&["Count", "Error Message"], top_error_rows);
    } else {
        ui.success("No errors found in log file.");
    }

    let elapsed = start.elapsed().as_millis() as u64;

    if let Ok(audit) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id,
            "sharingan",
            "summary",
            Category::ToolExecution,
            &format!("Generated stats for: {}", source),
            serde_json::json!({
                "source": source,
                "format": format.to_string(),
                "total_lines": total,
                "error_count": error_count,
                "warn_count": warn_count,
                "error_rate": error_rate,
            }),
            Outcome::Success,
            elapsed,
        );
        let _ = audit.log(entry);
    }

    Ok(())
}
