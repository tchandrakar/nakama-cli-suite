use crate::ai_helper::{ask_ai, make_provider};
use crate::atlassian::AtlassianClient;
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Search Jira with natural language, translating to JQL via AI.
pub async fn run(config: &Config, ui: &NakamaUI, query: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("itachi", "jira");
    let start = Instant::now();

    // First, translate natural language to JQL using AI
    let spinner = ui.step_start("Translating query to JQL...");
    let (provider, model) = make_provider(config, ModelTier::Fast)?;

    let system_prompt = r#"You are a Jira JQL expert. Convert the natural language query to a JQL search string.
Return ONLY the JQL query, nothing else. No explanation.

Examples:
- "my open bugs" → assignee = currentUser() AND type = Bug AND status != Done
- "high priority tasks this sprint" → priority in (High, Highest) AND sprint in openSprints()
- "recently updated in PROJECT" → project = PROJECT AND updated >= -7d ORDER BY updated DESC"#;

    let jql = ask_ai(provider.as_ref(), system_prompt, query, &model, 256, 0.1).await?;
    let jql = jql.trim().trim_matches('`').trim();
    spinner.finish_with_success(&format!("JQL: {}", jql));

    // Execute the JQL search
    let spinner = ui.step_start("Searching Jira...");
    let client = AtlassianClient::new()?;
    let result = client.jira_search(jql, 20).await?;

    let elapsed = start.elapsed().as_millis() as u64;
    spinner.finish_with_success(&format!("Found {} issues", result.total));

    // Display results as table
    let rows: Vec<Vec<String>> = result.issues.iter().map(|issue| {
        vec![
            issue.key.clone(),
            issue.fields.issue_type.as_ref().map(|t| t.name.clone()).unwrap_or_default(),
            issue.fields.summary.clone(),
            issue.fields.status.as_ref().map(|s| s.name.clone()).unwrap_or_default(),
            issue.fields.priority.as_ref().map(|p| p.name.clone()).unwrap_or_default(),
        ]
    }).collect();

    if !rows.is_empty() {
        ui.table(&["Key", "Type", "Summary", "Status", "Priority"], rows);
    }

    if let Ok(audit) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id, "itachi", "jira", Category::ExternalApi,
            &format!("Jira search: {}", &query[..query.len().min(80)]),
            serde_json::json!({ "query": query, "jql": jql, "results": result.total }),
            Outcome::Success, elapsed,
        );
        let _ = audit.log(entry);
    }

    Ok(())
}
