//! The core multi-pass AI review engine.
//!
//! Runs each [`ReviewPass`] against a diff, collects results, and formats
//! them for display. This module is provider-agnostic — it receives a
//! `Box<dyn AiProvider>` and uses it for all passes.

use crate::dedup;
use crate::passes::{parse_findings, PassResult, ReviewPass, Severity};
use crate::platform;
use anyhow::{Context, Result};
use nakama_ai::types::{CompletionRequest, Message};
use nakama_ai::AiProvider;
use nakama_core::config::ByakuganPromptsConfig;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Maximum characters of diff to send per pass.
const MAX_DIFF_CHARS: usize = 60_000;

/// Run all review passes against the given diff text and display results.
pub async fn run_review(
    ui: &NakamaUI,
    provider: &dyn AiProvider,
    model: &str,
    diff: &str,
    context_label: &str,
) -> Result<Vec<PassResult>> {
    let default_prompts = ByakuganPromptsConfig::default();
    run_review_with_passes(ui, provider, model, diff, context_label, &[], &default_prompts).await
}

/// Run review with configurable passes and prompt overrides.
pub async fn run_review_with_passes(
    ui: &NakamaUI,
    provider: &dyn AiProvider,
    model: &str,
    diff: &str,
    context_label: &str,
    pass_names: &[String],
    prompts: &ByakuganPromptsConfig,
) -> Result<Vec<PassResult>> {
    let truncated_diff = nakama_core::diff::compress_diff(diff, MAX_DIFF_CHARS);

    ui.panel(
        "Byakugan Review",
        &format!(
            "Context: {}\nProvider: {}\nModel: {}\nDiff size: {} chars{}",
            context_label,
            provider.provider_name(),
            model,
            diff.len(),
            if diff.len() > MAX_DIFF_CHARS {
                format!(" (truncated to {})", MAX_DIFF_CHARS)
            } else {
                String::new()
            },
        ),
    );

    let passes = ReviewPass::from_names(pass_names);
    let mut results: Vec<PassResult> = Vec::with_capacity(passes.len());

    for &pass in &passes {
        let spinner = ui.step_start(&format!("Running {} pass...", pass.label()));

        let start = Instant::now();
        let result = run_single_pass(provider, model, &truncated_diff, pass, prompts).await;
        let elapsed = start.elapsed();

        match result {
            Ok(pass_result) => {
                let summary = if pass_result.finding_count == 0 {
                    format!(
                        "{} pass complete — no issues ({:.1}s)",
                        pass.label(),
                        elapsed.as_secs_f64()
                    )
                } else {
                    format!(
                        "{} pass complete — {} finding(s), max severity: {} ({:.1}s)",
                        pass.label(),
                        pass_result.finding_count,
                        pass_result.severity,
                        elapsed.as_secs_f64()
                    )
                };
                spinner.finish_with_success(&summary);
                results.push(pass_result);
            }
            Err(e) => {
                spinner.finish_with_error(&format!("{} pass failed: {}", pass.label(), e));
                // Record the failure as a pass result so the table still shows it.
                results.push(PassResult {
                    pass,
                    content: format!("Error: {}", e),
                    finding_count: 0,
                    severity: Severity::Ok,
                    input_tokens: 0,
                    output_tokens: 0,
                });
            }
        }
    }

    // Display summary table.
    display_summary_table(ui, &results);

    // Display detailed findings for each pass.
    display_detailed_findings(ui, &results);

    Ok(results)
}

/// Run a single review pass against the diff.
async fn run_single_pass(
    provider: &dyn AiProvider,
    model: &str,
    diff: &str,
    pass: ReviewPass,
    prompts: &ByakuganPromptsConfig,
) -> Result<PassResult> {
    let user_message = format!(
        "Please review the following code diff:\n\n```diff\n{}\n```",
        diff
    );

    let request = CompletionRequest {
        system_prompt: pass.system_prompt_with_config(prompts),
        messages: vec![Message::user(user_message)],
        model: model.to_string(),
        max_tokens: 2048,
        temperature: 0.2,
    };

    let response = provider
        .complete(request)
        .await
        .context(format!("AI completion failed for {} pass", pass.label()))?;

    let (finding_count, severity) = parse_findings(&response.content);

    Ok(PassResult {
        pass,
        content: response.content,
        finding_count,
        severity,
        input_tokens: response.usage.input_tokens,
        output_tokens: response.usage.output_tokens,
    })
}

