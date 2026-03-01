//! `shinigami review` — AI-powered code review of uncommitted changes.

use crate::ai_helper;
use crate::git;
use anyhow::Result;
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::types::ModelTier;
use nakama_core::TraceContext;
use nakama_ui::NakamaUI;
use std::time::Instant;

const SYSTEM_PROMPT: &str = r#"You are a senior software engineer performing a thorough code review.

You will be given a diff of uncommitted changes in a repository. Provide a structured review with these sections:

## Summary
A 1-2 sentence summary of what the changes do.

## Issues Found
List any bugs, logic errors, security concerns, or correctness problems. For each:
- Severity: [critical / warning / info]
- File/area affected
- What the issue is and how to fix it

If no issues found, say "No issues found."

## Suggestions
List style, performance, maintainability, or readability improvements. These are optional, non-blocking suggestions.

## Verdict
One of:
- APPROVE — changes look good, no blocking issues
- REQUEST_CHANGES — there are issues that should be fixed before committing
- COMMENT — informational review, up to the developer

Keep the review concise and actionable. Do not repeat the diff back.
Do not use code fences for the overall structure — just use markdown headings and bullet points.
"#;

const MAX_DIFF_CHARS: usize = 10000;

/// Run the review subcommand.
pub async fn run(config: &Config, ui: &NakamaUI) -> Result<()> {
    let start = Instant::now();
    let trace = TraceContext::new("shinigami", "review");

    let repo = git::open_repo()?;

    // Get all uncommitted changes (staged + unstaged)
    let diff = git::get_all_uncommitted_diff(&repo)?;

    if diff.trim().is_empty() {
        ui.warn("No uncommitted changes found. Nothing to review.");
        return Ok(());
    }

    let branch = git::current_branch(&repo).unwrap_or_else(|_| "unknown".to_string());
    ui.step_done(&format!("Reviewing uncommitted changes on branch '{}'", branch));

    let diff_for_ai = nakama_core::diff::compress_diff(&diff, MAX_DIFF_CHARS);

    let spinner = ui.step_start("AI is reviewing your changes...");
    let provider = ai_helper::build_provider(config, ModelTier::Balanced)?;
    let model = config.resolve_model(config.ai.default_provider, ModelTier::Balanced);

    let user_message = format!(
        "Review the following uncommitted changes on branch '{}':\n\n```diff\n{}\n```",
        branch, diff_for_ai
    );

    let review = ai_helper::ask_ai(
        provider.as_ref(),
        SYSTEM_PROMPT,
        &user_message,
        &model,
        1500,
        0.2,
    )
    .await?;

    spinner.finish_with_success("Review complete");

    ui.panel("Code Review", review.trim());

    // Audit
    let duration = start.elapsed().as_millis() as u64;
    log_audit(config, &trace, Outcome::Success, duration);

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
            "review",
            Category::AiInteraction,
            "AI code review of uncommitted changes",
            serde_json::json!({}),
            outcome,
            duration_ms,
        );
        let _ = audit_log.log(entry);
    }
}
