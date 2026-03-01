//! NMP (Nakama Message Protocol) integration for Byakugan.
//!
//! Emits structured messages to stdout when piped, and consumes NMP input
//! from other Nakama tools for context enrichment.

use crate::passes::PassResult;
use crate::rules::RuleFinding;
use nakama_ipc::{NmpMessage, pipe};
use serde_json::json;

/// Emit an NMP review message to stdout (only when piped).
pub fn emit_review_message(
    context: &str,
    results: &[PassResult],
) {
    if !pipe::is_pipe_input() && !is_stdout_piped() {
        return;
    }

    let passes: Vec<serde_json::Value> = results
        .iter()
        .map(|r| {
            json!({
                "pass": r.pass.label(),
                "finding_count": r.finding_count,
                "severity": r.severity.label(),
                "input_tokens": r.input_tokens,
                "output_tokens": r.output_tokens,
            })
        })
        .collect();

    let total_findings: usize = results.iter().map(|r| r.finding_count).sum();
    let max_severity = results
        .iter()
        .map(|r| r.severity)
        .max()
        .map(|s| s.label())
        .unwrap_or("OK");

    let data = json!({
        "context": context,
        "passes": passes,
        "total_findings": total_findings,
        "max_severity": max_severity,
    });

    let msg = NmpMessage::new("byakugan", "review", "byakugan.review.v1", data);
    let _ = pipe::write_stdout(&msg);
}

/// Emit an NMP scan message to stdout.
pub fn emit_scan_message(findings: &[RuleFinding]) {
    if !pipe::is_pipe_input() && !is_stdout_piped() {
        return;
    }

    let violations: Vec<serde_json::Value> = findings
        .iter()
        .map(|f| {
            json!({
                "rule": f.rule_name,
                "severity": f.severity.label(),
                "file": f.file,
                "line": f.line,
                "description": f.description,
                "matched_text": f.matched_text,
            })
        })
        .collect();

    let data = json!({
        "total_violations": findings.len(),
        "violations": violations,
    });

    let msg = NmpMessage::new("byakugan", "scan", "byakugan.scan.v1", data);
    let _ = pipe::write_stdout(&msg);
}

/// Try to read enrichment context from NMP input on stdin.
pub fn read_enrichment_context() -> Option<serde_json::Value> {
    if !pipe::is_pipe_input() {
        return None;
    }

    match pipe::read_stdin() {
        Ok(msg) => Some(msg.data),
        Err(_) => None,
    }
}

/// Check if stdout is piped (not a TTY).
fn is_stdout_piped() -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        extern "C" {
            fn isatty(fd: std::os::raw::c_int) -> std::os::raw::c_int;
        }
        unsafe { isatty(std::io::stdout().as_raw_fd()) == 0 }
    }
    #[cfg(not(unix))]
    {
        false
    }
}
