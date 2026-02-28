use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;

const TOOL_NAME: &str = "shinigami";

/// Shinigami - AI-powered git workflow automator
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate an AI-crafted commit message from staged changes
    Commit,

    /// Summarize commits between two refs (reap the changelog)
    Reap {
        /// Starting ref (defaults to last tag)
        #[arg(long)]
        from: Option<String>,

        /// Ending ref (defaults to HEAD)
        #[arg(long)]
        to: Option<String>,
    },

    /// Create a well-named branch from a natural-language description
    Branch {
        /// Description of what the branch is for
        #[arg()]
        description: String,
    },

    /// Interactively squash commits with AI-generated squash message
    Squash,

    /// Prepare release notes for a given version
    Release {
        /// The version to release (e.g., "1.2.0")
        #[arg()]
        version: String,
    },

    /// AI-powered review of uncommitted changes
    Review,

    /// Manage git hooks (install, remove, list)
    Hook {
        /// Action to perform: install, remove, or list
        #[arg()]
        action: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;

    println!("{} v{}", TOOL_NAME, env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    match cli.command {
        Commands::Commit => {
            println!("[commit] Coming soon: AI-generated commit message from staged changes");
        }
        Commands::Reap { from, to } => {
            println!("[reap] Coming soon: changelog between refs");
            if let Some(f) = from {
                println!("  From: {}", f);
            }
            if let Some(t) = to {
                println!("  To: {}", t);
            }
        }
        Commands::Branch { description } => {
            println!("[branch] Coming soon: AI-named branch creation");
            println!("  Description: {}", description);
        }
        Commands::Squash => {
            println!("[squash] Coming soon: interactive squash with AI message");
        }
        Commands::Release { version } => {
            println!("[release] Coming soon: release notes generation");
            println!("  Version: {}", version);
        }
        Commands::Review => {
            println!("[review] Coming soon: AI review of uncommitted changes");
        }
        Commands::Hook { action } => {
            println!("[hook] Coming soon: git hook management");
            println!("  Action: {}", action);
        }
    }

    Ok(())
}
