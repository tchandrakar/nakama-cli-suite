//! Finding deduplication across multiple review passes.
//!
//! When multiple review passes (security, performance, logic, etc.) are run
//! against the same diff, they may flag the same code locations. This module
//! detects and merges duplicate or overlapping findings.

use crate::passes::{PassResult, Severity};

/// A deduplicated finding with merged context from multiple passes.
#[derive(Debug, Clone)]
pub struct DedupFinding {
    pub title: String,
    pub severity: Severity,
    pub passes: Vec<String>,
    pub content: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub score: f64,
}

/// Deduplicate findings across multiple pass results.
///
/// Findings that reference the same file/line or have similar titles are
/// merged, keeping the highest severity and noting all contributing passes.
pub fn deduplicate_findings(results: &[PassResult]) -> Vec<DedupFinding> {
    let mut findings: Vec<DedupFinding> = Vec::new();

    for result in results {
        let pass_label = result.pass.label().to_string();
        let pass_findings = extract_findings_from_content(&result.content, &pass_label);

        for finding in pass_findings {
            if let Some(existing) = findings.iter_mut().find(|f| is_similar(f, &finding)) {
                // Merge: keep higher severity, add pass.
                if finding.severity > existing.severity {
                    existing.severity = finding.severity;
                }
                if !existing.passes.contains(&pass_label) {
                    existing.passes.push(pass_label.clone());
                }
                existing.score += 1.0;
            } else {
                findings.push(finding);
            }
        }
    }

    // Sort by score (most cross-referenced first), then severity.
    findings.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.severity.cmp(&a.severity))
    });

    findings
}

/// Check if two findings are similar enough to merge.
fn is_similar(a: &DedupFinding, b: &DedupFinding) -> bool {
    // Same file and line → definitely similar.
    if let (Some(fa), Some(fb)) = (&a.file, &b.file) {
        if fa == fb {
            if let (Some(la), Some(lb)) = (a.line, b.line) {
                if (la as i64 - lb as i64).unsigned_abs() <= 3 {
                    return true;
                }
            }
        }
    }

    // Similar titles (basic similarity: shared significant words).
    let words_a: Vec<&str> = a.title.split_whitespace().collect();
    let words_b: Vec<&str> = b.title.split_whitespace().collect();
    let shared = words_a
        .iter()
        .filter(|w| w.len() > 3 && words_b.contains(w))
        .count();
    let max_words = words_a.len().max(words_b.len());

    if max_words > 0 && shared as f64 / max_words as f64 > 0.5 {
        return true;
    }

    false
}

/// Extract individual findings from a pass's AI response content.
fn extract_findings_from_content(content: &str, pass_label: &str) -> Vec<DedupFinding> {
    let mut findings = Vec::new();
    let lower = content.to_lowercase();

    // Check for "no issues" patterns.
    let clean = [
        "no security issues found",
        "no performance concerns found",
        "no style issues found",
        "no logic issues found",
        "no issues found",
        "no concerns found",
    ];
    for pat in &clean {
        if lower.contains(pat) {
            return findings;
        }
    }

    // Split by numbered items (1., 2., etc.).
    let mut current_title = String::new();
    let mut current_content = String::new();
    let mut current_severity = Severity::Low;

    for line in content.lines() {
        let trimmed = line.trim();

        // Check for numbered finding start.
        let is_numbered = trimmed.len() >= 2
            && trimmed.chars().next().map_or(false, |c| c.is_ascii_digit())
            && trimmed.chars().nth(1) == Some('.');

        if is_numbered {
            // Save previous finding.
            if !current_title.is_empty() {
                findings.push(DedupFinding {
                    title: current_title.clone(),
                    severity: current_severity,
                    passes: vec![pass_label.to_string()],
                    content: current_content.clone(),
                    file: extract_file_reference(&current_content),
                    line: extract_line_reference(&current_content),
                    score: severity_score(current_severity),
                });
            }

            current_title = trimmed
                .splitn(2, '.')
                .nth(1)
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            current_content = String::new();
            current_severity = detect_severity(trimmed);
        } else {
            current_content.push_str(trimmed);
            current_content.push('\n');
            let line_severity = detect_severity(trimmed);
            if line_severity > current_severity {
                current_severity = line_severity;
            }
        }
    }

    // Save last finding.
    if !current_title.is_empty() {
        let file = extract_file_reference(&current_content);
        let line = extract_line_reference(&current_content);
        findings.push(DedupFinding {
            title: current_title,
            severity: current_severity,
            passes: vec![pass_label.to_string()],
            content: current_content,
            file,
            line,
            score: severity_score(current_severity),
        });
    }

    findings
}

fn detect_severity(text: &str) -> Severity {
    let lower = text.to_lowercase();
    if lower.contains("critical") {
        Severity::Critical
    } else if lower.contains("high") {
        Severity::High
    } else if lower.contains("medium") {
        Severity::Medium
    } else if lower.contains("low") {
        Severity::Low
    } else {
        Severity::Ok
    }
}

fn severity_score(s: Severity) -> f64 {
    match s {
        Severity::Critical => 5.0,
        Severity::High => 4.0,
        Severity::Medium => 3.0,
        Severity::Low => 2.0,
        Severity::Ok => 1.0,
    }
}

fn extract_file_reference(content: &str) -> Option<String> {
    // Look for file path patterns in the content.
    for line in content.lines() {
        let trimmed = line.trim();
        // Match patterns like "src/main.rs" or "file: path/to/file"
        if trimmed.contains('/') && trimmed.contains('.') {
            for word in trimmed.split_whitespace() {
                let clean = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '/' && c != '.' && c != '_' && c != '-');
                if clean.contains('/') && clean.contains('.') && clean.len() > 3 {
                    return Some(clean.to_string());
                }
            }
        }
    }
    None
}

fn extract_line_reference(content: &str) -> Option<u32> {
    // Look for "line X" or "Line X" patterns.
    let lower = content.to_lowercase();
    if let Some(idx) = lower.find("line ") {
        let after = &content[idx + 5..];
        let num_str: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(n) = num_str.parse() {
            return Some(n);
        }
    }
    None
}

/// Filter findings by severity threshold.
pub fn filter_by_threshold(findings: &[DedupFinding], threshold: &str) -> Vec<DedupFinding> {
    let min_severity = match threshold.to_lowercase().as_str() {
        "critical" => Severity::Critical,
        "high" => Severity::High,
        "medium" => Severity::Medium,
        "low" => Severity::Low,
        _ => Severity::Ok,
    };

    findings
        .iter()
        .filter(|f| f.severity >= min_severity)
        .cloned()
        .collect()
}
