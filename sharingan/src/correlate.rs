//! Read two log files, find overlapping timestamps, look for related events.

use crate::ai_helper::{ask_ai, make_provider};
use crate::parser::{detect_format, parse_line};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::fs;
use std::time::Instant;

/// Read two log files, find overlapping timestamps, and look for related events.
pub async fn run(config: &Config, ui: &NakamaUI, source1: &str, source2: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("sharingan", "correlate");
    let start = Instant::now();

    // Read both files
    let spinner = ui.step_start(&format!("Reading log files: {} and {}", source1, source2));
    let content1 = fs::read_to_string(source1).map_err(|e| {
        nakama_core::error::NakamaError::Tool {
            tool: "sharingan".to_string(),
            message: format!("Failed to read log file '{}': {}", source1, e),
        }
    })?;
    let content2 = fs::read_to_string(source2).map_err(|e| {
        nakama_core::error::NakamaError::Tool {
            tool: "sharingan".to_string(),
            message: format!("Failed to read log file '{}': {}", source2, e),
        }
    })?;
    spinner.finish_with_success("Both log files loaded");

    // Parse both files
    let lines1: Vec<&str> = content1.lines().collect();
    let lines2: Vec<&str> = content2.lines().collect();
    let format1 = detect_format(&lines1);
    let format2 = detect_format(&lines2);

    let spinner = ui.step_start("Parsing log files...");
    let parsed1: Vec<_> = lines1.iter().map(|l| parse_line(l, format1)).collect();
    let parsed2: Vec<_> = lines2.iter().map(|l| parse_line(l, format2)).collect();
    spinner.finish_with_success("Parsing complete");

    // Collect timestamps from both files
    let ts1: Vec<&str> = parsed1
        .iter()
        .filter_map(|e| e.timestamp.as_deref())
        .collect();
    let ts2: Vec<&str> = parsed2
        .iter()
        .filter_map(|e| e.timestamp.as_deref())
        .collect();

    // Display file summaries
    let rows = vec![
        vec![
            source1.to_string(),
            format1.to_string(),
            parsed1.len().to_string(),
            ts1.first().unwrap_or(&"N/A").to_string(),
            ts1.last().unwrap_or(&"N/A").to_string(),
        ],
        vec![
            source2.to_string(),
            format2.to_string(),
            parsed2.len().to_string(),
            ts2.first().unwrap_or(&"N/A").to_string(),
            ts2.last().unwrap_or(&"N/A").to_string(),
        ],
    ];
    ui.table(
        &["File", "Format", "Lines", "First Timestamp", "Last Timestamp"],
        rows,
    );

    // Collect error lines from both files for correlation
    let errors1: Vec<String> = parsed1
        .iter()
        .filter(|e| e.level == Some(crate::parser::LogLevel::Error))
        .take(15)
        .map(|e| {
            format!(
                "[{}] {}",
                e.timestamp.as_deref().unwrap_or("?"),
                e.message
            )
        })
        .collect();

    let errors2: Vec<String> = parsed2
        .iter()
        .filter(|e| e.level == Some(crate::parser::LogLevel::Error))
        .take(15)
        .map(|e| {
            format!(
                "[{}] {}",
                e.timestamp.as_deref().unwrap_or("?"),
                e.message
            )
        })
        .collect();

    // Build correlation prompt
    let user_message = format!(
        "File 1: {} (format: {}, {} lines)\nFile 2: {} (format: {}, {} lines)\n\n\
         File 1 time range: {} to {}\n\
         File 2 time range: {} to {}\n\n\
         File 1 errors ({}):\n{}\n\n\
         File 2 errors ({}):\n{}",
        source1,
        format1,
        parsed1.len(),
        source2,
        format2,
        parsed2.len(),
        ts1.first().unwrap_or(&"N/A"),
        ts1.last().unwrap_or(&"N/A"),
        ts2.first().unwrap_or(&"N/A"),
        ts2.last().unwrap_or(&"N/A"),
        errors1.len(),
        errors1.join("\n"),
        errors2.len(),
        errors2.join("\n"),
    );

    let spinner = ui.step_start("Correlating events with AI...");
    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Sharingan, an AI-powered log analyzer. You are given two log files to correlate. Analyze:

1. **Temporal Overlap**: Do the log files cover the same time period?
2. **Correlated Events**: Are there errors or events in one file that appear to cause or relate to events in the other?
3. **Causal Chain**: If there is a causal relationship, describe the chain of events.
4. **Common Patterns**: Any shared error patterns, IPs, service names, or identifiers.
5. **Recommendations**: What to investigate next.

Be concise and focus on actionable correlations."#;

    let result = ask_ai(provider.as_ref(), system_prompt, &user_message, &model, 2048, 0.3).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Correlation complete");
            ui.panel("Log Correlation", content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "sharingan",
                    "correlate",
                    Category::AiInteraction,
                    &format!("Correlated {} and {}", source1, source2),
                    serde_json::json!({
                        "source1": source1,
                        "source2": source2,
                        "lines1": parsed1.len(),
                        "lines2": parsed2.len(),
                        "model": model,
                    }),
                    Outcome::Success,
                    elapsed,
                );
                let _ = audit.log(entry);
            }
        }
        Err(e) => {
            spinner.finish_with_error(&format!("Correlation failed: {}", e));

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "sharingan",
                    "correlate",
                    Category::AiInteraction,
                    &format!("Correlation failed for {} and {}", source1, source2),
                    serde_json::json!({ "source1": source1, "source2": source2, "error": e.to_string() }),
                    Outcome::Failure,
                    elapsed,
                );
                let _ = audit.log(entry);
            }
        }
    }

    result.map(|_| ())
}
