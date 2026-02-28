//! Regex search in log file with AI-powered context explanation.

use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use regex::Regex;
use std::fs;
use std::time::Instant;

/// Search a log file using a regex pattern and optionally explain matches with AI.
pub async fn run(config: &Config, ui: &NakamaUI, query: &str, source: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("sharingan", "filter");
    let start = Instant::now();

    // Compile the regex
    let re = Regex::new(query).map_err(|e| {
        nakama_core::error::NakamaError::Tool {
            tool: "sharingan".to_string(),
            message: format!("Invalid regex pattern '{}': {}", query, e),
        }
    })?;

    let spinner = ui.step_start(&format!("Searching in: {}", source));
    let content = fs::read_to_string(source).map_err(|e| {
        nakama_core::error::NakamaError::Tool {
            tool: "sharingan".to_string(),
            message: format!("Failed to read log file '{}': {}", source, e),
        }
    })?;

    let mut matches: Vec<(usize, String)> = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if re.is_match(line) {
            matches.push((i + 1, line.to_string()));
        }
    }
    spinner.finish_with_success(&format!("Found {} matches", matches.len()));

    if matches.is_empty() {
        ui.warn(&format!("No matches found for pattern: {}", query));
        return Ok(());
    }

    // Display matches in a table
    let display_matches: Vec<Vec<String>> = matches
        .iter()
        .take(50)
        .map(|(line_num, text)| {
            let truncated = if text.len() > 120 {
                format!("{}...", &text[..120])
            } else {
                text.clone()
            };
            vec![line_num.to_string(), truncated]
        })
        .collect();
    ui.table(&["Line", "Content"], display_matches);

    if matches.len() > 50 {
        ui.warn(&format!("Showing 50 of {} matches", matches.len()));
    }

    // Send matches to AI for context explanation
    let spinner = ui.step_start("Getting AI explanation of matches...");
    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let sample_matches: Vec<String> = matches
        .iter()
        .take(20)
        .map(|(num, text)| format!("Line {}: {}", num, text))
        .collect();

    let user_message = format!(
        "Search pattern: {}\nFile: {}\nTotal matches: {}\n\nSample matching lines:\n{}",
        query,
        source,
        matches.len(),
        sample_matches.join("\n"),
    );

    let system_prompt = r#"You are Sharingan, an AI-powered log analyzer. The user searched for a pattern in a log file. Explain:

1. **Pattern Context**: What these matching log lines represent.
2. **Significance**: Whether these matches indicate a problem, normal behavior, or something noteworthy.
3. **Recommendations**: Any actions that should be taken based on these matches.

Be concise."#;

    let result = ask_ai(provider.as_ref(), system_prompt, &user_message, &model, 1024, 0.3).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Explanation ready");
            ui.panel("Search Analysis", content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "sharingan",
                    "filter",
                    Category::AiInteraction,
                    &format!("Searched logs for: {}", query),
                    serde_json::json!({
                        "query": query,
                        "source": source,
                        "match_count": matches.len(),
                        "model": model,
                    }),
                    Outcome::Success,
                    elapsed,
                );
                let _ = audit.log(entry);
            }
        }
        Err(e) => {
            spinner.finish_with_error(&format!("AI explanation failed: {}", e));

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "sharingan",
                    "filter",
                    Category::AiInteraction,
                    &format!("Search explanation failed for: {}", query),
                    serde_json::json!({ "query": query, "error": e.to_string() }),
                    Outcome::Failure,
                    elapsed,
                );
                let _ = audit.log(entry);
            }
        }
    }

    result.map(|_| ())
}
