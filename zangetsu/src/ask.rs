//! The `ask` subcommand — translate a natural-language question into a shell command.

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

/// Execute the `ask` subcommand.
pub async fn run(config: &Config, ui: &NakamaUI, query: &str) -> Result<()> {
    let trace = TraceContext::new("zangetsu", "ask");
    let ctx = ShellContext::collect();
    let start = Instant::now();

    let spinner = ui.step_start("Thinking...");

    let system_prompt = build_ask_system_prompt(&ctx);
    let user_prompt = format!(
        "I want to: {}\n\nProvide ONLY the shell command(s) needed. \
         Format your response as:\n\
         COMMAND: <the exact command to run>\n\
         EXPLANATION: <one-line explanation of what it does>\n\
         If multiple commands are needed, provide each on its own COMMAND: line.",
        query
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
            spinner.finish_with_success("Got it!");
            resp
        }
        Err(e) => {
            spinner.finish_with_error("AI request failed");
            log_audit(config, &trace, query, Outcome::Failure, start.elapsed().as_millis() as u64);
            return Err(anyhow::anyhow!("AI provider error: {}", e));
        }
    };

    // Parse the response to extract commands and explanations
    let parsed = parse_ask_response(&response.content);

    // Build the display output
    let mut display = String::new();
    for (i, entry) in parsed.iter().enumerate() {
        if parsed.len() > 1 {
            display.push_str(&format!("Step {}:\n", i + 1));
        }
        display.push_str(&format!("  $ {}\n", entry.command));
        if !entry.explanation.is_empty() {
            display.push_str(&format!("  {}\n", entry.explanation));
        }

        // Risk assessment
        let risk = assess_risk(&entry.command);
        display.push('\n');
        display.push_str(&format_risk_display(&risk));
        display.push('\n');
    }

    ui.panel("Command Suggestion", display.trim());

    // Log the interaction
    log_audit(config, &trace, query, Outcome::Success, start.elapsed().as_millis() as u64);

    Ok(())
}

/// Build the system prompt for the `ask` command.
fn build_ask_system_prompt(ctx: &ShellContext) -> String {
    let mut prompt = String::new();
    prompt.push_str(
        "You are Zangetsu, an expert shell command assistant. \
         Your job is to translate natural-language requests into precise shell commands.\n\n",
    );
    prompt.push_str("Rules:\n");
    prompt.push_str("1. Output commands appropriate for the user's OS and shell.\n");
    prompt.push_str("2. Prefer standard, widely-available tools.\n");
    prompt.push_str("3. Use safe defaults (e.g., prefer interactive/confirmation flags when available).\n");
    prompt.push_str("4. If a task requires multiple steps, provide each as a separate COMMAND: line.\n");
    prompt.push_str("5. NEVER fabricate flags or options that do not exist.\n");
    prompt.push_str("6. Format response strictly as COMMAND: and EXPLANATION: lines.\n\n");
    prompt.push_str(&build_context_prompt(ctx));
    prompt
}

/// A parsed command entry from the AI response.
struct ParsedCommand {
    command: String,
    explanation: String,
}

/// Parse the AI response to extract COMMAND: and EXPLANATION: entries.
fn parse_ask_response(response: &str) -> Vec<ParsedCommand> {
    let mut entries = Vec::new();
    let mut current_command: Option<String> = None;
    let mut current_explanation = String::new();

    for line in response.lines() {
        let trimmed = line.trim();

        if let Some(cmd) = trimmed.strip_prefix("COMMAND:") {
            // Save previous entry if any
            if let Some(cmd_text) = current_command.take() {
                entries.push(ParsedCommand {
                    command: cmd_text,
                    explanation: current_explanation.trim().to_string(),
                });
                current_explanation.clear();
            }
            current_command = Some(cmd.trim().to_string());
        } else if let Some(expl) = trimmed.strip_prefix("EXPLANATION:") {
            current_explanation = expl.trim().to_string();
        }
    }

    // Save last entry
    if let Some(cmd_text) = current_command {
        entries.push(ParsedCommand {
            command: cmd_text,
            explanation: current_explanation.trim().to_string(),
        });
    }

    // If we couldn't parse the structured format, treat the whole response as one command
    if entries.is_empty() {
        // Try to extract a command from code blocks
        let content = response.trim();
        if let Some(code) = extract_code_block(content) {
            entries.push(ParsedCommand {
                command: code,
                explanation: String::new(),
            });
        } else {
            // Fall back to using the first line as the command
            let first_line = content.lines().next().unwrap_or(content);
            entries.push(ParsedCommand {
                command: first_line.trim().to_string(),
                explanation: if content.lines().count() > 1 {
                    content
                        .lines()
                        .skip(1)
                        .collect::<Vec<_>>()
                        .join("\n")
                        .trim()
                        .to_string()
                } else {
                    String::new()
                },
            });
        }
    }

    entries
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
    query: &str,
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
            "ask",
            Category::AiInteraction,
            &format!("Ask query: {}", query),
            serde_json::json!({ "query": query }),
            outcome,
            duration_ms,
        );
        if let Err(e) = audit.log(entry) {
            tracing::warn!("Failed to write audit log: {}", e);
        }
    }
}
