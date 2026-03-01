//! Platform abstraction layer for PR/MR operations.
//!
//! Provides a unified [`PlatformAdapter`] trait that encapsulates the differences
//! between GitHub, GitLab, and Bitbucket APIs. A factory function
//! [`create_adapter`] instantiates the correct adapter based on configuration
//! or auto-detection from the git remote URL.

pub mod bitbucket;
pub mod github;
pub mod gitlab;

use anyhow::{Context, Result};
use async_trait::async_trait;
use nakama_core::config::PlatformsConfig;
use nakama_vault::{CredentialStore, Vault};
use serde::{Deserialize, Serialize};
use std::fmt;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Supported hosting platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    GitHub,
    GitLab,
    Bitbucket,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Platform::GitHub => write!(f, "github"),
            Platform::GitLab => write!(f, "gitlab"),
            Platform::Bitbucket => write!(f, "bitbucket"),
        }
    }
}

impl Platform {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "github" | "gh" => Ok(Platform::GitHub),
            "gitlab" | "gl" => Ok(Platform::GitLab),
            "bitbucket" | "bb" => Ok(Platform::Bitbucket),
            other => anyhow::bail!(
                "Unknown platform '{}'. Use: github, gitlab, or bitbucket",
                other
            ),
        }
    }
}

/// Verdict for a review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewVerdict {
    Approve,
    RequestChanges,
    Comment,
}

impl fmt::Display for ReviewVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReviewVerdict::Approve => write!(f, "APPROVE"),
            ReviewVerdict::RequestChanges => write!(f, "REQUEST_CHANGES"),
            ReviewVerdict::Comment => write!(f, "COMMENT"),
        }
    }
}

// ---------------------------------------------------------------------------
// Data models
// ---------------------------------------------------------------------------

/// Unified pull/merge request metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub number: u64,
    pub title: String,
    pub author: String,
    pub base_branch: String,
    pub head_branch: String,
    pub body: String,
    pub state: String,
    pub url: String,
}

/// Unified diff representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedDiff {
    pub raw: String,
    pub files: Vec<DiffFile>,
}

/// A single file within a diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffFile {
    pub path: String,
    pub old_path: Option<String>,
    pub hunks: Vec<Hunk>,
    pub additions: usize,
    pub deletions: usize,
}

/// A hunk within a diff file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hunk {
    pub header: String,
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
    pub lines: Vec<DiffLine>,
}

/// A single line within a hunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    pub origin: char,
    pub content: String,
    pub old_lineno: Option<u32>,
    pub new_lineno: Option<u32>,
}

/// A review comment to post on a PR/MR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub body: String,
    pub path: Option<String>,
    pub line: Option<u32>,
}

/// A full review to post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub body: String,
    pub verdict: ReviewVerdict,
    pub comments: Vec<Comment>,
}

// ---------------------------------------------------------------------------
// Trait
// ---------------------------------------------------------------------------

/// Platform-agnostic adapter for PR/MR operations.
#[async_trait]
pub trait PlatformAdapter: Send + Sync {
    /// Which platform this adapter targets.
    fn platform(&self) -> Platform;

