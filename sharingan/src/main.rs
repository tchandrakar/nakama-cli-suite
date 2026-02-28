mod ai_helper;
mod analyze;
mod correlate;
mod parser;
mod search;
mod stats;
mod watch;

use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;
use nakama_ui::NakamaUI;

const TOOL_NAME: &str = "sharingan";

/// Sharingan - AI-powered log analyzer
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Tail a log file with color-highlighted errors and warnings
    Tail {
        /// Path to the log file to tail
        #[arg()]
        source: String,
    },

    /// Explain a log file's contents and highlight anomalies
    Explain {
        /// Path to the log file to explain
        #[arg()]
        logfile: String,
    },

    /// Scan a log file for errors, warnings, and patterns
    Scan {
        /// Path to the log file to scan
        #[arg()]
        logfile: String,
    },

    /// Correlate events across two log sources
    Correlate {
        /// First log source
        #[arg()]
        source1: String,

        /// Second log source
        #[arg()]
        source2: String,
    },

    /// Predict potential issues from log patterns
    Predict {
        /// The log source to analyze for predictions
        #[arg()]
        source: String,
    },

    /// Filter logs using a regex pattern with AI-powered context
    Filter {
        /// Regex pattern to search for
        #[arg()]
        query: String,

        /// The log source to filter
        #[arg()]
        source: String,
    },

    /// Generate a summary of a log source with statistics
    Summary {
        /// The log source to summarize
        #[arg()]
        source: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;
    let ui = NakamaUI::from_config(&config);

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Tail { source } => watch::run(&config, &ui, &source).await,
        Commands::Explain { logfile } => analyze::run(&config, &ui, &logfile).await,
        Commands::Scan { logfile } => stats::run(&config, &ui, &logfile).await,
        Commands::Correlate { source1, source2 } => {
            correlate::run(&config, &ui, &source1, &source2).await
        }
        Commands::Filter { query, source } => search::run(&config, &ui, &query, &source).await,
        Commands::Summary { source } => stats::run(&config, &ui, &source).await,
        Commands::Predict { source } => {
            println!("[predict] Coming soon: predictive log analysis for {}", source);
            Ok(())
        }
    };

    if let Err(e) = result {
        ui.error(&format!("{}", e));
        std::process::exit(1);
    }

    Ok(())
}
