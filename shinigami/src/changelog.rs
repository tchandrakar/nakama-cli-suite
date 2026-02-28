//! `shinigami reap` — Generates a changelog from recent commits using AI.

use crate::ai_helper;
use crate::git;
use anyhow::Result;
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::types::ModelTier;
use nakama_core::TraceContext;
use nakama_ui::NakamaUI;
use std::time::Instant;

const SYSTEM_PROMPT: &str = r#"You are a changelog writer. Given a list of git commit messages (some may follow conventional commits format), produce a well-organized changelog.

Format the changelog as markdown with these sections (omit empty sections):

### Features
- Description of new features

### Bug Fixes
- Description of bug fixes

### Performance
- Performance improvements

### Refactoring
- Code refactoring changes

### Documentation
- Documentation updates

### Other
- Anything that doesn't fit above

Rules:
- Group commits by type (feat -> Features, fix -> Bug Fixes, etc.)
- Rewrite each entry to be user-friendly and concise
- Include the short commit hash in parentheses at the end of each entry
- Do not include merge commits or trivial changes (like version bumps) unless significant
- Output ONLY the changelog markdown, no extra commentary
"#;

/// Run the reap (changelog) subcommand.
pub async fn run(
    config: &Config,
    ui: &NakamaUI,
    from: Option<String>,
    to: Option<String>,
) -> Result<()> {
    let start = Instant::now();
    let trace = TraceContext::new("shinigami", "reap");

    let repo = git::open_repo()?;

    // Get commits in range
    let entries = git::get_log_range(
        &repo,
        from.as_deref(),
        to.as_deref(),
    )?;

    if entries.is_empty() {
        ui.warn("No commits found in the specified range.");
        return Ok(());
    }

    let from_label = from.as_deref().unwrap_or("(earliest/last tag)");
    let to_label = to.as_deref().unwrap_or("HEAD");
    ui.step_done(&format!(
        "Found {} commit(s) between {} and {}",
        entries.len(),
        from_label,
        to_label
    ));

    // Display commits in a table
    let table_rows: Vec<Vec<String>> = entries
        .iter()
        .take(20) // show at most 20 in the table
        .map(|e| vec![e.short_hash.clone(), e.summary.clone(), e.author.clone()])
        .collect();
    ui.table(&["Hash", "Message", "Author"], table_rows);

    // Build commit list for AI
    let commit_list: String = entries
        .iter()
        .map(|e| {
            let body_str = e.body.as_deref().unwrap_or("");
            if body_str.is_empty() {
                format!("{} {}", e.short_hash, e.summary)
            } else {
                format!("{} {}\n  {}", e.short_hash, e.summary, body_str)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Truncate if very long
    let commit_list = git::truncate_diff(&commit_list, 6000);

    let spinner = ui.step_start("Generating changelog...");
    let provider = ai_helper::build_provider(config, ModelTier::Balanced)?;
    let model = config.resolve_model(config.ai.default_provider, ModelTier::Balanced);

    let user_message = format!(
        "Generate a changelog from these {} commits (range: {} .. {}):\n\n{}",
        entries.len(),
        from_label,
        to_label,
        commit_list
    );

    let changelog = ai_helper::ask_ai(
        provider.as_ref(),
        SYSTEM_PROMPT,
        &user_message,
        &model,
        1500,
        0.3,
    )
    .await?;

    spinner.finish_with_success("Changelog generated");

    ui.panel("Changelog", changelog.trim());

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
            "reap",
            Category::AiInteraction,
            "Generated changelog from commits",
            serde_json::json!({}),
            outcome,
            duration_ms,
        );
        let _ = audit_log.log(entry);
    }
}
