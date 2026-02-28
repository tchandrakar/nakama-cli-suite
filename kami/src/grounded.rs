use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Fact-check a claim using AI analysis.
pub async fn run(config: &Config, ui: &NakamaUI, claim: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("kami", "grounded");
    let spinner = ui.step_start("Fact-checking...");
    let start = Instant::now();

    let (provider, model) = make_provider(config, ModelTier::Powerful)?;

    let system_prompt = r#"You are Kami, a fact-checking assistant. The user will provide a claim to verify.

Analyze the claim and respond with this structure:

## Verdict
One of: TRUE, MOSTLY TRUE, MIXED, MOSTLY FALSE, FALSE, UNVERIFIABLE

## Analysis
Explain why this claim is or isn't accurate. Break down the claim into verifiable components.

## Evidence For
- List evidence or reasoning that supports the claim.

## Evidence Against
- List evidence or reasoning that contradicts the claim.

## Nuance
Important context, caveats, or conditions that affect the claim's accuracy.

## Confidence
Rate your confidence: HIGH, MEDIUM, or LOW — and explain why."#;

    let user_msg = format!("Please fact-check this claim: \"{}\"", claim);
    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 2048, 0.2).await;

    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Fact-check complete");
            ui.panel(&format!("Fact-Check: {}", &claim[..claim.len().min(60)]), content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "kami",
                    "grounded",
                    Category::AiInteraction,
                    &format!("Fact-checked: {}", &claim[..claim.len().min(100)]),
                    serde_json::json!({
                        "claim": claim,
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
            spinner.finish_with_error(&format!("Fact-check failed: {}", e));
        }
    }

    result.map(|_| ())
}
