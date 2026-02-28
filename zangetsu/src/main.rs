use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;

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

    /// Translate a natural-language query into a shell command and execute it
    Run {
        /// The natural-language query describing what you want to run
        #[arg()]
        query: String,
    },

    /// Explain what a shell command does in plain English
    Explain {
        /// The shell command to explain
        #[arg()]
        command: String,
    },

    /// Show and search your shell history with AI-powered context
    History,

    /// Create a smart alias from a natural-language description
    Alias {
        /// Name for the alias
        #[arg()]
        name: String,

        /// Natural-language description of what the alias should do
        #[arg()]
        query: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;

    println!("{} v{}", TOOL_NAME, env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    match cli.command {
        Commands::Ask { query } => {
            println!("[ask] Coming soon: translating query to shell command");
            println!("  Query: {}", query);
        }
        Commands::Run { query } => {
            println!("[run] Coming soon: executing AI-generated command");
            println!("  Query: {}", query);
        }
        Commands::Explain { command } => {
            println!("[explain] Coming soon: explaining command");
            println!("  Command: {}", command);
        }
        Commands::History => {
            println!("[history] Coming soon: AI-enhanced shell history");
        }
        Commands::Alias { name, query } => {
            println!("[alias] Coming soon: creating smart alias");
            println!("  Name: {}", name);
            println!("  Query: {}", query);
        }
    }

    Ok(())
}
