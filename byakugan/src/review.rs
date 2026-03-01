//! The core multi-pass AI review engine.
//!
//! Runs each [`ReviewPass`] against a diff, collects results, and formats
//! them for display. This module is provider-agnostic — it receives a
//! `Box<dyn AiProvider>` and uses it for all passes.

use crate::git;
use crate::passes::{parse_findings, PassResult, ReviewPass, Severity};
use anyhow::{Context, Result};
use nakama_ai::types::{CompletionRequest, Message};
use nakama_ai::AiProvider;
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
    run_review_with_passes(ui, provider, model, diff, context_label, &[]).await
}

/// Run review with configurable passes.
pub async fn run_review_with_passes(
    ui: &NakamaUI,
    provider: &dyn AiProvider,
    model: &str,
    diff: &str,
    context_label: &str,
    pass_names: &[String],
) -> Result<Vec<PassResult>> {
    let truncated_diff = git::truncate_diff(diff, MAX_DIFF_CHARS);

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
        let result = run_single_pass(provider, model, &truncated_diff, pass).await;
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
) -> Result<PassResult> {
    let user_message = format!(
        "Please review the following code diff:\n\n```diff\n{}\n```",
        diff
    );

    let request = CompletionRequest {
        system_prompt: pass.system_prompt().to_string(),
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
