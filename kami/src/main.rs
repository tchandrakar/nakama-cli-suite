use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;

const TOOL_NAME: &str = "kami";

/// Kami - Gemini-powered search and research
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search the web using Gemini-powered intelligence
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

    /// Ask a question and get a grounded, sourced answer
    Ask {
        /// The question to answer
        #[arg()]
        question: String,
    },

    /// Fact-check a claim with grounded sources
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

    println!("{} v{}", TOOL_NAME, env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    match cli.command {
        Commands::Search { query } => {
            println!("[search] Coming soon: Gemini-powered web search");
            println!("  Query: {}", query);
        }
        Commands::Deep { query } => {
            println!("[deep] Coming soon: deep research dive");
            println!("  Query: {}", query);
        }
        Commands::Summarize { url } => {
            println!("[summarize] Coming soon: URL content summarization");
            println!("  URL: {}", url);
        }
        Commands::Ask { question } => {
            println!("[ask] Coming soon: grounded Q&A");
            println!("  Question: {}", question);
        }
        Commands::Grounded { claim } => {
            println!("[grounded] Coming soon: claim fact-checking");
            println!("  Claim: {}", claim);
        }
        Commands::Compare { items } => {
            println!("[compare] Coming soon: multi-item comparison");
            println!("  Items: {}", items.join(", "));
        }
    }

    Ok(())
}
