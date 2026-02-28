use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::{NakamaError, NakamaResult};
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::path::Path;
use std::time::Instant;

/// Analyze a source file and suggest mutations to validate test quality.
pub async fn run(config: &Config, ui: &NakamaUI, file: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("mugen", "mutate");
    let start = Instant::now();

    let path = Path::new(file);
    if !path.exists() {
        return Err(NakamaError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", file),
        )));
    }

    let content = std::fs::read_to_string(path)?;
    let spinner = ui.step_start(&format!("Analyzing {} for mutation testing...", file));

    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Mugen, a mutation testing expert. Analyze the given source code and suggest mutations that would test whether the existing test suite catches bugs.

For each mutation:
1. **Location**: Line number or code section
2. **Original**: The original code
3. **Mutation**: The mutated version
4. **Type**: Category (boundary, operator, logic, return value, null check, etc.)
5. **Expected**: Should tests catch this? If not, what test is missing?

Common mutation types:
- Replace `>` with `>=`, `<` with `<=`
- Replace `&&` with `||`
- Replace `+` with `-`
- Remove null checks
- Return early with default values
- Off-by-one errors
- Swap true/false returns

Suggest 8-12 mutations, prioritized by likelihood of catching real bugs."#;

    let truncated = if content.len() > 10000 { &content[..10000] } else { &content };
    let user_msg = format!("Suggest mutations for this file ({}):\n\n```\n{}\n```", file, truncated);
    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 3072, 0.4).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(analysis) => {
            spinner.finish_with_success("Mutation analysis complete");
            ui.panel(&format!("Mutation Testing: {}", file), analysis);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id, "mugen", "mutate", Category::AiInteraction,
                    &format!("Mutation analysis for {}", file),
                    serde_json::json!({ "file": file, "model": model }),
                    Outcome::Success, elapsed,
                );
                let _ = audit.log(entry);
            }
        }
        Err(e) => spinner.finish_with_error(&format!("Failed: {}", e)),
    }

    result.map(|_| ())
}
