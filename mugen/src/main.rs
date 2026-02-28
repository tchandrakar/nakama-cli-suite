mod ai_helper;
mod cover;
mod edge;
mod fuzz;
mod gen;
mod mutate;
mod review;

use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;
use nakama_ui::NakamaUI;

const TOOL_NAME: &str = "mugen";

/// Mugen - AI-powered test generator
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate tests for a target file
    Gen {
        /// The file path to generate tests for
        #[arg()]
        target: String,
    },

    /// Analyze test coverage and suggest missing tests
    Cover,

    /// Suggest mutations to validate test quality
    Mutate {
        /// The source file to analyze
        #[arg()]
        file: String,
    },

    /// Generate edge-case tests for a function (use file:function format)
    Edge {
        /// The function to target (e.g. "src/lib.rs:parse_config")
        #[arg()]
        function: String,
    },

    /// Generate fuzz test harnesses
    Fuzz {
        /// The function to fuzz (e.g. "src/lib.rs:parse_input")
        #[arg()]
        function: String,
    },

    /// Review an existing test file for quality
    Review {
        /// Path to the test file
        #[arg()]
        test_file: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;
    let ui = NakamaUI::from_config(&config);

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Gen { target } => gen::run(&config, &ui, &target).await,
        Commands::Cover => cover::run(&config, &ui).await,
        Commands::Mutate { file } => mutate::run(&config, &ui, &file).await,
        Commands::Edge { function } => edge::run(&config, &ui, &function).await,
        Commands::Fuzz { function } => fuzz::run(&config, &ui, &function).await,
        Commands::Review { test_file } => review::run(&config, &ui, &test_file).await,
    };

    if let Err(e) = result {
        ui.error(&format!("{}", e));
        std::process::exit(1);
    }

    Ok(())
}
