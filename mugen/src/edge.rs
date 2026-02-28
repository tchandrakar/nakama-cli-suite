use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::path::Path;
use std::time::Instant;

/// Generate edge-case tests for a function in a file.
pub async fn run(config: &Config, ui: &NakamaUI, function: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("mugen", "edge");
    let start = Instant::now();

    // If function contains a colon, treat as file:function
    let (file_content, func_name) = if function.contains(':') {
        let parts: Vec<&str> = function.splitn(2, ':').collect();
        let path = parts[0];
        if Path::new(path).exists() {
            (std::fs::read_to_string(path)?, parts[1].to_string())
        } else {
            (String::new(), function.to_string())
        }
    } else {
        (String::new(), function.to_string())
    };

    let spinner = ui.step_start(&format!("Generating edge-case tests for {}...", func_name));

    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Mugen, an expert at finding edge cases. Generate edge-case tests for the given function.

Focus on:
1. Boundary values (0, -1, MAX, MIN, empty)
2. Null/None/nil inputs
3. Type boundaries (overflow, underflow)
4. Empty collections, single-element collections
5. Unicode and special characters for string inputs
6. Concurrent access patterns if applicable
7. Error conditions and exception paths
8. Large inputs (performance boundaries)

For each test, explain WHY this edge case is important.
Output ready-to-use test code."#;

    let user_msg = if file_content.is_empty() {
        format!("Generate edge-case tests for this function: {}", func_name)
    } else {
        let truncated = if file_content.len() > 8000 { &file_content[..8000] } else { &file_content };
        format!("Generate edge-case tests for the function `{}` in this code:\n\n```\n{}\n```", func_name, truncated)
    };

    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 3072, 0.4).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Edge-case tests generated");
            ui.panel(&format!("Edge Cases: {}", func_name), content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id, "mugen", "edge", Category::AiInteraction,
                    &format!("Edge-case tests for {}", func_name),
                    serde_json::json!({ "function": func_name, "model": model }),
                    Outcome::Success, elapsed,
                );
                let _ = audit.log(entry);
            }
        }
        Err(e) => spinner.finish_with_error(&format!("Failed: {}", e)),
    }

    result.map(|_| ())
}
