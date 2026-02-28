//! Read entire log file, parse with parser, send patterns to AI for analysis.

use crate::ai_helper::{ask_ai, make_provider};
use crate::parser::{detect_format, parse_line, LogLevel};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

/// Read a log file, parse it, and send patterns to AI for analysis.
pub async fn run(config: &Config, ui: &NakamaUI, logfile: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("sharingan", "explain");
    let start = Instant::now();

    let spinner = ui.step_start(&format!("Reading log file: {}", logfile));
    let content = fs::read_to_string(logfile).map_err(|e| {
        nakama_core::error::NakamaError::Tool {
            tool: "sharingan".to_string(),
            message: format!("Failed to read log file '{}': {}", logfile, e),
        }
    })?;
    spinner.finish_with_success("Log file loaded");

    let lines: Vec<&str> = content.lines().collect();
    let format = detect_format(&lines);

    let spinner = ui.step_start(&format!("Parsing {} lines (format: {})...", lines.len(), format));
    let parsed: Vec<_> = lines.iter().map(|l| parse_line(l, format)).collect();
    spinner.finish_with_success("Parsing complete");

    // Gather statistics
    let mut level_counts: HashMap<String, usize> = HashMap::new();
    let mut error_messages: Vec<String> = Vec::new();

    for entry in &parsed {
        let level_name = entry
            .level
            .map(|l| l.to_string())
            .unwrap_or_else(|| "UNKNOWN".to_string());
        *level_counts.entry(level_name).or_insert(0) += 1;

        if entry.level == Some(LogLevel::Error) {
            error_messages.push(entry.message.clone());
        }
    }

    // Display summary
    let total = parsed.len();
    let error_count = level_counts.get("ERROR").copied().unwrap_or(0);
    let warn_count = level_counts.get("WARN").copied().unwrap_or(0);

    let rows = vec![
        vec!["Format Detected".to_string(), format.to_string()],
        vec!["Total Lines".to_string(), total.to_string()],
        vec!["Errors".to_string(), error_count.to_string()],
        vec!["Warnings".to_string(), warn_count.to_string()],
        vec![
            "Error Rate".to_string(),
            if total > 0 {
                format!("{:.2}%", (error_count as f64 / total as f64) * 100.0)
            } else {
                "N/A".to_string()
            },
        ],
    ];
    ui.table(&["Metric", "Value"], rows);

    // Build AI prompt with error samples
    let sample_errors: Vec<&str> = error_messages
        .iter()
        .take(20)
        .map(|s| s.as_str())
        .collect();

    let summary = format!(
        "Log file: {}\nFormat: {}\nTotal lines: {}\nErrors: {}\nWarnings: {}\n\nSample error messages:\n{}",
        logfile,
        format,
        total,
        error_count,
        warn_count,
        sample_errors.join("\n"),
    );

    // Send to AI
    let spinner = ui.step_start("Analyzing patterns with AI...");
    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Sharingan, an AI-powered log analyzer. Analyze the log file summary and error patterns. Provide:

1. **Overview**: What this log file represents and its overall health.
2. **Key Findings**: The most important patterns and anomalies found.
3. **Error Analysis**: Root causes of the errors seen.
4. **Recommendations**: Concrete steps to address the issues.
5. **Risk Assessment**: Rate the overall risk level (LOW, MEDIUM, HIGH, CRITICAL).

Be concise and actionable."#;

    let result = ask_ai(provider.as_ref(), system_prompt, &summary, &model, 2048, 0.3).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Analysis complete");
            ui.panel("Log Analysis", content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "sharingan",
                    "explain",
                    Category::AiInteraction,
                    &format!("Analyzed log file: {}", logfile),
                    serde_json::json!({
                        "logfile": logfile,
                        "format": format.to_string(),
                        "total_lines": total,
                        "error_count": error_count,
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
                    "sharingan",
                    "explain",
                    Category::AiInteraction,
                    &format!("Log analysis failed for: {}", logfile),
                    serde_json::json!({ "logfile": logfile, "error": e.to_string() }),
                    Outcome::Failure,
                    elapsed,
                );
                let _ = audit.log(entry);
            }
        }
    }

    result.map(|_| ())
}
