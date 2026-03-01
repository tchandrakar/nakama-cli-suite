//! Output formatting layer.
//!
//! Supports three output formats: Terminal (NakamaUI), JSON, and Markdown.
//! Used by review, scan, and report commands for consistent output.

use crate::dedup::DedupFinding;
use crate::passes::{PassResult, Severity};
use crate::rules::RuleFinding;
use serde::Serialize;
use std::fmt;

/// Output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Terminal,
    Json,
    Markdown,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "markdown" | "md" => OutputFormat::Markdown,
            _ => OutputFormat::Terminal,
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Terminal => write!(f, "terminal"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Markdown => write!(f, "markdown"),
        }
    }
}

// ---------------------------------------------------------------------------
// JSON output structures
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct JsonReviewOutput {
    pub context: String,
    pub passes: Vec<JsonPassResult>,
    pub summary: JsonSummary,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rule_findings: Vec<JsonRuleFinding>,
}

#[derive(Debug, Serialize)]
pub struct JsonPassResult {
    pub pass: String,
    pub finding_count: usize,
    pub severity: String,
    pub content: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Serialize)]
pub struct JsonSummary {
    pub total_findings: usize,
    pub max_severity: String,
    pub total_input_tokens: u32,
    pub total_output_tokens: u32,
}

#[derive(Debug, Serialize)]
pub struct JsonRuleFinding {
    pub rule: String,
    pub severity: String,
    pub file: String,
    pub line: u32,
    pub description: String,
    pub matched_text: String,
}

// ---------------------------------------------------------------------------
// Format functions
// ---------------------------------------------------------------------------

/// Format pass results as JSON.
pub fn format_json(
    context: &str,
    results: &[PassResult],
    rule_findings: &[RuleFinding],
) -> String {
    let total_findings: usize = results.iter().map(|r| r.finding_count).sum();
    let max_severity = results
        .iter()
        .map(|r| r.severity)
        .max()
        .unwrap_or(Severity::Ok);

    let output = JsonReviewOutput {
        context: context.to_string(),
        passes: results
            .iter()
            .map(|r| JsonPassResult {
                pass: r.pass.label().to_string(),
                finding_count: r.finding_count,
                severity: r.severity.label().to_string(),
                content: r.content.clone(),
                input_tokens: r.input_tokens,
                output_tokens: r.output_tokens,
            })
            .collect(),
        summary: JsonSummary {
            total_findings,
            max_severity: max_severity.label().to_string(),
            total_input_tokens: results.iter().map(|r| r.input_tokens).sum(),
            total_output_tokens: results.iter().map(|r| r.output_tokens).sum(),
        },
        rule_findings: rule_findings
            .iter()
            .map(|f| JsonRuleFinding {
                rule: f.rule_name.clone(),
                severity: f.severity.label().to_string(),
                file: f.file.clone(),
                line: f.line,
                description: f.description.clone(),
                matched_text: f.matched_text.clone(),
            })
            .collect(),
    };

    serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
}

/// Format pass results as Markdown.
pub fn format_markdown(
    context: &str,
    results: &[PassResult],
    rule_findings: &[RuleFinding],
) -> String {
    let mut md = String::new();

    md.push_str(&format!("# Byakugan Review Report\n\n"));
    md.push_str(&format!("**Context:** {}\n\n", context));

    // Summary table.
    md.push_str("## Summary\n\n");
    md.push_str("| Pass | Findings | Severity | Tokens |\n");
    md.push_str("|------|----------|----------|--------|\n");

    for r in results {
        let findings = if r.finding_count == 0 {
            "No issues".to_string()
        } else {
            format!("{} concern(s)", r.finding_count)
        };
        md.push_str(&format!(
            "| {} | {} | {} | {}/{} |\n",
            r.pass.label(),
            findings,
            r.severity.label(),
            r.input_tokens,
            r.output_tokens,
        ));
    }

    // Detailed findings.
    md.push_str("\n## Detailed Findings\n\n");
    for r in results {
        if r.finding_count > 0 || r.pass == crate::passes::ReviewPass::Summary {
            md.push_str(&format!("### {} Review\n\n", r.pass.label()));
            md.push_str(&r.content);
            md.push_str("\n\n");
        }
    }

    // Rule findings.
    if !rule_findings.is_empty() {
        md.push_str("## Rule Violations\n\n");
        for (i, f) in rule_findings.iter().enumerate() {
            md.push_str(&format!(
                "{}. **[{}]** {} (`{}:{}`)\n   - {}\n   - Match: `{}`\n\n",
                i + 1,
                f.severity,
                f.rule_name,
                f.file,
                f.line,
                f.description,
                f.matched_text.trim(),
            ));
        }
    }

    md.push_str("---\n*Generated by Byakugan v0.2.0*\n");
    md
}

/// Format deduplicated findings as a summary.
pub fn format_dedup_summary(findings: &[DedupFinding]) -> String {
    if findings.is_empty() {
        return "No deduplicated findings.".to_string();
    }

    let mut output = String::new();
    output.push_str(&format!("Unique findings: {}\n\n", findings.len()));

    for (i, f) in findings.iter().enumerate() {
        let location = match (&f.file, f.line) {
            (Some(file), Some(line)) => format!(" ({}:{})", file, line),
            (Some(file), None) => format!(" ({})", file),
            _ => String::new(),
        };
        output.push_str(&format!(
            "{}. [{}] {}{}\n   Flagged by: {}\n\n",
            i + 1,
            f.severity,
            f.title,
            location,
            f.passes.join(", "),
        ));
    }

    output
}

/// Format scan-only results (no AI) as JSON.
pub fn format_scan_json(rule_findings: &[RuleFinding]) -> String {
    let output: Vec<JsonRuleFinding> = rule_findings
        .iter()
        .map(|f| JsonRuleFinding {
            rule: f.rule_name.clone(),
            severity: f.severity.label().to_string(),
            file: f.file.clone(),
            line: f.line,
            description: f.description.clone(),
            matched_text: f.matched_text.clone(),
        })
        .collect();

    serde_json::to_string_pretty(&output).unwrap_or_else(|_| "[]".to_string())
}
