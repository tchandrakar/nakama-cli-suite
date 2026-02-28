use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;

const TOOL_NAME: &str = "jogan";

/// Jogan - Cross-layer infrastructure debugger
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan the local environment for common infrastructure issues
    Scan,

    /// Diagnose a symptom across infrastructure layers
    Diagnose {
        /// The symptom to investigate (e.g., "high latency", "connection refused")
        #[arg()]
        symptom: String,
    },

    /// Trace a request through a service and its dependencies
    Trace {
        /// The service name or endpoint to trace
        #[arg()]
        service: String,
    },

    /// Run a health check across all configured services
    Health,

    /// Explain an infrastructure resource or concept
    Explain {
        /// The resource or concept to explain (e.g., "k8s pod", "nginx config")
        #[arg()]
        resource: String,
    },

    /// Watch infrastructure metrics in real-time
    Watch,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;

    println!("{} v{}", TOOL_NAME, env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    match cli.command {
        Commands::Scan => {
            println!("[scan] Coming soon: infrastructure issue scanner");
        }
        Commands::Diagnose { symptom } => {
            println!("[diagnose] Coming soon: cross-layer diagnosis");
            println!("  Symptom: {}", symptom);
        }
        Commands::Trace { service } => {
            println!("[trace] Coming soon: request tracing");
            println!("  Service: {}", service);
        }
        Commands::Health => {
            println!("[health] Coming soon: service health checker");
        }
        Commands::Explain { resource } => {
            println!("[explain] Coming soon: infrastructure explainer");
            println!("  Resource: {}", resource);
        }
        Commands::Watch => {
            println!("[watch] Coming soon: real-time infrastructure monitoring");
        }
    }

    Ok(())
}
