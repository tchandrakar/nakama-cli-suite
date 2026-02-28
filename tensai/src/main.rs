use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;

const TOOL_NAME: &str = "tensai";

/// Tensai - Your personal dev briefing dashboard
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate your morning dev briefing (PRs, issues, deployments, etc.)
    Brief,

    /// Generate a standup summary from your recent activity
    Standup,

    /// Plan your day based on priorities and deadlines
    Plan,

    /// Show current status across all connected services
    Status,

    /// Review your day's accomplishments and pending items
    Review,

    /// Enter focus mode -- minimize distractions and track deep work
    Focus,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;

    println!("{} v{}", TOOL_NAME, env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    match cli.command {
        Commands::Brief => {
            println!("[brief] Coming soon: morning dev briefing");
        }
        Commands::Standup => {
            println!("[standup] Coming soon: standup summary generation");
        }
        Commands::Plan => {
            println!("[plan] Coming soon: AI-powered day planning");
        }
        Commands::Status => {
            println!("[status] Coming soon: cross-service status dashboard");
        }
        Commands::Review => {
            println!("[review] Coming soon: end-of-day review");
        }
        Commands::Focus => {
            println!("[focus] Coming soon: deep work focus mode");
        }
    }

    Ok(())
}
