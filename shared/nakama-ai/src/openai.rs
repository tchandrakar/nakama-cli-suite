//! OpenAI provider implementation.

use crate::provider::AiProvider;
use crate::types::{CompletionRequest, CompletionResponse, Role, TokenUsage};
use async_trait::async_trait;
use nakama_core::error::{NakamaError, NakamaResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const DEFAULT_BASE_URL: &str = "https://api.openai.com";

/// OpenAI provider (GPT models).
pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    default_model: String,
    base_url: String,
}

impl OpenAIProvider {
    /// Create a new OpenAI provider.
    pub fn new(api_key: &str, model: &str, base_url: Option<&str>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            default_model: model.to_string(),
            base_url: base_url.unwrap_or(DEFAULT_BASE_URL).trim_end_matches('/').to_string(),
        }
    }
}

// --- OpenAI API request/response types ---

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
    model: String,
    usage: Option<OpenAIUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIResponseMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponseMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct OpenAIError {
    error: OpenAIErrorDetail,
}

#[derive(Debug, Deserialize)]
struct OpenAIErrorDetail {
    message: String,
}

#[async_trait]
impl AiProvider for OpenAIProvider {
    async fn complete(&self, request: CompletionRequest) -> NakamaResult<CompletionResponse> {
        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        // Build message list -- OpenAI expects system messages inline.
        let mut messages = Vec::new();

        if !request.system_prompt.is_empty() {
            messages.push(OpenAIMessage {
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
            messages.push(OpenAIMessage {
                role: role.to_string(),
                content: msg.content.clone(),
            });
        }

        let body = OpenAIRequest {
            model: model.to_string(),
            messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
        };

        let url = format!("{}/v1/chat/completions", self.base_url);

        tracing::debug!(
            provider = "openai",
            model = %model,
            url = %url,
            "Sending completion request"
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| NakamaError::Network {
                message: format!("OpenAI API request failed: {e}"),
                source: Some(Box::new(e)),
            })?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| NakamaError::Network {
            message: format!("Failed to read OpenAI response body: {e}"),
            source: Some(Box::new(e)),
        })?;

        if !status.is_success() {
            let error_msg = serde_json::from_str::<OpenAIError>(&response_text)
                .map(|e| e.error.message)
                .unwrap_or_else(|_| response_text.clone());

            return Err(NakamaError::AiProvider {
                message: format!("OpenAI API error (HTTP {status}): {error_msg}"),
                source: None,
            });
        }

        let api_response: OpenAIResponse =
            serde_json::from_str(&response_text).map_err(|e| NakamaError::AiProvider {
                message: format!("Failed to parse OpenAI response: {e}"),
                source: Some(Box::new(e)),
            })?;

        let content = api_response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();

        let usage = api_response.usage.as_ref();

        Ok(CompletionResponse {
            content,
            model: api_response.model,
            usage: TokenUsage {
                input_tokens: usage.map(|u| u.prompt_tokens).unwrap_or(0),
                output_tokens: usage.map(|u| u.completion_tokens).unwrap_or(0),
            },
        })
    }

    fn provider_name(&self) -> &str {
        "openai"
    }
}
