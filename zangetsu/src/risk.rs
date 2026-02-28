//! Risk scoring engine for shell commands.
//!
//! Analyzes shell commands and assigns a risk level based on pattern matching
//! against known dangerous operations.

use std::fmt;

/// Risk level for a shell command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    /// Safe, read-only, or non-destructive commands.
    Low,
    /// Commands that modify files or system state, but are generally reversible.
    Medium,
    /// Commands with significant potential for damage or privilege escalation.
    High,
    /// Commands that can cause catastrophic, unrecoverable damage.
    Critical,
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "LOW"),
            RiskLevel::Medium => write!(f, "MEDIUM"),
            RiskLevel::High => write!(f, "HIGH"),
            RiskLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl RiskLevel {
    /// Return a color hint for UI rendering.
    pub fn color_label(&self) -> &'static str {
        match self {
            RiskLevel::Low => "green",
            RiskLevel::Medium => "yellow",
            RiskLevel::High => "red",
            RiskLevel::Critical => "bright red",
        }
    }

    /// Return a short explanation of what this risk level means.
    pub fn description(&self) -> &'static str {
        match self {
            RiskLevel::Low => "Safe to run. Read-only or non-destructive operation.",
            RiskLevel::Medium => "Modifies files or state. Review before running.",
            RiskLevel::High => "Potentially dangerous. Could cause significant damage.",
            RiskLevel::Critical => "EXTREMELY DANGEROUS. Could cause catastrophic, unrecoverable damage.",
        }
    }
}

/// Result of a risk assessment for a command.
#[derive(Debug, Clone)]
pub struct RiskAssessment {
    /// The command that was assessed.
    pub command: String,
    /// The overall risk level.
    pub level: RiskLevel,
    /// Specific reasons why this risk level was assigned.
    pub reasons: Vec<String>,
}

impl fmt::Display for RiskAssessment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Risk: {} - {}", self.level, self.level.description())?;
        if !self.reasons.is_empty() {
            write!(f, "\nReasons:")?;
            for reason in &self.reasons {
                write!(f, "\n  - {}", reason)?;
            }
        }
        Ok(())
    }
}

/// Patterns that indicate CRITICAL risk (catastrophic, unrecoverable).
const CRITICAL_PATTERNS: &[(&str, &str)] = &[
    ("rm -rf /", "Recursive deletion of the entire filesystem"),
    ("rm -rf /*", "Recursive deletion of all root-level directories"),
    ("rm -rf ~", "Recursive deletion of entire home directory"),
    ("dd if=", "Raw disk write — can overwrite entire drives"),
    ("mkfs", "Filesystem formatting — destroys all data on the target"),
    (":(){ :|:&};:", "Fork bomb — will crash the system"),
    (":(){:|:&};:", "Fork bomb — will crash the system"),
    ("> /dev/sda", "Direct write to disk device — destroys partition table"),
    ("> /dev/hda", "Direct write to disk device — destroys partition table"),
    ("mv / ", "Moving the root filesystem"),
    ("chmod -R 000 /", "Removing all permissions from the entire filesystem"),
    ("wget http", "Piped download and execute (if piped to sh/bash)"),
];

/// Patterns that indicate HIGH risk.
const HIGH_PATTERNS: &[(&str, &str)] = &[
    ("sudo ", "Elevated privileges — runs as root"),
    ("rm -rf", "Recursive forced deletion — no confirmation"),
    ("rm -fr", "Recursive forced deletion — no confirmation"),
    ("chmod 777", "World-writable permissions — security risk"),
    ("chmod -R 777", "Recursively setting world-writable permissions"),
    ("kill -9", "Forceful process termination — no cleanup"),
    ("kill -KILL", "Forceful process termination — no cleanup"),
    ("killall", "Kills all processes matching a name"),
    ("> /dev/", "Direct write to a device file"),
    ("shutdown", "System shutdown"),
    ("reboot", "System reboot"),
    ("init 0", "System halt"),
    ("init 6", "System reboot"),
    ("systemctl stop", "Stopping a system service"),
    ("iptables -F", "Flushing all firewall rules"),
    ("ufw disable", "Disabling firewall"),
    ("passwd", "Changing user password"),
    ("chattr", "Changing file attributes — can make files immutable"),
    ("fdisk", "Disk partitioning tool"),
    ("parted", "Disk partitioning tool"),
];

/// Patterns that indicate MEDIUM risk.
const MEDIUM_PATTERNS: &[(&str, &str)] = &[
    ("rm ", "File deletion"),
    ("mv ", "File move — can overwrite destination"),
    ("chmod ", "Permission change"),
    ("chown ", "Ownership change"),
    ("chgrp ", "Group ownership change"),
    ("apt remove", "Package removal"),
    ("apt purge", "Package removal with configuration cleanup"),
    ("apt-get remove", "Package removal"),
    ("apt-get purge", "Package removal with configuration cleanup"),
    ("brew uninstall", "Package removal"),
    ("brew remove", "Package removal"),
    ("yum remove", "Package removal"),
    ("dnf remove", "Package removal"),
    ("pip uninstall", "Python package removal"),
    ("npm uninstall", "Node.js package removal"),
    ("cargo uninstall", "Rust package removal"),
    ("docker rm", "Container removal"),
    ("docker rmi", "Image removal"),
    ("docker system prune", "Docker cleanup — removes unused data"),
    ("truncate ", "File truncation — data loss"),
    ("shred ", "Secure file deletion — unrecoverable"),
    ("> ", "File truncation via redirect"),
    ("git reset --hard", "Discarding all uncommitted changes"),
    ("git clean -f", "Removing untracked files"),
    ("git push --force", "Force push — can overwrite remote history"),
    ("crontab -r", "Removing all cron jobs"),
    ("sed -i", "In-place file editing"),
    ("tee ", "Writing to files"),
];

