mod ai_helper;
mod ask;
mod deps;
mod index_cmd;
mod map;

use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;
use nakama_ui::NakamaUI;

const TOOL_NAME: &str = "senku";

/// Senku - Codebase knowledge base and Q&A
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Index the current codebase: count files by extension and total lines
    Index,

    /// Ask a question about the codebase using AI
    Ask {
        /// The question to ask about the codebase
        #[arg()]
        question: String,
    },

    /// Generate a directory tree visualization of the codebase
    Map,

    /// List project dependencies from Cargo.toml, package.json, or requirements.txt
    Deps,

    /// Search the codebase with natural-language queries
    Search {
        /// The search query
        #[arg()]
        query: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;
    let ui = NakamaUI::from_config(&config);

    let update_rx = nakama_update::spawn_check(&config.updates, env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Index => index_cmd::run(&config, &ui).await,
        Commands::Ask { question } => ask::run(&config, &ui, &question).await,
        Commands::Map => map::run(&config, &ui).await,
        Commands::Deps => deps::run(&config, &ui).await,
        Commands::Search { query } => {
            ui.panel("Search", &format!("Searching for: {}\n\nNatural-language search is coming soon.", query));
            Ok(())
        }
    };

    if let Err(e) = result {
        ui.error(&format!("{}", e));
        nakama_update::maybe_show_update(&ui, update_rx);
        std::process::exit(1);
    }

    nakama_update::maybe_show_update(&ui, update_rx);

    Ok(())
}
