//! The `chain` subcommand — generate multi-step command pipelines from a description.

use anyhow::Result;
use nakama_ai::{CompletionRequest, Message};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::TraceContext;
use nakama_ui::NakamaUI;
use std::time::Instant;

use crate::context::{build_context_prompt, ShellContext};
use crate::provider::create_ai_provider;
use crate::risk::{assess_risk, format_risk_display, RiskLevel};

/// Execute the `chain` subcommand.
pub async fn run(config: &Config, ui: &NakamaUI, query: &str) -> Result<()> {
    let trace = TraceContext::new("zangetsu", "chain");
    let ctx = ShellContext::collect();
    let start = Instant::now();

    let spinner = ui.step_start("Building command pipeline...");

    let system_prompt = build_chain_system_prompt(&ctx);
    let user_prompt = format!(
        "I need a multi-step command pipeline to: {}\n\n\
         Provide the pipeline as numbered steps. For each step use this format:\n\
         STEP <n>: <short description>\n\
         COMMAND: <the exact shell command>\n\
         EXPLANATION: <what this step does and why>\n\n\
         At the end, also provide a single combined pipeline if possible:\n\
         PIPELINE: <the full one-liner pipeline using pipes or &&>",
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
        max_tokens: 4096,
        temperature: 0.3,
    };

    let response = match provider.complete(request).await {
        Ok(resp) => {
            spinner.finish_with_success("Pipeline ready!");
            resp
        }
        Err(e) => {
            spinner.finish_with_error("AI request failed");
            log_audit(config, &trace, query, Outcome::Failure, start.elapsed().as_millis() as u64);
            return Err(anyhow::anyhow!("AI provider error: {}", e));
        }
    };

    // Parse the response
    let parsed = parse_chain_response(&response.content);

    // Build the step-by-step display
    let mut output = String::new();

    if !parsed.steps.is_empty() {
        output.push_str("Steps:\n\n");
        let mut highest_risk = RiskLevel::Low;

        for step in &parsed.steps {
            output.push_str(&format!("  Step {}: {}\n", step.number, step.description));
            output.push_str(&format!("    $ {}\n", step.command));
            if !step.explanation.is_empty() {
                output.push_str(&format!("    {}\n", step.explanation));
            }

            let risk = assess_risk(&step.command);
            if risk.level > highest_risk {
                highest_risk = risk.level;
            }
            output.push('\n');
        }

        // Show the combined pipeline if available
        if !parsed.pipeline.is_empty() {
            output.push_str("---\n\n");
            output.push_str("Combined pipeline:\n");
            output.push_str(&format!("  $ {}\n\n", parsed.pipeline));

            let pipeline_risk = assess_risk(&parsed.pipeline);
            if pipeline_risk.level > highest_risk {
                highest_risk = pipeline_risk.level;
            }
        }

        // Overall risk assessment
        let overall_risk = crate::risk::RiskAssessment {
            command: parsed.pipeline.clone(),
            level: highest_risk,
            reasons: vec![format!(
                "Highest risk across {} step(s)",
                parsed.steps.len()
            )],
        };
        output.push_str(&format_risk_display(&overall_risk));
    } else {
        // Fallback: show the raw response
        output.push_str(&response.content);
    }

    ui.panel("Command Pipeline", output.trim());

    // Log the interaction
    log_audit(config, &trace, query, Outcome::Success, start.elapsed().as_millis() as u64);

    Ok(())
}

/// Build the system prompt for the `chain` command.
fn build_chain_system_prompt(ctx: &ShellContext) -> String {
    let mut prompt = String::new();
    prompt.push_str(
        "You are Zangetsu, an expert shell pipeline builder. \
         Your job is to create multi-step command pipelines from natural-language descriptions.\n\n",
    );
    prompt.push_str("Rules:\n");
    prompt.push_str("1. Break complex tasks into clear, sequential steps.\n");
    prompt.push_str("2. Each step should be a single, runnable command.\n");
    prompt.push_str("3. Use pipes (|), logical AND (&&), or sequential execution (;) appropriately.\n");
    prompt.push_str("4. Prefer standard POSIX tools when possible.\n");
    prompt.push_str("5. Consider the user's OS and shell when choosing tools.\n");
    prompt.push_str("6. If possible, also provide a single combined pipeline.\n");
    prompt.push_str("7. Handle edge cases (empty output, missing files, etc.) gracefully.\n");
    prompt.push_str("8. Format response strictly with STEP, COMMAND, EXPLANATION, and PIPELINE markers.\n\n");
    prompt.push_str(&build_context_prompt(ctx));
    prompt
}

