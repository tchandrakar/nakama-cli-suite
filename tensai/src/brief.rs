use crate::ai_helper::{ask_ai, make_provider};
use crate::git_info::GitInfo;
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Generate a morning dev briefing.
pub async fn run(config: &Config, ui: &NakamaUI) -> NakamaResult<()> {
    let trace = TraceContext::new("tensai", "brief");
    let start = Instant::now();

    let spinner = ui.step_start("Gathering dev context...");

    let git = GitInfo::collect().ok();
    let git_summary = git.as_ref().map(|g| g.to_summary()).unwrap_or_else(|| "Not in a git repository.".to_string());

    // Try to get PR info from gh CLI
    let pr_info = std::process::Command::new("gh")
        .args(["pr", "list", "--limit", "5", "--json", "number,title,author,updatedAt"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_else(|_| "GitHub CLI not available.".to_string());

    spinner.finish_with_success("Context gathered");

    let spinner = ui.step_start("Generating briefing...");
    let (provider, model) = make_provider(config, ModelTier::Fast)?;

    let system_prompt = r#"You are Tensai, a dev productivity assistant. Generate a concise morning briefing.

Format:
## Good Morning! Here's your dev briefing:

### Git Status
Summary of branch, recent work, and pending changes.

### Open PRs
PRs that need attention (if any).

### Priorities
Based on recent activity, suggest 3-5 priorities for today.

### Heads Up
Any potential issues or blockers to watch for.

Keep it brief and actionable. No fluff."#;

    let user_msg = format!(
        "Generate my morning briefing.\n\nGit info:\n{}\n\nOpen PRs:\n{}",
        git_summary, pr_info,
    );

    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 1536, 0.4).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Briefing ready");
            ui.panel("Dev Briefing", content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id, "tensai", "brief", Category::AiInteraction,
                    "Generated morning briefing",
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
