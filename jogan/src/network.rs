//! Network connectivity and DNS checks.

use std::process::Command;

/// Run a command and return its stdout as a trimmed string.
fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

/// Run a command and return (stdout, stderr, success).
fn run_cmd_full(cmd: &str, args: &[&str]) -> (String, String, bool) {
    match Command::new(cmd).args(args).output() {
        Ok(o) => (
            String::from_utf8_lossy(&o.stdout).trim().to_string(),
            String::from_utf8_lossy(&o.stderr).trim().to_string(),
            o.status.success(),
        ),
        Err(_) => (String::new(), String::new(), false),
    }
}

/// A single network check result.
pub struct NetworkCheck {
    pub name: String,
    pub status: CheckStatus,
    pub detail: String,
}

#[derive(Clone, Copy)]
pub enum CheckStatus {
    Ok,
    Warning,
    Error,
}

impl CheckStatus {
    pub fn label(&self) -> &'static str {
        match self {
            CheckStatus::Ok => "OK",
            CheckStatus::Warning => "WARN",
            CheckStatus::Error => "FAIL",
        }
    }
}

/// Run all network checks and return results.
pub fn run_network_checks() -> Vec<NetworkCheck> {
    let mut checks = Vec::new();

    // 1. Internet connectivity (ping)
    checks.push(check_ping("8.8.8.8", "Internet (Google DNS)"));
    checks.push(check_ping("1.1.1.1", "Internet (Cloudflare DNS)"));

    // 2. DNS resolution
    checks.push(check_dns("google.com"));
    checks.push(check_dns("github.com"));

    // 3. Default gateway
    checks.push(check_default_gateway());

    // 4. Active interfaces
    checks.push(check_network_interfaces());

    // 5. DNS server configuration
    checks.push(check_dns_config());

    checks
}

/// Ping a host with a short timeout.
fn check_ping(host: &str, label: &str) -> NetworkCheck {
    // macOS: -c count, -W timeout (ms); Linux: -c count, -W timeout (s)
    let (stdout, _stderr, success) = run_cmd_full("ping", &["-c", "1", "-W", "3", host]);

    if success {
        // Extract round-trip time from output
        let rtt = stdout
            .lines()
            .find(|l| l.contains("time=") || l.contains("time<"))
            .and_then(|l| {
                l.split("time=")
                    .nth(1)
                    .or_else(|| l.split("time<").nth(1))
                    .map(|t| t.split_whitespace().next().unwrap_or("?").to_string())
            })
            .unwrap_or_else(|| "?".to_string());

        NetworkCheck {
            name: label.to_string(),
            status: CheckStatus::Ok,
            detail: format!("Reachable (rtt: {} ms)", rtt),
        }
    } else {
        NetworkCheck {
            name: label.to_string(),
            status: CheckStatus::Error,
            detail: "Unreachable".to_string(),
        }
    }
}

/// Check DNS resolution for a hostname.
fn check_dns(hostname: &str) -> NetworkCheck {
    // Try dig first, fall back to nslookup
    let (stdout, _stderr, success) = run_cmd_full("dig", &["+short", hostname]);

    if success && !stdout.is_empty() {
        let first_result = stdout.lines().next().unwrap_or("?");
        NetworkCheck {
            name: format!("DNS: {}", hostname),
            status: CheckStatus::Ok,
            detail: format!("Resolved to {}", first_result),
        }
    } else {
        // Fallback to nslookup
        let (stdout2, _stderr2, success2) = run_cmd_full("nslookup", &[hostname]);
        if success2 && stdout2.contains("Address") {
            let addr = stdout2
                .lines()
                .rev()
                .find(|l| l.contains("Address") && !l.contains("#"))
                .unwrap_or("?");
            NetworkCheck {
                name: format!("DNS: {}", hostname),
                status: CheckStatus::Ok,
                detail: format!("Resolved ({})", addr.trim()),
            }
        } else {
            NetworkCheck {
                name: format!("DNS: {}", hostname),
                status: CheckStatus::Error,
                detail: "DNS resolution failed".to_string(),
            }
        }
    }
}