/// A parsed step from the AI response.
struct ChainStep {
    number: u32,
    description: String,
    command: String,
    explanation: String,
}

/// Parsed chain response.
struct ParsedChain {
    steps: Vec<ChainStep>,
    pipeline: String,
}

/// Parse the AI response to extract steps and pipeline.
fn parse_chain_response(response: &str) -> ParsedChain {
    let mut steps: Vec<ChainStep> = Vec::new();
    let mut pipeline = String::new();

    let mut current_step_num: Option<u32> = None;
    let mut current_description = String::new();
    let mut current_command = String::new();
    let mut current_explanation = String::new();

    for line in response.lines() {
        let trimmed = line.trim();

        // Check for PIPELINE: line
        if let Some(p) = trimmed.strip_prefix("PIPELINE:") {
            // Save any pending step
            if let Some(num) = current_step_num.take() {
                steps.push(ChainStep {
                    number: num,
                    description: current_description.trim().to_string(),
                    command: current_command.trim().to_string(),
                    explanation: current_explanation.trim().to_string(),
                });
                current_description.clear();
                current_command.clear();
                current_explanation.clear();
            }
            pipeline = p.trim().to_string();
            continue;
        }

        // Check for STEP N: pattern
        if let Some(rest) = trimmed.strip_prefix("STEP ") {
            // Save any pending step
            if let Some(num) = current_step_num.take() {
                steps.push(ChainStep {
                    number: num,
                    description: current_description.trim().to_string(),
                    command: current_command.trim().to_string(),
                    explanation: current_explanation.trim().to_string(),
                });
                current_description.clear();
                current_command.clear();
                current_explanation.clear();
            }

            // Parse "N: description"
            if let Some(colon_idx) = rest.find(':') {
                let num_str = &rest[..colon_idx];
                if let Ok(num) = num_str.trim().parse::<u32>() {
                    current_step_num = Some(num);
                    current_description = rest[colon_idx + 1..].trim().to_string();
                }
            }
            continue;
        }

        // Check for COMMAND: line
        if let Some(cmd) = trimmed.strip_prefix("COMMAND:") {
            current_command = cmd.trim().to_string();
            continue;
        }

        // Check for EXPLANATION: line
        if let Some(expl) = trimmed.strip_prefix("EXPLANATION:") {
            current_explanation = expl.trim().to_string();
            continue;
        }
    }

    // Save last pending step
    if let Some(num) = current_step_num {
        steps.push(ChainStep {
            number: num,
            description: current_description.trim().to_string(),
            command: current_command.trim().to_string(),
            explanation: current_explanation.trim().to_string(),
        });
    }

    // If no steps were parsed, try to extract commands from the raw response
    if steps.is_empty() {
        // Treat each line that looks like a command as a step
        let mut step_num = 1u32;
        for line in response.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('$') || trimmed.starts_with('#') {
                let cmd = trimmed.trim_start_matches(['$', '#', ' ']);
                if !cmd.is_empty() {
                    steps.push(ChainStep {
                        number: step_num,
                        description: String::new(),
                        command: cmd.to_string(),
                        explanation: String::new(),
                    });
                    step_num += 1;
                }
            }
        }
    }

    ParsedChain { steps, pipeline }
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
            "chain",
            Category::AiInteraction,
            &format!("Chain query: {}", query),
            serde_json::json!({ "query": query }),
            outcome,
            duration_ms,
        );
        if let Err(e) = audit.log(entry) {
            tracing::warn!("Failed to write audit log: {}", e);
        }
    }
}
