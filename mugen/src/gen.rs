use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::{NakamaError, NakamaResult};
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::path::Path;
use std::time::Instant;

/// Generate tests for a target file.
pub async fn run(config: &Config, ui: &NakamaUI, target: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("mugen", "gen");
    let start = Instant::now();

    // Read the target file
    let path = Path::new(target);
    if !path.exists() {
        ui.error(&format!("File not found: {}", target));
        return Err(NakamaError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", target),
        )));
    }

    let content = std::fs::read_to_string(path)?;
    let lang = detect_language(target);

    let spinner = ui.step_start(&format!("Generating {} tests for {}...", lang, target));

    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = format!(
        r#"You are Mugen, an expert test generator. Generate comprehensive tests for the given {} source code.

Rules:
1. Generate unit tests that cover all public functions/methods.
2. Include edge cases: empty inputs, boundary values, error conditions.
3. Use the standard testing framework for the language ({}).
4. Include descriptive test names that explain what's being tested.
5. Add comments explaining the test strategy for each test.
6. Group related tests together.
7. Generate the test code ONLY — no explanations outside of code comments.

Output format: Return the complete test file/module ready to be saved."#,
        lang,
        test_framework(&lang),
    );

    let truncated = if content.len() > 10000 {
        format!("{}...\n[Truncated]", &content[..10000])
    } else {
        content.clone()
    };

    let user_msg = format!("Generate tests for this {} file ({}):\n\n```{}\n{}\n```", lang, target, lang.to_lowercase(), truncated);
    let result = ask_ai(provider.as_ref(), &system_prompt, &user_msg, &model, 4096, 0.3).await;

    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(tests) => {
            spinner.finish_with_success("Tests generated");
            ui.panel(&format!("Generated Tests for {}", target), tests);

            // Suggest output file
            let test_file = suggest_test_file(target, &lang);
            ui.step_done(&format!("Suggested output: {}", test_file));

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "mugen",
                    "gen",
                    Category::AiInteraction,
                    &format!("Generated tests for {}", target),
                    serde_json::json!({ "target": target, "language": lang, "model": model }),
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

fn detect_language(path: &str) -> String {
    match Path::new(path).extension().and_then(|e| e.to_str()) {
        Some("rs") => "Rust".to_string(),
        Some("py") => "Python".to_string(),
        Some("js") => "JavaScript".to_string(),
        Some("ts") => "TypeScript".to_string(),
        Some("go") => "Go".to_string(),
        Some("java") => "Java".to_string(),
        Some("rb") => "Ruby".to_string(),
        Some("cpp" | "cc" | "cxx") => "C++".to_string(),
        Some("c") => "C".to_string(),
        Some(ext) => ext.to_string(),
        None => "Unknown".to_string(),
    }
}

fn test_framework(lang: &str) -> &'static str {
    match lang {
        "Rust" => "#[cfg(test)] mod tests with #[test]",
        "Python" => "pytest",
        "JavaScript" | "TypeScript" => "Jest or Vitest",
        "Go" => "testing package",
        "Java" => "JUnit 5",
        "Ruby" => "RSpec",
        "C++" => "Google Test",
        _ => "standard test framework",
    }
}

fn suggest_test_file(path: &str, lang: &str) -> String {
    let p = Path::new(path);
    let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("test");
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("txt");

    match lang {
        "Rust" => format!("{}_test.rs", stem),
        "Python" => format!("test_{}.py", stem),
        "JavaScript" => format!("{}.test.js", stem),
        "TypeScript" => format!("{}.test.ts", stem),
        "Go" => format!("{}_test.go", stem),
        _ => format!("{}_test.{}", stem, ext),
    }
}
