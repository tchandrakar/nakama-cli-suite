use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;

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
    /// Send an HTTP request
    Send {
        /// HTTP method (GET, POST, PUT, DELETE, PATCH, etc.)
        #[arg()]
        method: String,

        /// The URL to send the request to
        #[arg()]
        url: String,
    },

    /// Interactively explore an API starting from a base URL
    Explore {
        /// The base URL of the API to explore
        #[arg()]
        base_url: String,
    },

    /// Import an API specification (OpenAPI, Swagger, Postman, etc.)
    Import {
        /// Path or URL to the API specification file
        #[arg()]
        spec: String,
    },

    /// Replay a previously recorded request by ID
    Replay {
        /// The request ID to replay
        #[arg()]
        request_id: String,
    },

    /// Run a named request flow (sequence of API calls)
    Flow {
        /// Name of the flow to execute
        #[arg()]
        name: String,
    },

    /// Start a mock server from an API specification
    Mock {
        /// Path or URL to the API specification file
        #[arg()]
        spec: String,
    },

    /// Diff two recorded API responses
    Diff {
        /// First request ID or file path
        #[arg()]
        req1: String,

        /// Second request ID or file path
        #[arg()]
        req2: String,
    },

    /// Generate documentation for an API collection
    Doc {
        /// Name of the API collection to document
        #[arg()]
        collection: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;

    println!("{} v{}", TOOL_NAME, env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    match cli.command {
        Commands::Send { method, url } => {
            println!("[send] Coming soon: HTTP request sender");
            println!("  Method: {}", method);
            println!("  URL: {}", url);
        }
        Commands::Explore { base_url } => {
            println!("[explore] Coming soon: interactive API explorer");
            println!("  Base URL: {}", base_url);
        }
        Commands::Import { spec } => {
            println!("[import] Coming soon: API spec importer");
            println!("  Spec: {}", spec);
        }
        Commands::Replay { request_id } => {
            println!("[replay] Coming soon: request replay");
            println!("  Request ID: {}", request_id);
        }
        Commands::Flow { name } => {
            println!("[flow] Coming soon: request flow runner");
            println!("  Flow: {}", name);
        }
        Commands::Mock { spec } => {
            println!("[mock] Coming soon: mock server");
            println!("  Spec: {}", spec);
        }
        Commands::Diff { req1, req2 } => {
            println!("[diff] Coming soon: response diff");
            println!("  Request 1: {}", req1);
            println!("  Request 2: {}", req2);
        }
        Commands::Doc { collection } => {
            println!("[doc] Coming soon: API documentation generator");
            println!("  Collection: {}", collection);
        }
    }

    Ok(())
}
