mod ai_helper;
mod ask;
mod compare;
mod deep;
mod grounded;
mod search;
mod summarize;

use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;
use nakama_ui::NakamaUI;

const TOOL_NAME: &str = "kami";

/// Kami - AI-powered search, research, and fact-checking
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search using AI intelligence
    Search {
        /// The search query
        #[arg()]
        query: String,
    },

    /// Perform a deep research dive on a topic
    Deep {
        /// The topic to research in depth
        #[arg()]
        query: String,
    },

    /// Summarize the content of a URL
    Summarize {
        /// The URL to summarize
        #[arg()]
        url: String,
    },

    /// Ask a question and get a grounded answer
    Ask {
        /// The question to answer
        #[arg()]
        question: String,
    },

    /// Fact-check a claim
    Grounded {
        /// The claim to verify
        #[arg()]
        claim: String,
    },

    /// Compare multiple items, technologies, or concepts
    Compare {
        /// Items to compare (provide two or more)
        #[arg(required = true, num_args = 2..)]
        items: Vec<String>,
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
        Commands::Search { query } => search::run(&config, &ui, &query).await,
        Commands::Deep { query } => deep::run(&config, &ui, &query).await,
        Commands::Summarize { url } => summarize::run(&config, &ui, &url).await,
        Commands::Ask { question } => ask::run(&config, &ui, &question).await,
        Commands::Grounded { claim } => grounded::run(&config, &ui, &claim).await,
        Commands::Compare { items } => compare::run(&config, &ui, &items).await,
    };

    if let Err(e) = result {
        ui.error(&format!("{}", e));
        nakama_update::maybe_show_update(&ui, update_rx);
        std::process::exit(1);
    }

    nakama_update::maybe_show_update(&ui, update_rx);

    Ok(())
}
