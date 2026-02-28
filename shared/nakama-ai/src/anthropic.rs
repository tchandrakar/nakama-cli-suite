//! Anthropic (Claude) provider implementation.

use crate::provider::AiProvider;
use crate::types::{CompletionRequest, CompletionResponse, Role, TokenUsage};
use async_trait::async_trait;
use nakama_core::error::{NakamaError, NakamaResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const DEFAULT_BASE_URL: &str = "https://api.anthropic.com";
const ANTHROPIC_VERSION: &str = "2023-06-01";

/// Anthropic Claude provider.
pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    default_model: String,
    base_url: String,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider.
    pub fn new(api_key: &str, model: &str, base_url: Option<&str>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            default_model: model.to_string(),
            base_url: base_url.unwrap_or(DEFAULT_BASE_URL).trim_end_matches('/').to_string(),
        }
    }
}

// --- Anthropic API request/response types ---

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
    model: String,
    usage: AnthropicUsage,
}

#[derive(Debug, Deserialize)]
struct AnthropicContent {
    text: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct AnthropicError {
    error: AnthropicErrorDetail,
}

#[derive(Debug, Deserialize)]
struct AnthropicErrorDetail {
    message: String,
}

#[async_trait]
impl AiProvider for AnthropicProvider {
    async fn complete(&self, request: CompletionRequest) -> NakamaResult<CompletionResponse> {
        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        // Build the messages list, filtering out system messages (sent separately).
        let messages: Vec<AnthropicMessage> = request
            .messages
            .iter()
            .filter(|m| m.role != Role::System)
            .map(|m| AnthropicMessage {
                role: match m.role {
                    Role::User => "user".to_string(),
                    Role::Assistant => "assistant".to_string(),
                    Role::System => "user".to_string(), // unreachable due to filter
                },
                content: m.content.clone(),
            })
            .collect();

        let system_prompt = if request.system_prompt.is_empty() {
            None
        } else {
            Some(request.system_prompt.clone())
        };

        let body = AnthropicRequest {
            model: model.to_string(),
            max_tokens: request.max_tokens,
            system: system_prompt,
            messages,
            temperature: Some(request.temperature),
        };

        let url = format!("{}/v1/messages", self.base_url);

        tracing::debug!(
            provider = "anthropic",
            model = %model,
            url = %url,
            "Sending completion request"
        );

        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| NakamaError::Network {
                message: format!("Anthropic API request failed: {e}"),
                source: Some(Box::new(e)),
            })?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| NakamaError::Network {
            message: format!("Failed to read Anthropic response body: {e}"),
            source: Some(Box::new(e)),
        })?;

        if !status.is_success() {
            let error_msg = serde_json::from_str::<AnthropicError>(&response_text)
                .map(|e| e.error.message)
                .unwrap_or_else(|_| response_text.clone());

            return Err(NakamaError::AiProvider {
                message: format!("Anthropic API error (HTTP {status}): {error_msg}"),
                source: None,
            });
        }

        let api_response: AnthropicResponse =
            serde_json::from_str(&response_text).map_err(|e| NakamaError::AiProvider {
                message: format!("Failed to parse Anthropic response: {e}"),
                source: Some(Box::new(e)),
            })?;

        let content = api_response
            .content
            .into_iter()
            .map(|c| c.text)
            .collect::<Vec<_>>()
            .join("");

        Ok(CompletionResponse {
            content,
            model: api_response.model,
            usage: TokenUsage {
                input_tokens: api_response.usage.input_tokens,
                output_tokens: api_response.usage.output_tokens,
            },
        })
    }

    fn provider_name(&self) -> &str {
        "anthropic"
    }
}