/// Check the default gateway.
fn check_default_gateway() -> NetworkCheck {
    // macOS
    let gateway = run_cmd("route", &["-n", "get", "default"]);
    if !gateway.is_empty() {
        let gw_addr = gateway
            .lines()
            .find(|l| l.contains("gateway"))
            .map(|l| l.split(':').nth(1).unwrap_or("?").trim().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        return NetworkCheck {
            name: "Default Gateway".to_string(),
            status: CheckStatus::Ok,
            detail: gw_addr,
        };
    }

    // Linux fallback
    let route = run_cmd("ip", &["route", "show", "default"]);
    if !route.is_empty() {
        let gw_addr = route
            .split_whitespace()
            .nth(2)
            .unwrap_or("unknown")
            .to_string();
        NetworkCheck {
            name: "Default Gateway".to_string(),
            status: CheckStatus::Ok,
            detail: gw_addr,
        }
    } else {
        NetworkCheck {
            name: "Default Gateway".to_string(),
            status: CheckStatus::Error,
            detail: "No default gateway found".to_string(),
        }
    }
}

/// Check active network interfaces.
fn check_network_interfaces() -> NetworkCheck {
    // macOS / Linux: ifconfig or ip addr
    let ifconfig = run_cmd("ifconfig", &[]);
    if !ifconfig.is_empty() {
        let active_count = ifconfig
            .split("\n\n")
            .filter(|block| block.contains("status: active") || block.contains("RUNNING"))
            .count();
        return NetworkCheck {
            name: "Network Interfaces".to_string(),
            status: if active_count > 0 {
                CheckStatus::Ok
            } else {
                CheckStatus::Warning
            },
            detail: format!("{} active interface(s)", active_count),
        };
    }

    let ip_output = run_cmd("ip", &["link", "show"]);
    let up_count = ip_output
        .lines()
        .filter(|l| l.contains("state UP"))
        .count();
    NetworkCheck {
        name: "Network Interfaces".to_string(),
        status: if up_count > 0 {
            CheckStatus::Ok
        } else {
            CheckStatus::Warning
        },
        detail: format!("{} active interface(s)", up_count),
    }
}

/// Check DNS resolver configuration.
fn check_dns_config() -> NetworkCheck {
    // macOS: scutil --dns; Linux: /etc/resolv.conf
    let scutil = run_cmd("scutil", &["--dns"]);
    if !scutil.is_empty() {
        let nameservers: Vec<&str> = scutil
            .lines()
            .filter(|l| l.contains("nameserver["))
            .take(3)
            .collect();
        let ns_list: Vec<String> = nameservers
            .iter()
            .map(|l| l.split(':').nth(1).unwrap_or("?").trim().to_string())
            .collect();
        return NetworkCheck {
            name: "DNS Servers".to_string(),
            status: if ns_list.is_empty() {
                CheckStatus::Warning
            } else {
                CheckStatus::Ok
            },
            detail: if ns_list.is_empty() {
                "No DNS servers configured".to_string()
            } else {
                ns_list.join(", ")
            },
        };
    }

    // Linux fallback
    let resolv = run_cmd("cat", &["/etc/resolv.conf"]);
    let ns_lines: Vec<String> = resolv
        .lines()
        .filter(|l| l.starts_with("nameserver"))
        .map(|l| l.split_whitespace().nth(1).unwrap_or("?").to_string())
        .collect();
    NetworkCheck {
        name: "DNS Servers".to_string(),
        status: if ns_lines.is_empty() {
            CheckStatus::Warning
        } else {
            CheckStatus::Ok
        },
        detail: if ns_lines.is_empty() {
            "No DNS servers configured".to_string()
        } else {
            ns_lines.join(", ")
        },
    }
}

/// Format all network check results as a report string suitable for AI analysis.
pub fn format_network_report(checks: &[NetworkCheck]) -> String {
    let mut report = String::from("Network Diagnostics Report\n");
    report.push_str(&"=".repeat(40));
    report.push('\n');
    for check in checks {
        report.push_str(&format!(
            "[{}] {} -- {}\n",
            check.status.label(),
            check.name,
            check.detail,
        ));
    }
    report
}
