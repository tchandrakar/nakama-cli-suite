//! `shinigami release` — Generate release notes for a version.

use crate::ai_helper;
use crate::git;
use anyhow::Result;
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::types::ModelTier;
use nakama_core::TraceContext;
use nakama_ui::NakamaUI;
use std::time::Instant;

const SYSTEM_PROMPT: &str = r#"You are a release notes writer for a software project. Given a version number and a list of commits since the last release, write professional release notes.

Format:
# Release v<version>

## Highlights
A brief paragraph (2-3 sentences) summarizing the most important changes in this release.

## What's New
### Features
- Feature descriptions

### Bug Fixes
- Bug fix descriptions

### Improvements
- Performance, refactoring, and other improvements

### Breaking Changes
- Any breaking changes (if none, omit this section)

## Contributors
List of unique authors from the commits.

Rules:
- Be concise and user-friendly
- Focus on what matters to users, not internal details
- Include commit hashes in parentheses where relevant
- Omit empty sections
- Output ONLY the release notes, no extra commentary
"#;

/// Run the release subcommand.
pub async fn run(config: &Config, ui: &NakamaUI, version: &str) -> Result<()> {
    let start = Instant::now();
    let trace = TraceContext::new("shinigami", "release");

    let repo = git::open_repo()?;

    // Get commits since last tag (or all commits if no tags)
    let entries = git::get_log_range(&repo, None, None)?;

    if entries.is_empty() {
        ui.warn("No commits found for release notes.");
        return Ok(());
    }

    ui.step_done(&format!(
        "Generating release notes for v{} ({} commits)",
        version,
        entries.len()
    ));

    // Build commit list
    let commit_list: String = entries
        .iter()
        .map(|e| {
            format!("{} {} ({})", e.short_hash, e.summary, e.author)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let commit_list = nakama_core::diff::truncate_diff(&commit_list, 6000);

    let spinner = ui.step_start("Generating release notes...");
    let provider = ai_helper::build_provider(config, ModelTier::Balanced)?;
    let model = config.resolve_model(config.ai.default_provider, ModelTier::Balanced);

    let user_message = format!(
        "Write release notes for version {}.\n\nCommits since last release:\n{}",
        version, commit_list
    );

    let notes = ai_helper::ask_ai(
        provider.as_ref(),
        SYSTEM_PROMPT,
        &user_message,
        &model,
        2000,
        0.3,
    )
    .await?;

    spinner.finish_with_success("Release notes generated");

    ui.panel(&format!("Release v{}", version), notes.trim());

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
            "release",
            Category::AiInteraction,
            "Generated release notes",
            serde_json::json!({}),
            outcome,
            duration_ms,
        );
        let _ = audit_log.log(entry);
    }
}
