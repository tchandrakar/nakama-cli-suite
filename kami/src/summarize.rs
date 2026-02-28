use crate::ai_helper::{ask_ai, make_provider};
use nakama_audit::{AuditEntry, AuditLog, Category, Outcome};
use nakama_core::config::Config;
use nakama_core::error::{NakamaError, NakamaResult};
use nakama_core::trace::TraceContext;
use nakama_core::types::ModelTier;
use nakama_ui::NakamaUI;
use std::time::Instant;

/// Fetch a URL's content and summarize it using AI.
pub async fn run(config: &Config, ui: &NakamaUI, url: &str) -> NakamaResult<()> {
    let trace = TraceContext::new("kami", "summarize");
    let spinner = ui.step_start(&format!("Fetching {}...", url));
    let start = Instant::now();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("kami/0.1.0 (Nakama CLI Suite)")
        .build()
        .map_err(|e| NakamaError::Network {
            message: format!("Failed to create HTTP client: {}", e),
            source: Some(Box::new(e)),
        })?;

    let response = client.get(url).send().await.map_err(|e| NakamaError::Network {
        message: format!("Failed to fetch URL: {}", e),
        source: Some(Box::new(e)),
    })?;

    if !response.status().is_success() {
        spinner.finish_with_error(&format!("HTTP {}", response.status()));
        return Err(NakamaError::Network {
            message: format!("HTTP error: {}", response.status()),
            source: None,
        });
    }

    let body = response.text().await.map_err(|e| NakamaError::Network {
        message: format!("Failed to read response body: {}", e),
        source: Some(Box::new(e)),
    })?;

    spinner.finish_with_success("Content fetched");

    // Strip HTML tags
    let text = strip_html(&body);

    // Truncate to reasonable size for AI context
    let truncated = if text.len() > 12000 {
        format!("{}...\n\n[Content truncated at 12000 characters]", &text[..12000])
    } else {
        text
    };

    let spinner = ui.step_start("Summarizing...");

    let (provider, model) = make_provider(config, ModelTier::Balanced)?;

    let system_prompt = r#"You are Kami, a content summarization assistant. The user will provide text content from a URL.
Provide a structured summary:

## Summary
A concise 2-3 paragraph summary of the content.

## Key Takeaways
- The most important points as bullet items.

## Topics Covered
- List of main topics/sections covered.

Be concise and accurate. Focus on the most important information."#;

    let user_msg = format!("Please summarize this content from {}:\n\n{}", url, truncated);
    let result = ask_ai(provider.as_ref(), system_prompt, &user_msg, &model, 2048, 0.3).await;

    let elapsed = start.elapsed().as_millis() as u64;

    match &result {
        Ok(content) => {
            spinner.finish_with_success("Summary complete");
            ui.panel(&format!("Summary: {}", url), content);

            if let Ok(audit) = AuditLog::new(&config.audit) {
                let entry = AuditEntry::new(
                    &trace.trace_id,
                    "kami",
                    "summarize",
                    Category::ExternalApi,
                    &format!("Summarized URL: {}", url),
                    serde_json::json!({
                        "url": url,
                        "content_length": body.len(),
                        "model": model,
                    }),
                    Outcome::Success,
                    elapsed,
                );
                let _ = audit.log(entry);
            }
        }
        Err(e) => {
            spinner.finish_with_error(&format!("Summarization failed: {}", e));
        }
    }

    result.map(|_| ())
}

/// Basic HTML tag stripping.
fn strip_html(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;

    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }

    // Decode common HTML entities and collapse whitespace
    result
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
