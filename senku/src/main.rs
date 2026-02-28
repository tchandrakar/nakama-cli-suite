use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;

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
    /// Index the current codebase for AI-powered search and Q&A
    Index,

    /// Ask a question about the codebase
    Ask {
        /// The question to ask about the codebase
        #[arg()]
        question: String,
    },

    /// Explain a file, function, module, or concept in the codebase
    Explain {
        /// The target to explain (file path, function name, module, etc.)
        #[arg()]
        target: String,
    },

    /// Generate an architectural map of the codebase
    Map,

    /// Generate an onboarding guide for new contributors
    Onboard,

    /// Search the codebase with natural-language queries
    Search {
        /// The search query
        #[arg()]
        query: String,
    },

    /// Explain the current git diff in plain English
    #[command(name = "diff-explain")]
    DiffExplain,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;

    println!("{} v{}", TOOL_NAME, env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    match cli.command {
        Commands::Index => {
            println!("[index] Coming soon: codebase indexing for AI search");
        }
        Commands::Ask { question } => {
            println!("[ask] Coming soon: codebase Q&A");
            println!("  Question: {}", question);
        }
        Commands::Explain { target } => {
            println!("[explain] Coming soon: codebase explainer");
            println!("  Target: {}", target);
        }
        Commands::Map => {
            println!("[map] Coming soon: architectural map generation");
        }
        Commands::Onboard => {
            println!("[onboard] Coming soon: onboarding guide generation");
        }
        Commands::Search { query } => {
            println!("[search] Coming soon: natural-language codebase search");
            println!("  Query: {}", query);
        }
        Commands::DiffExplain => {
            println!("[diff-explain] Coming soon: AI-powered diff explanation");
        }
    }

    Ok(())
}
