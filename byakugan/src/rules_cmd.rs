//! `rules` subcommand — List, test, and validate rules from config.

use crate::rules;
use anyhow::Result;
use nakama_core::config::ByakuganRule;
use nakama_ui::NakamaUI;

/// List all configured rules.
pub fn cmd_list(ui: &NakamaUI, config_rules: &[ByakuganRule]) {
    if config_rules.is_empty() {
        ui.panel(
            "Rules",
            "No custom rules configured.\n\
             Add rules to ~/.nakama/config.toml under [[byakugan.rules]].\n\n\
             Example:\n\
             [[byakugan.rules]]\n\
             name = \"no-todo\"\n\
             description = \"Disallow TODO comments\"\n\
             severity = \"low\"\n\
             pattern = \"TODO|FIXME|HACK\"\n\
             exclude = [\"*.md\", \"CHANGELOG*\"]",
        );
        return;
    }

    let entries = rules::list_rules(config_rules);

    let headers = &["Name", "Severity", "Pattern", "Description"];
    let rows: Vec<Vec<String>> = entries
        .iter()
        .map(|(name, severity, pattern, desc)| {
            let truncated_pattern = if pattern.len() > 40 {
                format!("{}...", &pattern[..37])
            } else {
                pattern.clone()
            };
            vec![
                name.clone(),
                severity.clone(),
                truncated_pattern,
                desc.clone(),
            ]
        })
        .collect();

    ui.panel("Custom Rules", &format!("{} rule(s) configured", entries.len()));
    ui.table(headers, rows);
}

/// Validate all rule patterns.
pub fn cmd_validate(ui: &NakamaUI, config_rules: &[ByakuganRule]) {
    if config_rules.is_empty() {
        ui.panel("Validate", "No rules to validate.");
        return;
    }

    let errors = rules::validate_rules(config_rules);

    if errors.is_empty() {
        ui.success(&format!(
            "All {} rule(s) have valid regex patterns.",
            config_rules.len()
        ));
    } else {
        ui.error(&format!("{} rule(s) have invalid patterns:", errors.len()));
        for (name, err) in &errors {
            ui.panel(&format!("Invalid: {}", name), err);
        }
    }
}

/// Test a rule pattern against sample text.
pub fn cmd_test(ui: &NakamaUI, pattern: &str, sample: &str) -> Result<()> {
    match rules::test_rule(pattern, sample) {
        Ok(matches) => {
            if matches.is_empty() {
                ui.panel("Test Result", "No matches found.");
            } else {
                let mut output = format!("{} match(es) found:\n\n", matches.len());
                for (i, m) in matches.iter().enumerate() {
                    output.push_str(&format!("  {}. \"{}\"\n", i + 1, m));
                }
                ui.panel("Test Result", &output);
            }
        }
        Err(e) => {
            ui.error(&format!("Invalid pattern: {}", e));
        }
    }
    Ok(())
}
