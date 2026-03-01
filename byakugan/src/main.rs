//! Byakugan — AI-powered multi-pass PR and code reviewer.
//!
//! Subcommands:
//! - `review`              — Review the current branch's diff against main/master
//! - `pr <number>`         — Fetch and review a PR/MR by number
//! - `diff <file>`         — Review changes in a specific file
//! - `suggest`             — Suggest improvements for current changes
//! - `scan`                — Run custom rules against local diff (no AI)
//! - `report`              — Combined AI review + rule scan
//! - `comment <number>`    — Post a comment to a PR/MR
//! - `rules`               — Manage custom rules (list/test/validate)
//! - `watch`               — Polling daemon for auto-review

mod analysis;
mod auth;
mod comment;
mod dedup;
mod diff;
mod git;
mod ipc;
mod output;
mod passes;
mod platform;
mod pr;
mod report;
mod review;
mod rules;
mod rules_cmd;
mod scan;
mod suggest;
mod watch;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use nakama_ai::{create_provider, AiProvider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::types::{ModelTier, Provider};
use nakama_log::init_logging;
use nakama_ui::NakamaUI;
use nakama_vault::{CredentialStore, Vault};
use output::OutputFormat;
use std::time::Instant;

const TOOL_NAME: &str = "byakugan";

/// Byakugan - AI-powered multi-pass PR and code reviewer
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    /// AI provider to use (anthropic, openai, google, ollama)
    #[arg(long, default_value = "anthropic")]
    provider: String,

    /// Model tier: fast, balanced, powerful
    #[arg(long, default_value = "balanced")]
    tier: String,

    /// Output format: terminal, json, markdown
    #[arg(long, default_value = "terminal")]
    format: String,

    /// Platform: github, gitlab, bitbucket (auto-detected if omitted)
    #[arg(long)]
    platform: Option<String>,

    /// Repository owner (auto-detected from git remote if omitted)
    #[arg(long)]
    owner: Option<String>,

    /// Repository name (auto-detected from git remote if omitted)
    #[arg(long)]
    repo: Option<String>,

    /// Post results as comments/review to the PR/MR
    #[arg(long)]
    post: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Review the current branch's diff against main/master (multi-pass AI review)
    Review,

    /// Fetch and review a PR/MR by number
    Pr {
        /// The PR/MR number to review
        #[arg()]
        number: u64,
    },

    /// Review changes in a specific file
    Diff {
        /// Path to the file to review
        #[arg()]
        file: String,
    },

    /// Suggest improvements for current changes
    Suggest,

    /// Run custom rules against local diff (no AI)
    Scan,

    /// Combined AI review + rule scan report
    Report,

    /// Post a comment to a PR/MR
    Comment {
        /// The PR/MR number
        #[arg()]
        number: u64,

        /// Comment body text
        #[arg()]
        body: String,
    },

    /// Manage custom rules from config
    Rules {
        #[command(subcommand)]
        action: RulesAction,
    },

    /// Watch for new PRs and auto-review
    Watch {
        /// Run a single poll iteration and exit
        #[arg(long)]
        once: bool,
    },
}

