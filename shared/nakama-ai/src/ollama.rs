//! Ollama (local models) provider implementation.

use crate::provider::AiProvider;
use crate::types::{CompletionRequest, CompletionResponse, Role, TokenUsage};
use async_trait::async_trait;
use nakama_core::error::{NakamaError, NakamaResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Ollama local model provider.
pub struct OllamaProvider {
    client: Client,
    default_model: String,
    base_url: String,
}

impl OllamaProvider {
    /// Create a new Ollama provider.
    ///
    /// No API key is required since Ollama runs locally.
    pub fn new(model: &str, base_url: &str) -> Self {
        Self {
            client: Client::new(),
            default_model: model.to_string(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }
}

// --- Ollama API request/response types ---

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    message: OllamaResponseMessage,
    model: String,
    #[serde(default)]
    prompt_eval_count: Option<u32>,
    #[serde(default)]
    eval_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct OllamaResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OllamaError {
    error: String,
}

#[async_trait]
impl AiProvider for OllamaProvider {
    async fn complete(&self, request: CompletionRequest) -> NakamaResult<CompletionResponse> {
        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        // Build message list -- Ollama supports system role directly.
        let mut messages = Vec::new();

        if !request.system_prompt.is_empty() {
            messages.push(OllamaMessage {
                role: "system".to_string(),
                content: request.system_prompt.clone(),
            });
        }

        for msg in &request.messages {
            let role = match msg.role {
                Role::System => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
            };
            messages.push(OllamaMessage {
                role: role.to_string(),
                content: msg.content.clone(),
            });
        }

        let body = OllamaRequest {
            model: model.to_string(),
            messages,
            stream: false,
            options: Some(OllamaOptions {
                temperature: Some(request.temperature),
                num_predict: Some(request.max_tokens),
            }),
        };

        let url = format!("{}/api/chat", self.base_url);

        tracing::debug!(
            provider = "ollama",
            model = %model,
            url = %url,
            "Sending completion request"
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| NakamaError::Network {
                message: format!("Ollama API request failed: {e}"),
                source: Some(Box::new(e)),
            })?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| NakamaError::Network {
            message: format!("Failed to read Ollama response body: {e}"),
            source: Some(Box::new(e)),
        })?;

        if !status.is_success() {
            let error_msg = serde_json::from_str::<OllamaError>(&response_text)
                .map(|e| e.error)
                .unwrap_or_else(|_| response_text.clone());

            return Err(NakamaError::AiProvider {
                message: format!("Ollama API error (HTTP {status}): {error_msg}"),
                source: None,
            });
        }

        let api_response: OllamaResponse =
            serde_json::from_str(&response_text).map_err(|e| NakamaError::AiProvider {
                message: format!("Failed to parse Ollama response: {e}"),
                source: Some(Box::new(e)),
            })?;

        Ok(CompletionResponse {
            content: api_response.message.content,
            model: api_response.model,
            usage: TokenUsage {
                input_tokens: api_response.prompt_eval_count.unwrap_or(0),
                output_tokens: api_response.eval_count.unwrap_or(0),
            },
        })
    }

    fn provider_name(&self) -> &str {
        "ollama"
    }
}
