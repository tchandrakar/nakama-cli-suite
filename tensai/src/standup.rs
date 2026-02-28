use crate::ai_helper::{ask_ai, make_provider};
use crate::git_info::GitInfo;
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Generate a standup report from recent git activity.
pub async fn run(config: &Config, ui: &NakamaUI) -> NakamaResult<()> {
    let trace = TraceContext::new("tensai", "standup");
    let start = Instant::now();

    let spinner = ui.step_start("Analyzing recent activity...");

    // Get commits from last 24h
    let yesterday_commits = std::process::Command::new("git")
        .args(["log", "--since=yesterday", "--oneline", "--format=%h %s"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    let git = GitInfo::collect().ok();
    let git_summary = git.as_ref().map(|g| g.to_summary()).unwrap_or_default();

    spinner.finish_with_success("Activity collected");

    let spinner = ui.step_start("Generating standup...");
    let (provider, model) = make_provider(config, ModelTier::Fast)?;

    let system_prompt = r#"You are Tensai, a standup summary generator. Create a concise standup report.

Format exactly as:
## Yesterday
- What was accomplished (based on commits)

## Today
- Planned work (inferred from current branch and pending changes)

## Blockers
- Any potential blockers (inferred from context)

Be concise — each bullet should be one short sentence. Max 3-5 bullets per section."#;

    let user_msg = format!(
        "Generate standup from this activity.\n\nCommits since yesterday:\n{}\n\nCurrent state:\n{}",
        if yesterday_commits.is_empty() { "No commits in the last 24h." } else { &yesterday_commits },
        git_summary,
    );

    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 1024, 0.3).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Standup ready");
            ui.panel("Standup Report", content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id, "tensai", "standup", Category::AiInteraction,
                    "Generated standup report",
                    serde_json::json!({ "model": model }),
                    Outcome::Success, elapsed,
                );
                let _ = audit.log(entry);
            }
        }
        Err(e) => spinner.finish_with_error(&format!("Failed: {}", e)),
    }

    result.map(|_| ())
}
