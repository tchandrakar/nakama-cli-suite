use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Compare multiple items, technologies, or concepts.
pub async fn run(config: &Config, ui: &NakamaUI, items: &[String]) -> NakamaResult<()> {
    let trace = TraceContext::new("kami", "compare");
    let items_str = items.join(" vs ");
    let spinner = ui.step_start(&format!("Comparing: {}...", items_str));
    let start = Instant::now();

    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Kami, a comparison analyst. The user will provide items to compare.

Provide a structured comparison:

## Overview
Brief 1-2 sentence overview of what's being compared.

## Strengths
For each item, list its unique strengths (2-3 bullets each).

## Weaknesses
For each item, list its weaknesses or limitations (2-3 bullets each).

## Use Cases
When to prefer each item — provide specific scenarios.

## Recommendation
Your suggestion based on common use cases. Be opinionated but acknowledge trade-offs.

Be objective, specific, and practical. Use concrete examples over vague statements."#;

    let user_msg = format!("Please compare: {}", items_str);
    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 3072, 0.4).await;

    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Comparison complete");
            ui.panel(&format!("Comparison: {}", items_str), content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "kami",
                    "compare",
                    Category::AiInteraction,
                    &format!("Compared: {}", items_str),
                    serde_json::json!({
                        "items": items,
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
            spinner.finish_with_error(&format!("Comparison failed: {}", e));
        }
    }

    result.map(|_| ())
}
