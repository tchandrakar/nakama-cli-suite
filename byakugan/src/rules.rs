//! Custom rules engine.
//!
//! Compiles user-defined rules from the config into regex patterns and
//! glob exclusion filters. Matches rules against added lines in diffs
//! to produce findings without requiring AI.

use crate::analysis;
use crate::passes::Severity;
use crate::platform::UnifiedDiff;
use anyhow::Result;
use nakama_core::config::ByakuganRule;
use regex::Regex;

/// A compiled rule ready for matching.
pub struct CompiledRule {
    pub name: String,
    pub description: String,
    pub severity: Severity,
    pub pattern: Regex,
    pub exclude: Vec<glob::Pattern>,
}

/// A finding produced by the rules engine.
#[derive(Debug, Clone)]
pub struct RuleFinding {
    pub rule_name: String,
    pub description: String,
    pub severity: Severity,
    pub file: String,
    pub line: u32,
    pub matched_text: String,
}

/// Compile config rules into executable form.
pub fn compile_rules(rules: &[ByakuganRule]) -> Result<Vec<CompiledRule>> {
    let mut compiled = Vec::new();

    for rule in rules {
        let pattern = Regex::new(&rule.pattern).map_err(|e| {
            anyhow::anyhow!(
                "Invalid regex pattern in rule '{}': {} (pattern: '{}')",
                rule.name,
                e,
                rule.pattern
            )
        })?;

        let exclude: Vec<glob::Pattern> = rule
            .exclude
            .iter()
            .filter_map(|g| glob::Pattern::new(g).ok())
            .collect();

        let severity = parse_severity(&rule.severity);

        compiled.push(CompiledRule {
            name: rule.name.clone(),
            description: rule.description.clone(),
            severity,
            pattern,
            exclude,
        });
    }

    Ok(compiled)
}

/// Run compiled rules against a parsed diff. Only added lines are checked.
pub fn scan_diff(rules: &[CompiledRule], diff: &UnifiedDiff) -> Vec<RuleFinding> {
    let added = analysis::added_lines(diff);
    let mut findings = Vec::new();

    for (file, lineno, content) in &added {
        for rule in rules {
            // Check exclusions.
            if rule
                .exclude
                .iter()
                .any(|pat| pat.matches(file))
            {
                continue;
            }

            if rule.pattern.is_match(content) {
                findings.push(RuleFinding {
                    rule_name: rule.name.clone(),
                    description: rule.description.clone(),
                    severity: rule.severity,
                    file: file.clone(),
                    line: *lineno,
                    matched_text: content.clone(),
                });
            }
        }
    }

    findings
}

/// Validate that all rules in the config have valid regex patterns.
pub fn validate_rules(rules: &[ByakuganRule]) -> Vec<(String, String)> {
    let mut errors = Vec::new();

    for rule in rules {
        if let Err(e) = Regex::new(&rule.pattern) {
            errors.push((rule.name.clone(), format!("{}", e)));
        }
    }

    errors
}

/// List rules with their details.
pub fn list_rules(rules: &[ByakuganRule]) -> Vec<(String, String, String, String)> {
    rules
        .iter()
        .map(|r| {
            (
                r.name.clone(),
                r.severity.clone(),
                r.pattern.clone(),
                r.description.clone(),
            )
        })
        .collect()
}

/// Test a rule pattern against sample text.
pub fn test_rule(pattern: &str, sample: &str) -> Result<Vec<String>> {
    let regex = Regex::new(pattern)?;
    let matches: Vec<String> = regex
        .find_iter(sample)
        .map(|m| m.as_str().to_string())
        .collect();
    Ok(matches)
}

fn parse_severity(s: &str) -> Severity {
    match s.to_lowercase().as_str() {
        "critical" => Severity::Critical,
        "high" => Severity::High,
        "medium" => Severity::Medium,
        "low" => Severity::Low,
        _ => Severity::Medium,
    }
}

/// Format rule findings for display.
pub fn format_findings(findings: &[RuleFinding]) -> String {
    if findings.is_empty() {
        return "No rule violations found.".to_string();
    }

    let mut output = String::new();
    for (i, f) in findings.iter().enumerate() {
        output.push_str(&format!(
            "{}. [{}] {} ({}:{})\n   {}\n   Match: {}\n\n",
            i + 1,
            f.severity,
            f.rule_name,
            f.file,
            f.line,
            f.description,
            f.matched_text.trim(),
        ));
    }
    output
}
