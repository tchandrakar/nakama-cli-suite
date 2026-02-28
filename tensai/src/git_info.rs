use nakama_core::error::NakamaResult;
use std::process::Command;

/// Collect recent git activity for briefings.
pub struct GitInfo {
    pub branch: String,
    pub recent_commits: Vec<String>,
    pub uncommitted_changes: usize,
    pub ahead_behind: String,
}

impl GitInfo {
    pub fn collect() -> NakamaResult<Self> {
        let branch = run_cmd("git", &["branch", "--show-current"]);
        let log_output = run_cmd("git", &["log", "--oneline", "-10", "--format=%h %s (%ar)"]);
        let recent_commits: Vec<String> = log_output
            .lines()
            .map(|l| l.to_string())
            .collect();

        let status = run_cmd("git", &["status", "--porcelain"]);
        let uncommitted_changes = status.lines().count();

        let ahead_behind = run_cmd("git", &["status", "--branch", "--porcelain=v2"]);
        let ab = ahead_behind
            .lines()
            .find(|l| l.starts_with("# branch.ab"))
            .map(|l| l.replace("# branch.ab ", ""))
            .unwrap_or_else(|| "unknown".to_string());

        Ok(Self {
            branch,
            recent_commits,
            uncommitted_changes,
            ahead_behind: ab,
        })
    }

    pub fn to_summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str(&format!("Branch: {}\n", self.branch));
        summary.push_str(&format!("Uncommitted changes: {}\n", self.uncommitted_changes));
        summary.push_str(&format!("Ahead/Behind: {}\n", self.ahead_behind));
        summary.push_str("\nRecent commits:\n");
        for commit in &self.recent_commits {
            summary.push_str(&format!("  - {}\n", commit));
        }
        summary
    }
}

fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

/// Collect TODO/FIXME comments from the codebase.
pub fn find_todos() -> Vec<String> {
    let output = Command::new("grep")
        .args(&["-rn", "--include=*.rs", "--include=*.py", "--include=*.js",
                "--include=*.ts", "--include=*.go", "--include=*.java",
                "-E", "TODO|FIXME|HACK|XXX", "."])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    output
        .lines()
        .take(50)
        .map(|l| l.to_string())
        .collect()
}
