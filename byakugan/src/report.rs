//! `report` command — Combined AI review + rule scan.

use crate::analysis;
use crate::git;
use crate::output::{self, OutputFormat};
use crate::review;
use crate::rules;
use anyhow::Result;
use nakama_ai::AiProvider;
use nakama_core::config::ByakuganConfig;
use nakama_ui::NakamaUI;

/// Run a full report: AI review + custom rules scan.
pub async fn run_report(
    ui: &NakamaUI,
    provider: &dyn AiProvider,
    model: &str,
    byakugan_config: &ByakuganConfig,
    format: OutputFormat,
) -> Result<()> {
    let spinner = ui.step_start("Collecting diff...");

    // Try working-tree diff, fall back to branch diff.
    let (diff_text, context_label) = match git::get_working_diff() {
        Ok(diff) => (diff, "working tree changes".to_string()),
        Err(_) => {
            let branch = git::get_branch_diff()?;
            let label = format!("branch '{}' vs '{}'", branch.branch_name, branch.base_branch);
            (branch.diff_text, label)
        }
    };

    spinner.finish_with_success(&format!("Diff size: {} chars", diff_text.len()));

    // Run AI review.
    let results = review::run_review(ui, provider, model, &diff_text, &context_label).await?;

    // Run rule scan.
    let rule_findings = if !byakugan_config.rules.is_empty() {
        let spinner = ui.step_start("Running custom rules scan...");
        let compiled = rules::compile_rules(&byakugan_config.rules)?;
        let parsed = analysis::parse_unified_diff(&diff_text);
        let findings = rules::scan_diff(&compiled, &parsed);
        spinner.finish_with_success(&format!("{} rule violation(s)", findings.len()));
        findings
    } else {
        Vec::new()
    };

    // Output in requested format.
    match format {
        OutputFormat::Json => {
            println!("{}", output::format_json(&context_label, &results, &rule_findings));
        }
        OutputFormat::Markdown => {
            println!(
                "{}",
                output::format_markdown(&context_label, &results, &rule_findings)
            );
        }
        OutputFormat::Terminal => {
            // AI review already displayed by run_review.
            // Show rule findings if any.
            if !rule_findings.is_empty() {
                ui.panel("Rule Violations", &rules::format_findings(&rule_findings));
            }

            let stats = review::ReviewStats::from_results(&results);
            ui.panel(
                "Report Complete",
                &format!(
                    "Context: {}\nAI findings: {}\nRule violations: {}\nHighest severity: {}\nTokens: {} in / {} out",
                    context_label,
                    stats.total_findings,
                    rule_findings.len(),
                    stats.max_severity,
                    stats.total_input_tokens,
                    stats.total_output_tokens,
                ),
            );
        }
    }

    Ok(())
}
