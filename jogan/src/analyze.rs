//! Read a log file, extract error patterns, and send to AI for root cause analysis.

use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

/// Read a log file, extract error patterns, and send to AI for root cause analysis.
pub async fn run(config: &Config, ui: &NakamaUI, log_path: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("jogan", "analyze");
    let start = Instant::now();

    let spinner = ui.step_start(&format!("Reading log file: {}", log_path));
    let content = fs::read_to_string(log_path).map_err(|e| {
        nakama_core::error::NakamaError::Tool {
            tool: "jogan".to_string(),
            message: format!("Failed to read log file '{}': {}", log_path, e),
        }
    })?;
    spinner.finish_with_success("Log file loaded");

    // Extract error patterns
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    let mut error_lines = Vec::new();
    let mut warning_lines = Vec::new();
    let mut error_patterns: HashMap<String, usize> = HashMap::new();

    for line in &lines {
        let upper = line.to_uppercase();
        if upper.contains("ERROR") || upper.contains("FATAL") || upper.contains("CRITICAL") {
            error_lines.push(*line);
            // Extract a simplified pattern (first 80 chars)
            let pattern = if line.len() > 80 {
                &line[..80]
            } else {
                line
            };
            *error_patterns.entry(pattern.to_string()).or_insert(0) += 1;
        } else if upper.contains("WARN") {
            warning_lines.push(*line);
        }
    }

    // Display summary table
    let rows = vec![
        vec!["Total Lines".to_string(), total_lines.to_string()],
        vec!["Error Lines".to_string(), error_lines.len().to_string()],
        vec!["Warning Lines".to_string(), warning_lines.len().to_string()],
        vec![
            "Error Rate".to_string(),
            if total_lines > 0 {
                format!("{:.2}%", (error_lines.len() as f64 / total_lines as f64) * 100.0)
            } else {
                "N/A".to_string()
            },
        ],
        vec![
            "Unique Error Patterns".to_string(),
            error_patterns.len().to_string(),
        ],
    ];
    ui.table(&["Metric", "Value"], rows);

    // Build a summary of top error patterns for AI
    let mut sorted_patterns: Vec<(&String, &usize)> = error_patterns.iter().collect();
    sorted_patterns.sort_by(|a, b| b.1.cmp(a.1));
    let top_patterns: Vec<String> = sorted_patterns
        .iter()
        .take(10)
        .map(|(pattern, count)| format!("  [{}x] {}", count, pattern))
        .collect();

    // Pick some sample error lines (last 20 errors)
    let sample_errors: Vec<&str> = error_lines.iter().rev().take(20).copied().collect();

    let error_summary = format!(
        "Log file: {}\nTotal lines: {}\nErrors: {}\nWarnings: {}\n\nTop error patterns:\n{}\n\nRecent error lines:\n{}",
        log_path,
        total_lines,
        error_lines.len(),
        warning_lines.len(),
        top_patterns.join("\n"),
        sample_errors.join("\n"),
    );

    // Send to AI
    let spinner = ui.step_start("Analyzing log patterns with AI...");
    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Jogan, an infrastructure debugging assistant. Analyze the log file summary and error patterns provided. Give:

1. **Root Cause**: The most likely root cause of the errors.
2. **Error Classification**: Categorize the errors (e.g., connectivity, resource exhaustion, configuration, application bug).
3. **Timeline**: If timestamps are available, describe when issues started and any patterns.
4. **Recommendations**: Concrete steps to fix the issues.
5. **Priority**: Which errors to address first and why.

Be concise and actionable."#;

    let result = ask_ai(provider.as_ref(), system_prompt, &error_summary, &model, 2048, 0.3).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Analysis complete");
            ui.panel("Log Analysis", content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "jogan",
                    "analyze",
                    Category::AiInteraction,
                    &format!("Analyzed log file: {}", log_path),
                    serde_json::json!({
                        "log_path": log_path,
                        "total_lines": total_lines,
                        "error_count": error_lines.len(),
                        "model": model,
                    }),
                    Outcome::Success,
                    elapsed,
                );
                let _ = audit.log(entry);
            }
        }
        Err(e) => {
            spinner.finish_with_error(&format!("Analysis failed: {}", e));

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "jogan",
                    "analyze",
                    Category::AiInteraction,
                    &format!("Log analysis failed for: {}", log_path),
                    serde_json::json!({ "log_path": log_path, "error": e.to_string() }),
                    Outcome::Failure,
                    elapsed,
                );
                let _ = audit.log(entry);
            }
        }
    }

    result.map(|_| ())
}
