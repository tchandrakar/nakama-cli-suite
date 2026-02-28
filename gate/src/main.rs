mod ai_helper;
mod explore;
mod flow;
mod mock;
mod test_endpoint;

use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;
use nakama_ui::NakamaUI;

const TOOL_NAME: &str = "gate";

/// Gate - Interactive API explorer and HTTP client
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Send an HTTP request to an endpoint and display the response
    Test {
        /// The URL to test
        #[arg()]
        url: String,
    },

    /// Explore an API starting from a base URL with AI analysis
    Explore {
        /// The base URL of the API to explore
        #[arg()]
        base_url: String,
    },

    /// Read a JSON/YAML spec file and list the endpoints found
    Mock {
        /// Path to the API specification file (JSON or YAML)
        #[arg()]
        spec: String,
    },

    /// Run a sequence of API calls defined in a JSON config file
    Flow {
        /// Path to the flow configuration file
        #[arg()]
        config_file: String,
    },

    /// Show recent API request history
    History,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;
    let ui = NakamaUI::from_config(&config);

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Test { url } => test_endpoint::run(&config, &ui, &url).await,
        Commands::Explore { base_url } => explore::run(&config, &ui, &base_url).await,
        Commands::Mock { spec } => mock::run(&config, &ui, &spec).await,
        Commands::Flow { config_file } => flow::run(&config, &ui, &config_file).await,
        Commands::History => {
            ui.panel("Request History", "No request history recorded yet.");
            Ok(())
        }
    };

    if let Err(e) = result {
        ui.error(&format!("{}", e));
        std::process::exit(1);
    }

    Ok(())
}
