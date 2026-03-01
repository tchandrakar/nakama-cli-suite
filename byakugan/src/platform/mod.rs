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

/// Result from posting inline comments individually with per-comment resilience.
#[derive(Debug, Clone)]
pub struct InlinePostResult {
    pub posted: usize,
    pub failed: usize,
    pub errors: Vec<String>,
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

    /// Post inline comments individually with per-comment resilience.
    ///
    /// Default implementation calls `post_comment()` for each comment,
    /// continuing on failure and collecting error counts.
    async fn post_inline_comments(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
        comments: &[Comment],
    ) -> InlinePostResult {
        let mut result = InlinePostResult {
            posted: 0,
            failed: 0,
            errors: Vec::new(),
        };
        for comment in comments {
            match self.post_comment(owner, repo, number, comment).await {
                Ok(()) => result.posted += 1,
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(format!("{:#}", e));
                }
            }
        }
        result
    }
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
            // Resolve username: config → NAKAMA_BITBUCKET_USERNAME → BITBUCKET_USERNAME
            let username = config
                .bitbucket
                .username
                .clone()
                .or_else(|| std::env::var("NAKAMA_BITBUCKET_USERNAME").ok())
                .or_else(|| std::env::var("BITBUCKET_USERNAME").ok());

            // Resolve token/password: config → NAKAMA_BITBUCKET_API_KEY → vault → BITBUCKET_APP_PASSWORD
            let token = resolve_token(
                config.bitbucket.app_password.as_deref(),
                "bitbucket",
                "NAKAMA_BITBUCKET_API_KEY",
            )
            .or_else(|_| resolve_token(None, "bitbucket_app_password", "BITBUCKET_APP_PASSWORD"));

            match (username, token) {
                // Basic Auth with username + token (Atlassian API tokens require this).
                (Some(user), Ok(tok)) => Ok(Box::new(bitbucket::BitbucketAdapter::new(
                    &user,
                    &tok,
                    &config.bitbucket.api_url,
                ))),
                // Token only — try Bearer auth (workspace/repo access tokens).
                (None, Ok(tok)) => Ok(Box::new(
                    bitbucket::BitbucketAdapter::new_with_bearer_token(
                        &tok,
                        &config.bitbucket.api_url,
                    ),
                )),
                // No token found at all.
                _ => anyhow::bail!(
                    "Bitbucket credentials not found. Set NAKAMA_BITBUCKET_API_KEY \
                     (+ NAKAMA_BITBUCKET_USERNAME for Atlassian API tokens), \
                     or configure username + app_password in config.toml."
                ),
            }
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

/// Parsed components from a PR/MR URL.
#[derive(Debug, Clone)]
pub struct ParsedPrUrl {
    pub platform: Platform,
    pub owner: String,
    pub repo: String,
    pub number: u64,
}

/// Parse a PR/MR URL into its components.
///
/// Supported formats:
/// - `https://bitbucket.org/{workspace}/{repo}/pull-requests/{number}`
/// - `https://github.com/{owner}/{repo}/pull/{number}`
/// - `https://gitlab.com/{owner}/{repo}/-/merge_requests/{number}`
pub fn parse_pr_url(url: &str) -> Result<ParsedPrUrl> {
    let url = url.trim().trim_end_matches('/');

    // Strip scheme.
    let without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .context("PR URL must start with https:// or http://")?;

    let parts: Vec<&str> = without_scheme.split('/').collect();

    // Bitbucket: bitbucket.org/{workspace}/{repo}/pull-requests/{number}
    if without_scheme.starts_with("bitbucket.org") || without_scheme.starts_with("www.bitbucket.org") {
        // parts: [host, workspace, repo, "pull-requests", number]
        let skip = if parts[0].starts_with("www.") { 1 } else { 1 };
        if parts.len() >= skip + 4 && parts[skip + 2] == "pull-requests" {
            let number = parts[skip + 3]
                .parse::<u64>()
                .context("Invalid PR number in Bitbucket URL")?;
            return Ok(ParsedPrUrl {
                platform: Platform::Bitbucket,
                owner: parts[skip].to_string(),
                repo: parts[skip + 1].to_string(),
                number,
            });
        }
    }

    // GitHub: github.com/{owner}/{repo}/pull/{number}
    if without_scheme.starts_with("github.com") {
        if parts.len() >= 5 && parts[3] == "pull" {
            let number = parts[4]
                .parse::<u64>()
                .context("Invalid PR number in GitHub URL")?;
            return Ok(ParsedPrUrl {
                platform: Platform::GitHub,
                owner: parts[1].to_string(),
                repo: parts[2].to_string(),
                number,
            });
        }
    }

    // GitLab: gitlab.com/{owner}/{repo}/-/merge_requests/{number}
    if without_scheme.starts_with("gitlab.com") || without_scheme.contains("gitlab") {
        // Find the `/-/merge_requests/{number}` pattern.
        if let Some(mr_pos) = parts.iter().position(|&p| p == "merge_requests") {
            if mr_pos >= 2 && mr_pos + 1 < parts.len() {
                let number = parts[mr_pos + 1]
                    .parse::<u64>()
                    .context("Invalid MR number in GitLab URL")?;
                // Owner might be nested (groups/subgroups), repo is right before "-".
                let dash_pos = parts.iter().position(|&p| p == "-").unwrap_or(mr_pos);
                let owner = parts[1..dash_pos - 1].join("/");
                let repo = parts[dash_pos - 1].to_string();
                return Ok(ParsedPrUrl {
                    platform: Platform::GitLab,
                    owner,
                    repo,
                    number,
                });
            }
        }
    }

    anyhow::bail!(
        "Could not parse PR URL: {}\n\
         Supported formats:\n  \
         https://bitbucket.org/{{workspace}}/{{repo}}/pull-requests/{{number}}\n  \
         https://github.com/{{owner}}/{{repo}}/pull/{{number}}\n  \
         https://gitlab.com/{{owner}}/{{repo}}/-/merge_requests/{{number}}",
        url
    )
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

    #[test]
    fn test_parse_bitbucket_pr_url() {
        let parsed = parse_pr_url("https://bitbucket.org/myworkspace/myrepo/pull-requests/1554").unwrap();
        assert_eq!(parsed.platform, Platform::Bitbucket);
        assert_eq!(parsed.owner, "myworkspace");
        assert_eq!(parsed.repo, "myrepo");
        assert_eq!(parsed.number, 1554);
    }

    #[test]
    fn test_parse_github_pr_url() {
        let parsed = parse_pr_url("https://github.com/tchandrakar/nakama-cli-suite/pull/42").unwrap();
        assert_eq!(parsed.platform, Platform::GitHub);
        assert_eq!(parsed.owner, "tchandrakar");
        assert_eq!(parsed.repo, "nakama-cli-suite");
        assert_eq!(parsed.number, 42);
    }

    #[test]
    fn test_parse_gitlab_mr_url() {
        let parsed = parse_pr_url("https://gitlab.com/mygroup/myproject/-/merge_requests/99").unwrap();
        assert_eq!(parsed.platform, Platform::GitLab);
        assert_eq!(parsed.owner, "mygroup");
        assert_eq!(parsed.repo, "myproject");
        assert_eq!(parsed.number, 99);
    }
}