#[derive(Subcommand, Debug)]
enum RulesAction {
    /// List all configured rules
    List,
    /// Validate all rule patterns
    Validate,
    /// Test a pattern against sample text
    Test {
        /// Regex pattern to test
        #[arg()]
        pattern: String,
        /// Sample text to test against
        #[arg()]
        sample: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;

    let ui = NakamaUI::from_config(&config);

    let update_rx = nakama_update::spawn_check(&config.updates, env!("CARGO_PKG_VERSION"));

    ui.panel(
        "Byakugan",
        &format!("AI-Powered Code Reviewer v{}", env!("CARGO_PKG_VERSION")),
    );

    let cli = Cli::parse();
    let format = OutputFormat::from_str(&cli.format);

    // Check if this command needs an AI provider.
    let needs_ai = matches!(
        cli.command,
        Commands::Review
            | Commands::Pr { .. }
            | Commands::Diff { .. }
            | Commands::Suggest
            | Commands::Report
            | Commands::Watch { .. }
    );

    // Create AI provider only when needed.
    let ai_provider: Option<Box<dyn AiProvider>> = if needs_ai {
        let provider_enum = parse_provider(&cli.provider)?;
        let model_tier = parse_tier(&cli.tier)?;
        let _model = config.resolve_model(provider_enum, model_tier);
        Some(create_ai_provider(&config, provider_enum, &_model)?)
    } else {
        None
    };

    let provider_enum = parse_provider(&cli.provider)?;
    let model_tier = parse_tier(&cli.tier)?;
    let model = config.resolve_model(provider_enum, model_tier);

    // Open the audit log.
    let audit_log = AuditLog::new(&config.audit).ok();

    let start = Instant::now();

    let (command_name, outcome) = match cli.command {
        Commands::Review => {
            let result = cmd_review(
                &ui,
                ai_provider.as_ref().unwrap().as_ref(),
                &model,
                &config,
            )
            .await;
            ("review", result)
        }
        Commands::Pr { number } => {
            let result = cmd_pr(
                &ui,
                ai_provider.as_ref().unwrap().as_ref(),
                &model,
                number,
                &config,
                cli.platform.as_deref(),
                cli.owner.as_deref(),
                cli.repo.as_deref(),
                cli.post,
            )
            .await;
            ("pr", result)
        }
        Commands::Diff { ref file } => {
            let result =
                cmd_diff(&ui, ai_provider.as_ref().unwrap().as_ref(), &model, file).await;
            ("diff", result)
        }
        Commands::Suggest => {
            let result =
                cmd_suggest(&ui, ai_provider.as_ref().unwrap().as_ref(), &model).await;
            ("suggest", result)
        }
        Commands::Scan => {
            let result = cmd_scan(&ui, &config, format).await;
            ("scan", result)
        }
        Commands::Report => {
            let result = cmd_report(
                &ui,
                ai_provider.as_ref().unwrap().as_ref(),
                &model,
                &config,
                format,
            )
            .await;
            ("report", result)
        }
        Commands::Comment { number, ref body } => {
            let result = cmd_comment(
                &ui,
                &config,
                cli.platform.as_deref(),
                cli.owner.as_deref(),
                cli.repo.as_deref(),
                number,
                body,
            )
            .await;
            ("comment", result)
        }
        Commands::Rules { ref action } => {
            let result = cmd_rules(&ui, &config, action);
            ("rules", result)
        }
        Commands::Watch { once } => {
            let result = cmd_watch(
                &ui,
                ai_provider.as_ref().unwrap().as_ref(),
                &model,
                &config,
                once,
            )
            .await;
            ("watch", result)
        }
    };

    let duration = start.elapsed();

    // Audit the command execution.
    if let Some(ref log) = audit_log {
        let (audit_outcome, detail) = match &outcome {
            Ok(()) => (
                Outcome::Success,
                serde_json::json!({
                    "provider": cli.provider,
                    "model": model,
                    "duration_secs": duration.as_secs_f64(),
                }),
            ),
            Err(e) => (
                Outcome::Failure,
                serde_json::json!({
                    "provider": cli.provider,
                    "model": model,
                    "error": format!("{:#}", e),
                    "duration_secs": duration.as_secs_f64(),
                }),
            ),
        };

        let trace_id = format!("tr_{}", uuid::Uuid::new_v4().simple());
        let entry = AuditEntry::new(
            &trace_id,
            TOOL_NAME,
            command_name,
            Category::AiInteraction,
            &format!("byakugan {} review", command_name),
            detail,
            audit_outcome,
            duration.as_millis() as u64,
        );

        if let Err(e) = log.log(entry) {
            nakama_log::warn!("Failed to write audit log: {}", e);
        }
    }

    // Report the final outcome.
    let final_result = match outcome {
        Ok(()) => {
            ui.success(&format!(
                "Complete in {:.1}s",
                duration.as_secs_f64()
            ));
            Ok(())
        }
        Err(e) => {
            ui.error(&format!("{:#}", e));
            Err(e)
        }
    };

    nakama_update::maybe_show_update(&ui, update_rx);

    final_result
}

// ---------------------------------------------------------------------------
// Subcommand implementations
// ---------------------------------------------------------------------------

/// `byakugan review` — Review the current branch against main/master.
async fn cmd_review(
    ui: &NakamaUI,
    provider: &dyn AiProvider,
    model: &str,
    config: &Config,
) -> Result<()> {
    let spinner = ui.step_start("Collecting branch diff...");

    let branch_diff = git::get_branch_diff()?;

    spinner.finish_with_success(&format!(
        "Branch '{}' vs '{}': {} file(s), +{} -{} ({} chars)",
        branch_diff.branch_name,
        branch_diff.base_branch,
        branch_diff.files_changed,
        branch_diff.insertions,
        branch_diff.deletions,
        branch_diff.diff_text.len(),
    ));

    let context_label = format!(
        "branch '{}' vs '{}'",
        branch_diff.branch_name, branch_diff.base_branch
    );

    let results = review::run_review_with_passes(
        ui,
        provider,
        model,
        &branch_diff.diff_text,
        &context_label,
        &config.byakugan.passes,
    )
    .await?;

    let stats = review::ReviewStats::from_results(&results);
    ui.panel(
        "Branch Review Complete",
        &format!(
            "Branch: {} -> {}\nFiles changed: {}\nTotal findings: {}\nHighest severity: {}\nTokens: {} in / {} out",
            branch_diff.branch_name,
            branch_diff.base_branch,
            branch_diff.files_changed,
            stats.total_findings,
            stats.max_severity,
            stats.total_input_tokens,
            stats.total_output_tokens,
        ),
    );

    // Emit IPC message if piped.
    ipc::emit_review_message(&context_label, &results);

    Ok(())
}

/// `byakugan pr <number>` — Fetch and review a PR/MR.
#[allow(clippy::too_many_arguments)]
async fn cmd_pr(
    ui: &NakamaUI,
    provider: &dyn AiProvider,
    model: &str,
    number: u64,
    config: &Config,
    platform_name: Option<&str>,
    owner: Option<&str>,
    repo: Option<&str>,
    post: bool,
) -> Result<()> {
    let spinner = ui.step_start(&format!("Fetching PR #{}...", number));

    let pr_data = pr::fetch_pr(
        number,
        &config.platforms,
        platform_name,
        owner,
        repo,
    )
    .await?;

    spinner.finish_with_success(&format!(
        "PR #{}: \"{}\" by {} ({} files, {} chars diff)",
        pr_data.number,
        pr_data.title,
        pr_data.author,
        pr_data.changed_files,
        pr_data.diff.len(),
    ));

    ui.panel("Pull Request", &pr::format_pr_context(&pr_data));

    let context_label = format!("PR #{}: {}", pr_data.number, pr_data.title);

    let results = review::run_review_with_passes(
        ui,
        provider,
        model,
        &pr_data.diff,
        &context_label,
        &config.byakugan.passes,
    )
    .await?;

    let stats = review::ReviewStats::from_results(&results);
    ui.panel(
        "PR Review Complete",
        &format!(
            "PR #{}: {}\nTotal findings: {}\nHighest severity: {}\nTokens: {} in / {} out",
            pr_data.number,
            pr_data.title,
            stats.total_findings,
            stats.max_severity,
            stats.total_input_tokens,
            stats.total_output_tokens,
        ),
    );

    // Post review if --post flag is set.
    if post {
        if let Ok(plat) = platform_name
            .map(platform::Platform::from_str)
            .unwrap_or_else(|| {
                platform::detect_platform_from_remote()
                    .ok_or_else(|| anyhow::anyhow!("Cannot detect platform"))
            })
        {
            if let Ok(adapter) = platform::create_adapter(plat, &config.platforms) {
                let (resolved_owner, resolved_repo) = if let (Some(o), Some(r)) = (owner, repo) {
                    (o.to_string(), r.to_string())
                } else {
                    platform::parse_owner_repo_from_remote()
                        .unwrap_or(("".to_string(), "".to_string()))
                };

                if !resolved_owner.is_empty() {
                    // Build review body from summary pass.
                    let body = results
                        .iter()
                        .find(|r| r.pass == passes::ReviewPass::Summary)
                        .map(|r| r.content.clone())
                        .unwrap_or_else(|| "Review complete.".to_string());

                    let verdict = if stats.max_severity >= passes::Severity::High {
                        platform::ReviewVerdict::RequestChanges
                    } else {
                        platform::ReviewVerdict::Comment
                    };

                    let review = platform::Review {
                        body,
                        verdict,
                        comments: Vec::new(),
                    };

                    let spinner = ui.step_start("Posting review...");
                    match adapter
                        .post_review(&resolved_owner, &resolved_repo, number, &review)
                        .await
                    {
                        Ok(_) => spinner.finish_with_success("Review posted successfully"),
                        Err(e) => spinner.finish_with_error(&format!("Failed to post: {}", e)),
                    }
                }
            }
        }
    }

    ipc::emit_review_message(&context_label, &results);

    Ok(())
}

/// `byakugan diff <file>` — Review a specific file's changes.
async fn cmd_diff(
    ui: &NakamaUI,
    provider: &dyn AiProvider,
    model: &str,
    file: &str,
) -> Result<()> {
    diff::review_file(ui, provider, model, file).await
}

/// `byakugan suggest` — Suggest improvements for current changes.
async fn cmd_suggest(ui: &NakamaUI, provider: &dyn AiProvider, model: &str) -> Result<()> {
    suggest::suggest_improvements(ui, provider, model).await
}

/// `byakugan scan` — Run custom rules against local diff.
async fn cmd_scan(ui: &NakamaUI, config: &Config, format: OutputFormat) -> Result<()> {
    scan::run_scan(ui, &config.byakugan.rules, format).await
}

/// `byakugan report` — Combined AI review + rule scan.
async fn cmd_report(
    ui: &NakamaUI,
    provider: &dyn AiProvider,
    model: &str,
    config: &Config,
    format: OutputFormat,
) -> Result<()> {
    report::run_report(ui, provider, model, &config.byakugan, format).await
}

/// `byakugan comment` — Post a comment to a PR/MR.
async fn cmd_comment(
    ui: &NakamaUI,
    config: &Config,
    platform_name: Option<&str>,
    owner: Option<&str>,
    repo: Option<&str>,
    number: u64,
    body: &str,
) -> Result<()> {
    comment::post_comment(
        ui,
        &config.platforms,
        platform_name,
        owner,
        repo,
        number,
        body,
    )
    .await
}

/// `byakugan rules` — Manage custom rules.
fn cmd_rules(ui: &NakamaUI, config: &Config, action: &RulesAction) -> Result<()> {
    match action {
        RulesAction::List => {
            rules_cmd::cmd_list(ui, &config.byakugan.rules);
            Ok(())
        }
        RulesAction::Validate => {
            rules_cmd::cmd_validate(ui, &config.byakugan.rules);
            Ok(())
        }
        RulesAction::Test {
            ref pattern,
            ref sample,
        } => rules_cmd::cmd_test(ui, pattern, sample),
    }
}

/// `byakugan watch` — Watch daemon for auto-review.
async fn cmd_watch(
    ui: &NakamaUI,
    provider: &dyn AiProvider,
    model: &str,
    config: &Config,
    once: bool,
) -> Result<()> {
    watch::run_watch(
        ui,
        provider,
        model,
        &config.byakugan,
        &config.platforms,
        once,
    )
    .await
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse the provider string into a `Provider` enum.
fn parse_provider(s: &str) -> Result<Provider> {
    match s.to_lowercase().as_str() {
        "anthropic" | "claude" => Ok(Provider::Anthropic),
        "openai" | "gpt" => Ok(Provider::OpenAI),
        "google" | "gemini" => Ok(Provider::Google),
        "ollama" | "local" => Ok(Provider::Ollama),
        other => anyhow::bail!(
            "Unknown provider '{}'. Use: anthropic, openai, google, or ollama",
            other
        ),
    }
}

/// Parse the tier string into a `ModelTier` enum.
fn parse_tier(s: &str) -> Result<ModelTier> {
    match s.to_lowercase().as_str() {
        "fast" | "f" => Ok(ModelTier::Fast),
        "balanced" | "b" | "default" => Ok(ModelTier::Balanced),
        "powerful" | "p" | "max" => Ok(ModelTier::Powerful),
        other => anyhow::bail!(
            "Unknown tier '{}'. Use: fast, balanced, or powerful",
            other
        ),
    }
}

/// Create a boxed AI provider, fetching the API key from the vault.
fn create_ai_provider(
    config: &Config,
    provider: Provider,
    model: &str,
) -> Result<Box<dyn AiProvider>> {
    let api_key = get_api_key(provider)?;

    let base_url = match provider {
        Provider::Anthropic => config.ai.anthropic.base_url.as_deref(),
        Provider::OpenAI => config.ai.openai.base_url.as_deref(),
        Provider::Google => config.ai.google.base_url.as_deref(),
        Provider::Ollama => Some(config.ai.ollama.base_url.as_str()),
    };

    create_provider(provider, &api_key, model, base_url)
        .context("Failed to create AI provider")
        .map_err(|e| anyhow::anyhow!("{}", e))
}

/// Retrieve the API key for the given provider from the vault,
/// falling back to environment variables.
fn get_api_key(provider: Provider) -> Result<String> {
    if provider == Provider::Ollama {
        return Ok(String::new());
    }

    let (service, env_var) = match provider {
        Provider::Anthropic => ("anthropic", "ANTHROPIC_API_KEY"),
        Provider::OpenAI => ("openai", "OPENAI_API_KEY"),
        Provider::Google => ("google", "GOOGLE_API_KEY"),
        Provider::Ollama => unreachable!(),
    };

    if let Ok(vault) = Vault::new() {
        if let Ok(secret) = vault.retrieve("nakama", &format!("{}_api_key", service)) {
            return Ok(secret.expose_secret().to_string());
        }
    }

    std::env::var(env_var).context(format!(
        "API key for {} not found. Set the {} environment variable or store it with:\n  \
         nakama vault store nakama {}_api_key <your-key>",
        provider, env_var, service
    ))
}