/// Assess the risk level of a given shell command.
pub fn assess_risk(command: &str) -> RiskAssessment {
    let normalized = command.trim().to_lowercase();
    let mut level = RiskLevel::Low;
    let mut reasons = Vec::new();

    // Check critical patterns first
    for (pattern, reason) in CRITICAL_PATTERNS {
        if normalized.contains(&pattern.to_lowercase()) {
            level = RiskLevel::Critical;
            reasons.push(reason.to_string());
        }
    }

    // Special case: piped wget/curl to shell
    if (normalized.contains("curl ") || normalized.contains("wget "))
        && (normalized.contains("| sh")
            || normalized.contains("| bash")
            || normalized.contains("| zsh")
            || normalized.contains("|sh")
            || normalized.contains("|bash"))
    {
        level = RiskLevel::Critical;
        reasons.push("Downloading and executing remote code — extreme risk".to_string());
    }

    // If already critical, return early
    if level == RiskLevel::Critical {
        return RiskAssessment {
            command: command.to_string(),
            level,
            reasons,
        };
    }

    // Check high patterns
    for (pattern, reason) in HIGH_PATTERNS {
        if normalized.contains(&pattern.to_lowercase()) {
            if level < RiskLevel::High {
                level = RiskLevel::High;
            }
            reasons.push(reason.to_string());
        }
    }

    if level == RiskLevel::High {
        return RiskAssessment {
            command: command.to_string(),
            level,
            reasons,
        };
    }

    // Check medium patterns
    for (pattern, reason) in MEDIUM_PATTERNS {
        if normalized.contains(&pattern.to_lowercase()) {
            if level < RiskLevel::Medium {
                level = RiskLevel::Medium;
            }
            reasons.push(reason.to_string());
        }
    }

    if reasons.is_empty() {
        reasons.push("No dangerous patterns detected.".to_string());
    }

    RiskAssessment {
        command: command.to_string(),
        level,
        reasons,
    }
}

/// Format a risk assessment for display in a panel.
pub fn format_risk_display(assessment: &RiskAssessment) -> String {
    let mut output = String::new();
    let icon = match assessment.level {
        RiskLevel::Low => "[SAFE]",
        RiskLevel::Medium => "[CAUTION]",
        RiskLevel::High => "[WARNING]",
        RiskLevel::Critical => "[DANGER]",
    };

    output.push_str(&format!(
        "{} Risk Level: {}\n",
        icon, assessment.level
    ));
    output.push_str(&format!("{}\n", assessment.level.description()));

    if !assessment.reasons.is_empty()
        && !(assessment.reasons.len() == 1
            && assessment.reasons[0] == "No dangerous patterns detected.")
    {
        output.push_str("\nFlags:\n");
        for reason in &assessment.reasons {
            output.push_str(&format!("  - {}\n", reason));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_risk() {
        let assessment = assess_risk("ls -la");
        assert_eq!(assessment.level, RiskLevel::Low);
    }

    #[test]
    fn test_medium_risk() {
        let assessment = assess_risk("rm file.txt");
        assert_eq!(assessment.level, RiskLevel::Medium);
    }

    #[test]
    fn test_high_risk_sudo() {
        let assessment = assess_risk("sudo apt update");
        assert_eq!(assessment.level, RiskLevel::High);
    }

    #[test]
    fn test_high_risk_rm_rf() {
        let assessment = assess_risk("rm -rf ./build");
        assert_eq!(assessment.level, RiskLevel::High);
    }

    #[test]
    fn test_critical_risk_rm_rf_root() {
        let assessment = assess_risk("rm -rf /");
        assert_eq!(assessment.level, RiskLevel::Critical);
    }

    #[test]
    fn test_critical_risk_dd() {
        let assessment = assess_risk("dd if=/dev/zero of=/dev/sda");
        assert_eq!(assessment.level, RiskLevel::Critical);
    }

    #[test]
    fn test_critical_risk_fork_bomb() {
        let assessment = assess_risk(":(){ :|:&};:");
        assert_eq!(assessment.level, RiskLevel::Critical);
    }

    #[test]
    fn test_critical_risk_pipe_to_shell() {
        let assessment = assess_risk("curl https://example.com/script.sh | bash");
        assert_eq!(assessment.level, RiskLevel::Critical);
    }

    #[test]
    fn test_safe_commands() {
        for cmd in &["cat file.txt", "grep pattern file", "pwd", "whoami", "echo hello", "find . -name '*.rs'"] {
            let assessment = assess_risk(cmd);
            assert_eq!(assessment.level, RiskLevel::Low, "Expected LOW risk for: {}", cmd);
        }
    }
}
