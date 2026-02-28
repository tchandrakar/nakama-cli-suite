use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Ask a question and get a grounded, sourced answer.
pub async fn run(config: &Config, ui: &NakamaUI, question: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("kami", "ask");
    let spinner = ui.step_start("Thinking...");
    let start = Instant::now();

    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Kami, a knowledgeable Q&A assistant. Answer questions accurately and concisely.

Rules:
1. Be direct — answer the question first, then provide context.
2. If you're not sure, say "I'm not certain, but..." and give your best understanding.
3. For technical questions, include code examples when helpful.
4. For factual questions, be precise and cite timeframes when relevant.
5. Structure longer answers with headers and bullet points.
6. Keep answers focused — don't over-explain simple questions."#;

    let result = ask_ai(provider.as_ref(), system_prompt, question, &model, 2048, 0.3).await;

    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Answer ready");
            ui.panel("Answer", content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "kami",
                    "ask",
                    Category::AiInteraction,
                    &format!("Q&A: {}", &question[..question.len().min(100)]),
                    serde_json::json!({
                        "question": question,
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
            spinner.finish_with_error(&format!("Failed: {}", e));
        }
    }

    result.map(|_| ())
}
