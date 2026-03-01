//! Review pass definitions for the multi-pass AI review engine.
//!
//! Each pass targets a specific aspect of code quality and carries a
//! specialized system prompt that instructs the AI model to focus on that
//! dimension alone.

use nakama_core::config::ByakuganPromptsConfig;
use std::fmt;

/// The different review passes that Byakugan runs against a diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewPass {
    Security,
    Performance,
    Style,
    Logic,
    Summary,
}

impl ReviewPass {
    /// Return all passes in their canonical execution order.
    pub fn all() -> &'static [ReviewPass] {
        &[
            ReviewPass::Security,
            ReviewPass::Performance,
            ReviewPass::Style,
            ReviewPass::Logic,
            ReviewPass::Summary,
        ]
    }

    /// The system prompt sent to the AI model for this pass.
    pub fn system_prompt(&self) -> &'static str {
        match self {
            ReviewPass::Security => {
                "You are a senior security engineer performing a code review. \
                 Analyze the following diff for security vulnerabilities. Look specifically for:\n\
                 - SQL injection, command injection, or other injection attacks\n\
                 - Cross-site scripting (XSS) vulnerabilities\n\
                 - Authentication or authorization bypasses\n\
                 - Hardcoded secrets, API keys, tokens, or passwords in code\n\
                 - Insecure cryptographic practices\n\
                 - Path traversal vulnerabilities\n\
                 - Insecure deserialization\n\
                 - Missing input validation or sanitization\n\
                 - Information leakage through error messages or logs\n\n\
                 IMPORTANT: Your findings will be posted as inline comments on the PR diff. \
                 For each finding, you MUST include the exact file path and line number.\n\n\
                 Format each finding as:\n\
                 1. **Short title**\n\
                 **Severity:** CRITICAL | HIGH | MEDIUM | LOW\n\
                 **File:** `exact/path/to/file.ext` **Line:** N\n\
                 **Issue:** Brief explanation of the risk.\n\
                 **Fix:** Suggested fix or code snippet.\n\n\
                 Use the EXACT file paths from the diff headers (e.g., `src/main/java/com/example/Foo.java`), \
                 not shortened names. The line number must be from the NEW side of the diff (+ lines).\n\n\
                 If no issues are found, respond with: \"No security issues found.\"\n\
                 Be concise and precise. Do not invent problems that don't exist."
            }
            ReviewPass::Performance => {
                "You are a performance engineering specialist reviewing code changes. \
                 Analyze the following diff for performance concerns. Look specifically for:\n\
                 - N+1 query patterns or inefficient database access\n\
                 - Unnecessary memory allocations, copies, or clones\n\
                 - Blocking calls in async contexts\n\
                 - Missing caching opportunities\n\
                 - Algorithmic complexity issues (O(n^2) where O(n) is possible)\n\
                 - Unbounded collection growth\n\
                 - Unnecessary network round-trips\n\
                 - Hot-path allocations that could be avoided\n\
                 - Large structs passed by value instead of by reference\n\n\
                 IMPORTANT: Your findings will be posted as inline comments on the PR diff. \
                 For each finding, you MUST include the exact file path and line number.\n\n\
                 Format each finding as:\n\
                 1. **Short title**\n\
                 **Severity:** CRITICAL | HIGH | MEDIUM | LOW\n\
                 **File:** `exact/path/to/file.ext` **Line:** N\n\
                 **Issue:** Brief explanation of the impact.\n\
                 **Fix:** Suggested optimization.\n\n\
                 Use the EXACT file paths from the diff headers (e.g., `src/main/java/com/example/Foo.java`), \
                 not shortened names. The line number must be from the NEW side of the diff (+ lines).\n\n\
                 If no issues are found, respond with: \"No performance concerns found.\"\n\
                 Be concise and precise. Do not invent problems that don't exist."
            }
            ReviewPass::Style => {
                "You are a code style and maintainability reviewer. \
                 Analyze the following diff for style and maintainability issues. Look for:\n\
                 - Inconsistent naming conventions\n\
                 - Poor code organization or structure\n\
                 - DRY (Don't Repeat Yourself) violations\n\
                 - Overly complex functions that should be decomposed\n\
                 - Missing or misleading documentation/comments\n\
                 - Magic numbers or strings that should be constants\n\
                 - Dead code or unreachable branches\n\
                 - Inconsistent error message formatting\n\
                 - Overly broad imports or unused dependencies\n\n\
                 IMPORTANT: Your findings will be posted as inline comments on the PR diff. \
                 For each finding, you MUST include the exact file path and line number.\n\n\
                 Format each finding as:\n\
                 1. **Short title**\n\
                 **Severity:** HIGH | MEDIUM | LOW\n\
                 **File:** `exact/path/to/file.ext` **Line:** N\n\
                 **Issue:** Brief explanation.\n\
                 **Fix:** Suggested improvement.\n\n\
                 Use the EXACT file paths from the diff headers (e.g., `src/main/java/com/example/Foo.java`), \
                 not shortened names. The line number must be from the NEW side of the diff (+ lines).\n\n\
                 If no issues are found, respond with: \"No style issues found.\"\n\
                 Be concise. Focus on actionable improvements, not nitpicks."
            }
            ReviewPass::Logic => {
                "You are a senior software engineer reviewing code for logical correctness. \
                 Analyze the following diff for logic issues. Look specifically for:\n\
                 - Unhandled edge cases (empty inputs, null values, boundary conditions)\n\
                 - Missing or incorrect error handling\n\
                 - Race conditions or concurrency bugs\n\
                 - Off-by-one errors\n\
                 - Incorrect boolean logic or control flow\n\
                 - Resource leaks (unclosed handles, missing cleanup)\n\
                 - Incorrect type conversions or truncation\n\
                 - Assumptions that may not hold in production\n\
                 - Missing validation of preconditions or invariants\n\n\
                 IMPORTANT: Your findings will be posted as inline comments on the PR diff. \
                 For each finding, you MUST include the exact file path and line number.\n\n\
                 Format each finding as:\n\
                 1. **Short title**\n\
                 **Severity:** CRITICAL | HIGH | MEDIUM | LOW\n\
                 **File:** `exact/path/to/file.ext` **Line:** N\n\
                 **Issue:** Brief explanation of the bug or risk.\n\
                 **Fix:** Suggested fix.\n\n\
                 Use the EXACT file paths from the diff headers (e.g., `src/main/java/com/example/Foo.java`), \
                 not shortened names. The line number must be from the NEW side of the diff (+ lines).\n\n\
                 If no issues are found, respond with: \"No logic issues found.\"\n\
                 Be concise and precise. Do not invent problems that don't exist."
            }
            ReviewPass::Summary => {
                "You are a tech lead providing an overall code review summary. \
                 Based on the following diff, provide:\n\n\
                 1. **Overall Assessment**: A 2-3 sentence summary of the changes.\n\
                 2. **Quality Score**: Rate the code from 1-10 where:\n\
                    - 1-3: Serious problems, should not merge\n\
                    - 4-5: Significant concerns that need addressing\n\
                    - 6-7: Acceptable with minor improvements needed\n\
                    - 8-9: Good quality, minor suggestions only\n\
                    - 10: Excellent, no concerns\n\
                 3. **Key Recommendations**: Top 3-5 actionable items ranked by priority.\n\
                 4. **Merge Readiness**: READY, NEEDS_CHANGES, or BLOCK\n\n\
                 Format the score as: Score: X/10\n\
                 Format merge readiness as: Merge: READY|NEEDS_CHANGES|BLOCK\n\n\
                 Be constructive and balanced. Acknowledge what was done well."
            }
        }
    }

    /// Resolve the system prompt, using config overrides if present.
    ///
    /// If the config has a custom prompt for this pass, that is used; otherwise
    /// the hardcoded default is used. In either case, `preamble` (if set) is
    /// prepended.
    pub fn system_prompt_with_config(&self, prompts: &ByakuganPromptsConfig) -> String {
        let override_prompt = match self {
            ReviewPass::Security => prompts.security.as_deref(),
            ReviewPass::Performance => prompts.performance.as_deref(),
            ReviewPass::Style => prompts.style.as_deref(),
            ReviewPass::Logic => prompts.logic.as_deref(),
            ReviewPass::Summary => prompts.summary.as_deref(),
        };

        let base = override_prompt.unwrap_or_else(|| self.system_prompt());

        match &prompts.preamble {
            Some(preamble) => format!("{}\n\n{}", preamble, base),
            None => base.to_string(),
        }
    }

    /// Create a ReviewPass from a string name.
    pub fn from_name(name: &str) -> Option<ReviewPass> {
        match name.to_lowercase().as_str() {
            "security" => Some(ReviewPass::Security),
            "performance" | "perf" => Some(ReviewPass::Performance),
            "style" => Some(ReviewPass::Style),
            "logic" => Some(ReviewPass::Logic),
            "summary" => Some(ReviewPass::Summary),
            _ => None,
        }
    }

    /// Return passes matching the given names. Falls back to all passes if empty.
    pub fn from_names(names: &[String]) -> Vec<ReviewPass> {
        if names.is_empty() {
            return ReviewPass::all().to_vec();
        }
        let mut passes = Vec::new();
        for name in names {
            if let Some(pass) = ReviewPass::from_name(name) {
                if !passes.contains(&pass) {
                    passes.push(pass);
                }
            }
        }
        if passes.is_empty() {
            ReviewPass::all().to_vec()
        } else {
            passes
        }
    }

    /// A human-readable label for display in tables and panels.
    pub fn label(&self) -> &'static str {
        match self {
            ReviewPass::Security => "Security",
            ReviewPass::Performance => "Performance",
            ReviewPass::Style => "Style",
            ReviewPass::Logic => "Logic",
            ReviewPass::Summary => "Summary",
        }
    }
}

