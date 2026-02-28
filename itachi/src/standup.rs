use crate::ai_helper::{ask_ai, make_provider};
use crate::atlassian::AtlassianClient;
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;

/// Generate standup from your Jira activity.
pub async fn run(config: &Config, ui: &NakamaUI) -> NakamaResult<()> {
    let spinner = ui.step_start("Fetching your recent Jira activity...");

    let client = AtlassianClient::new()?;
    let jql = "assignee = currentUser() AND updated >= -2d ORDER BY updated DESC";
    let result = client.jira_search(jql, 15).await?;
    let issues = AtlassianClient::format_issues(&result);

    spinner.finish_with_success(&format!("Found {} recent issues", result.total));

    let spinner = ui.step_start("Generating standup...");
    let (provider, model) = make_provider(config, ModelTier::Fast)?;

    let system_prompt = r#"You are Itachi. Generate a standup summary from the user's Jira issues.

Format:
## Yesterday
- What was worked on (issues with recent updates)

## Today
- What will be worked on (in-progress issues)

## Blockers
- Any blocked or stalled issues

Reference actual issue keys. Be concise."#;

    let result = ask_ai(provider.as_ref(), system_prompt, &format!("My Jira issues:\n{}", issues), &model, 1024, 0.3).await;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Standup ready");
            ui.panel("Jira Standup", content);
        }
        Err(e) => spinner.finish_with_error(&format!("Failed: {}", e)),
    }

    result.map(|_| ())
}
