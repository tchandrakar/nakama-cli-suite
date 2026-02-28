use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Analyze test coverage in the current directory and suggest missing tests.
pub async fn run(config: &Config, ui: &NakamaUI) -> NakamaResult<()> {
    let trace = TraceContext::new("mugen", "cover");
    let start = Instant::now();

    let spinner = ui.step_start("Analyzing codebase for test coverage...");

    // Walk directory and collect source/test file info
    let cwd = std::env::current_dir()?;
    let mut source_files = Vec::new();
    let mut test_files = Vec::new();

    walk_dir(&cwd, &mut source_files, &mut test_files, 0);

    spinner.finish_with_success(&format!(
        "Found {} source files, {} test files",
        source_files.len(),
        test_files.len(),
    ));

    if source_files.is_empty() {
        ui.warn("No source files found in current directory.");
        return Ok(());
    }

    // Display coverage summary
    let rows: Vec<Vec<String>> = vec![
        vec!["Source files".to_string(), source_files.len().to_string()],
        vec!["Test files".to_string(), test_files.len().to_string()],
        vec![
            "Coverage ratio".to_string(),
            if source_files.is_empty() {
                "N/A".to_string()
            } else {
                format!("{:.0}%", (test_files.len() as f64 / source_files.len() as f64) * 100.0)
            },
        ],
    ];
    ui.table(&["Metric", "Value"], rows);

    // Find files without corresponding test files
    let untested: Vec<&String> = source_files
        .iter()
        .filter(|sf| {
            let stem = std::path::Path::new(sf)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            !test_files.iter().any(|tf| tf.contains(stem))
        })
        .collect();

    if untested.is_empty() {
        ui.success("All source files appear to have corresponding test files!");
        return Ok(());
    }

    // Send untested files to AI for analysis
    let spinner = ui.step_start("Analyzing gaps with AI...");
    let (provider, model) = make_provider(config, ModelTier::Fast)?;

    let file_list = untested.iter().take(30).map(|f| f.as_str()).collect::<Vec<_>>().join("\n");

    let system_prompt = r#"You are Mugen, a test coverage analyst. Given a list of source files without tests, prioritize which files need tests most urgently.

For each file, assess:
1. Risk level (HIGH/MEDIUM/LOW) based on the filename/path
2. Why tests are important for this file
3. What types of tests to write

Output a prioritized list with the most critical files first."#;

    let user_msg = format!(
        "These {} source files have no corresponding test files:\n\n{}",
        untested.len(),
        file_list,
    );

    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 2048, 0.3).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(analysis) => {
            spinner.finish_with_success("Coverage analysis complete");
            ui.panel("Test Coverage Gaps", analysis);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id, "mugen", "cover", Category::AiInteraction,
                    "Analyzed test coverage gaps",
                    serde_json::json!({
                        "source_files": source_files.len(),
                        "test_files": test_files.len(),
                        "untested": untested.len(),
                    }),
                    Outcome::Success, elapsed,
                );
                let _ = audit.log(entry);
            }
        }
        Err(e) => spinner.finish_with_error(&format!("Failed: {}", e)),
    }

    result.map(|_| ())
}

fn walk_dir(dir: &std::path::Path, source: &mut Vec<String>, tests: &mut Vec<String>, depth: u32) {
    if depth > 10 {
        return;
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden dirs, target, node_modules, etc.
        if name.starts_with('.') || name == "target" || name == "node_modules" || name == "vendor" {
            continue;
        }

        if path.is_dir() {
            walk_dir(&path, source, tests, depth + 1);
        } else if is_source_file(&name) {
            let rel = path.to_string_lossy().to_string();
            if is_test_file(&name) {
                tests.push(rel);
            } else {
                source.push(rel);
            }
        }
    }
}

fn is_source_file(name: &str) -> bool {
    matches!(
        std::path::Path::new(name).extension().and_then(|e| e.to_str()),
        Some("rs" | "py" | "js" | "ts" | "go" | "java" | "rb" | "cpp" | "c")
    )
}

fn is_test_file(name: &str) -> bool {
    name.contains("_test.") || name.contains(".test.") || name.contains("test_") || name.contains("_spec.")
}
