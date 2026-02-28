use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::{NakamaError, NakamaResult};
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Test an API endpoint by sending an HTTP GET request and displaying the response.
pub async fn run(config: &Config, ui: &NakamaUI, url: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("gate", "test");
    let spinner = ui.step_start(&format!("Testing {}...", url));
    let start = Instant::now();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("gate/0.1.0 (Nakama CLI Suite)")
        .build()
        .map_err(|e| NakamaError::Network {
            message: format!("Failed to create HTTP client: {}", e),
            source: Some(Box::new(e)),
        })?;

    let response = client.get(url).send().await.map_err(|e| NakamaError::Network {
        message: format!("Failed to send request: {}", e),
        source: Some(Box::new(e)),
    })?;

    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await.map_err(|e| NakamaError::Network {
        message: format!("Failed to read response body: {}", e),
        source: Some(Box::new(e)),
    })?;

    let elapsed = start.elapsed().as_millis() as u64;
    spinner.finish_with_success(&format!("Response received ({} ms)", elapsed));

    // Display status
    let status_label = if status.is_success() {
        format!("Status: {} (OK)", status)
    } else if status.is_client_error() {
        format!("Status: {} (Client Error)", status)
    } else if status.is_server_error() {
        format!("Status: {} (Server Error)", status)
    } else {
        format!("Status: {}", status)
    };
    ui.panel("Response Status", &status_label);

    // Display headers
    let header_lines: Vec<String> = headers
        .iter()
        .map(|(name, value)| {
            format!("{}: {}", name, value.to_str().unwrap_or("<binary>"))
        })
        .collect();
    ui.panel("Response Headers", &header_lines.join("\n"));

    // Display body (truncate if very long)
    let display_body = if body.len() > 5000 {
        format!("{}...\n\n[Body truncated at 5000 characters, total: {} bytes]", &body[..5000], body.len())
    } else {
        body.clone()
    };

    // Try to pretty-print JSON
    let formatted_body = if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&display_body) {
        serde_json::to_string_pretty(&json_val).unwrap_or(display_body)
    } else {
        display_body
    };
    ui.panel("Response Body", &formatted_body);

    // Use AI to explain the response
    let ai_spinner = ui.step_start("Analyzing response with AI...");
    match make_provider(config, ModelTier::Fast) {
        Ok((provider, model)) => {
            let system_prompt = r#"You are Gate, an API analysis assistant. Explain the API response concisely.

Rules:
1. Summarize what the response contains.
2. Note the status code and what it means.
3. Highlight any interesting headers (auth, rate limiting, caching).
4. If the body is JSON, describe the data structure briefly.
5. Keep it to 3-5 sentences max."#;

            let user_msg = format!(
                "Analyze this API response:\nURL: {}\nStatus: {}\nHeaders: {}\nBody (first 2000 chars): {}",
                url,
                status,
                header_lines.join(", "),
                &body[..body.len().min(2000)]
            );

            match ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 512, 0.2).await {
                Ok(analysis) => {
                    ai_spinner.finish_with_success("Analysis complete");
                    ui.panel("AI Analysis", &analysis);
                }
                Err(e) => {
                    ai_spinner.finish_with_error(&format!("AI analysis failed: {}", e));
                }
            }
        }
        Err(e) => {
            ai_spinner.finish_with_error(&format!("AI unavailable: {}", e));
        }
    }

    // Audit log
    if let Ok(audit) = AuditLog::new(&config.audit) {
        let entry = AuditEntry::new(
            &trace.trace_id,
            "gate",
            "test",
            Category::ExternalApi,
            &format!("Tested endpoint: {}", url),
            serde_json::json!({
                "url": url,
                "status": status.as_u16(),
                "response_size": body.len(),
            }),
            if status.is_success() { Outcome::Success } else { Outcome::Failure },
            elapsed,
        );
        let _ = audit.log(entry);
    }

    Ok(())
}
