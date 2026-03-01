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

/// Strip leading markdown decoration (`#`, `*`, `-`, whitespace) from a line
/// to expose the underlying numbered item (e.g., "### 1. Title" → "1. Title").
fn strip_markdown_prefix(line: &str) -> &str {
    let s = line.trim();
    let s = s.trim_start_matches('#').trim_start_matches('*').trim_start_matches('-').trim();
    // Also strip bold markers: "**1." → "1."
    s.trim_start_matches("**").trim()
}

/// Check if a (stripped) line starts with a numbered item like "1." or "12.".
fn is_numbered_item(stripped: &str) -> bool {
    let mut chars = stripped.chars();
    let first = match chars.next() {
        Some(c) if c.is_ascii_digit() => c,
        _ => return false,
    };
    // Allow multi-digit numbers (e.g., "10.")
    let _ = first;
    for c in chars {
        if c == '.' {
            return true;
        }
        if !c.is_ascii_digit() {
            return false;
        }
    }
    false
}

/// Extract the title text after the "N." prefix.
fn extract_numbered_title(stripped: &str) -> String {
    if let Some(dot_pos) = stripped.find('.') {
        stripped[dot_pos + 1..].trim().to_string()
    } else {
        stripped.to_string()
    }
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

    // Split by numbered items (1., 2., etc.) — also handles "### 1.", "**1.", etc.
    let mut current_title = String::new();
    let mut current_content = String::new();
    let mut current_severity = Severity::Low;

    for line in content.lines() {
        let stripped = strip_markdown_prefix(line);

        if is_numbered_item(stripped) {
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

            current_title = extract_numbered_title(stripped);
            current_content = String::new();
            current_severity = detect_severity(stripped);
        } else {
            let trimmed = line.trim();
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

/// Known source-code file extensions for simple filename matching.
const SOURCE_EXTENSIONS: &[&str] = &[
    ".java", ".ts", ".tsx", ".js", ".jsx", ".rs", ".py", ".go", ".rb", ".cs",
    ".cpp", ".c", ".h", ".hpp", ".kt", ".swift", ".scala", ".php", ".vue",
    ".svelte", ".yaml", ".yml", ".toml", ".json", ".xml", ".sql", ".sh",
    ".tf", ".proto", ".graphql", ".css", ".scss", ".html",
];

fn extract_file_reference(content: &str) -> Option<String> {
    // Collect all candidate tokens: split on whitespace AND extract backtick-delimited spans.
    let candidates = extract_candidate_tokens(content);

    // First pass: look for full paths with / and .
    for token in &candidates {
        let clean = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '/' && c != '.' && c != '_' && c != '-');
        if clean.contains('/') && clean.contains('.') && clean.len() > 3 {
            return Some(clean.to_string());
        }
    }

    // Second pass: simple filename with a known extension (e.g., "AISettingsController.java")
    for token in &candidates {
        let clean = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '/' && c != '.' && c != '_' && c != '-');
        if clean.len() > 3 && SOURCE_EXTENSIONS.iter().any(|ext| clean.ends_with(ext)) {
            if let Some(dot_pos) = clean.rfind('.') {
                if dot_pos > 0 {
                    return Some(clean.to_string());
                }
            }
        }
    }

    None
}

/// Extract candidate tokens from content: words from whitespace splitting
/// plus text inside backticks (`` `...` ``).
fn extract_candidate_tokens(content: &str) -> Vec<String> {
    let mut tokens = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Extract backtick-delimited tokens.
        let mut rest = trimmed;
        while let Some(start) = rest.find('`') {
            let after_tick = &rest[start + 1..];
            if let Some(end) = after_tick.find('`') {
                let inside = &after_tick[..end];
                if !inside.is_empty() && !inside.contains('\n') {
                    tokens.push(inside.to_string());
                }
                rest = &after_tick[end + 1..];
            } else {
                break;
            }
        }

        // Also add plain whitespace-split words.
        for word in trimmed.split_whitespace() {
            tokens.push(word.to_string());
        }
    }

    tokens
}

fn extract_line_reference(content: &str) -> Option<u32> {
    let lower = content.to_lowercase();

    // Try multiple patterns:
    // "line 42", "Line 42", "line: 42", "lines 42-50" (take first), "L42"
    let patterns: &[&str] = &["line ", "line: ", "lines ", "line:", "l:"];
    for pat in patterns {
        let mut search_from = 0;
        while let Some(idx) = lower[search_from..].find(pat) {
            let abs_idx = search_from + idx + pat.len();
            let after = &content[abs_idx..];
            // Skip any whitespace
            let after = after.trim_start();
            let num_str: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(n) = num_str.parse::<u32>() {
                if n > 0 && n < 100_000 {
                    return Some(n);
                }
            }
            search_from = abs_idx;
        }
    }

    // Also look for "@line N" or "at line N" patterns.
    if let Some(idx) = lower.find("at line ") {
        let after = &content[idx + 8..];
        let num_str: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(n) = num_str.parse::<u32>() {
            if n > 0 && n < 100_000 {
                return Some(n);
            }
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
