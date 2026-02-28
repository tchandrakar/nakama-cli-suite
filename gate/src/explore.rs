use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::{NakamaError, NakamaResult};
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Explore an API by sending a GET to the base URL and analyzing the response with AI.
pub async fn run(config: &Config, ui: &NakamaUI, base_url: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("gate", "explore");
    let spinner = ui.step_start(&format!("Exploring {}...", base_url));
    let start = Instant::now();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("gate/0.1.0 (Nakama CLI Suite)")
        .build()
        .map_err(|e| NakamaError::Network {
            message: format!("Failed to create HTTP client: {}", e),
            source: Some(Box::new(e)),
        })?;

    let response = client.get(base_url).send().await.map_err(|e| NakamaError::Network {
        message: format!("Failed to send request to {}: {}", base_url, e),
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

    // Display basic info
    ui.panel("Explore Target", &format!("URL: {}\nStatus: {}", base_url, status));

    // Display headers summary
    let header_lines: Vec<String> = headers
        .iter()
        .map(|(name, value)| {
            format!("{}: {}", name, value.to_str().unwrap_or("<binary>"))
        })
        .collect();
    ui.panel("Response Headers", &header_lines.join("\n"));

    // Try to pretty-print if JSON
    let body_preview = if body.len() > 3000 {
        format!("{}...\n\n[Truncated at 3000 chars, total: {} bytes]", &body[..3000], body.len())
    } else {
        body.clone()
    };

    let formatted = if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&body_preview) {
        serde_json::to_string_pretty(&json_val).unwrap_or(body_preview)
    } else {
        body_preview
    };
    ui.panel("Response Body", &formatted);

    // AI analysis of the API
    let ai_spinner = ui.step_start("Analyzing API with AI...");
    match make_provider(config, ModelTier::Fast) {
        Ok((provider, model)) => {
            let system_prompt = r#"You are Gate, an API exploration assistant. Analyze this API response and provide:

1. What kind of API this appears to be (REST, GraphQL, etc.)
2. Any discoverable endpoints or links found in the response
3. Authentication requirements if visible (API keys, OAuth, etc.)
4. Content type and data format
5. Suggestions for further exploration

Be concise and actionable."#;

            let user_msg = format!(
                "Explore this API:\nBase URL: {}\nStatus: {}\nHeaders: {}\nBody (first 2000 chars): {}",
                base_url,
                status,
                header_lines.join(", "),
                &body[..body.len().min(2000)]
            );

            match ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 1024, 0.3).await {
                Ok(analysis) => {
                    ai_spinner.finish_with_success("API analysis complete");
                    ui.panel("AI API Analysis", &analysis);
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
            "explore",
            Category::ExternalApi,
            &format!("Explored API: {}", base_url),
            serde_json::json!({
                "base_url": base_url,
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
