//! PR/MR fetching — uses PlatformAdapter with `gh` CLI fallback.

use crate::platform::{self, Platform, PlatformAdapter};
use anyhow::{Context, Result};
use nakama_core::config::PlatformsConfig;
use std::process::Command;

/// Metadata and diff for a pull/merge request.
pub struct PrData {
    pub number: u64,
    pub title: String,
    pub author: String,
    pub base_branch: String,
    pub head_branch: String,
    pub body: String,
    pub diff: String,
    pub changed_files: usize,
}

/// Fetch a PR/MR using the platform adapter, with `gh` CLI fallback for GitHub.
pub async fn fetch_pr(
    pr_number: u64,
    platforms_config: &PlatformsConfig,
    platform_name: Option<&str>,
    owner: Option<&str>,
    repo: Option<&str>,
) -> Result<PrData> {
    // Determine platform.
    let plat = if let Some(name) = platform_name {
        Platform::from_str(name)?
    } else {
        platform::detect_platform_from_remote().unwrap_or(Platform::GitHub)
    };

    // Determine owner/repo.
    let (resolved_owner, resolved_repo) = if let (Some(o), Some(r)) = (owner, repo) {
        (o.to_string(), r.to_string())
    } else {
        platform::parse_owner_repo_from_remote()
            .unwrap_or_else(|_| ("".to_string(), "".to_string()))
    };

    // Try adapter first if we have owner/repo.
    if !resolved_owner.is_empty() && !resolved_repo.is_empty() {
        if let Ok(adapter) = platform::create_adapter(plat, platforms_config) {
            return fetch_via_adapter(
                adapter.as_ref(),
                &resolved_owner,
                &resolved_repo,
                pr_number,
            )
            .await;
        }
    }

    // Fallback to gh CLI for GitHub.
    if plat == Platform::GitHub {
        return fetch_pr_gh(pr_number);
    }

    anyhow::bail!(
        "Could not fetch PR #{} from {}. Configure a token or use --owner/--repo flags.",
        pr_number,
        plat
    )
}

/// Fetch PR via a PlatformAdapter.
async fn fetch_via_adapter(
    adapter: &dyn PlatformAdapter,
    owner: &str,
    repo: &str,
    number: u64,
) -> Result<PrData> {
    let pr = adapter.get_pull_request(owner, repo, number).await?;
    let diff = adapter.get_pull_request_diff(owner, repo, number).await?;

    if diff.trim().is_empty() {
        anyhow::bail!("PR/MR #{} has an empty diff.", number);
    }

    // Estimate changed files from diff.
    let changed_files = diff.lines().filter(|l| l.starts_with("diff --git") || l.starts_with("---")).count() / 2;

    Ok(PrData {
        number: pr.number,
        title: pr.title,
        author: pr.author,
        base_branch: pr.base_branch,
        head_branch: pr.head_branch,
        body: pr.body,
        diff,
        changed_files: changed_files.max(1),
    })
}

/// Fetch a GitHub PR using the `gh` CLI (legacy fallback).
fn fetch_pr_gh(pr_number: u64) -> Result<PrData> {
    ensure_gh_installed()?;

    let meta_output = Command::new("gh")
        .args([
            "pr",
            "view",
            &pr_number.to_string(),
            "--json",
            "number,title,author,baseRefName,headRefName,body,changedFiles",
        ])
        .output()
        .context("Failed to execute `gh pr view`. Is `gh` installed and authenticated?")?;

    if !meta_output.status.success() {
        let stderr = String::from_utf8_lossy(&meta_output.stderr);
        anyhow::bail!(
            "gh pr view failed (exit {}):\n{}",
            meta_output.status.code().unwrap_or(-1),
            stderr.trim()
        );
    }

    let meta_json: serde_json::Value =
        serde_json::from_slice(&meta_output.stdout).context("Failed to parse gh pr view JSON")?;

    let number = meta_json["number"].as_u64().unwrap_or(pr_number);
    let title = meta_json["title"]
        .as_str()
        .unwrap_or("(untitled)")
        .to_string();
    let author = meta_json["author"]["login"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let base_branch = meta_json["baseRefName"]
        .as_str()
        .unwrap_or("main")
        .to_string();
    let head_branch = meta_json["headRefName"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let body = meta_json["body"].as_str().unwrap_or("").to_string();
    let changed_files = meta_json["changedFiles"].as_u64().unwrap_or(0) as usize;

    let diff_output = Command::new("gh")
        .args(["pr", "diff", &pr_number.to_string()])
        .output()
        .context("Failed to execute `gh pr diff`")?;

    if !diff_output.status.success() {
        let stderr = String::from_utf8_lossy(&diff_output.stderr);
        anyhow::bail!(
            "gh pr diff failed (exit {}):\n{}",
            diff_output.status.code().unwrap_or(-1),
            stderr.trim()
        );
    }

    let diff = String::from_utf8_lossy(&diff_output.stdout).to_string();

    if diff.trim().is_empty() {
        anyhow::bail!("PR #{} has an empty diff.", pr_number);
    }

    Ok(PrData {
        number,
        title,
        author,
        base_branch,
        head_branch,
        body,
        diff,
        changed_files,
    })
}

/// Format PR metadata into a human-readable context string.
pub fn format_pr_context(pr: &PrData) -> String {
    let mut ctx = format!(
        "PR #{}: {}\nAuthor: {}\nBranch: {} -> {}\nChanged files: {}",
        pr.number, pr.title, pr.author, pr.head_branch, pr.base_branch, pr.changed_files,
    );
    if !pr.body.is_empty() {
        ctx.push_str(&format!("\n\nDescription:\n{}", pr.body));
    }
    ctx
}

/// Ensure the `gh` CLI is installed and accessible.
fn ensure_gh_installed() -> Result<()> {
    let output = Command::new("gh")
        .arg("--version")
        .output()
        .context(
            "The GitHub CLI (`gh`) is not installed or not in PATH.\n\
             Install it from: https://cli.github.com/\n\
             Then authenticate with: gh auth login",
        )?;

    if !output.status.success() {
        anyhow::bail!(
            "The GitHub CLI (`gh`) returned an error. \
             Please ensure it is properly installed and authenticated."
        );
    }

    Ok(())
}
