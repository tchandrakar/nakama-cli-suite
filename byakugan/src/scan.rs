//! `scan` command — Run custom rules against a local diff without AI.

use crate::analysis;
use crate::git;
use crate::output::{self, OutputFormat};
use crate::rules;
use anyhow::Result;
use nakama_core::config::ByakuganRule;
use nakama_ui::NakamaUI;

/// Run a scan of custom rules against the current diff.
pub async fn run_scan(
    ui: &NakamaUI,
    config_rules: &[ByakuganRule],
    format: OutputFormat,
) -> Result<()> {
    if config_rules.is_empty() {
        ui.panel(
            "Scan",
            "No custom rules configured.\n\
             Add rules to ~/.nakama/config.toml under [byakugan.rules].",
        );
        return Ok(());
    }

    let spinner = ui.step_start("Compiling rules...");
    let compiled = rules::compile_rules(config_rules)?;
    spinner.finish_with_success(&format!("{} rule(s) compiled", compiled.len()));

    let spinner = ui.step_start("Collecting diff...");

    // Try working-tree diff first, fall back to branch diff.
    let diff_text = git::get_working_diff()
        .or_else(|_| git::get_branch_diff().map(|d| d.diff_text))?;

    spinner.finish_with_success(&format!("Diff size: {} chars", diff_text.len()));

    let spinner = ui.step_start("Scanning for rule violations...");
    let parsed = analysis::parse_unified_diff(&diff_text);
    let findings = rules::scan_diff(&compiled, &parsed);
    spinner.finish_with_success(&format!("{} violation(s) found", findings.len()));

    match format {
        OutputFormat::Json => {
            println!("{}", output::format_scan_json(&findings));
        }
        OutputFormat::Markdown => {
            let md = if findings.is_empty() {
                "# Scan Report\n\nNo rule violations found.\n".to_string()
            } else {
                let mut md = "# Scan Report\n\n".to_string();
                md.push_str(&rules::format_findings(&findings));
                md
            };
            println!("{}", md);
        }
        OutputFormat::Terminal => {
            if findings.is_empty() {
                ui.success("No rule violations found.");
            } else {
                ui.panel("Rule Violations", &rules::format_findings(&findings));

                // Summary table.
                let headers = &["Rule", "Severity", "File", "Line"];
                let rows: Vec<Vec<String>> = findings
                    .iter()
                    .map(|f| {
                        vec![
                            f.rule_name.clone(),
                            f.severity.label().to_string(),
                            f.file.clone(),
                            f.line.to_string(),
                        ]
                    })
                    .collect();
                ui.table(headers, rows);
            }
        }
    }

    Ok(())
}
