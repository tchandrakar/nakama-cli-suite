//! GitLab platform adapter using the GitLab REST API v4.

use super::{Comment, Platform, PlatformAdapter, PullRequest, Review, ReviewVerdict};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

pub struct GitLabAdapter {
    client: reqwest::Client,
    api_url: String,
}

impl GitLabAdapter {
    pub fn new(token: &str, api_url: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "PRIVATE-TOKEN",
            HeaderValue::from_str(token).unwrap_or_else(|_| HeaderValue::from_static("invalid")),
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("byakugan/0.2.0"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            api_url: api_url.trim_end_matches('/').to_string(),
        }
    }

    /// URL-encode the project path (owner/repo → owner%2Frepo).
    fn project_path(owner: &str, repo: &str) -> String {
        format!("{}%2F{}", owner, repo)
    }
}

#[async_trait]
impl PlatformAdapter for GitLabAdapter {
    fn platform(&self) -> Platform {
        Platform::GitLab
    }

    async fn get_pull_request(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<PullRequest> {
        let project = Self::project_path(owner, repo);
        let url = format!(
            "{}/projects/{}/merge_requests/{}",
            self.api_url, project, number
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch MR from GitLab API")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitLab API error ({}): {}", status, body);
        }

        let json: serde_json::Value =
            resp.json().await.context("Failed to parse GitLab MR JSON")?;

        Ok(PullRequest {
            number: json["iid"].as_u64().unwrap_or(number),
            title: json["title"].as_str().unwrap_or("(untitled)").to_string(),
            author: json["author"]["username"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
            base_branch: json["target_branch"]
                .as_str()
                .unwrap_or("main")
                .to_string(),
            head_branch: json["source_branch"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
            body: json["description"].as_str().unwrap_or("").to_string(),
            state: json["state"].as_str().unwrap_or("unknown").to_string(),
            url: json["web_url"].as_str().unwrap_or("").to_string(),
        })
    }

    async fn get_pull_request_diff(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<String> {
        let project = Self::project_path(owner, repo);
        let url = format!(
            "{}/projects/{}/merge_requests/{}/changes",
            self.api_url, project, number
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch MR changes from GitLab API")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitLab API error fetching diff ({}): {}", status, body);
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .context("Failed to parse GitLab MR changes JSON")?;

        // Reconstruct a unified diff from the changes array.
        let mut diff = String::new();
        if let Some(changes) = json["changes"].as_array() {
            for change in changes {
                let path = change["new_path"].as_str().unwrap_or("unknown");
                let old_path = change["old_path"].as_str().unwrap_or(path);
                diff.push_str(&format!("--- a/{}\n+++ b/{}\n", old_path, path));
                if let Some(d) = change["diff"].as_str() {
                    diff.push_str(d);
                    if !d.ends_with('\n') {
                        diff.push('\n');
                    }
                }
            }
        }

        Ok(diff)
    }

    async fn list_pull_requests(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<PullRequest>> {
        let project = Self::project_path(owner, repo);
        let url = format!(
            "{}/projects/{}/merge_requests?state=opened&per_page=30",
            self.api_url, project
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to list MRs from GitLab API")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitLab API error listing MRs ({}): {}", status, body);
        }

        let json: Vec<serde_json::Value> =
            resp.json().await.context("Failed to parse GitLab MR list")?;

        Ok(json
            .iter()
            .map(|mr| PullRequest {
                number: mr["iid"].as_u64().unwrap_or(0),
                title: mr["title"].as_str().unwrap_or("").to_string(),
                author: mr["author"]["username"].as_str().unwrap_or("").to_string(),
                base_branch: mr["target_branch"].as_str().unwrap_or("").to_string(),
                head_branch: mr["source_branch"].as_str().unwrap_or("").to_string(),
                body: mr["description"].as_str().unwrap_or("").to_string(),
                state: mr["state"].as_str().unwrap_or("").to_string(),
                url: mr["web_url"].as_str().unwrap_or("").to_string(),
            })
            .collect())
    }

    async fn post_review(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
        review: &Review,
    ) -> Result<()> {
        // GitLab uses discussions/notes instead of reviews.
        // Post the review body as a note.
        let project = Self::project_path(owner, repo);
        let url = format!(
            "{}/projects/{}/merge_requests/{}/notes",
            self.api_url, project, number
        );

        let verdict_label = match review.verdict {
            ReviewVerdict::Approve => "**Verdict: APPROVED**",
            ReviewVerdict::RequestChanges => "**Verdict: CHANGES REQUESTED**",
            ReviewVerdict::Comment => "**Verdict: COMMENT**",
        };

        let body = format!("{}\n\n{}", verdict_label, review.body);

        let resp = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "body": body }))
            .send()
            .await
            .context("Failed to post review note to GitLab")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitLab API error posting review ({}): {}", status, body);
        }

        // Post inline comments as individual discussion threads.
        for comment in &review.comments {
            if let (Some(path), Some(line)) = (&comment.path, comment.line) {
                let disc_url = format!(
                    "{}/projects/{}/merge_requests/{}/discussions",
                    self.api_url, project, number
                );

                let disc_body = serde_json::json!({
                    "body": comment.body,
                    "position": {
                        "position_type": "text",
                        "new_path": path,
                        "new_line": line,
                        "base_sha": "",
                        "head_sha": "",
                        "start_sha": "",
                    }
                });

                let _ = self.client.post(&disc_url).json(&disc_body).send().await;
            }
        }

        Ok(())
    }

    async fn post_comment(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
        comment: &Comment,
    ) -> Result<()> {
        let project = Self::project_path(owner, repo);
        let url = format!(
            "{}/projects/{}/merge_requests/{}/notes",
            self.api_url, project, number
        );

        let resp = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "body": comment.body }))
            .send()
            .await
            .context("Failed to post comment to GitLab")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitLab API error posting comment ({}): {}", status, body);
        }

        Ok(())
    }
}
