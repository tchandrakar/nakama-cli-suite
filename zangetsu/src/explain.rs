//! The `explain` subcommand — explain what a shell command does in plain English.

use anyhow::Result;
use nakama_ai::{CompletionRequest, Message};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::TraceContext;
use nakama_ui::NakamaUI;
use std::time::Instant;

use crate::context::{build_context_prompt, ShellContext};
use crate::provider::create_ai_provider;
use crate::risk::{assess_risk, format_risk_display};

/// Execute the `explain` subcommand.
pub async fn run(config: &Config, ui: &NakamaUI, command: &str) -> Result<()> {
    let trace = TraceContext::new("zangetsu", "explain");
    let ctx = ShellContext::collect();
    let start = Instant::now();

    let spinner = ui.step_start("Analyzing command...");

    let system_prompt = build_explain_system_prompt(&ctx);
    let user_prompt = format!(
        "Explain this shell command in plain English:\n\n```\n{}\n```\n\n\
         Break it down part by part. For each component (command, flag, argument, pipe, redirect), \
         explain what it does. Then give a one-sentence summary at the end.",
        command
    );

    let provider = create_ai_provider(config)?;
    let model = config.resolve_model(
        config.ai.default_provider,
        nakama_core::types::ModelTier::Balanced,
    );

    let request = CompletionRequest {
        system_prompt: system_prompt.clone(),
        messages: vec![Message::user(&user_prompt)],
        model,
        max_tokens: 2048,
        temperature: 0.3,
    };

    let response = match provider.complete(request).await {
        Ok(resp) => {
            spinner.finish_with_success("Analysis complete!");
            resp
        }
        Err(e) => {
            spinner.finish_with_error("AI request failed");
            log_audit(config, &trace, command, Outcome::Failure, start.elapsed().as_millis() as u64);
            return Err(anyhow::anyhow!("AI provider error: {}", e));
        }
    };

    // Show the command being explained
    let mut output = String::new();
    output.push_str(&format!("Command: {}\n\n", command));

    // Risk assessment
    let risk = assess_risk(command);
    output.push_str(&format_risk_display(&risk));
    output.push_str("\n---\n\n");

    // AI explanation
    output.push_str(&response.content);

    ui.panel("Command Explanation", output.trim());

    // Log the interaction
    log_audit(config, &trace, command, Outcome::Success, start.elapsed().as_millis() as u64);

    Ok(())
}

/// Build the system prompt for the `explain` command.
fn build_explain_system_prompt(ctx: &ShellContext) -> String {
    let mut prompt = String::new();
    prompt.push_str(
        "You are Zangetsu, an expert shell command explainer. \
         Your job is to break down shell commands and explain them in clear, plain English.\n\n",
    );
    prompt.push_str("Rules:\n");
    prompt.push_str("1. Break down each component: the base command, flags/options, arguments, pipes, and redirections.\n");
    prompt.push_str("2. Explain what each part does individually.\n");
    prompt.push_str("3. Use simple language accessible to beginners.\n");
    prompt.push_str("4. Mention any important side effects or caveats.\n");
    prompt.push_str("5. End with a one-sentence overall summary.\n");
    prompt.push_str("6. If the command is dangerous, clearly warn about the risks.\n\n");
    prompt.push_str(&build_context_prompt(ctx));
    prompt
}

/// Log the interaction to the audit database.
fn log_audit(
    config: &Config,
    trace: &TraceContext,
    command: &str,
    outcome: Outcome,
    duration_ms: u64,
) {
    if !config.audit.enabled {
        return;
    }
    if let Ok(audit) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id,
            "zangetsu",
            "explain",
            Category::AiInteraction,
            &format!("Explain command: {}", command),
            serde_json::json!({ "command": command }),
            outcome,
            duration_ms,
        );
        if let Err(e) = audit.log(entry) {
            tracing::warn!("Failed to write audit log: {}", e);
        }
    }
}