    /// Fetch a single pull/merge request by number.
    async fn get_pull_request(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<PullRequest>;

    /// Fetch the diff for a pull/merge request.
    async fn get_pull_request_diff(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<String>;

    /// List open pull/merge requests.
    async fn list_pull_requests(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<PullRequest>>;

    /// Post a review to a pull/merge request.
    async fn post_review(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
        review: &Review,
    ) -> Result<()>;

    /// Post a single comment to a pull/merge request.
    async fn post_comment(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
        comment: &Comment,
    ) -> Result<()>;
}

// ---------------------------------------------------------------------------
// Factory
// ---------------------------------------------------------------------------

/// Create the appropriate platform adapter based on the platform enum and config.
pub fn create_adapter(
    platform: Platform,
    config: &PlatformsConfig,
) -> Result<Box<dyn PlatformAdapter>> {
    match platform {
        Platform::GitHub => {
            let token = resolve_token(
                config.github.token.as_deref(),
                "github",
                "GITHUB_TOKEN",
            )?;
            Ok(Box::new(github::GitHubAdapter::new(
                &token,
                &config.github.api_url,
            )))
        }
        Platform::GitLab => {
            let token = resolve_token(
                config.gitlab.token.as_deref(),
                "gitlab",
                "GITLAB_TOKEN",
            )?;
            Ok(Box::new(gitlab::GitLabAdapter::new(
                &token,
                &config.gitlab.api_url,
            )))
        }
        Platform::Bitbucket => {
            let username = config
                .bitbucket
                .username
                .clone()
                .or_else(|| std::env::var("BITBUCKET_USERNAME").ok())
                .context(
                    "Bitbucket username not found. Set it in config.toml or BITBUCKET_USERNAME env var.",
                )?;
            let app_password = resolve_token(
                config.bitbucket.app_password.as_deref(),
                "bitbucket",
                "BITBUCKET_APP_PASSWORD",
            )?;
            Ok(Box::new(bitbucket::BitbucketAdapter::new(
                &username,
                &app_password,
                &config.bitbucket.api_url,
            )))
        }
    }
}

/// Resolve a token: config value → vault → environment variable.
fn resolve_token(
    config_value: Option<&str>,
    vault_service: &str,
    env_var: &str,
) -> Result<String> {
    // 1. Config value
    if let Some(token) = config_value {
        if !token.is_empty() {
            return Ok(token.to_string());
        }
    }

    // 2. Vault
    if let Ok(vault) = Vault::new() {
        let key = format!("{}_token", vault_service);
        if let Ok(secret) = vault.retrieve("nakama", &key) {
            return Ok(secret.expose_secret().to_string());
        }
    }

    // 3. Environment variable
    std::env::var(env_var).context(format!(
        "Token for {} not found. Set {} env var, store in vault, or add to config.toml",
        vault_service, env_var
    ))
}

/// Auto-detect platform from the git remote URL.
pub fn detect_platform_from_remote() -> Option<Platform> {
    let repo = git2::Repository::discover(".").ok()?;
    let remote = repo.find_remote("origin").ok()?;
    let url = remote.url()?;

    if url.contains("github.com") {
        Some(Platform::GitHub)
    } else if url.contains("gitlab.com") || url.contains("gitlab") {
        Some(Platform::GitLab)
    } else if url.contains("bitbucket.org") || url.contains("bitbucket") {
        Some(Platform::Bitbucket)
    } else {
        None
    }
}

/// Parse owner/repo from the git remote URL.
pub fn parse_owner_repo_from_remote() -> Result<(String, String)> {
    let repo = git2::Repository::discover(".")
        .context("Not in a git repository")?;
    let remote = repo
        .find_remote("origin")
        .context("No 'origin' remote found")?;
    let url = remote
        .url()
        .context("Remote URL is not valid UTF-8")?
        .to_string();

    parse_owner_repo_from_url(&url)
}

/// Parse owner/repo from a remote URL string.
fn parse_owner_repo_from_url(url: &str) -> Result<(String, String)> {
    // Handle SSH: git@github.com:owner/repo.git
    if let Some(rest) = url.strip_prefix("git@") {
        if let Some(path) = rest.split(':').nth(1) {
            let path = path.trim_end_matches(".git");
            let parts: Vec<&str> = path.splitn(2, '/').collect();
            if parts.len() == 2 {
                return Ok((parts[0].to_string(), parts[1].to_string()));
            }
        }
    }

    // Handle HTTPS: https://github.com/owner/repo.git
    if url.starts_with("https://") || url.starts_with("http://") {
        let path = url
            .split("://")
            .nth(1)
            .and_then(|s| s.split('/').skip(1).collect::<Vec<_>>().first().copied().map(|_| {
                let parts: Vec<&str> = s.split('/').skip(1).collect();
                parts
            }));
        if let Some(parts) = path {
            if parts.len() >= 2 {
                let owner = parts[0].to_string();
                let repo = parts[1].trim_end_matches(".git").to_string();
                return Ok((owner, repo));
            }
        }
    }

    anyhow::bail!("Could not parse owner/repo from remote URL: {}", url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ssh_url() {
        let (owner, repo) =
            parse_owner_repo_from_url("git@github.com:tchandrakar/nakama-cli-suite.git").unwrap();
        assert_eq!(owner, "tchandrakar");
        assert_eq!(repo, "nakama-cli-suite");
    }

    #[test]
    fn test_parse_https_url() {
        let (owner, repo) =
            parse_owner_repo_from_url("https://github.com/tchandrakar/nakama-cli-suite.git")
                .unwrap();
        assert_eq!(owner, "tchandrakar");
        assert_eq!(repo, "nakama-cli-suite");
    }

    #[test]
    fn test_platform_from_str() {
        assert_eq!(Platform::from_str("github").unwrap(), Platform::GitHub);
        assert_eq!(Platform::from_str("gitlab").unwrap(), Platform::GitLab);
        assert_eq!(Platform::from_str("bb").unwrap(), Platform::Bitbucket);
    }
}
