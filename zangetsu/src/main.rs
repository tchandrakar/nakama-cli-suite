//! Zangetsu — Your AI-powered shell companion.
//!
//! Translates natural-language queries into shell commands, explains commands,
//! fixes failures, builds pipelines, and keeps a history of interactions.

use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;
use nakama_ui::NakamaUI;

mod ask;
mod chain;
mod context;
mod explain;
mod fix;
mod history;
mod provider;
mod risk;

const TOOL_NAME: &str = "zangetsu";

/// Zangetsu - Your AI-powered shell companion
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Ask a natural-language question and get a shell command suggestion
    Ask {
        /// The natural-language query describing what you want to do
        #[arg()]
        query: String,
    },

    /// Explain what a shell command does in plain English
    Explain {
        /// The shell command to explain
        #[arg()]
        command: String,
    },

    /// Read the last failed command from shell history and suggest a fix
    Fix,

    /// Generate a multi-step command pipeline from a description
    Chain {
        /// Natural-language description of the pipeline you want
        #[arg()]
        query: String,
    },

    /// Show AI interaction history for zangetsu
    History,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;
    let ui = NakamaUI::from_config(&config);

    let update_rx = nakama_update::spawn_check(&config.updates, env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Ask { query } => ask::run(&config, &ui, &query).await,
        Commands::Explain { command } => explain::run(&config, &ui, &command).await,
        Commands::Fix => fix::run(&config, &ui).await,
        Commands::Chain { query } => chain::run(&config, &ui, &query).await,
        Commands::History => history::run(&config, &ui).await,
    };

    if let Err(ref e) = result {
        ui.error(&format!("{:#}", e));
    }

    nakama_update::maybe_show_update(&ui, update_rx);

    result
}
