//! `comment` command — Post a comment to a PR/MR via platform adapter.

use crate::platform::{self, Comment as PlatformComment, Platform};
use anyhow::{Context, Result};
use nakama_core::config::PlatformsConfig;
use nakama_ui::NakamaUI;

/// Post a comment to a PR/MR.
pub async fn post_comment(
    ui: &NakamaUI,
    platforms_config: &PlatformsConfig,
    platform_name: Option<&str>,
    owner: Option<&str>,
    repo: Option<&str>,
    number: u64,
    body: &str,
) -> Result<()> {
    let spinner = ui.step_start("Resolving platform...");

    // Determine platform.
    let plat = if let Some(name) = platform_name {
        Platform::from_str(name)?
    } else {
        platform::detect_platform_from_remote()
            .context("Could not detect platform from git remote. Use --platform to specify.")?
    };

    // Determine owner/repo.
    let (resolved_owner, resolved_repo) = if let (Some(o), Some(r)) = (owner, repo) {
        (o.to_string(), r.to_string())
    } else {
        platform::parse_owner_repo_from_remote()?
    };

    spinner.finish_with_success(&format!(
        "{} {}/{} #{}",
        plat, resolved_owner, resolved_repo, number
    ));

    let spinner = ui.step_start("Posting comment...");
    let adapter = platform::create_adapter(plat, platforms_config)?;

    let comment = PlatformComment {
        body: body.to_string(),
        path: None,
        line: None,
    };

    adapter
        .post_comment(&resolved_owner, &resolved_repo, number, &comment)
        .await?;

    spinner.finish_with_success("Comment posted successfully");
    Ok(())
}
