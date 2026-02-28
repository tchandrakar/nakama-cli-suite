use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;
use nakama_ui::NakamaUI;

mod ai_helper;
mod branch;
mod changelog;
mod commit;
mod git;
mod hook;
mod release;
mod review;
mod squash;

const TOOL_NAME: &str = "shinigami";

/// Shinigami - AI-powered git workflow automator
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate an AI-crafted commit message from staged changes
    Commit,

    /// Summarize commits between two refs (reap the changelog)
    Reap {
        /// Starting ref (defaults to last tag)
        #[arg(long)]
        from: Option<String>,

        /// Ending ref (defaults to HEAD)
        #[arg(long)]
        to: Option<String>,
    },

    /// Create a well-named branch from a natural-language description
    Branch {
        /// Description of what the branch is for
        #[arg()]
        description: String,
    },

    /// Interactively squash commits with AI-generated squash message
    Squash,

    /// Prepare release notes for a given version
    Release {
        /// The version to release (e.g., "1.2.0")
        #[arg()]
        version: String,
    },

    /// AI-powered review of uncommitted changes
    Review,

    /// Manage git hooks (install, remove, list)
    Hook {
        /// Action to perform: install, remove, or list
        #[arg()]
        action: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;
    let ui = NakamaUI::from_config(&config);

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Commit => commit::run(&config, &ui).await,
        Commands::Reap { from, to } => changelog::run(&config, &ui, from, to).await,
        Commands::Branch { description } => branch::run(&config, &ui, &description).await,
        Commands::Squash => squash::run(&config, &ui).await,
        Commands::Release { version } => release::run(&config, &ui, &version).await,
        Commands::Review => review::run(&config, &ui).await,
        Commands::Hook { action } => hook::run(&ui, &action),
    };

    if let Err(e) = &result {
        ui.error(&format!("{:#}", e));
    }

    result
}
