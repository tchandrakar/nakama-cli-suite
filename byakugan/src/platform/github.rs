//! GitHub platform adapter using the GitHub REST API v3.

use super::{Comment, Platform, PlatformAdapter, PullRequest, Review, ReviewVerdict};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};

pub struct GitHubAdapter {
    client: reqwest::Client,
    api_url: String,
}

impl GitHubAdapter {
    pub fn new(token: &str, api_url: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap_or_else(|_| {
                HeaderValue::from_static("Bearer invalid")
            }),
        );
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("byakugan/0.2.0"),
        );
        headers.insert(
            "X-GitHub-Api-Version",
            HeaderValue::from_static("2022-11-28"),
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
}

#[async_trait]
impl PlatformAdapter for GitHubAdapter {
    fn platform(&self) -> Platform {
        Platform::GitHub
    }

    async fn get_pull_request(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<PullRequest> {
        let url = format!(
            "{}/repos/{}/{}/pulls/{}",
            self.api_url, owner, repo, number
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch PR from GitHub API")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error ({}): {}", status, body);
        }

        let json: serde_json::Value = resp.json().await.context("Failed to parse GitHub PR JSON")?;

        Ok(PullRequest {
            number: json["number"].as_u64().unwrap_or(number),
            title: json["title"].as_str().unwrap_or("(untitled)").to_string(),
            author: json["user"]["login"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
            base_branch: json["base"]["ref"]
                .as_str()
                .unwrap_or("main")
                .to_string(),
            head_branch: json["head"]["ref"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
            body: json["body"].as_str().unwrap_or("").to_string(),
            state: json["state"].as_str().unwrap_or("unknown").to_string(),
            url: json["html_url"].as_str().unwrap_or("").to_string(),
        })
    }

    async fn get_pull_request_diff(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<String> {
        let url = format!(
            "{}/repos/{}/{}/pulls/{}",
            self.api_url, owner, repo, number
        );

        let resp = self
            .client
            .get(&url)
            .header(ACCEPT, "application/vnd.github.v3.diff")
            .send()
            .await
            .context("Failed to fetch PR diff from GitHub API")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error fetching diff ({}): {}", status, body);
        }

        resp.text()
            .await
            .context("Failed to read GitHub diff response body")
    }

    async fn list_pull_requests(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<PullRequest>> {
        let url = format!(
            "{}/repos/{}/{}/pulls?state=open&per_page=30",
            self.api_url, owner, repo
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to list PRs from GitHub API")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error listing PRs ({}): {}", status, body);
        }

        let json: Vec<serde_json::Value> =
            resp.json().await.context("Failed to parse GitHub PR list JSON")?;

        Ok(json
            .iter()
            .map(|pr| PullRequest {
                number: pr["number"].as_u64().unwrap_or(0),
                title: pr["title"].as_str().unwrap_or("").to_string(),
                author: pr["user"]["login"].as_str().unwrap_or("").to_string(),
                base_branch: pr["base"]["ref"].as_str().unwrap_or("").to_string(),
                head_branch: pr["head"]["ref"].as_str().unwrap_or("").to_string(),
                body: pr["body"].as_str().unwrap_or("").to_string(),
                state: pr["state"].as_str().unwrap_or("").to_string(),
                url: pr["html_url"].as_str().unwrap_or("").to_string(),
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
        let url = format!(
            "{}/repos/{}/{}/pulls/{}/reviews",
            self.api_url, owner, repo, number
        );

        let event = match review.verdict {
            ReviewVerdict::Approve => "APPROVE",
            ReviewVerdict::RequestChanges => "REQUEST_CHANGES",
            ReviewVerdict::Comment => "COMMENT",
        };

        let comments: Vec<serde_json::Value> = review
            .comments
            .iter()
            .filter_map(|c| {
                let path = c.path.as_ref()?;
                let line = c.line?;
                Some(serde_json::json!({
                    "path": path,
                    "line": line,
                    "body": c.body,
                }))
            })
            .collect();

        let body = serde_json::json!({
            "body": review.body,
            "event": event,
            "comments": comments,
        });

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("Failed to post review to GitHub")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error posting review ({}): {}", status, body);
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
        let url = format!(
            "{}/repos/{}/{}/issues/{}/comments",
            self.api_url, owner, repo, number
        );

        let body = serde_json::json!({
            "body": comment.body,
        });

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("Failed to post comment to GitHub")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error posting comment ({}): {}", status, body);
        }

        Ok(())
    }
}
