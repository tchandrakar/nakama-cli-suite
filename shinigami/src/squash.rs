//! `shinigami squash` — Interactive squash with AI-generated combined commit message.
//!
//! This does a soft-reset style squash: it resets the branch pointer back N
//! commits while keeping the working tree and index intact, then creates a
//! single new commit with an AI-generated message summarizing all the squashed
//! commits.

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

You will be given a list of commit messages that are about to be squashed into a single commit. Write a single, well-crafted conventional commit message that summarizes all the changes.

Format:
<type>(<scope>): <description>

<body>

Rules:
- type MUST be one of: feat, fix, refactor, docs, test, chore, ci, perf, style
- If commits span multiple types, pick the most significant one (feat > fix > refactor > chore)
- scope is optional — use it if all commits share a common area
- description: lowercase, imperative mood, max 72 chars, no period
- body: summarize what was done across all commits, wrapped at 72 chars
- If multiple features/fixes, use bullet points in the body
- Output ONLY the commit message, no markdown fences or commentary
"#;

/// Run the squash subcommand.
pub async fn run(config: &Config, ui: &NakamaUI) -> Result<()> {
    let start = Instant::now();
    let trace = TraceContext::new("shinigami", "squash");

    let repo = git::open_repo()?;
    let branch = git::current_branch(&repo)?;

    // Get recent commits
    let all_commits = git::get_log(&repo, 30)?;
    if all_commits.len() < 2 {
        ui.warn("Need at least 2 commits to squash. Not enough history.");
        return Ok(());
    }

    ui.step_done(&format!(
        "On branch '{}' with {} recent commits",
        branch,
        all_commits.len()
    ));

    // Show recent commits in a table so user can decide how many to squash
    let table_rows: Vec<Vec<String>> = all_commits
        .iter()
        .enumerate()
        .take(15)
        .map(|(i, e)| {
            vec![
                format!("{}", i + 1),
                e.short_hash.clone(),
                e.summary.clone(),
                e.date.clone(),
            ]
        })
        .collect();
    ui.table(&["#", "Hash", "Message", "Date"], table_rows);

    // Ask how many to squash
    let count = prompt_squash_count(ui, all_commits.len())?;
    if count < 2 {
        ui.warn("Need at least 2 commits to squash. Aborting.");
        return Ok(());
    }

    let commits_to_squash = &all_commits[..count];

    ui.step_done(&format!("Squashing {} commits", count));

    // Build commit list for AI
    let commit_list: String = commits_to_squash
        .iter()
        .map(|e| {
            let body_str = e.body.as_deref().unwrap_or("");
            if body_str.is_empty() {
                format!("- {} {}", e.short_hash, e.summary)
            } else {
                format!("- {} {}\n  {}", e.short_hash, e.summary, body_str)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Ask AI for a squash message
    let spinner = ui.step_start("Generating squash commit message...");
    let provider = ai_helper::build_provider(config, ModelTier::Balanced)?;
    let model = config.resolve_model(config.ai.default_provider, ModelTier::Balanced);

    let user_message = format!(
        "Squash these {} commits into one:\n\n{}\n\nWrite a single conventional commit message.",
        count, commit_list
    );

    let squash_msg = ai_helper::ask_ai(
        provider.as_ref(),
        SYSTEM_PROMPT,
        &user_message,
        &model,
        512,
        0.3,
    )
    .await?;

    let squash_msg = squash_msg.trim().to_string();
    spinner.finish_with_success("Squash message generated");

    ui.panel("Squash Commit Message", &squash_msg);

    // Confirm before squashing
    if ui.is_tty() {
        let confirmed = ui.confirm("Proceed with squash? This will rewrite history.")?;
        if !confirmed {
            ui.warn("Squash aborted.");
            log_audit(config, &trace, Outcome::Skipped, start.elapsed().as_millis() as u64);
            return Ok(());
        }
    }

    // Perform the squash via soft reset + recommit
    perform_squash(&repo, count, &squash_msg)?;

    ui.success(&format!(
        "Squashed {} commits into one on '{}'",
        count, branch
    ));

    let duration = start.elapsed().as_millis() as u64;
    log_audit(config, &trace, Outcome::Success, duration);

    Ok(())
}

fn prompt_squash_count(ui: &NakamaUI, max: usize) -> Result<usize> {
    if !ui.is_tty() {
        bail!("Cannot prompt for squash count in non-interactive mode");
    }

    print!("  > How many commits to squash? (2-{}): ", max.min(30));
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let trimmed = input.trim();
    let count: usize = trimmed
        .parse()
        .context("Expected a number")?;

    if count < 2 || count > max {
        bail!("Count must be between 2 and {}", max);
    }

    Ok(count)
}

/// Perform a soft-reset squash: reset HEAD back N commits, then create a new commit.
fn perform_squash(repo: &git2::Repository, count: usize, message: &str) -> Result<()> {
    // Find the commit to reset to (N commits back from HEAD)
    let head = repo.head()?.peel_to_commit()?;
    let mut target = head.clone();
    for _ in 0..count {
        target = target
            .parent(0)
            .context("Not enough parent commits for squash")?;
    }

    // Soft reset: moves HEAD to target, keeps index and working dir
    let target_obj = target.as_object();
    repo.reset(target_obj, git2::ResetType::Soft, None)
        .context("Failed to soft-reset for squash")?;

    // Now create a new commit with the combined message
    // The index still has all the changes from the squashed commits
    git::create_commit(repo, message)?;

    Ok(())
}

fn log_audit(config: &Config, trace: &TraceContext, outcome: Outcome, duration_ms: u64) {
    if !config.audit.enabled {
        return;
    }
    if let Ok(audit_log) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id,
            "shinigami",
            "squash",
            Category::AiInteraction,
            "Interactive squash with AI message",
            serde_json::json!({}),
            outcome,
            duration_ms,
        );
        let _ = audit_log.log(entry);
    }
}