/// Display the summary table of all pass results.
fn display_summary_table(ui: &NakamaUI, results: &[PassResult]) {
    let headers = &["Pass", "Findings", "Severity", "Tokens"];
    let rows: Vec<Vec<String>> = results
        .iter()
        .map(|r| {
            let findings_str = if r.finding_count == 0 {
                "No issues found".to_string()
            } else {
                format!("{} concern(s)", r.finding_count)
            };

            let severity_str = r.severity.label().to_string();
            let tokens_str = format!("{}/{}", r.input_tokens, r.output_tokens);

            vec![
                r.pass.label().to_string(),
                findings_str,
                severity_str,
                tokens_str,
            ]
        })
        .collect();

    println!(); // spacing
    ui.table(headers, rows);
}

/// Display detailed findings for each pass that has them.
fn display_detailed_findings(ui: &NakamaUI, results: &[PassResult]) {
    for result in results {
        if result.content.starts_with("Error:") {
            ui.panel(
                &format!("{} (ERROR)", result.pass.label()),
                &result.content,
            );
            continue;
        }

        // Always show the summary pass, but only show other passes if they
        // have findings.
        if result.pass == ReviewPass::Summary || result.finding_count > 0 {
            ui.panel(
                &format!("{} Review", result.pass.label()),
                &result.content,
            );
        }
    }
}

/// Build a comprehensive markdown review body from all pass results.
///
/// This is intended for posting as a single PR comment that covers every pass,
/// not just the summary.
pub fn build_review_body(results: &[PassResult], model: &str, context_label: &str) -> String {
    let stats = ReviewStats::from_results(results);
    let version = env!("CARGO_PKG_VERSION");

    let mut body = String::with_capacity(4096);

    // Header
    body.push_str(&format!(
        "## Byakugan AI Review\n\n\
         > **v{}** | model: `{}` | context: {}\n\n",
        version, model, context_label,
    ));

    // Summary table
    body.push_str("| Pass | Findings | Severity |\n|------|----------|----------|\n");
    for r in results {
        let findings = if r.finding_count == 0 {
            "No issues".to_string()
        } else {
            format!("{} finding(s)", r.finding_count)
        };
        body.push_str(&format!(
            "| {} | {} | {} |\n",
            r.pass.label(),
            findings,
            r.severity.label(),
        ));
    }
    body.push('\n');

    // Overall verdict
    let verdict = if stats.max_severity >= Severity::High {
        "NEEDS CHANGES"
    } else if stats.total_findings == 0 {
        "LOOKS GOOD"
    } else {
        "MINOR CONCERNS"
    };
    body.push_str(&format!(
        "**Overall: {} ({} finding(s), max severity: {})**\n\n---\n\n",
        verdict, stats.total_findings, stats.max_severity,
    ));

    // Detailed findings per pass
    for r in results {
        // Always include Summary; skip other passes with 0 findings
        if r.pass != ReviewPass::Summary && r.finding_count == 0 {
            continue;
        }
        if r.content.starts_with("Error:") {
            body.push_str(&format!("### {} (ERROR)\n\n{}\n\n", r.pass.label(), r.content));
            continue;
        }
        body.push_str(&format!("### {} Review\n\n{}\n\n", r.pass.label(), r.content));
    }

    // Footer with token usage
    body.push_str(&format!(
        "---\n*Tokens: {} in / {} out*\n",
        stats.total_input_tokens, stats.total_output_tokens,
    ));

    body
}

/// Maximum number of inline comments to post (avoid API spam).
const MAX_INLINE_COMMENTS: usize = 25;

/// Parse full file paths from diff headers (`+++ b/path/to/file`).
fn parse_diff_paths(diff: &str) -> Vec<String> {
    diff.lines()
        .filter_map(|line| {
            line.strip_prefix("+++ b/").map(|p| p.to_string())
        })
        .collect()
}

/// Resolve a potentially short filename (e.g., `AISettingsController.java`) to
/// the full path from the diff (e.g., `kaguya-api/.../AISettingsController.java`).
fn resolve_path(short: &str, diff_paths: &[String]) -> String {
    // If it already contains a `/`, assume it's a full (or partial) path.
    if short.contains('/') {
        // Still try to find an exact match or suffix match in diff paths.
        if let Some(full) = diff_paths.iter().find(|p| p.as_str() == short || p.ends_with(short)) {
            return full.clone();
        }
        return short.to_string();
    }

    // Simple filename — find the diff path that ends with this filename.
    let suffix = format!("/{}", short);
    if let Some(full) = diff_paths.iter().find(|p| p.ends_with(&suffix) || p.as_str() == short) {
        return full.clone();
    }

    // No match found; return as-is (may still work if the platform is lenient).
    short.to_string()
}

