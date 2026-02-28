use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Generate fuzz test harnesses for a function.
pub async fn run(config: &Config, ui: &NakamaUI, function: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("mugen", "fuzz");
    let start = Instant::now();

    // Try reading file if function is path:function format
    let (file_content, func_name) = if function.contains(':') {
        let parts: Vec<&str> = function.splitn(2, ':').collect();
        let content = std::fs::read_to_string(parts[0]).unwrap_or_default();
        (content, parts[1].to_string())
    } else {
        (String::new(), function.to_string())
    };

    let spinner = ui.step_start(&format!("Generating fuzz tests for {}...", func_name));

    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Mugen, a fuzz testing expert. Generate fuzz test harnesses for the given function.

Depending on the language, use the appropriate fuzz framework:
- Rust: cargo-fuzz / libfuzzer (use `#![no_main]` and `libfuzzer_sys::fuzz_target!`)
- Python: hypothesis or atheris
- Go: go-fuzz / testing.F
- JavaScript: jsfuzz
- Other: property-based testing approach

Include:
1. The fuzz harness code
2. Seed corpus suggestions (interesting inputs to start with)
3. Instructions to run the fuzzer
4. What kinds of bugs the fuzzer might find

Make the fuzz target robust — handle panics, don't crash on invalid UTF-8, etc."#;

    let user_msg = if file_content.is_empty() {
        format!("Generate a fuzz test harness for: {}", func_name)
    } else {
        let truncated = if file_content.len() > 8000 { &file_content[..8000] } else { &file_content };
        format!(
            "Generate a fuzz test harness for `{}` in this code:\n\n```\n{}\n```",
            func_name, truncated
        )
    };

    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 3072, 0.3).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Fuzz test generated");
            ui.panel(&format!("Fuzz Test: {}", func_name), content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id, "mugen", "fuzz", Category::AiInteraction,
                    &format!("Fuzz test for {}", func_name),
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
