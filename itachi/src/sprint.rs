use crate::atlassian::AtlassianClient;
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_ui::NakamaUI;

/// Show sprint information.
pub async fn run(_config: &Config, ui: &NakamaUI, _board: Option<&str>) -> NakamaResult<()> {
    let spinner = ui.step_start("Fetching sprint info...");

    let client = AtlassianClient::new()?;
    let jql = "sprint in openSprints() ORDER BY priority DESC";
    let result = client.jira_search(jql, 50).await?;

    spinner.finish_with_success(&format!("Sprint: {} issues", result.total));

    let rows: Vec<Vec<String>> = result.issues.iter().map(|issue| {
        vec![
            issue.key.clone(),
            issue.fields.status.as_ref().map(|s| s.name.clone()).unwrap_or_default(),
            issue.fields.summary.chars().take(60).collect(),
            issue.fields.assignee.as_ref().map(|a| a.display_name.clone()).unwrap_or_else(|| "Unassigned".to_string()),
            issue.fields.priority.as_ref().map(|p| p.name.clone()).unwrap_or_default(),
        ]
    }).collect();

    if rows.is_empty() {
        ui.warn("No issues in current sprint.");
    } else {
        ui.table(&["Key", "Status", "Summary", "Assignee", "Priority"], rows);
    }

    Ok(())
}