/// Extract inline review comments from pass results.
///
/// Uses [`dedup::deduplicate_findings`] to get structured findings with file/line
/// information, then converts each finding that has **both** a file path and a
/// line number into a [`platform::Comment`]. Findings without location info are
/// skipped (they remain in the overview body).
///
/// The `diff` parameter is used to resolve short filenames (e.g., `Foo.java`)
/// to the full paths present in the diff (e.g., `src/main/java/com/example/Foo.java`).
pub fn extract_inline_comments(results: &[PassResult], diff: &str) -> Vec<platform::Comment> {
    let findings = dedup::deduplicate_findings(results);
    let diff_paths = parse_diff_paths(diff);

    findings
        .into_iter()
        .filter_map(|f| {
            let short_path = f.file?;
            let line = f.line?;
            let path = resolve_path(&short_path, &diff_paths);
            let passes = f.passes.join(", ");
            let body = format!(
                "**[{}] {}: {}**\n\n{}",
                f.severity.label(),
                passes,
                f.title,
                f.content.trim(),
            );
            Some(platform::Comment {
                body,
                path: Some(path),
                line: Some(line),
            })
        })
        .take(MAX_INLINE_COMMENTS)
        .collect()
}

/// Build a brief summary body for the overview comment.
///
/// Contains ONLY: header, summary table, verdict, Summary pass content, and
/// token footer. Detailed findings from non-Summary passes are omitted — those
/// are posted as inline comments.
pub fn build_summary_body(
    results: &[PassResult],
    inline_count: usize,
    model: &str,
    context_label: &str,
) -> String {
    let stats = ReviewStats::from_results(results);
    let version = env!("CARGO_PKG_VERSION");

    let mut body = String::with_capacity(2048);

    // Header
    body.push_str(&format!(
        "## Byakugan AI Review\n\n\
         > **v{}** | model: `{}` | context: {}\n\n",
        version, model, context_label,
    ));

    // Summary table
    body.push_str("| Pass | Findings | Severity |\n|------|----------|----------|\n");
    for r in results {
        let findings = if r.finding_count == 0 {
            "No issues".to_string()
        } else {
            format!("{} finding(s)", r.finding_count)
        };
        body.push_str(&format!(
            "| {} | {} | {} |\n",
            r.pass.label(),
            findings,
            r.severity.label(),
        ));
    }
    body.push('\n');

    // Overall verdict
    let verdict = if stats.max_severity >= Severity::High {
        "NEEDS CHANGES"
    } else if stats.total_findings == 0 {
        "LOOKS GOOD"
    } else {
        "MINOR CONCERNS"
    };
    body.push_str(&format!(
        "**Overall: {} ({} finding(s), max severity: {})**\n\n",
        verdict, stats.total_findings, stats.max_severity,
    ));

    // Inline comments note
    if inline_count > 0 {
        body.push_str(&format!(
            "*{} inline comment(s) posted on specific files.*\n\n",
            inline_count,
        ));
    }

    body.push_str("---\n\n");

    // Only include the Summary pass content in the overview body.
    for r in results {
        if r.pass == ReviewPass::Summary && !r.content.starts_with("Error:") {
            body.push_str(&format!("### {} Review\n\n{}\n\n", r.pass.label(), r.content));
        }
    }

    // Footer with token usage
    body.push_str(&format!(
        "---\n*Tokens: {} in / {} out*\n",
        stats.total_input_tokens, stats.total_output_tokens,
    ));

    body
}

/// Compute aggregate statistics from a set of pass results.
pub struct ReviewStats {
    pub total_findings: usize,
    pub max_severity: Severity,
    pub total_input_tokens: u32,
    pub total_output_tokens: u32,
}

impl ReviewStats {
    pub fn from_results(results: &[PassResult]) -> Self {
        let total_findings: usize = results.iter().map(|r| r.finding_count).sum();
        let max_severity = results
            .iter()
            .map(|r| r.severity)
            .max()
            .unwrap_or(Severity::Ok);
        let total_input_tokens: u32 = results.iter().map(|r| r.input_tokens).sum();
        let total_output_tokens: u32 = results.iter().map(|r| r.output_tokens).sum();

        Self {
            total_findings,
            max_severity,
            total_input_tokens,
            total_output_tokens,
        }
    }
}
