use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::{NakamaError, NakamaResult};
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::path::Path;
use std::time::Instant;

/// Review an existing test file for quality and completeness.
pub async fn run(config: &Config, ui: &NakamaUI, test_file: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("mugen", "review");
    let start = Instant::now();

    let path = Path::new(test_file);
    if !path.exists() {
        return Err(NakamaError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Test file not found: {}", test_file),
        )));
    }

    let content = std::fs::read_to_string(path)?;
    let spinner = ui.step_start(&format!("Reviewing test file: {}...", test_file));

    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Mugen, a test quality expert. Review the given test file and provide:

## Quality Score
Rate 1-10 with justification.

## Strengths
What the tests do well.

## Missing Coverage
Functions or branches not covered by tests.

## Improvements
Specific suggestions to improve test quality:
- Missing edge cases
- Brittle assertions
- Test isolation issues
- Missing error path tests
- Naming/organization improvements

## Suggested New Tests
List specific new tests that should be added."#;

    let truncated = if content.len() > 10000 {
        format!("{}...\n[Truncated]", &content[..10000])
    } else {
        content.clone()
    };

    let user_msg = format!("Review this test file ({}):\n\n```\n{}\n```", test_file, truncated);
    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 3072, 0.3).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(review) => {
            spinner.finish_with_success("Review complete");
            ui.panel(&format!("Test Review: {}", test_file), review);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id, "mugen", "review", Category::AiInteraction,
                    &format!("Reviewed test file {}", test_file),
                    serde_json::json!({ "file": test_file, "model": model }),
                    Outcome::Success, elapsed,
                );
                let _ = audit.log(entry);
            }
        }
        Err(e) => spinner.finish_with_error(&format!("Failed: {}", e)),
    }

    result.map(|_| ())
}
