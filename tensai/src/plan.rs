use crate::ai_helper::{ask_ai, make_provider};
use crate::git_info::{find_todos, GitInfo};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// AI-powered day planning.
pub async fn run(config: &Config, ui: &NakamaUI) -> NakamaResult<()> {
    let spinner = ui.step_start("Analyzing your priorities...");
    let _start = Instant::now();

    let git = GitInfo::collect().ok();
    let todos = find_todos();

    let git_summary = git.as_ref().map(|g| g.to_summary()).unwrap_or_default();
    let todo_summary = if todos.is_empty() {
        "No TODOs found.".to_string()
    } else {
        todos.iter().take(20).cloned().collect::<Vec<_>>().join("\n")
    };

    spinner.finish_with_success("Context gathered");

    let spinner = ui.step_start("Planning your day...");
    let (provider, model) = make_provider(config, ModelTier::Fast)?;

    let system_prompt = r#"You are Tensai, a productivity planner. Create a focused day plan.

Format:
## Today's Plan

### High Priority (do first)
1. Task — reasoning

### Medium Priority
1. Task — reasoning

### If Time Permits
1. Task — reasoning

### Time Blocks (suggested)
- 9:00-10:30: Deep work on [X]
- 10:30-11:00: Code review
- etc.

Base priorities on: current branch work, uncommitted changes, TODOs, and pending PRs.
Be specific and actionable."#;

    let user_msg = format!(
        "Plan my day.\n\nGit state:\n{}\n\nTODOs in codebase:\n{}",
        git_summary, todo_summary,
    );

    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 1536, 0.4).await;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Plan ready");
            ui.panel("Day Plan", content);
        }
        Err(e) => spinner.finish_with_error(&format!("Failed: {}", e)),
    }

    result.map(|_| ())
}
