use crate::ai_helper::{ask_ai, make_provider};
use crate::atlassian::AtlassianClient;
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;

/// Ask a question across both Jira and Confluence.
pub async fn run(config: &Config, ui: &NakamaUI, question: &str) -> NakamaResult<()> {
    let spinner = ui.step_start("Searching across Jira and Confluence...");

    let client = AtlassianClient::new()?;

    // Search both in parallel
    let sanitized: String = question.replace('"', "\\\"").chars().take(100).collect();
    let jira_jql = format!("text ~ \"{}\" ORDER BY updated DESC", sanitized);
    let confluence_cql = format!("type = page AND text ~ \"{}\"", sanitized);

    let jira_future = client.jira_search(&jira_jql, 5);
    let confluence_future = client.confluence_search(&confluence_cql, 5);

    let (jira_result, confluence_result) = tokio::join!(jira_future, confluence_future);

    let jira_context = match &jira_result {
        Ok(r) => AtlassianClient::format_issues(r),
        Err(e) => format!("Jira search failed: {}", e),
    };

    let wiki_context = match &confluence_result {
        Ok(v) => {
            let results = v.get("results").and_then(|r| r.as_array());
            match results {
                Some(pages) => pages.iter().map(|p| {
                    let title = p.get("title").and_then(|t| t.as_str()).unwrap_or("?");
                    format!("- {}", title)
                }).collect::<Vec<_>>().join("\n"),
                None => "No Confluence results.".to_string(),
            }
        }
        Err(e) => format!("Confluence search failed: {}", e),
    };

    spinner.finish_with_success("Search complete");

    let spinner = ui.step_start("Analyzing results...");
    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Itachi, an Atlassian intelligence assistant. Answer the user's question using the provided Jira issues and Confluence pages as context.

Be specific — reference issue keys and page titles. If the context doesn't fully answer the question, say what's missing."#;

    let user_msg = format!(
        "Question: {}\n\nJira Issues:\n{}\n\nConfluence Pages:\n{}",
        question, jira_context, wiki_context,
    );

    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 2048, 0.3).await;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Answer ready");
            ui.panel("Answer", content);
        }
        Err(e) => spinner.finish_with_error(&format!("Failed: {}", e)),
    }

    result.map(|_| ())
}
