//! Improvement suggestions for current working-tree changes.
//!
//! Instead of the full multi-pass review, this command runs a single
//! focused AI call that produces actionable improvement suggestions.

use crate::git;
use anyhow::{Context, Result};
use nakama_ai::types::{CompletionRequest, Message};
use nakama_ai::AiProvider;
use nakama_ui::NakamaUI;

const SUGGEST_SYSTEM_PROMPT: &str = "\
You are a senior software engineer reviewing uncommitted changes. \
Your goal is to suggest concrete, actionable improvements. \
For each suggestion, provide:\n\
1. A one-line title\n\
2. Where in the code it applies (file/function/line context)\n\
3. The current code pattern\n\
4. Your suggested improvement\n\
5. Why it is better (one sentence)\n\n\
Focus on the most impactful improvements. Aim for 3-7 suggestions. \
Order them by impact (highest first). \
If the code is already well-written, say so and suggest only minor polish.\n\
Do not repeat the entire diff back. Be concise.";

/// Maximum diff chars for the suggest command.
const MAX_DIFF_CHARS: usize = 50_000;

/// Suggest improvements for current uncommitted changes.
pub async fn suggest_improvements(
    ui: &NakamaUI,
    provider: &dyn AiProvider,
    model: &str,
) -> Result<()> {
    let spinner = ui.step_start("Collecting working-tree changes...");

    // Try working-tree diff first; fall back to branch diff.
    let diff = match git::get_working_diff() {
        Ok(d) => {
            spinner.finish_with_success(&format!(
                "Working-tree diff collected ({} chars)",
                d.len()
            ));
            d
        }
        Err(_) => {
            // No uncommitted changes, use branch diff instead.
            let branch_diff = git::get_branch_diff()
                .context("No uncommitted changes and no branch diff found.")?;
            spinner.finish_with_success(&format!(
                "Branch diff collected ({} -> {}, {} chars)",
                branch_diff.branch_name, branch_diff.base_branch, branch_diff.diff_text.len()
            ));
            branch_diff.diff_text
        }
    };

    let truncated = nakama_core::diff::compress_diff(&diff, MAX_DIFF_CHARS);

    let ai_spinner = ui.step_start("Generating improvement suggestions...");

    let user_message = format!(
        "Please suggest improvements for the following code changes:\n\n```diff\n{}\n```",
        truncated
    );

    let request = CompletionRequest {
        system_prompt: SUGGEST_SYSTEM_PROMPT.to_string(),
        messages: vec![Message::user(user_message)],
        model: model.to_string(),
        max_tokens: 2048,
        temperature: 0.3,
    };

    let response = provider
        .complete(request)
        .await
        .context("AI suggestion generation failed")?;

    ai_spinner.finish_with_success(&format!(
        "Suggestions generated ({}/{} tokens)",
        response.usage.input_tokens, response.usage.output_tokens
    ));

    ui.panel("Improvement Suggestions", &response.content);

    Ok(())
}
