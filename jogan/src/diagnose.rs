//! Run system diagnostics and send results to AI for analysis.

use crate::ai_helper::{ask_ai, make_provider};
use crate::system::SystemInfo;
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::NakamaResult;
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Run system diagnostics, display results in a table, and send to AI for analysis.
pub async fn run(config: &Config, ui: &NakamaUI, symptom: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("jogan", "diagnose");
    let start = Instant::now();

    // Collect system information
    let spinner = ui.step_start("Collecting system diagnostics...");
    let info = SystemInfo::collect();
    spinner.finish_with_success("System diagnostics collected");

    // Display summary table
    let headers = &["Metric", "Value"];
    let rows = info.summary_rows();
    ui.table(headers, rows);

    // Send to AI for analysis
    let spinner = ui.step_start("Analyzing diagnostics with AI...");
    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Jogan, an infrastructure debugging assistant. The user reports a symptom and provides system diagnostics. Analyze the diagnostics data and:

1. **Diagnosis**: Identify the most likely root cause of the symptom.
2. **Evidence**: Point to specific metrics that support your diagnosis.
3. **Recommendations**: Suggest concrete steps to resolve the issue.
4. **Severity**: Rate the severity as LOW, MEDIUM, HIGH, or CRITICAL.

Be concise and actionable."#;

    let report = info.format_report();
    let user_message = format!(
        "Symptom: {}\n\nSystem Diagnostics:\n{}",
        symptom, report
    );

    let result = ask_ai(provider.as_ref(), system_prompt, &user_message, &model, 2048, 0.3).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Analysis complete");
            ui.panel(&format!("Diagnosis: {}", symptom), content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "jogan",
                    "diagnose",
                    Category::AiInteraction,
                    &format!("Diagnosed symptom: {}", truncate(symptom, 100)),
                    serde_json::json!({
                        "symptom": symptom,
                        "model": model,
                        "provider": provider.provider_name(),
                    }),
                    Outcome::Success,
                    elapsed,
                );
                let _ = audit.log(entry);
            }
        }
        Err(e) => {
            spinner.finish_with_error(&format!("Analysis failed: {}", e));

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "jogan",
                    "diagnose",
                    Category::AiInteraction,
                    &format!("Diagnosis failed for: {}", truncate(symptom, 100)),
                    serde_json::json!({ "symptom": symptom, "error": e.to_string() }),
                    Outcome::Failure,
                    elapsed,
                );
                let _ = audit.log(entry);
            }
        }
    }

    result.map(|_| ())
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        s
    } else {
        &s[..max]
    }
}
