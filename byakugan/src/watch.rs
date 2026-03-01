//! Watch daemon — polling loop for auto-reviewing new/updated PRs.

use crate::platform::{self, Platform};
use crate::review;
use anyhow::{Context, Result};
use nakama_ai::AiProvider;
use nakama_core::config::{ByakuganConfig, PlatformsConfig};
use nakama_ui::NakamaUI;
use std::collections::HashMap;
use std::time::Duration;

/// Parsed repository specification from config.
struct RepoSpec {
    platform: Platform,
    owner: String,
    repo: String,
}

/// Run the watch daemon.
pub async fn run_watch(
    ui: &NakamaUI,
    provider: &dyn AiProvider,
    model: &str,
    byakugan_config: &ByakuganConfig,
    platforms_config: &PlatformsConfig,
    once: bool,
) -> Result<()> {
    let repos = parse_repo_specs(&byakugan_config.watch.repos)?;

    if repos.is_empty() {
        ui.panel(
            "Watch",
            "No repositories configured for watching.\n\
             Add repos to ~/.nakama/config.toml:\n\n\
             [byakugan.watch]\n\
             repos = [\"github:owner/repo\", \"gitlab:owner/repo\"]",
        );
        return Ok(());
    }

    let interval = Duration::from_secs(byakugan_config.watch.poll_interval_seconds);

    ui.panel(
        "Byakugan Watch",
        &format!(
            "Watching {} repo(s), poll interval: {}s, auto-review: {}\nMode: {}",
            repos.len(),
            byakugan_config.watch.poll_interval_seconds,
            byakugan_config.watch.auto_review,
            if once { "single poll" } else { "daemon" },
        ),
    );

    // Track last-seen PR timestamps/numbers.
    let mut seen: HashMap<String, u64> = HashMap::new();

    loop {
        for spec in &repos {
            let key = format!("{}:{}/{}", spec.platform, spec.owner, spec.repo);
            let spinner = ui.step_start(&format!("Polling {}...", key));

            match poll_repo(
                ui,
                provider,
                model,
                byakugan_config,
                platforms_config,
                spec,
                &mut seen,
            )
            .await
            {
                Ok(new_count) => {
                    spinner.finish_with_success(&format!(
                        "{}: {} new/updated PR(s)",
                        key, new_count
                    ));
                }
                Err(e) => {
                    spinner.finish_with_error(&format!("{}: {}", key, e));
                }
            }
        }

        if once {
            ui.success("Watch poll complete (--once mode).");
            break;
        }

        ui.step_start(&format!("Sleeping {}s...", interval.as_secs()))
            .finish_with_success("Waking up");
        tokio::time::sleep(interval).await;
    }

    Ok(())
}

/// Poll a single repository for new PRs.
async fn poll_repo(
    ui: &NakamaUI,
    provider: &dyn AiProvider,
    model: &str,
    byakugan_config: &ByakuganConfig,
    platforms_config: &PlatformsConfig,
    spec: &RepoSpec,
    seen: &mut HashMap<String, u64>,
) -> Result<usize> {
    let adapter = platform::create_adapter(spec.platform, platforms_config)?;
    let prs = adapter
        .list_pull_requests(&spec.owner, &spec.repo)
        .await?;

    let key = format!("{}:{}/{}", spec.platform, spec.owner, spec.repo);
    let last_seen = seen.get(&key).copied().unwrap_or(0);

    let new_prs: Vec<_> = prs.iter().filter(|pr| pr.number > last_seen).collect();
    let count = new_prs.len();

    if let Some(max_num) = new_prs.iter().map(|pr| pr.number).max() {
        seen.insert(key, max_num);
    }

    if byakugan_config.watch.auto_review {
        for pr in &new_prs {
            if let Ok(diff) = adapter
                .get_pull_request_diff(&spec.owner, &spec.repo, pr.number)
                .await
            {
                let context = format!("PR #{}: {}", pr.number, pr.title);
                let _ = review::run_review(ui, provider, model, &diff, &context).await;
            }
        }
    } else if byakugan_config.watch.notify && !new_prs.is_empty() {
        let mut msg = format!("{} new PR(s):\n", count);
        for pr in &new_prs {
            msg.push_str(&format!("  #{}: {} (by {})\n", pr.number, pr.title, pr.author));
        }
        ui.panel(&format!("New PRs in {}/{}", spec.owner, spec.repo), &msg);
    }

    Ok(count)
}

/// Parse repo specs from config strings like "github:owner/repo".
fn parse_repo_specs(specs: &[String]) -> Result<Vec<RepoSpec>> {
    let mut result = Vec::new();

    for spec in specs {
        let (platform_str, path) = spec
            .split_once(':')
            .context(format!(
                "Invalid repo spec '{}'. Use format: platform:owner/repo",
                spec
            ))?;

        let platform = Platform::from_str(platform_str)?;

        let (owner, repo) = path
            .split_once('/')
            .context(format!(
                "Invalid repo path '{}'. Use format: owner/repo",
                path
            ))?;

        result.push(RepoSpec {
            platform,
            owner: owner.to_string(),
            repo: repo.to_string(),
        });
    }

    Ok(result)
}
