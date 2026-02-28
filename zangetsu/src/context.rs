//! Shell context collection for AI prompt enrichment.
//!
//! Gathers information about the current shell environment so that the AI
//! provider can produce commands tailored to the user's system.

use std::env;
use std::fmt;
use std::path::PathBuf;

/// Snapshot of the user's shell environment at the time of invocation.
#[derive(Debug, Clone)]
pub struct ShellContext {
    /// Current working directory.
    pub cwd: PathBuf,
    /// Operating system name (e.g. "macos", "linux", "windows").
    pub os: String,
    /// Detected shell (e.g. "zsh", "bash", "fish").
    pub shell: String,
    /// Username of the current user.
    pub user: String,
    /// Recent shell history lines (last 10).
    pub recent_history: Vec<String>,
}

impl ShellContext {
    /// Collect the current shell context from the environment.
    pub fn collect() -> Self {
        Self {
            cwd: env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            os: detect_os(),
            shell: detect_shell(),
            user: env::var("USER")
                .or_else(|_| env::var("USERNAME"))
                .unwrap_or_else(|_| "unknown".to_string()),
            recent_history: read_recent_history(10),
        }
    }
}

impl fmt::Display for ShellContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "OS: {}", self.os)?;
        writeln!(f, "Shell: {}", self.shell)?;
        writeln!(f, "CWD: {}", self.cwd.display())?;
        writeln!(f, "User: {}", self.user)?;
        if !self.recent_history.is_empty() {
            writeln!(f, "Recent commands:")?;
            for (i, cmd) in self.recent_history.iter().enumerate() {
                writeln!(f, "  {}. {}", i + 1, cmd)?;
            }
        }
        Ok(())
    }
}

/// Detect the operating system and return a human-friendly name.
fn detect_os() -> String {
    if cfg!(target_os = "macos") {
        "macOS".to_string()
    } else if cfg!(target_os = "linux") {
        // Try to read the distro name from /etc/os-release
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if let Some(name) = line.strip_prefix("PRETTY_NAME=") {
                    return format!("Linux ({})", name.trim_matches('"'));
                }
            }
        }
        "Linux".to_string()
    } else if cfg!(target_os = "windows") {
        "Windows".to_string()
    } else {
        env::consts::OS.to_string()
    }
}

/// Detect the current shell from the SHELL environment variable.
fn detect_shell() -> String {
    if let Ok(shell_path) = env::var("SHELL") {
        if let Some(name) = shell_path.rsplit('/').next() {
            return name.to_string();
        }
        return shell_path;
    }
    // Fallback for Windows
    if let Ok(comspec) = env::var("COMSPEC") {
        if comspec.to_lowercase().contains("powershell") {
            return "powershell".to_string();
        }
        return "cmd".to_string();
    }
    "unknown".to_string()
}

/// Read the last `n` lines from the user's shell history file.
fn read_recent_history(n: usize) -> Vec<String> {
    let history_path = find_history_file();
    let path = match history_path {
        Some(p) => p,
        None => return Vec::new(),
    };

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let shell = detect_shell();
    let lines: Vec<String> = content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return None;
            }
            // zsh history lines can start with `: <timestamp>:0;` prefix
            if shell == "zsh" {
                if let Some(rest) = trimmed.strip_prefix(": ") {
                    // Format: ": 1234567890:0;actual command"
                    if let Some(idx) = rest.find(';') {
                        return Some(rest[idx + 1..].to_string());
                    }
                }
            }
            Some(trimmed.to_string())
        })
        .collect();

    // Take the last n non-empty lines
    lines.into_iter().rev().take(n).rev().collect()
}

/// Attempt to locate the user's shell history file.
fn find_history_file() -> Option<PathBuf> {
    // Check HISTFILE env var first
    if let Ok(histfile) = env::var("HISTFILE") {
        let path = PathBuf::from(&histfile);
        if path.exists() {
            return Some(path);
        }
    }

    let home = dirs::home_dir()?;
    let shell = detect_shell();

    let candidates = match shell.as_str() {
        "zsh" => vec![
            home.join(".zsh_history"),
            home.join(".zhistory"),
        ],
        "bash" => vec![
            home.join(".bash_history"),
        ],
        "fish" => vec![
            home.join(".local/share/fish/fish_history"),
        ],
        _ => vec![
            home.join(".bash_history"),
            home.join(".zsh_history"),
        ],
    };

    candidates.into_iter().find(|p: &PathBuf| p.exists())
}

/// Build a system prompt section describing the shell context.
pub fn build_context_prompt(ctx: &ShellContext) -> String {
    let mut prompt = String::new();
    prompt.push_str("## Current Environment\n");
    prompt.push_str(&format!("- Operating System: {}\n", ctx.os));
    prompt.push_str(&format!("- Shell: {}\n", ctx.shell));
    prompt.push_str(&format!("- Working Directory: {}\n", ctx.cwd.display()));
    prompt.push_str(&format!("- User: {}\n", ctx.user));

    if !ctx.recent_history.is_empty() {
        prompt.push_str("\n## Recent Shell History\n");
        for cmd in &ctx.recent_history {
            prompt.push_str(&format!("- `{}`\n", cmd));
        }
    }

    prompt
}
