use crate::ai_helper::{ask_ai, make_provider};
use crate::atlassian::AtlassianClient;
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;

/// Generate a team briefing from recent Jira activity.
pub async fn run(config: &Config, ui: &NakamaUI, team: Option<&str>) -> NakamaResult<()> {
    let spinner = ui.step_start("Fetching recent activity...");

    let client = AtlassianClient::new()?;

    let jql = if let Some(t) = team {
        format!("updated >= -7d AND (assignee in membersOf(\"{}\") OR reporter in membersOf(\"{}\")) ORDER BY updated DESC", t, t)
    } else {
        "updated >= -7d ORDER BY updated DESC".to_string()
    };

    let result = client.jira_search(&jql, 30).await?;
    let issues_summary = AtlassianClient::format_issues(&result);

    spinner.finish_with_success(&format!("Found {} recent issues", result.total));

    let spinner = ui.step_start("Generating briefing...");
    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Itachi, a team briefing generator. Create a concise team briefing from recent Jira activity.

Format:
## Team Briefing

### In Progress
Issues actively being worked on.

### Recently Completed
Issues closed/done recently.

### Blockers & Risks
Issues that look stuck or high-priority unresolved items.

### Key Metrics
- Total active issues
- Completion rate this week

Keep it actionable and brief."#;

    let team_label = team.unwrap_or("All Teams");
    let user_msg = format!("Generate briefing for {}.\n\nRecent issues:\n{}", team_label, issues_summary);

    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 2048, 0.3).await;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Briefing ready");
            ui.panel(&format!("Briefing: {}", team_label), content);
        }
        Err(e) => spinner.finish_with_error(&format!("Failed: {}", e)),
    }

    result.map(|_| ())
}
