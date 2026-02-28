use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Perform an AI-powered search query.
pub async fn run(config: &Config, ui: &NakamaUI, query: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("kami", "search");
    let spinner = ui.step_start("Searching...");
    let start = Instant::now();

    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Kami, an AI-powered search assistant. The user will provide a search query.
Provide a comprehensive, well-structured answer with the following sections:

## Answer
A clear, concise answer to the query.

## Key Points
- Bullet points of the most important information.

## Details
More detailed explanation if needed.

## Related Topics
- Suggest related topics the user might want to explore.

Be factual, concise, and helpful. If you're not certain about something, say so."#;

    let result = ask_ai(provider.as_ref(), system_prompt, query, &model, 2048, 0.4).await;

    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Search complete");
            ui.panel(&format!("Search: {}", query), content);

            // Audit log
            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "kami",
                    "search",
                    Category::AiInteraction,
                    &format!("AI search for: {}", truncate(query, 100)),
                    serde_json::json!({
                        "query": query,
                        "model": model,
                        "provider": provider.provider_name(),
                    }),
                    Outcome::Success,
                    elapsed,
                );
                let _ = audit.log(entry);
            }
        }
        Err(e) => {
            spinner.finish_with_error(&format!("Search failed: {}", e));

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "kami",
                    "search",
                    Category::AiInteraction,
                    &format!("AI search failed for: {}", truncate(query, 100)),
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

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        s
    } else {
        &s[..max]
    }
}
