//! System information collection via shell commands.

use std::process::Command;

/// Run a command and return its stdout as a trimmed string.
fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

/// Collected system information snapshot.
pub struct SystemInfo {
    pub hostname: String,
    pub os_type: String,
    pub kernel_version: String,
    pub uptime: String,
    pub cpu_count: String,
    pub cpu_usage: String,
    pub memory_total: String,
    pub memory_used: String,
    pub memory_pressure: String,
    pub disk_usage: String,
    pub load_average: String,
    pub top_processes: String,
}

impl SystemInfo {
    /// Gather a snapshot of current system information.
    pub fn collect() -> Self {
        let hostname = run_cmd("hostname", &[]);
        let os_type = run_cmd("uname", &["-s"]);
        let kernel_version = run_cmd("uname", &["-r"]);
        let uptime = run_cmd("uptime", &[]);

        // CPU info — works on macOS (sysctl) and Linux (nproc)
        let cpu_count = {
            let val = run_cmd("sysctl", &["-n", "hw.ncpu"]);
            if val.is_empty() {
                run_cmd("nproc", &[])
            } else {
                val
            }
        };

        // CPU usage from top (macOS: -l 1, Linux: -bn1)
        let cpu_usage = {
            let output = run_cmd("top", &["-l", "1", "-n", "0", "-s", "0"]);
            if output.is_empty() {
                // Linux fallback
                let linux_top = run_cmd("top", &["-bn1"]);
                linux_top
                    .lines()
                    .find(|l| l.contains("Cpu"))
                    .unwrap_or("")
                    .to_string()
            } else {
                output
                    .lines()
                    .find(|l| l.contains("CPU usage"))
                    .unwrap_or("")
                    .to_string()
            }
        };

        // Memory info — macOS uses vm_stat, Linux uses /proc/meminfo
        let (memory_total, memory_used, memory_pressure) = collect_memory_info();

        // Disk usage
        let disk_usage = run_cmd("df", &["-h", "/"]);

        // Load average
        let load_average = {
            let val = run_cmd("sysctl", &["-n", "vm.loadavg"]);
            if val.is_empty() {
                // Linux fallback: read /proc/loadavg
                run_cmd("cat", &["/proc/loadavg"])
            } else {
                val
            }
        };

        // Top 5 processes by CPU
        let top_processes = run_cmd("ps", &["aux", "--sort=-%cpu"]);
        let top_processes = if top_processes.is_empty() {
            // macOS ps doesn't support --sort; use a different approach
            let raw = run_cmd("ps", &["aux"]);
            let mut lines: Vec<&str> = raw.lines().collect();
            // Keep header + first 5 data lines
            if lines.len() > 6 {
                lines.truncate(6);
            }
            lines.join("\n")
        } else {
            let mut lines: Vec<&str> = top_processes.lines().collect();
            if lines.len() > 6 {
                lines.truncate(6);
            }
            lines.join("\n")
        };

        SystemInfo {
            hostname,
            os_type,
            kernel_version,
            uptime,
            cpu_count,
            cpu_usage,
            memory_total,
            memory_used,
            memory_pressure,
            disk_usage,
            load_average,
            top_processes,
        }
    }

    /// Format the system info as a human-readable report.
    pub fn format_report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!("Hostname:        {}\n", self.hostname));
        report.push_str(&format!("OS:              {}\n", self.os_type));
        report.push_str(&format!("Kernel:          {}\n", self.kernel_version));
        report.push_str(&format!("Uptime:          {}\n", self.uptime));
        report.push_str(&format!("CPU Cores:       {}\n", self.cpu_count));
        report.push_str(&format!("CPU Usage:       {}\n", self.cpu_usage));
        report.push_str(&format!("Memory Total:    {}\n", self.memory_total));
        report.push_str(&format!("Memory Used:     {}\n", self.memory_used));
        report.push_str(&format!("Memory Pressure: {}\n", self.memory_pressure));
        report.push_str(&format!("Load Average:    {}\n", self.load_average));
        report.push_str("\n--- Disk Usage ---\n");
        report.push_str(&self.disk_usage);
        report.push_str("\n\n--- Top Processes ---\n");
        report.push_str(&self.top_processes);
        report
    }

    /// Produce a summary table suitable for NakamaUI::table().
    pub fn summary_rows(&self) -> Vec<Vec<String>> {
        vec![
            vec!["Hostname".to_string(), self.hostname.clone()],
            vec!["OS".to_string(), format!("{} {}", self.os_type, self.kernel_version)],
            vec!["Uptime".to_string(), self.uptime.clone()],
            vec!["CPU Cores".to_string(), self.cpu_count.clone()],
            vec!["CPU Usage".to_string(), self.cpu_usage.clone()],
            vec!["Memory Total".to_string(), self.memory_total.clone()],
            vec!["Memory Used".to_string(), self.memory_used.clone()],
            vec!["Memory Pressure".to_string(), self.memory_pressure.clone()],
            vec!["Load Average".to_string(), self.load_average.clone()],
        ]
    }
}

