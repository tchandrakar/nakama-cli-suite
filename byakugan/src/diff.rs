//! Single-file diff review.
//!
//! Reviews changes in a specific file using the multi-pass engine.

use crate::git;
use crate::review;
use anyhow::Result;
use nakama_ai::AiProvider;
use nakama_ui::NakamaUI;

/// Review changes to a single file.
///
/// Obtains the diff for the specified file (working tree or branch diff),
/// then runs all review passes against it.
pub async fn review_file(
    ui: &NakamaUI,
    provider: &dyn AiProvider,
    model: &str,
    file_path: &str,
) -> Result<()> {
    // Validate the file exists.
    git::validate_file_path(file_path)?;

    let spinner = ui.step_start(&format!("Collecting diff for {}...", file_path));

    let diff = git::get_file_diff(file_path)?;

    spinner.finish_with_success(&format!(
        "Diff collected for {} ({} chars)",
        file_path,
        diff.len()
    ));

    let context_label = format!("file: {}", file_path);
    let results = review::run_review(ui, provider, model, &diff, &context_label).await?;

    let stats = review::ReviewStats::from_results(&results);
    ui.panel(
        "File Review Complete",
        &format!(
            "File: {}\nTotal findings: {}\nHighest severity: {}\nTokens used: {} in / {} out",
            file_path,
            stats.total_findings,
            stats.max_severity,
            stats.total_input_tokens,
            stats.total_output_tokens,
        ),
    );

    Ok(())
}
