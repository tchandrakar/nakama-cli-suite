//! `shinigami commit` — AI-generated conventional commit messages from staged changes.

use crate::ai_helper;
use crate::git;
use anyhow::{bail, Context, Result};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::types::ModelTier;
use nakama_core::TraceContext;
use nakama_ui::NakamaUI;
use std::io::{self, Write};
use std::time::Instant;

const SYSTEM_PROMPT: &str = r#"You are an expert at writing git commit messages following the Conventional Commits specification.

Given a diff of staged changes, produce a commit message in this exact format:

<type>(<scope>): <description>

<body>

Rules:
- type MUST be one of: feat, fix, refactor, docs, test, chore, ci, perf, style
- scope is optional but recommended — it should be a short module/area name (lowercase, no spaces)
- description MUST be lowercase, imperative mood, no period at the end, max 72 chars
- body should explain WHAT changed and WHY (not how), wrapped at 72 chars
- If there are breaking changes, add a footer: BREAKING CHANGE: <explanation>
- Do NOT include any markdown formatting, code fences, or commentary
- Output ONLY the commit message, nothing else

Examples of good commit messages:
  feat(auth): add OAuth2 PKCE flow for CLI login
  fix(parser): handle empty input without panicking
  refactor(db): extract connection pooling into separate module
  docs(readme): add quickstart guide for new contributors
  chore(deps): bump tokio to 1.35
"#;

const MAX_DIFF_CHARS: usize = 8000;

/// Run the commit subcommand.
pub async fn run(config: &Config, ui: &NakamaUI) -> Result<()> {
    let start = Instant::now();
    let trace = TraceContext::new("shinigami", "commit");

    // 1. Open repo and get staged diff
    let repo = git::open_repo()?;
    let staged_files = git::staged_file_names(&repo)?;

    if staged_files.is_empty() {
        ui.warn("No staged changes found. Stage files with `git add` first.");
        return Ok(());
    }

    ui.step_done(&format!(
        "Found {} staged file(s): {}",
        staged_files.len(),
        staged_files.join(", ")
    ));

    let diff = git::get_staged_diff(&repo)?;
    if diff.trim().is_empty() {
        ui.warn("Staged diff is empty. Nothing to commit.");
        return Ok(());
    }

    // 2. Truncate diff if too large
    let diff_for_ai = git::truncate_diff(&diff, MAX_DIFF_CHARS);

    // 3. Build AI provider and request commit message
    let spinner = ui.step_start("Generating commit message...");
    let provider = ai_helper::build_provider(config, ModelTier::Balanced)?;
    let model = config.resolve_model(config.ai.default_provider, ModelTier::Balanced);

    let user_message = format!(
        "Here are the staged changes:\n\n```diff\n{}\n```\n\nFiles changed: {}\n\nGenerate a conventional commit message for these changes.",
        diff_for_ai,
        staged_files.join(", ")
    );

    let commit_msg = ai_helper::ask_ai(
        provider.as_ref(),
        SYSTEM_PROMPT,
        &user_message,
        &model,
        512,
        0.3,
    )
    .await?;

    let commit_msg = commit_msg.trim().to_string();
    spinner.finish_with_success("Commit message generated");

    // 4. Display the suggested message
    ui.panel("Suggested Commit Message", &commit_msg);

    // 5. Ask user to confirm
    let action = prompt_commit_action(ui)?;

    match action {
        CommitAction::Accept => {
            let oid = git::create_commit(&repo, &commit_msg)?;
            ui.success(&format!("Committed: {}", &oid.to_string()[..7]));
            log_audit(&config, &trace, "commit", &commit_msg, Outcome::Success, start.elapsed().as_millis() as u64);
        }
        CommitAction::Edit => {
            ui.info("Opening editor...");
            let edited = edit_message(&commit_msg)?;
            if edited.trim().is_empty() {
                ui.warn("Empty commit message. Aborting.");
                log_audit(&config, &trace, "commit", "aborted (empty edit)", Outcome::Skipped, start.elapsed().as_millis() as u64);
                return Ok(());
            }
            let oid = git::create_commit(&repo, &edited)?;
            ui.success(&format!("Committed: {}", &oid.to_string()[..7]));
            log_audit(&config, &trace, "commit", &edited, Outcome::Success, start.elapsed().as_millis() as u64);
        }
        CommitAction::Reject => {
            ui.warn("Commit aborted.");
            log_audit(&config, &trace, "commit", "rejected by user", Outcome::Skipped, start.elapsed().as_millis() as u64);
        }
    }

    Ok(())
}

#[derive(Debug)]
enum CommitAction {
    Accept,
    Edit,
    Reject,
}

fn prompt_commit_action(ui: &NakamaUI) -> Result<CommitAction> {
    if !ui.is_tty() {
        // Non-interactive: auto-accept
        return Ok(CommitAction::Accept);
    }

    print!("  > Accept (y), edit (e), or reject (n)? [y/e/n] ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().to_lowercase().as_str() {
        "" | "y" | "yes" => Ok(CommitAction::Accept),
        "e" | "edit" => Ok(CommitAction::Edit),
        "n" | "no" => Ok(CommitAction::Reject),
        other => {
            eprintln!("  Unknown option '{}', defaulting to reject.", other);
            Ok(CommitAction::Reject)
        }
    }
}

/// Open the user's $EDITOR (or vi) with the message pre-filled.
fn edit_message(initial: &str) -> Result<String> {
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vi".to_string());

    let tmp_dir = std::env::temp_dir();
    let tmp_file = tmp_dir.join("shinigami_commit_msg.txt");
    std::fs::write(&tmp_file, initial)?;

    let status = std::process::Command::new(&editor)
        .arg(&tmp_file)
        .status()
        .with_context(|| format!("Failed to launch editor '{}'", editor))?;

    if !status.success() {
        bail!("Editor exited with non-zero status");
    }

    let edited = std::fs::read_to_string(&tmp_file)?;
    let _ = std::fs::remove_file(&tmp_file);
    Ok(edited)
}

fn log_audit(config: &Config, trace: &TraceContext, command: &str, action: &str, outcome: Outcome, duration_ms: u64) {
    if !config.audit.enabled {
        return;
    }
    if let Ok(audit_log) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id,
            "shinigami",
            command,
            Category::AiInteraction,
            action,
            serde_json::json!({}),
            outcome,
            duration_ms,
        );
        let _ = audit_log.log(entry);
    }
}
