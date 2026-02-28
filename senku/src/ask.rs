use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

/// Collect a codebase summary plus the user question, then send to AI for analysis.
pub async fn run(config: &Config, ui: &NakamaUI, question: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("senku", "ask");
    let start = Instant::now();

    // Collect codebase summary
    let summary_spinner = ui.step_start("Collecting codebase summary...");
    let cwd = std::env::current_dir().map_err(|e| nakama_core::error::NakamaError::Tool {
        tool: "senku".to_string(),
        message: format!("Failed to get current directory: {}", e),
    })?;

    let summary = collect_codebase_summary(&cwd);
    summary_spinner.finish_with_success("Codebase summary collected");

    // Ask AI
    let ai_spinner = ui.step_start("Asking AI about the codebase...");
    match make_provider(config, ModelTier::Balanced) {
        Ok((provider, model)) => {
            let system_prompt = r#"You are Senku, a codebase knowledge assistant. You help developers understand their codebase.

Given a summary of the codebase structure and a question, provide a helpful, accurate answer.

Rules:
1. Base your answer on the codebase summary provided.
2. If you cannot determine the answer from the summary alone, say so clearly.
3. Be concise but thorough.
4. Reference specific files and directories when relevant.
5. Use code formatting for file paths and technical terms."#;

            let user_msg = format!(
                "Codebase Summary:\n{}\n\nQuestion: {}",
                summary, question
            );

            match ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 2048, 0.3).await {
                Ok(answer) => {
                    ai_spinner.finish_with_success("Answer received");
                    ui.panel("Answer", &answer);
                }
                Err(e) => {
                    ai_spinner.finish_with_error(&format!("AI query failed: {}", e));
                }
            }
        }
        Err(e) => {
            ai_spinner.finish_with_error(&format!("AI unavailable: {}", e));
        }
    }

    let elapsed = start.elapsed().as_millis() as u64;

    // Audit log
    if let Ok(audit) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id,
            "senku",
            "ask",
            Category::AiInteraction,
            "Asked question about codebase",
            serde_json::json!({
                "question": question,
                "directory": cwd.display().to_string(),
            }),
            Outcome::Success,
            elapsed,
        );
        let _ = audit.log(entry);
    }

    Ok(())
}

/// Collect a textual summary of the codebase for AI context.
fn collect_codebase_summary(dir: &Path) -> String {
    let mut ext_counts: HashMap<String, usize> = HashMap::new();
    let mut key_files: Vec<String> = Vec::new();
    let mut total_files: usize = 0;

    collect_stats(dir, dir, &mut ext_counts, &mut key_files, &mut total_files, 0);

    let mut summary = format!("Project root: {}\nTotal files: {}\n\n", dir.display(), total_files);

    // File type breakdown
    let mut ext_list: Vec<(&String, &usize)> = ext_counts.iter().collect();
    ext_list.sort_by(|a, b| b.1.cmp(a.1));
    summary.push_str("File types:\n");
    for (ext, count) in &ext_list {
        summary.push_str(&format!("  {} - {} files\n", ext, count));
    }

    // Key files
    if !key_files.is_empty() {
        summary.push_str("\nKey files found:\n");
        for f in &key_files[..key_files.len().min(50)] {
            summary.push_str(&format!("  {}\n", f));
        }
    }

    summary
}

fn collect_stats(
    root: &Path,
    dir: &Path,
    ext_counts: &mut HashMap<String, usize>,
    key_files: &mut Vec<String>,
    total_files: &mut usize,
    depth: usize,
) {
    if depth > 8 {
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name.starts_with('.') || name == "target" || name == "node_modules" || name == "__pycache__" {
            continue;
        }

        if path.is_dir() {
            collect_stats(root, &path, ext_counts, key_files, total_files, depth + 1);
        } else if path.is_file() {
            *total_files += 1;
            let ext = path
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_else(|| "(no ext)".to_string());
            *ext_counts.entry(ext).or_insert(0) += 1;

            // Track key project files
            let lower_name = name.to_lowercase();
            if lower_name == "cargo.toml"
                || lower_name == "package.json"
                || lower_name == "requirements.txt"
                || lower_name == "main.rs"
                || lower_name == "lib.rs"
                || lower_name == "mod.rs"
                || lower_name == "makefile"
                || lower_name == "dockerfile"
            {
                let relative = path.strip_prefix(root).unwrap_or(&path);
                key_files.push(relative.display().to_string());
            }
        }
    }
}
