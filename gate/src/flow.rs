use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::{NakamaError, NakamaResult};
use nakama_core::trace::TraceContext;
use nakama_ui::NakamaUI;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Instant;

/// A single step in a flow configuration.
#[derive(Debug, Deserialize)]
struct FlowStep {
    /// A descriptive name for this step
    name: String,
    /// HTTP method (GET, POST, PUT, DELETE, PATCH)
    method: String,
    /// The URL to call
    url: String,
    /// Optional request headers
    #[serde(default)]
    headers: HashMap<String, String>,
    /// Optional request body (JSON)
    body: Option<serde_json::Value>,
}

/// Top-level flow configuration.
#[derive(Debug, Deserialize)]
struct FlowConfig {
    /// Name of the flow
    name: String,
    /// Optional description
    #[serde(default)]
    description: String,
    /// Ordered sequence of API calls
    steps: Vec<FlowStep>,
}

/// Run a sequence of API calls defined in a JSON config file.
pub async fn run(config: &Config, ui: &NakamaUI, config_file: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("gate", "flow");
    let start = Instant::now();

    // Read and parse the flow config
    let content = std::fs::read_to_string(config_file).map_err(|e| NakamaError::Tool {
        tool: "gate".to_string(),
        message: format!("Failed to read flow config '{}': {}", config_file, e),
    })?;

    let flow: FlowConfig = serde_json::from_str(&content).map_err(|e| NakamaError::Tool {
        tool: "gate".to_string(),
        message: format!("Failed to parse flow config: {}", e),
    })?;

    ui.panel(
        "Flow",
        &format!(
            "Name: {}\nDescription: {}\nSteps: {}",
            flow.name,
            if flow.description.is_empty() { "N/A" } else { &flow.description },
            flow.steps.len()
        ),
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("gate/0.1.0 (Nakama CLI Suite)")
        .build()
        .map_err(|e| NakamaError::Network {
            message: format!("Failed to create HTTP client: {}", e),
            source: Some(Box::new(e)),
        })?;

    let mut results: Vec<Vec<String>> = Vec::new();
    let mut all_success = true;

    for (i, step) in flow.steps.iter().enumerate() {
        let step_spinner = ui.step_start(&format!(
            "Step {}/{}: {} {} {}",
            i + 1,
            flow.steps.len(),
            step.name,
            step.method.to_uppercase(),
            step.url
        ));
        let step_start = Instant::now();

        let method = step.method.to_uppercase();
        let mut request = match method.as_str() {
            "GET" => client.get(&step.url),
            "POST" => client.post(&step.url),
            "PUT" => client.put(&step.url),
            "DELETE" => client.delete(&step.url),
            "PATCH" => client.patch(&step.url),
            "HEAD" => client.head(&step.url),
            other => {
                step_spinner.finish_with_error(&format!("Unsupported method: {}", other));
                results.push(vec![
                    format!("{}", i + 1),
                    step.name.clone(),
                    format!("{} {}", other, step.url),
                    "ERROR".to_string(),
                    "Unsupported method".to_string(),
                ]);
                all_success = false;
                continue;
            }
        };

        // Add headers
        for (key, value) in &step.headers {
            request = request.header(key.as_str(), value.as_str());
        }

        // Add body if present
        if let Some(body) = &step.body {
            request = request.json(body);
        }

        match request.send().await {
            Ok(response) => {
                let status = response.status();
                let step_elapsed = step_start.elapsed().as_millis();
                let body = response.text().await.unwrap_or_default();

                if status.is_success() {
                    step_spinner.finish_with_success(&format!(
                        "{} ({} ms, {} bytes)",
                        status,
                        step_elapsed,
                        body.len()
                    ));
                } else {
                    step_spinner.finish_with_error(&format!(
                        "{} ({} ms)",
                        status, step_elapsed
                    ));
                    all_success = false;
                }

                results.push(vec![
                    format!("{}", i + 1),
                    step.name.clone(),
                    format!("{} {}", method, step.url),
                    format!("{}", status.as_u16()),
                    format!("{} ms", step_elapsed),
                ]);
            }
            Err(e) => {
                step_spinner.finish_with_error(&format!("Request failed: {}", e));
                all_success = false;
                results.push(vec![
                    format!("{}", i + 1),
                    step.name.clone(),
                    format!("{} {}", method, step.url),
                    "ERROR".to_string(),
                    format!("{}", e),
                ]);
            }
        }
    }

    let elapsed = start.elapsed().as_millis() as u64;

    // Display summary table
    let headers = &["#", "Name", "Request", "Status", "Duration"];
    ui.table(headers, results);

    let outcome_label = if all_success { "All steps passed" } else { "Some steps failed" };
    ui.panel("Flow Summary", &format!("{} (total: {} ms)", outcome_label, elapsed));

    // Audit log
    if let Ok(audit) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id,
            "gate",
            "flow",
            Category::ExternalApi,
            &format!("Executed flow: {}", flow.name),
            serde_json::json!({
                "flow_name": flow.name,
                "config_file": config_file,
                "step_count": flow.steps.len(),
                "all_success": all_success,
            }),
            if all_success { Outcome::Success } else { Outcome::Failure },
            elapsed,
        );
        let _ = audit.log(entry);
    }

    Ok(())
}
