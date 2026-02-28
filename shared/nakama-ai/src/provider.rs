//! The [`AiProvider`] trait and factory function.

use crate::anthropic::AnthropicProvider;
use crate::google::GoogleProvider;
use crate::ollama::OllamaProvider;
use crate::openai::OpenAIProvider;
use crate::types::{CompletionRequest, CompletionResponse};
use async_trait::async_trait;
use nakama_core::error::NakamaResult;
use nakama_core::types::Provider;

/// Unified interface for AI completion providers.
#[async_trait]
pub trait AiProvider: Send + Sync {
    /// Send a completion request and return the generated response.
    async fn complete(&self, request: CompletionRequest) -> NakamaResult<CompletionResponse>;

    /// Human-readable name of this provider (e.g. `"anthropic"`, `"openai"`).
    fn provider_name(&self) -> &str;
}

/// Create a boxed [`AiProvider`] for the given provider enum variant.
///
/// # Arguments
///
/// * `provider`  -- Which provider to instantiate.
/// * `api_key`   -- The API key (ignored for Ollama).
/// * `model`     -- The default model identifier.
/// * `base_url`  -- Optional base URL override (e.g. for proxies or self-hosted instances).
pub fn create_provider(
    provider: Provider,
    api_key: &str,
    model: &str,
    base_url: Option<&str>,
) -> NakamaResult<Box<dyn AiProvider>> {
    match provider {
        Provider::Anthropic => Ok(Box::new(AnthropicProvider::new(
            api_key,
            model,
            base_url,
        ))),
        Provider::OpenAI => Ok(Box::new(OpenAIProvider::new(api_key, model, base_url))),
        Provider::Google => Ok(Box::new(GoogleProvider::new(api_key, model, base_url))),
        Provider::Ollama => Ok(Box::new(OllamaProvider::new(
            model,
            base_url.unwrap_or("http://localhost:11434"),
        ))),
    }
}
