use crate::git_info::GitInfo;
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_ui::NakamaUI;

/// Show current status across git and local services.
pub async fn run(config: &Config, ui: &NakamaUI) -> NakamaResult<()> {
    let _ = config;
    let spinner = ui.step_start("Checking status...");

    let git = GitInfo::collect().ok();

    // Git status
    let git_rows: Vec<Vec<String>> = if let Some(g) = &git {
        vec![
            vec!["Branch".to_string(), g.branch.clone()],
            vec!["Uncommitted".to_string(), g.uncommitted_changes.to_string()],
            vec!["Ahead/Behind".to_string(), g.ahead_behind.clone()],
            vec!["Recent commits".to_string(), g.recent_commits.len().to_string()],
        ]
    } else {
        vec![vec!["Git".to_string(), "Not in a repository".to_string()]]
    };

    // Check if common dev services are running
    let services = check_services();

    spinner.finish_with_success("Status collected");

    ui.table(&["Property", "Value"], git_rows);
    ui.table(&["Service", "Status"], services);

    Ok(())
}

fn check_services() -> Vec<Vec<String>> {
    let checks: Vec<(&str, &str, Vec<&str>)> = vec![
        ("Docker", "docker", vec!["info", "--format", "{{.ServerVersion}}"]),
        ("Node.js", "node", vec!["--version"]),
        ("Python", "python3", vec!["--version"]),
        ("PostgreSQL", "pg_isready", vec!["-q"]),
        ("Redis", "redis-cli", vec!["ping"]),
    ];

    checks
        .into_iter()
        .map(|(name, cmd, args)| {
            let status = std::process::Command::new(cmd)
                .args(&args)
                .output()
                .map(|o| {
                    if o.status.success() {
                        let ver = String::from_utf8_lossy(&o.stdout).trim().to_string();
                        if ver.is_empty() { "Running".to_string() } else { ver }
                    } else {
                        "Not running".to_string()
                    }
                })
                .unwrap_or_else(|_| "Not installed".to_string());
            vec![name.to_string(), status]
        })
        .collect()
}
