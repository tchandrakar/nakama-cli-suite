mod ai_helper;
mod analyze;
mod diagnose;
mod explain;
mod health;
mod network;
mod system;

use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;
use nakama_ui::NakamaUI;

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
    /// Diagnose a symptom across infrastructure layers
    Diagnose {
        /// The symptom to investigate (e.g., "high latency", "connection refused")
        #[arg()]
        symptom: String,
    },

    /// Analyze a log file for error patterns and root causes
    Analyze {
        /// Path to the log file to analyze
        #[arg()]
        logfile: String,
    },

    /// Run a comprehensive health check (disk, memory, processes, network)
    Health,

    /// Explain an error message or infrastructure concept
    Explain {
        /// The error message or concept to explain (e.g., "ECONNREFUSED", "k8s pod")
        #[arg()]
        resource: String,
    },

    /// Scan the local environment for common infrastructure issues
    Scan,

    /// Trace a request through a service and its dependencies
    Trace {
        /// The service name or endpoint to trace
        #[arg()]
        service: String,
    },

    /// Watch infrastructure metrics in real-time
    Watch,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;
    let ui = NakamaUI::from_config(&config);

    let update_rx = nakama_update::spawn_check(&config.updates, env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Diagnose { symptom } => diagnose::run(&config, &ui, &symptom).await,
        Commands::Analyze { logfile } => analyze::run(&config, &ui, &logfile).await,
        Commands::Health => health::run(&config, &ui).await,
        Commands::Explain { resource } => explain::run(&config, &ui, &resource).await,
        Commands::Scan => {
            println!("[scan] Coming soon: infrastructure issue scanner");
            Ok(())
        }
        Commands::Trace { service } => {
            println!("[trace] Coming soon: request tracing for {}", service);
            Ok(())
        }
        Commands::Watch => {
            println!("[watch] Coming soon: real-time infrastructure monitoring");
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
