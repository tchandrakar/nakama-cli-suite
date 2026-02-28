use anyhow::Result;
use clap::{Parser, Subcommand};
use nakama_core::Config;
use nakama_log::init_logging;

const TOOL_NAME: &str = "itachi";

/// Itachi - Jira & Confluence intelligence hub
#[derive(Parser, Debug)]
#[command(name = TOOL_NAME, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search and query Jira issues with natural language
    Jira {
        /// Natural-language query for Jira issues
        #[arg()]
        query: String,
    },

    /// Search and query Confluence wiki pages
    Wiki {
        /// Natural-language query for Confluence pages
        #[arg()]
        query: String,
    },

    /// Ask a question across both Jira and Confluence
    Ask {
        /// The question to answer from Jira and Confluence data
        #[arg()]
        question: String,
    },

    /// Generate a team briefing from recent Jira and Confluence activity
    Brief {
        /// Filter by team name (optional)
        #[arg()]
        team: Option<String>,
    },

    /// Generate an onboarding guide for a project from Jira and Confluence
    Onboard {
        /// The project key or name to generate onboarding for
        #[arg()]
        project: String,
    },

    /// Generate a standup summary from your Jira activity
    Standup,

    /// Create a new Jira issue
    Create {
        /// Issue type (e.g., "bug", "story", "task", "epic")
        #[arg()]
        issue_type: String,

        /// Issue summary / title
        #[arg()]
        summary: String,
    },

    /// Link a Jira issue to a Confluence document
    Link {
        /// The Jira issue key (e.g., "PROJ-123")
        #[arg()]
        issue: String,

        /// The Confluence document URL or page ID
        #[arg()]
        doc: String,
    },

    /// Show sprint information and progress
    Sprint {
        /// The board name or ID (optional, defaults to your default board)
        #[arg()]
        board: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(TOOL_NAME).unwrap_or_default();
    let _log_guard = init_logging(TOOL_NAME, &config.logging)?;

    println!("{} v{}", TOOL_NAME, env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    match cli.command {
        Commands::Jira { query } => {
            println!("[jira] Coming soon: natural-language Jira search");
            println!("  Query: {}", query);
        }
        Commands::Wiki { query } => {
            println!("[wiki] Coming soon: natural-language Confluence search");
            println!("  Query: {}", query);
        }
        Commands::Ask { question } => {
            println!("[ask] Coming soon: cross-platform Q&A");
            println!("  Question: {}", question);
        }
        Commands::Brief { team } => {
            println!("[brief] Coming soon: team briefing");
            if let Some(t) = team {
                println!("  Team: {}", t);
            }
        }
        Commands::Onboard { project } => {
            println!("[onboard] Coming soon: project onboarding guide");
            println!("  Project: {}", project);
        }
        Commands::Standup => {
            println!("[standup] Coming soon: Jira-based standup summary");
        }
        Commands::Create { issue_type, summary } => {
            println!("[create] Coming soon: Jira issue creation");
            println!("  Type: {}", issue_type);
            println!("  Summary: {}", summary);
        }
        Commands::Link { issue, doc } => {
            println!("[link] Coming soon: Jira-Confluence linking");
            println!("  Issue: {}", issue);
            println!("  Doc: {}", doc);
        }
        Commands::Sprint { board } => {
            println!("[sprint] Coming soon: sprint dashboard");
            if let Some(b) = board {
                println!("  Board: {}", b);
            }
        }
    }

    Ok(())
}
