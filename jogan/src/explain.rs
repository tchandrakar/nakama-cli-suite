//! Take an error message and send to AI for explanation.

use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Take an error message or infrastructure concept and explain it via AI.
pub async fn run(config: &Config, ui: &NakamaUI, resource: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("jogan", "explain");
    let start = Instant::now();

    let spinner = ui.step_start(&format!("Explaining: {}", truncate(resource, 60)));
    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Jogan, an infrastructure debugging assistant. The user will provide an error message, infrastructure resource, or concept. Explain it clearly:

1. **What It Means**: A clear, plain-language explanation.
2. **Common Causes**: Why this error or situation typically occurs.
3. **How to Fix**: Step-by-step resolution instructions.
4. **Prevention**: How to prevent this from happening again.

If it is a concept rather than an error (e.g., "k8s pod", "nginx config"), explain what it is, how it works, and common best practices.

Be concise and practical."#;

    let result = ask_ai(provider.as_ref(), system_prompt, resource, &model, 2048, 0.3).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Explanation ready");
            ui.panel(&format!("Explain: {}", truncate(resource, 60)), content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "jogan",
                    "explain",
                    Category::AiInteraction,
                    &format!("Explained: {}", truncate(resource, 100)),
                    serde_json::json!({
                        "resource": resource,
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
            spinner.finish_with_error(&format!("Explanation failed: {}", e));

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "jogan",
                    "explain",
                    Category::AiInteraction,
                    &format!("Explain failed for: {}", truncate(resource, 100)),
                    serde_json::json!({ "resource": resource, "error": e.to_string() }),
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
