//! Google Gemini provider implementation.

use crate::provider::AiProvider;
use crate::types::{CompletionRequest, CompletionResponse, Role, TokenUsage};
use async_trait::async_trait;
use nakama_core::error::{NakamaError, NakamaResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const DEFAULT_BASE_URL: &str = "https://generativelanguage.googleapis.com";

/// Google Gemini provider.
pub struct GoogleProvider {
    client: Client,
    api_key: String,
    default_model: String,
    base_url: String,
}

impl GoogleProvider {
    /// Create a new Google Gemini provider.
    pub fn new(api_key: &str, model: &str, base_url: Option<&str>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            default_model: model.to_string(),
            base_url: base_url.unwrap_or(DEFAULT_BASE_URL).trim_end_matches('/').to_string(),
        }
    }
}

// --- Google Gemini API request/response types ---

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GeminiGenerationConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiPart {
    text: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiGenerationConfig {
    max_output_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
    usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiUsageMetadata {
    prompt_token_count: Option<u32>,
    candidates_token_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct GeminiError {
    error: GeminiErrorDetail,
}

#[derive(Debug, Deserialize)]
struct GeminiErrorDetail {
    message: String,
}

#[async_trait]
impl AiProvider for GoogleProvider {
    async fn complete(&self, request: CompletionRequest) -> NakamaResult<CompletionResponse> {
        let model = if request.model.is_empty() {
            &self.default_model
        } else {
            &request.model
        };

        // Build conversation contents.
        let contents: Vec<GeminiContent> = request
            .messages
            .iter()
            .filter(|m| m.role != Role::System)
            .map(|m| GeminiContent {
                role: Some(match m.role {
                    Role::User => "user".to_string(),
                    Role::Assistant => "model".to_string(),
                    Role::System => "user".to_string(), // unreachable due to filter
                }),
                parts: vec![GeminiPart {
                    text: m.content.clone(),
                }],
            })
            .collect();

        let system_instruction = if request.system_prompt.is_empty() {
            None
        } else {
            Some(GeminiContent {
                role: None,
                parts: vec![GeminiPart {
                    text: request.system_prompt.clone(),
                }],
            })
        };

        let body = GeminiRequest {
            contents,
            system_instruction,
            generation_config: Some(GeminiGenerationConfig {
                max_output_tokens: request.max_tokens,
                temperature: request.temperature,
            }),
        };

        let url = format!(
            "{}/v1beta/models/{}:generateContent?key={}",
            self.base_url, model, self.api_key
        );

        tracing::debug!(
            provider = "google",
            model = %model,
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
                message: format!("Google Gemini API request failed: {e}"),
                source: Some(Box::new(e)),
            })?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| NakamaError::Network {
            message: format!("Failed to read Google Gemini response body: {e}"),
            source: Some(Box::new(e)),
        })?;

        if !status.is_success() {
            let error_msg = serde_json::from_str::<GeminiError>(&response_text)
                .map(|e| e.error.message)
                .unwrap_or_else(|_| response_text.clone());

            return Err(NakamaError::AiProvider {
                message: format!("Google Gemini API error (HTTP {status}): {error_msg}"),
                source: None,
            });
        }

        let api_response: GeminiResponse =
            serde_json::from_str(&response_text).map_err(|e| NakamaError::AiProvider {
                message: format!("Failed to parse Google Gemini response: {e}"),
                source: Some(Box::new(e)),
            })?;

        let content = api_response
            .candidates
            .as_ref()
            .and_then(|c| c.first())
            .map(|candidate| {
                candidate
                    .content
                    .parts
                    .iter()
                    .map(|p| p.text.as_str())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .unwrap_or_default();

        let usage_meta = api_response.usage_metadata.as_ref();

        Ok(CompletionResponse {
            content,
            model: model.to_string(),
            usage: TokenUsage {
                input_tokens: usage_meta.and_then(|u| u.prompt_token_count).unwrap_or(0),
                output_tokens: usage_meta
                    .and_then(|u| u.candidates_token_count)
                    .unwrap_or(0),
            },
        })
    }

    fn provider_name(&self) -> &str {
        "google"
    }
}
