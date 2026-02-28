mod ai_helper;
mod brief;
mod focus;
mod git_info;
mod plan;
mod review_day;
mod standup;
mod status;

use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;
use nakama_ui::NakamaUI;

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
    /// Generate your morning dev briefing
    Brief,

    /// Generate a standup summary from recent activity
    Standup,

    /// Plan your day based on priorities
    Plan,

    /// Show current status across git and services
    Status,

    /// Review your day's accomplishments
    Review,

    /// Enter focus mode with a Pomodoro timer
    Focus,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;
    let ui = NakamaUI::from_config(&config);

    let update_rx = nakama_update::spawn_check(&config.updates, env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Brief => brief::run(&config, &ui).await,
        Commands::Standup => standup::run(&config, &ui).await,
        Commands::Plan => plan::run(&config, &ui).await,
        Commands::Status => status::run(&config, &ui).await,
        Commands::Review => review_day::run(&config, &ui).await,
        Commands::Focus => focus::run(&config, &ui).await,
    };

    if let Err(e) = result {
        ui.error(&format!("{}", e));
        nakama_update::maybe_show_update(&ui, update_rx);
        std::process::exit(1);
    }

    nakama_update::maybe_show_update(&ui, update_rx);

    Ok(())
}
