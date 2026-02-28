//! The `fix` subcommand — reads the last failed command from shell history and suggests a fix.

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

/// Execute the `fix` subcommand.
pub async fn run(config: &Config, ui: &NakamaUI) -> Result<()> {
    let trace = TraceContext::new("zangetsu", "fix");
    let ctx = ShellContext::collect();
    let start = Instant::now();

    // Get the last command from history
    let last_command = match ctx.recent_history.last() {
        Some(cmd) => cmd.clone(),
        None => {
            ui.step_fail("Could not read shell history. No recent commands found.");
            return Err(anyhow::anyhow!(
                "No recent commands found in shell history. \
                 Make sure your shell history file is accessible."
            ));
        }
    };

    ui.step_done(&format!("Last command: {}", last_command));

    let spinner = ui.step_start("Diagnosing and suggesting fix...");

    let system_prompt = build_fix_system_prompt(&ctx);
    let user_prompt = format!(
        "The following shell command was run and likely failed:\n\n\
         ```\n{}\n```\n\n\
         The user's recent command history (newest last) is:\n{}\n\n\
         Diagnose what likely went wrong and suggest a corrected command.\n\n\
         Format your response as:\n\
         DIAGNOSIS: <what likely went wrong>\n\
         FIX: <the corrected command>\n\
         EXPLANATION: <why this fix should work>",
        last_command,
        ctx.recent_history
            .iter()
            .map(|c| format!("  - `{}`", c))
            .collect::<Vec<_>>()
            .join("\n"),
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
            spinner.finish_with_success("Fix found!");
            resp
        }
        Err(e) => {
            spinner.finish_with_error("AI request failed");
            log_audit(
                config,
                &trace,
                &last_command,
                Outcome::Failure,
                start.elapsed().as_millis() as u64,
            );
            return Err(anyhow::anyhow!("AI provider error: {}", e));
        }
    };

    // Parse the response
    let parsed = parse_fix_response(&response.content);

    // Build display output
    let mut output = String::new();
    output.push_str(&format!("Failed command:\n  $ {}\n\n", last_command));

    if !parsed.diagnosis.is_empty() {
        output.push_str(&format!("Diagnosis:\n  {}\n\n", parsed.diagnosis));
    }

    if !parsed.fix.is_empty() {
        output.push_str(&format!("Suggested fix:\n  $ {}\n\n", parsed.fix));

        // Risk assessment for the suggested fix
        let risk = assess_risk(&parsed.fix);
        output.push_str(&format_risk_display(&risk));
        output.push('\n');
    }

    if !parsed.explanation.is_empty() {
        output.push_str(&format!("Why:\n  {}\n", parsed.explanation));
    }

    ui.panel("Command Fix", output.trim());

    // Log the interaction
    log_audit(
        config,
        &trace,
        &last_command,
        Outcome::Success,
        start.elapsed().as_millis() as u64,
    );

    Ok(())
}

/// Build the system prompt for the `fix` command.
fn build_fix_system_prompt(ctx: &ShellContext) -> String {
    let mut prompt = String::new();
    prompt.push_str(
        "You are Zangetsu, an expert shell command debugger. \
         Your job is to diagnose failed shell commands and suggest corrected versions.\n\n",
    );
    prompt.push_str("Rules:\n");
    prompt.push_str("1. Analyze the command to identify common failure modes (typos, wrong flags, missing arguments, permission issues, etc.).\n");
    prompt.push_str("2. Consider the user's OS, shell, and working directory.\n");
    prompt.push_str("3. Suggest the SIMPLEST fix that addresses the most likely cause.\n");
    prompt.push_str("4. If the command seems correct but might fail for environmental reasons, suggest diagnostic steps.\n");
    prompt.push_str("5. Format response strictly as DIAGNOSIS:, FIX:, and EXPLANATION: lines.\n");
    prompt.push_str("6. Provide only ONE corrected command in the FIX: line.\n\n");
    prompt.push_str(&build_context_prompt(ctx));
    prompt
}

/// Parsed fix response from the AI.
struct ParsedFix {
    diagnosis: String,
    fix: String,
    explanation: String,
}

/// Parse the AI response to extract DIAGNOSIS:, FIX:, and EXPLANATION: entries.
fn parse_fix_response(response: &str) -> ParsedFix {
    let mut diagnosis = String::new();
    let mut fix = String::new();
    let mut explanation = String::new();
    let mut current_section: Option<&str> = None;

    for line in response.lines() {
        let trimmed = line.trim();

        if let Some(d) = trimmed.strip_prefix("DIAGNOSIS:") {
            diagnosis = d.trim().to_string();
            current_section = Some("diagnosis");
        } else if let Some(f) = trimmed.strip_prefix("FIX:") {
            fix = f.trim().to_string();
            current_section = Some("fix");
        } else if let Some(e) = trimmed.strip_prefix("EXPLANATION:") {
            explanation = e.trim().to_string();
            current_section = Some("explanation");
        } else if !trimmed.is_empty() {
            // Append continuation lines to the current section
            match current_section {
                Some("diagnosis") => {
                    diagnosis.push(' ');
                    diagnosis.push_str(trimmed);
                }
                Some("explanation") => {
                    explanation.push(' ');
                    explanation.push_str(trimmed);
                }
                _ => {}
            }
        }
    }

    // If structured parsing failed, use the whole response
    if fix.is_empty() && diagnosis.is_empty() {
        // Try to find a code block
        if let Some(code) = extract_code_block(response) {
            fix = code;
            // The rest is the explanation
            let without_code = response
                .replace(&format!("```\n{}\n```", fix), "")
                .replace(&format!("```bash\n{}\n```", fix), "")
                .replace(&format!("```sh\n{}\n```", fix), "");
            explanation = without_code.trim().to_string();
        } else {
            explanation = response.trim().to_string();
        }
    }

    ParsedFix {
        diagnosis,
        fix,
        explanation,
    }
}

/// Extract the content of the first code block from markdown text.
fn extract_code_block(text: &str) -> Option<String> {
    let start_markers = ["```bash", "```sh", "```shell", "```zsh", "```"];
    for marker in &start_markers {
        if let Some(start_idx) = text.find(marker) {
            let code_start = start_idx + marker.len();
            if let Some(end_idx) = text[code_start..].find("```") {
                let code = text[code_start..code_start + end_idx].trim();
                if !code.is_empty() {
                    return Some(code.to_string());
                }
            }
        }
    }
    None
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
            "fix",
            Category::AiInteraction,
            &format!("Fix command: {}", command),
            serde_json::json!({ "failed_command": command }),
            outcome,
            duration_ms,
        );
        if let Err(e) = audit.log(entry) {
            tracing::warn!("Failed to write audit log: {}", e);
        }
    }
}
