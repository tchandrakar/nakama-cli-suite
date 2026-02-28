//! Byakugan — AI-powered multi-pass PR and code reviewer.
//!
//! Subcommands:
//! - `review`        — Review the current branch's diff against main/master
//! - `pr <number>`   — Fetch and review a GitHub PR by number
//! - `diff <file>`   — Review changes in a specific file
//! - `suggest`       — Suggest improvements for current changes

mod diff;
mod git;
mod passes;
mod pr;
mod review;
mod suggest;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use nakama_ai::{create_provider, AiProvider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::types::{ModelTier, Provider};
use nakama_log::init_logging;
use nakama_ui::NakamaUI;
use nakama_vault::{CredentialStore, Vault};
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

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Review the current branch's diff against main/master (multi-pass AI review)
    Review,

    /// Fetch and review a GitHub PR by number
    Pr {
        /// The PR number to review
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

    // Resolve provider and model from CLI args + config.
    let provider_enum = parse_provider(&cli.provider)?;
    let model_tier = parse_tier(&cli.tier)?;
    let model = config.resolve_model(provider_enum, model_tier);

    // Create the AI provider (retrieve API key from vault).
    let ai_provider = create_ai_provider(&config, provider_enum, &model)?;

    // Open the audit log.
    let audit_log = AuditLog::new(&config.audit).ok();

    let start = Instant::now();

    let (command_name, outcome) = match cli.command {
        Commands::Review => {
            let result = cmd_review(&ui, ai_provider.as_ref(), &model).await;
            ("review", result)
        }
        Commands::Pr { number } => {
            let result = cmd_pr(&ui, ai_provider.as_ref(), &model, number).await;
            ("pr", result)
        }
        Commands::Diff { ref file } => {
            let result = cmd_diff(&ui, ai_provider.as_ref(), &model, file).await;
            ("diff", result)
        }
        Commands::Suggest => {
            let result = cmd_suggest(&ui, ai_provider.as_ref(), &model).await;
            ("suggest", result)
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
            // Audit failures should not block the user.
            nakama_log::warn!("Failed to write audit log: {}", e);
        }
    }

    // Report the final outcome.
    let final_result = match outcome {
        Ok(()) => {
            ui.success(&format!(
                "Review complete in {:.1}s",
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
async fn cmd_review(ui: &NakamaUI, provider: &dyn AiProvider, model: &str) -> Result<()> {
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

    let results =
        review::run_review(ui, provider, model, &branch_diff.diff_text, &context_label).await?;

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

    Ok(())
}

/// `byakugan pr <number>` — Fetch and review a GitHub PR.
async fn cmd_pr(ui: &NakamaUI, provider: &dyn AiProvider, model: &str, number: u64) -> Result<()> {
    let spinner = ui.step_start(&format!("Fetching PR #{}...", number));

    let pr_data = pr::fetch_pr(number)?;

    spinner.finish_with_success(&format!(
        "PR #{}: \"{}\" by {} ({} files, {} chars diff)",
        pr_data.number,
        pr_data.title,
        pr_data.author,
        pr_data.changed_files,
        pr_data.diff.len(),
    ));

    // Show PR metadata panel.
    ui.panel("Pull Request", &pr::format_pr_context(&pr_data));

    let context_label = format!("PR #{}: {}", pr_data.number, pr_data.title);

    let results =
        review::run_review(ui, provider, model, &pr_data.diff, &context_label).await?;

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
    // For Ollama (local), no API key is needed.
    if provider == Provider::Ollama {
        return Ok(String::new());
    }

    let (service, env_var) = match provider {
        Provider::Anthropic => ("anthropic", "ANTHROPIC_API_KEY"),
        Provider::OpenAI => ("openai", "OPENAI_API_KEY"),
        Provider::Google => ("google", "GOOGLE_API_KEY"),
        Provider::Ollama => unreachable!(),
    };

    // Try the vault first.
    if let Ok(vault) = Vault::new() {
        if let Ok(secret) = vault.retrieve("nakama", &format!("{}_api_key", service)) {
            return Ok(secret.expose_secret().to_string());
        }
    }

    // Fall back to environment variable.
    std::env::var(env_var).context(format!(
        "API key for {} not found. Set the {} environment variable or store it with:\n  \
         nakama vault store nakama {}_api_key <your-key>",
        provider, env_var, service
    ))
}
