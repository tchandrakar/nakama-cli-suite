use crate::ai_helper::{ask_ai, make_provider};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;

/// Review your day's accomplishments.
pub async fn run(config: &Config, ui: &NakamaUI) -> NakamaResult<()> {
    let spinner = ui.step_start("Reviewing today's activity...");

    let today_commits = std::process::Command::new("git")
        .args(["log", "--since=midnight", "--oneline", "--format=%h %s (%ar)"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    let diff_stat = std::process::Command::new("git")
        .args(["diff", "--stat", "HEAD~10..HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    spinner.finish_with_success("Activity collected");

    let spinner = ui.step_start("Generating review...");
    let (provider, model) = make_provider(config, ModelTier::Fast)?;

    let system_prompt = r#"You are Tensai, an end-of-day review assistant. Summarize the day's work.

Format:
## End of Day Review

### Accomplished
- What was done today (from commits)

### Stats
- Files changed, lines added/removed

### Carry Forward
- What should be continued tomorrow

### Reflection
One sentence on productivity and focus.

Be encouraging and constructive."#;

    let user_msg = format!(
        "Review my day.\n\nToday's commits:\n{}\n\nDiff stats:\n{}",
        if today_commits.is_empty() { "No commits today." } else { &today_commits },
        if diff_stat.is_empty() { "No recent changes." } else { &diff_stat },
    );

    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 1024, 0.4).await;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Review ready");
            ui.panel("Day Review", content);
        }
        Err(e) => spinner.finish_with_error(&format!("Failed: {}", e)),
    }

    result.map(|_| ())
}