impl fmt::Display for ReviewPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// The result of a single review pass.
#[derive(Debug, Clone)]
pub struct PassResult {
    /// Which pass produced this result.
    pub pass: ReviewPass,
    /// The raw AI response text.
    pub content: String,
    /// Number of findings extracted (0 = clean).
    pub finding_count: usize,
    /// The highest severity found in this pass.
    pub severity: Severity,
    /// Token usage for this pass.
    pub input_tokens: u32,
    pub output_tokens: u32,
}

/// Severity levels for review findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Ok,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    /// Display label for tables.
    pub fn label(&self) -> &'static str {
        match self {
            Severity::Ok => "OK",
            Severity::Low => "LOW",
            Severity::Medium => "MEDIUM",
            Severity::High => "HIGH",
            Severity::Critical => "CRITICAL",
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// Parse the AI response to estimate finding count and max severity.
pub fn parse_findings(content: &str) -> (usize, Severity) {
    let lower = content.to_lowercase();

    // Check for "no issues" / "no concerns" / "no problems" patterns.
    let clean_patterns = [
        "no security issues found",
        "no performance concerns found",
        "no style issues found",
        "no logic issues found",
        "no issues found",
        "no concerns found",
        "no problems found",
    ];
    for pat in &clean_patterns {
        if lower.contains(pat) {
            return (0, Severity::Ok);
        }
    }

    // Count numbered findings (e.g., "1.", "### 2.", "**3." at line starts).
    let mut count = 0usize;
    for line in content.lines() {
        let trimmed = line.trim();
        // Strip leading markdown decoration (#, *, -)
        let stripped = trimmed
            .trim_start_matches('#')
            .trim_start_matches('*')
            .trim_start_matches('-')
            .trim()
            .trim_start_matches("**")
            .trim();
        if stripped.len() >= 2 {
            let first_char = stripped.chars().next().unwrap_or(' ');
            if first_char.is_ascii_digit() {
                // Check for "N." pattern (possibly multi-digit)
                if stripped.chars().skip_while(|c| c.is_ascii_digit()).next() == Some('.') {
                    count += 1;
                }
            }
        }
    }

    // If no numbered findings, count severity keyword mentions as a proxy.
    if count == 0 {
        let severity_keywords = ["critical", "high", "medium", "low"];
        for kw in &severity_keywords {
            count += lower.matches(kw).count();
        }
        // Deduplicate: each unique mention is roughly one finding.
        // Cap at a reasonable number since keywords can appear in explanations.
        count = count.min(20);
    }

    // Determine the highest severity mentioned.
    let severity = if lower.contains("critical") {
        Severity::Critical
    } else if lower.contains("high") {
        Severity::High
    } else if lower.contains("medium") {
        Severity::Medium
    } else if lower.contains("low") {
        Severity::Low
    } else if count > 0 {
        Severity::Low
    } else {
        Severity::Ok
    };

    // If we truly found zero explicit findings but have content, there's
    // probably at least one implicit finding.
    if count == 0 && content.len() > 100 && severity != Severity::Ok {
        (1, severity)
    } else {
        (count.max(if severity > Severity::Ok { 1 } else { 0 }), severity)
    }
}
