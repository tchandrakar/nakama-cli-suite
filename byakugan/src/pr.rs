//! GitHub PR fetching via the `gh` CLI.
//!
//! Shells out to the GitHub CLI (`gh`) to retrieve PR metadata and diffs.
//! This avoids hard-coding GitHub API tokens and leverages the user's
//! existing `gh auth` session.

use anyhow::{Context, Result};
use std::process::Command;

/// Metadata and diff for a GitHub pull request.
pub struct PrData {
    /// PR number.
    pub number: u64,
    /// PR title.
    pub title: String,
    /// PR author login.
    pub author: String,
    /// PR base branch (e.g., "main").
    pub base_branch: String,
    /// PR head branch (e.g., "feature/foo").
    pub head_branch: String,
    /// PR body/description.
    pub body: String,
    /// The unified diff of the PR.
    pub diff: String,
    /// Number of changed files.
    pub changed_files: usize,
}

/// Fetch a GitHub PR by number using the `gh` CLI.
///
/// Requires `gh` to be installed and authenticated (`gh auth login`).
pub fn fetch_pr(pr_number: u64) -> Result<PrData> {
    // Verify gh is available.
    ensure_gh_installed()?;

    // Fetch PR metadata as JSON.
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

    // Fetch the PR diff.
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
