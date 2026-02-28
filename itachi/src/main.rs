mod ai_helper;
mod ask;
mod atlassian;
mod brief;
mod create;
mod jira;
mod sprint;
mod standup;
mod wiki;

use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;
use nakama_ui::NakamaUI;

const TOOL_NAME: &str = "itachi";

/// Itachi - Jira & Confluence intelligence hub
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search Jira with natural language
    Jira {
        #[arg()]
        query: String,
    },

    /// Search Confluence wiki pages
    Wiki {
        #[arg()]
        query: String,
    },

    /// Ask a question across Jira and Confluence
    Ask {
        #[arg()]
        question: String,
    },

    /// Generate a team briefing from recent activity
    Brief {
        #[arg()]
        team: Option<String>,
    },

    /// Generate a standup from your Jira activity
    Standup,

    /// Create a new Jira issue
    Create {
        /// Issue type (bug, story, task, epic)
        #[arg()]
        issue_type: String,
        /// Issue summary
        #[arg()]
        summary: String,
    },

    /// Show sprint information
    Sprint {
        /// Board name (optional)
        #[arg()]
        board: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;
    let ui = NakamaUI::from_config(&config);

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Jira { query } => jira::run(&config, &ui, &query).await,
        Commands::Wiki { query } => wiki::run(&config, &ui, &query).await,
        Commands::Ask { question } => ask::run(&config, &ui, &question).await,
        Commands::Brief { team } => brief::run(&config, &ui, team.as_deref()).await,
        Commands::Standup => standup::run(&config, &ui).await,
        Commands::Create { issue_type, summary } => create::run(&config, &ui, &issue_type, &summary).await,
        Commands::Sprint { board } => sprint::run(&config, &ui, board.as_deref()).await,
    };

    if let Err(e) = result {
        ui.error(&format!("{}", e));
        std::process::exit(1);
    }

    Ok(())
}