/// Collect memory info using platform-appropriate commands.
fn collect_memory_info() -> (String, String, String) {
    // Try macOS approach first (sysctl + vm_stat)
    let mem_total = run_cmd("sysctl", &["-n", "hw.memsize"]);
    if !mem_total.is_empty() {
        // macOS path
        let total_bytes: u64 = mem_total.parse().unwrap_or(0);
        let total_gb = total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);

        let vm_stat = run_cmd("vm_stat", &[]);
        let pages_active = parse_vm_stat_value(&vm_stat, "Pages active");
        let pages_wired = parse_vm_stat_value(&vm_stat, "Pages wired down");
        let pages_compressed = parse_vm_stat_value(&vm_stat, "Pages occupied by compressor");
        // Page size is typically 16384 on Apple Silicon, 4096 on Intel
        let page_size_str = run_cmd("sysctl", &["-n", "hw.pagesize"]);
        let page_size: u64 = page_size_str.parse().unwrap_or(4096);

        let used_bytes = (pages_active + pages_wired + pages_compressed) * page_size;
        let used_gb = used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);

        let pressure = run_cmd("sysctl", &["-n", "kern.memorystatus_vm_pressure_level"]);
        let pressure_str = match pressure.as_str() {
            "1" => "Normal".to_string(),
            "2" => "Warning".to_string(),
            "4" => "Critical".to_string(),
            _ => format!("Level {}", if pressure.is_empty() { "unknown" } else { &pressure }),
        };

        (
            format!("{:.1} GB", total_gb),
            format!("{:.1} GB", used_gb),
            pressure_str,
        )
    } else {
        // Linux path — read /proc/meminfo
        let meminfo = run_cmd("cat", &["/proc/meminfo"]);
        let total = parse_meminfo_value(&meminfo, "MemTotal");
        let available = parse_meminfo_value(&meminfo, "MemAvailable");
        let used = total.saturating_sub(available);

        let total_gb = total as f64 / (1024.0 * 1024.0);
        let used_gb = used as f64 / (1024.0 * 1024.0);
        let pressure_pct = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        let pressure_str = if pressure_pct > 90.0 {
            format!("Critical ({:.0}%)", pressure_pct)
        } else if pressure_pct > 70.0 {
            format!("Warning ({:.0}%)", pressure_pct)
        } else {
            format!("Normal ({:.0}%)", pressure_pct)
        };

        (
            format!("{:.1} GB", total_gb),
            format!("{:.1} GB", used_gb),
            pressure_str,
        )
    }
}

/// Parse a value line from vm_stat output (macOS).
fn parse_vm_stat_value(vm_stat: &str, key: &str) -> u64 {
    vm_stat
        .lines()
        .find(|l| l.contains(key))
        .and_then(|l| {
            l.split(':')
                .nth(1)
                .map(|v| v.trim().trim_end_matches('.').parse::<u64>().unwrap_or(0))
        })
        .unwrap_or(0)
}

/// Parse a value from /proc/meminfo (Linux). Values are in kB.
fn parse_meminfo_value(meminfo: &str, key: &str) -> u64 {
    meminfo
        .lines()
        .find(|l| l.starts_with(key))
        .and_then(|l| {
            l.split_whitespace()
                .nth(1)
                .and_then(|v| v.parse::<u64>().ok())
        })
        .unwrap_or(0)
}
