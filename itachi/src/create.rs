use crate::atlassian::AtlassianClient;
use nakama_core::config::Config;
use nakama_core::error::{NakamaError, NakamaResult};
use nakama_ui::NakamaUI;

/// Create a Jira issue.
pub async fn run(_config: &Config, ui: &NakamaUI, issue_type: &str, summary: &str) -> NakamaResult<()> {
    let spinner = ui.step_start("Creating Jira issue...");

    let client = AtlassianClient::new()?;

    let body = serde_json::json!({
        "fields": {
            "project": { "key": std::env::var("JIRA_PROJECT").unwrap_or_else(|_| "PROJ".to_string()) },
            "issuetype": { "name": capitalize(issue_type) },
            "summary": summary,
        }
    });

    let url = format!("{}/rest/api/3/issue", client.base_url);
    let http_client = reqwest::Client::new();
    let response = http_client
        .post(&url)
        .basic_auth(&client.email, Some(&client.api_token))
        .json(&body)
        .send()
        .await
        .map_err(|e| NakamaError::Network {
            message: format!("Failed to create issue: {}", e),
            source: Some(Box::new(e)),
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        spinner.finish_with_error(&format!("HTTP {}: {}", status, text));
        return Err(NakamaError::Network { message: format!("Create failed: {}", status), source: None });
    }

    let result: serde_json::Value = response.json().await.map_err(|e| NakamaError::Network {
        message: format!("Failed to parse response: {}", e),
        source: Some(Box::new(e)),
    })?;

    let key = result.get("key").and_then(|k| k.as_str()).unwrap_or("unknown");
    spinner.finish_with_success(&format!("Created: {}", key));
    ui.panel("Issue Created", &format!("Key: {}\nType: {}\nSummary: {}", key, issue_type, summary));

    Ok(())
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
