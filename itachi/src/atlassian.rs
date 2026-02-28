use nakama_core::error::{NakamaError, NakamaResult};
use nakama_vault::{CredentialStore, Vault};
use serde::Deserialize;

/// Atlassian API client configuration.
pub struct AtlassianClient {
    pub base_url: String,
    pub email: String,
    pub api_token: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
pub struct JiraSearchResult {
    pub issues: Vec<JiraIssue>,
    pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct JiraIssue {
    pub key: String,
    pub fields: JiraFields,
}

#[derive(Debug, Deserialize)]
pub struct JiraFields {
    pub summary: String,
    pub status: Option<JiraStatus>,
    pub assignee: Option<JiraUser>,
    pub priority: Option<JiraPriority>,
    #[serde(rename = "issuetype")]
    pub issue_type: Option<JiraIssueType>,
}

#[derive(Debug, Deserialize)]
pub struct JiraStatus {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraUser {
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct JiraPriority {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct JiraIssueType {
    pub name: String,
}

impl AtlassianClient {
    /// Create client from vault credentials.
    pub fn new() -> NakamaResult<Self> {
        let vault = Vault::new()?;

        let base_url = match vault.retrieve("atlassian", "base_url") {
            Ok(s) => s.expose_secret().to_string(),
            Err(_) => std::env::var("ATLASSIAN_BASE_URL").map_err(|_| NakamaError::Auth {
                message: "No Atlassian URL. Set ATLASSIAN_BASE_URL (e.g. https://yourorg.atlassian.net)".to_string(),
            })?,
        };

        let email = match vault.retrieve("atlassian", "email") {
            Ok(s) => s.expose_secret().to_string(),
            Err(_) => std::env::var("ATLASSIAN_EMAIL").map_err(|_| NakamaError::Auth {
                message: "No Atlassian email. Set ATLASSIAN_EMAIL env var.".to_string(),
            })?,
        };

        let api_token = match vault.retrieve("atlassian", "api_token") {
            Ok(s) => s.expose_secret().to_string(),
            Err(_) => std::env::var("ATLASSIAN_API_TOKEN").map_err(|_| NakamaError::Auth {
                message: "No Atlassian API token. Set ATLASSIAN_API_TOKEN env var.".to_string(),
            })?,
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| NakamaError::Network {
                message: format!("Failed to create HTTP client: {}", e),
                source: Some(Box::new(e)),
            })?;

        Ok(Self { base_url, email, api_token, client })
    }

    /// Search Jira issues with JQL.
    pub async fn jira_search(&self, jql: &str, max_results: u32) -> NakamaResult<JiraSearchResult> {
        let url = format!("{}/rest/api/3/search", self.base_url);
        let response = self.client
            .get(&url)
            .basic_auth(&self.email, Some(&self.api_token))
            .query(&[
                ("jql", jql),
                ("maxResults", &max_results.to_string()),
                ("fields", "summary,status,assignee,priority,issuetype"),
            ])
            .send()
            .await
            .map_err(|e| NakamaError::Network {
                message: format!("Jira API request failed: {}", e),
                source: Some(Box::new(e)),
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(NakamaError::Network {
                message: format!("Jira API error ({}): {}", status, body),
                source: None,
            });
        }

        response.json::<JiraSearchResult>().await.map_err(|e| NakamaError::Network {
            message: format!("Failed to parse Jira response: {}", e),
            source: Some(Box::new(e)),
        })
    }

    /// Search Confluence with CQL.
    pub async fn confluence_search(&self, cql: &str, limit: u32) -> NakamaResult<serde_json::Value> {
        let url = format!("{}/wiki/rest/api/content/search", self.base_url);
        let response = self.client
            .get(&url)
            .basic_auth(&self.email, Some(&self.api_token))
            .query(&[("cql", cql), ("limit", &limit.to_string())])
            .send()
            .await
            .map_err(|e| NakamaError::Network {
                message: format!("Confluence API request failed: {}", e),
                source: Some(Box::new(e)),
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(NakamaError::Network {
                message: format!("Confluence API error ({}): {}", status, body),
                source: None,
            });
        }

        response.json::<serde_json::Value>().await.map_err(|e| NakamaError::Network {
            message: format!("Failed to parse Confluence response: {}", e),
            source: Some(Box::new(e)),
        })
    }

    /// Format issues as a summary string.
    pub fn format_issues(result: &JiraSearchResult) -> String {
        if result.issues.is_empty() {
            return "No issues found.".to_string();
        }
        let mut out = String::new();
        for issue in &result.issues {
            let status = issue.fields.status.as_ref().map(|s| s.name.as_str()).unwrap_or("Unknown");
            let priority = issue.fields.priority.as_ref().map(|p| p.name.as_str()).unwrap_or("-");
            let itype = issue.fields.issue_type.as_ref().map(|t| t.name.as_str()).unwrap_or("-");
            out.push_str(&format!(
                "{} [{}] {} | {} | {}\n",
                issue.key, status, issue.fields.summary, itype, priority
            ));
        }
        out
    }
}
