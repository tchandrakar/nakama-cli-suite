use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::{NakamaError, NakamaResult};
use nakama_core::trace::TraceContext;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Read a JSON or YAML spec file and list all endpoints found.
pub async fn run(config: &Config, ui: &NakamaUI, spec_path: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("gate", "mock");
    let spinner = ui.step_start(&format!("Reading spec file: {}...", spec_path));
    let start = Instant::now();

    let content = std::fs::read_to_string(spec_path).map_err(|e| NakamaError::Tool {
        tool: "gate".to_string(),
        message: format!("Failed to read spec file '{}': {}", spec_path, e),
    })?;

    // Try to parse as JSON
    let spec: serde_json::Value = serde_json::from_str(&content).map_err(|e| NakamaError::Tool {
        tool: "gate".to_string(),
        message: format!("Failed to parse spec file as JSON: {}", e),
    })?;

    spinner.finish_with_success("Spec file loaded");

    // Extract endpoints from an OpenAPI-style spec
    let mut endpoints: Vec<(String, String)> = Vec::new();

    if let Some(paths) = spec.get("paths").and_then(|p| p.as_object()) {
        for (path, methods) in paths {
            if let Some(methods_obj) = methods.as_object() {
                for (method, detail) in methods_obj {
                    let summary = detail
                        .get("summary")
                        .and_then(|s| s.as_str())
                        .or_else(|| detail.get("description").and_then(|s| s.as_str()))
                        .unwrap_or("No description");
                    endpoints.push((
                        format!("{} {}", method.to_uppercase(), path),
                        summary.to_string(),
                    ));
                }
            }
        }
    }

    // Also check for a flat array of endpoints (custom format)
    if endpoints.is_empty() {
        if let Some(eps) = spec.get("endpoints").and_then(|e| e.as_array()) {
            for ep in eps {
                let method = ep.get("method").and_then(|m| m.as_str()).unwrap_or("GET");
                let path = ep.get("path").and_then(|p| p.as_str()).unwrap_or("/");
                let desc = ep
                    .get("description")
                    .and_then(|d| d.as_str())
                    .unwrap_or("No description");
                endpoints.push((format!("{} {}", method.to_uppercase(), path), desc.to_string()));
            }
        }
    }

    let elapsed = start.elapsed().as_millis() as u64;

    if endpoints.is_empty() {
        ui.warn("No endpoints found in the spec file.");
        ui.panel(
            "Spec Info",
            &format!(
                "File: {}\nFormat: JSON\nNo recognizable endpoint definitions found.\nExpected 'paths' (OpenAPI) or 'endpoints' (custom) key.",
                spec_path
            ),
        );
    } else {
        // Display spec info
        let title = spec
            .get("info")
            .and_then(|i| i.get("title"))
            .and_then(|t| t.as_str())
            .unwrap_or("Unknown API");
        let version = spec
            .get("info")
            .and_then(|i| i.get("version"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        ui.panel(
            "API Specification",
            &format!("Title: {}\nVersion: {}\nFile: {}\nEndpoints: {}", title, version, spec_path, endpoints.len()),
        );

        // Display endpoints as a table
        let headers = &["Endpoint", "Description"];
        let rows: Vec<Vec<String>> = endpoints
            .iter()
            .map(|(ep, desc)| vec![ep.clone(), desc.clone()])
            .collect();
        ui.table(headers, rows);
    }

    // Audit log
    if let Ok(audit) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id,
            "gate",
            "mock",
            Category::ToolExecution,
            &format!("Listed endpoints from spec: {}", spec_path),
            serde_json::json!({
                "spec_path": spec_path,
                "endpoint_count": endpoints.len(),
            }),
            Outcome::Success,
            elapsed,
        );
        let _ = audit.log(entry);
    }

    Ok(())
}
