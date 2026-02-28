use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Perform a deep research dive on a topic using the most powerful model.
pub async fn run(config: &Config, ui: &NakamaUI, query: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("kami", "deep");
    let spinner = ui.step_start("Deep researching (this may take a moment)...");
    let start = Instant::now();

    let (provider, model) = make_provider(config, ModelTier::Powerful)?;

    let system_prompt = r#"You are Kami, performing a deep research analysis. Provide a thorough, well-structured research report on the given topic.

Structure your response as follows:

## Executive Summary
A brief 2-3 sentence overview.

## Background
Historical context and foundational concepts.

## Current State
What is the current state of this topic? Recent developments, trends.

## Key Findings
Detailed analysis with the most important discoveries and insights:
1. Finding 1: ...
2. Finding 2: ...
3. Finding 3: ...

## Technical Details
If applicable, provide technical depth with code examples, specifications, or data.

## Challenges & Considerations
Known issues, trade-offs, or areas of debate.

## Future Outlook
Where is this heading? Predictions and emerging trends.

## Conclusion
Final summary and recommendations.

Be thorough and analytical. Cite concepts and frameworks where relevant. Aim for depth over breadth."#;

    let result = ask_ai(provider.as_ref(), system_prompt, query, &model, 4096, 0.5).await;

    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success(&format!("Deep research complete ({:.1}s)", elapsed as f64 / 1000.0));
            ui.panel(&format!("Deep Research: {}", query), content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "kami",
                    "deep",
                    Category::AiInteraction,
                    &format!("Deep research on: {}", &query[..query.len().min(100)]),
                    serde_json::json!({
                        "query": query,
                        "model": model,
                        "provider": provider.provider_name(),
                        "duration_ms": elapsed,
                    }),
                    Outcome::Success,
                    elapsed,
                );
                let _ = audit.log(entry);
            }
        }
        Err(e) => {
            spinner.finish_with_error(&format!("Deep research failed: {}", e));
        }
    }

    result.map(|_| ())
}
