//! Bitbucket platform adapter using the Bitbucket Cloud REST API 2.0.

use super::{Comment, InlinePostResult, Platform, PlatformAdapter, PullRequest, Review, ReviewVerdict};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

pub struct BitbucketAdapter {
    client: reqwest::Client,
    api_url: String,
}

impl BitbucketAdapter {
    pub fn new(username: &str, app_password: &str, api_url: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("byakugan/0.2.0"),
        );

        // Encode Basic Auth header.
        let credentials =
            base64_encode(&format!("{}:{}", username, app_password));
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", credentials))
                .unwrap_or_else(|_| HeaderValue::from_static("Basic invalid")),
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

    /// Create an adapter using a Bearer token (workspace/repository access token).
    pub fn new_with_bearer_token(token: &str, api_url: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("byakugan/0.2.0"),
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))
                .unwrap_or_else(|_| HeaderValue::from_static("Bearer invalid")),
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

/// Simple base64 encoder for Basic Auth (avoids extra dependency).
fn base64_encode(input: &str) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

#[async_trait]
impl PlatformAdapter for BitbucketAdapter {
    fn platform(&self) -> Platform {
        Platform::Bitbucket
    }

    async fn get_pull_request(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<PullRequest> {
        let url = format!(
            "{}/repositories/{}/{}/pullrequests/{}",
            self.api_url, owner, repo, number
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch PR from Bitbucket API")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Bitbucket API error ({}): {}", status, body);
        }

        let json: serde_json::Value =
            resp.json().await.context("Failed to parse Bitbucket PR JSON")?;

        Ok(PullRequest {
            number: json["id"].as_u64().unwrap_or(number),
            title: json["title"].as_str().unwrap_or("(untitled)").to_string(),
            author: json["author"]["display_name"]
                .as_str()
                .or_else(|| json["author"]["username"].as_str())
                .unwrap_or("unknown")
                .to_string(),
            base_branch: json["destination"]["branch"]["name"]
                .as_str()
                .unwrap_or("main")
                .to_string(),
            head_branch: json["source"]["branch"]["name"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
            body: json["description"].as_str().unwrap_or("").to_string(),
            state: json["state"].as_str().unwrap_or("unknown").to_string(),
            url: json["links"]["html"]["href"]
                .as_str()
                .unwrap_or("")
                .to_string(),
        })
    }

    async fn get_pull_request_diff(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<String> {
        let url = format!(
            "{}/repositories/{}/{}/pullrequests/{}/diff",
            self.api_url, owner, repo, number
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch PR diff from Bitbucket API")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!(
                "Bitbucket API error fetching diff ({}): {}",
                status,
                body
            );
        }

        resp.text()
            .await
            .context("Failed to read Bitbucket diff response body")
    }

    async fn list_pull_requests(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<PullRequest>> {
        let url = format!(
            "{}/repositories/{}/{}/pullrequests?state=OPEN&pagelen=30",
            self.api_url, owner, repo
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to list PRs from Bitbucket API")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Bitbucket API error listing PRs ({}): {}", status, body);
        }

        let json: serde_json::Value =
            resp.json().await.context("Failed to parse Bitbucket PR list")?;

        let values = json["values"].as_array().cloned().unwrap_or_default();

        Ok(values
            .iter()
            .map(|pr| PullRequest {
                number: pr["id"].as_u64().unwrap_or(0),
                title: pr["title"].as_str().unwrap_or("").to_string(),
                author: pr["author"]["display_name"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                base_branch: pr["destination"]["branch"]["name"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                head_branch: pr["source"]["branch"]["name"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                body: pr["description"].as_str().unwrap_or("").to_string(),
                state: pr["state"].as_str().unwrap_or("").to_string(),
                url: pr["links"]["html"]["href"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
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
        // Bitbucket doesn't have a formal review API like GitHub.
        // Post as a comment with verdict prefix.
        let verdict_label = match review.verdict {
            ReviewVerdict::Approve => "**APPROVED**",
            ReviewVerdict::RequestChanges => "**CHANGES REQUESTED**",
            ReviewVerdict::Comment => "**REVIEW COMMENT**",
        };

        let body = format!("{}\n\n{}", verdict_label, review.body);
        let comment = Comment {
            body,
            path: None,
            line: None,
        };
        self.post_comment(owner, repo, number, &comment).await?;

        // Post inline comments separately.
        for c in &review.comments {
            if c.path.is_some() {
                self.post_comment(owner, repo, number, c).await?;
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
        let url = format!(
            "{}/repositories/{}/{}/pullrequests/{}/comments",
            self.api_url, owner, repo, number
        );

        let mut body = serde_json::json!({
            "content": {
                "raw": comment.body,
            }
        });

        // Add inline position if provided.
        if let (Some(path), Some(line)) = (&comment.path, comment.line) {
            body["inline"] = serde_json::json!({
                "path": path,
                "to": line,
            });
        }

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("Failed to post comment to Bitbucket")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!(
                "Bitbucket API error posting comment ({}): {}",
                status,
                body
            );
        }

        Ok(())
    }

    async fn post_inline_comments(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
        comments: &[Comment],
    ) -> InlinePostResult {
        let url = format!(
            "{}/repositories/{}/{}/pullrequests/{}/comments",
            self.api_url, owner, repo, number
        );

        let mut result = InlinePostResult {
            posted: 0,
            failed: 0,
            errors: Vec::new(),
        };

        for comment in comments {
            let (path, line) = match (&comment.path, comment.line) {
                (Some(p), Some(l)) => (p, l),
                _ => continue,
            };

            let body = serde_json::json!({
                "content": {
                    "raw": comment.body,
                },
                "inline": {
                    "path": path,
                    "to": line,
                }
            });

            match self.client.post(&url).json(&body).send().await {
                Ok(resp) if resp.status().is_success() => {
                    result.posted += 1;
                }
                Ok(resp) => {
                    let status = resp.status();
                    let err_body = resp.text().await.unwrap_or_default();
                    result.failed += 1;
                    result.errors.push(format!(
                        "{}:{} — Bitbucket {} {}",
                        path, line, status, err_body
                    ));
                }
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(format!("{}:{} — {}", path, line, e));
                }
            }
        }

        result
    }
}
