use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;

const TOOL_NAME: &str = "byakugan";

/// Byakugan - Platform-agnostic AI PR reviewer
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    /// Target platform: github, gitlab, or bitbucket
    #[arg(long, default_value = "github")]
    platform: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Review the current branch's PR with AI analysis
    Review,

    /// Scan a specific PR for issues, security risks, and improvements
    Scan {
        /// Override the platform for this scan
        #[arg(long)]
        platform: Option<String>,

        /// The PR identifier (number or URL)
        #[arg()]
        pr: String,
    },

    /// Generate a comprehensive review report
    Report,

    /// Watch for new PRs and auto-review them
    Watch,

    /// Manage review rules and policies
    Rules,

    /// Add a comment to a PR
    Comment {
        /// The PR identifier (number or URL)
        #[arg()]
        pr: String,

        /// The comment message
        #[arg()]
        message: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;

    println!("{} v{}", TOOL_NAME, env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();
    let platform = &cli.platform;

    match cli.command {
        Commands::Review => {
            println!("[review] Coming soon: AI-powered PR review");
            println!("  Platform: {}", platform);
        }
        Commands::Scan { platform: scan_platform, pr } => {
            let effective_platform = scan_platform.as_deref().unwrap_or(platform);
            println!("[scan] Coming soon: PR issue scanner");
            println!("  Platform: {}", effective_platform);
            println!("  PR: {}", pr);
        }
        Commands::Report => {
            println!("[report] Coming soon: comprehensive review report");
            println!("  Platform: {}", platform);
        }
        Commands::Watch => {
            println!("[watch] Coming soon: PR auto-review watcher");
            println!("  Platform: {}", platform);
        }
        Commands::Rules => {
            println!("[rules] Coming soon: review rules management");
        }
        Commands::Comment { pr, message } => {
            println!("[comment] Coming soon: PR commenting");
            println!("  Platform: {}", platform);
            println!("  PR: {}", pr);
            println!("  Message: {}", message);
        }
    }

    Ok(())
}
