//! Comprehensive health check: disk, memory, processes, network.

use crate::network::{run_network_checks, format_network_report, CheckStatus};
use crate::system::SystemInfo;
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Run a comprehensive health check across disk, memory, processes, and network.
pub async fn run(config: &Config, ui: &NakamaUI) -> NakamaResult<()> {
    let trace = TraceContext::new("jogan", "health");
    let start = Instant::now();

    // --- System Health ---
    let spinner = ui.step_start("Checking system health...");
    let info = SystemInfo::collect();
    spinner.finish_with_success("System health collected");

    let sys_rows = info.summary_rows();
    ui.table(&["Metric", "Value"], sys_rows);

    // --- Disk Health ---
    let spinner = ui.step_start("Checking disk health...");
    let disk_lines: Vec<&str> = info.disk_usage.lines().collect();
    let mut disk_rows = Vec::new();
    if disk_lines.len() > 1 {
        // Parse df -h output: Filesystem Size Used Avail Use% Mounted
        for line in &disk_lines[1..] {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let status = if let Some(pct_str) = parts.get(4) {
                    let pct: u32 = pct_str.trim_end_matches('%').parse().unwrap_or(0);
                    if pct >= 90 {
                        "CRITICAL"
                    } else if pct >= 75 {
                        "WARNING"
                    } else {
                        "OK"
                    }
                } else {
                    "UNKNOWN"
                };
                disk_rows.push(vec![
                    parts[0].to_string(),
                    parts.get(1).unwrap_or(&"?").to_string(),
                    parts.get(2).unwrap_or(&"?").to_string(),
                    parts.get(3).unwrap_or(&"?").to_string(),
                    parts.get(4).unwrap_or(&"?").to_string(),
                    status.to_string(),
                ]);
            }
        }
    }
    spinner.finish_with_success("Disk health collected");

    if !disk_rows.is_empty() {
        ui.table(
            &["Filesystem", "Size", "Used", "Avail", "Use%", "Status"],
            disk_rows,
        );
    }

    // --- Process Health ---
    let spinner = ui.step_start("Checking top processes...");
    let proc_lines: Vec<&str> = info.top_processes.lines().collect();
    let mut proc_rows = Vec::new();
    for line in proc_lines.iter().skip(1).take(5) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 11 {
            proc_rows.push(vec![
                parts[10..].join(" "),   // command
                parts[2].to_string(),    // %CPU
                parts[3].to_string(),    // %MEM
                parts[1].to_string(),    // PID
            ]);
        }
    }
    spinner.finish_with_success("Process check complete");

    if !proc_rows.is_empty() {
        ui.table(&["Process", "%CPU", "%MEM", "PID"], proc_rows);
    }

    // --- Network Health ---
    let spinner = ui.step_start("Running network checks...");
    let net_checks = run_network_checks();
    spinner.finish_with_success("Network checks complete");

    let net_rows: Vec<Vec<String>> = net_checks
        .iter()
        .map(|c| {
            vec![
                c.name.clone(),
                c.status.label().to_string(),
                c.detail.clone(),
            ]
        })
        .collect();
    ui.table(&["Check", "Status", "Detail"], net_rows);

    // Overall health summary
    let has_net_errors = net_checks.iter().any(|c| matches!(c.status, CheckStatus::Error));
    let mem_warning = info.memory_pressure.contains("Warning") || info.memory_pressure.contains("Critical");

    if has_net_errors || mem_warning {
        ui.warn("Some health checks reported issues. Review the details above.");
    } else {
        ui.success("All health checks passed.");
    }

    let elapsed = start.elapsed().as_millis() as u64;
    let net_report = format_network_report(&net_checks);

    if let Ok(audit) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id,
            "jogan",
            "health",
            Category::ToolExecution,
            "Ran comprehensive health check",
            serde_json::json!({
                "network_errors": has_net_errors,
                "memory_warning": mem_warning,
                "network_summary": net_report,
            }),
            if has_net_errors || mem_warning {
                Outcome::Success
            } else {
                Outcome::Success
            },
            elapsed,
        );
        let _ = audit.log(entry);
    }

    Ok(())
}
