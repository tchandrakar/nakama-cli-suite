//! `shinigami branch` — AI-powered branch name suggestion from natural language.

use crate::ai_helper;
use anyhow::Result;
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::types::ModelTier;
use nakama_core::TraceContext;
use nakama_ui::NakamaUI;
use std::io::{self, Write};
use std::time::Instant;

const SYSTEM_PROMPT: &str = r#"You are a git branch naming assistant. Given a natural-language description of a feature, bug fix, or task, suggest exactly 3 well-formed branch names.

Branch naming conventions:
- Use the format: <type>/<short-description>
- Types: feature, fix, refactor, docs, chore, test, ci, perf
- Use lowercase kebab-case for the description part
- Keep it under 50 characters total
- Be specific but concise
- No special characters except hyphens and forward slashes

Output exactly 3 suggestions, one per line, numbered:
1. <branch-name>
2. <branch-name>
3. <branch-name>

Output ONLY the numbered list, no other text.
"#;

/// Run the branch subcommand.
pub async fn run(config: &Config, ui: &NakamaUI, description: &str) -> Result<()> {
    let start = Instant::now();
    let trace = TraceContext::new("shinigami", "branch");

    ui.step_done(&format!("Description: {}", description));

    let spinner = ui.step_start("Suggesting branch names...");
    let provider = ai_helper::build_provider(config, ModelTier::Fast)?;
    let model = config.resolve_model(config.ai.default_provider, ModelTier::Fast);

    let user_message = format!(
        "Suggest branch names for: {}",
        description
    );

    let suggestions = ai_helper::ask_ai(
        provider.as_ref(),
        SYSTEM_PROMPT,
        &user_message,
        &model,
        256,
        0.5,
    )
    .await?;

    spinner.finish_with_success("Branch names generated");

    let suggestions = suggestions.trim();
    ui.panel("Suggested Branch Names", suggestions);

    // Parse the suggestions into a list
    let branch_names: Vec<String> = suggestions
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            // Strip leading number and punctuation: "1. feature/foo" -> "feature/foo"
            let name = trimmed
                .trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ')' || c == ' ');
            let name = name.trim();
            if name.is_empty() {
                None
            } else {
                Some(name.to_string())
            }
        })
        .collect();

    if branch_names.is_empty() {
        ui.warn("Could not parse branch name suggestions.");
        return Ok(());
    }

    // Prompt user to pick one or skip
    if ui.is_tty() {
        let choice = prompt_branch_choice(ui, &branch_names)?;
        if let Some(chosen) = choice {
            let repo = crate::git::open_repo()?;
            let head_commit = repo.head()?.peel_to_commit()?;
            repo.branch(&chosen, &head_commit, false)?;

            // Also checkout the new branch
            let refname = format!("refs/heads/{}", chosen);
            let obj = repo.revparse_single(&refname)?;
            repo.checkout_tree(&obj, None)?;
            repo.set_head(&refname)?;

            ui.success(&format!("Created and checked out branch '{}'", chosen));
        } else {
            ui.warn("No branch created.");
        }
    }

    let duration = start.elapsed().as_millis() as u64;
    log_audit(config, &trace, Outcome::Success, duration);

    Ok(())
}

fn prompt_branch_choice(ui: &NakamaUI, names: &[String]) -> Result<Option<String>> {
    if !ui.is_tty() {
        return Ok(None);
    }

    print!("  > Pick a branch (1-{}) or 'n' to skip: ", names.len());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let trimmed = input.trim().to_lowercase();
    if trimmed == "n" || trimmed == "no" || trimmed.is_empty() {
        return Ok(None);
    }

    if let Ok(idx) = trimmed.parse::<usize>() {
        if idx >= 1 && idx <= names.len() {
            return Ok(Some(names[idx - 1].clone()));
        }
    }

    ui.warn("Invalid selection.");
    Ok(None)
}

fn log_audit(config: &Config, trace: &TraceContext, outcome: Outcome, duration_ms: u64) {
    if !config.audit.enabled {
        return;
    }
    if let Ok(audit_log) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id,
            "shinigami",
            "branch",
            Category::AiInteraction,
            "AI-suggested branch name creation",
            serde_json::json!({}),
            outcome,
            duration_ms,
        );
        let _ = audit_log.log(entry);
    }
}
