//! Shared AI provider initialization and request helpers.

use anyhow::{Context, Result};
use nakama_ai::{create_provider, AiProvider, CompletionRequest, Message};
use nakama_core::config::Config;
use nakama_core::types::{ModelTier, Provider};
use nakama_vault::{CredentialStore, Vault};

/// Build an AI provider from the global config and vault credentials.
///
/// Resolves the API key from the vault using the provider name as the service
/// and `"api_key"` as the key.  For Ollama, no API key is required.
pub fn build_provider(config: &Config, tier: ModelTier) -> Result<Box<dyn AiProvider>> {
    let provider_enum = config.ai.default_provider;
    let model = config.resolve_model(provider_enum, tier);

    let (api_key, base_url) = match provider_enum {
        Provider::Ollama => {
            let url = config.ai.ollama.base_url.clone();
            (String::new(), Some(url))
        }
        _ => {
            let vault = Vault::new().context("Failed to initialize credential vault")?;
            let service = provider_enum.to_string();
            let secret = vault
                .retrieve(&service, "api_key")
                .with_context(|| {
                    format!(
                        "Failed to retrieve API key for provider '{}'. \
                         Store it with: nakama-vault store {} api_key <your-key>, \
                         or set NAKAMA_{}_API_KEY in your environment.",
                        service,
                        service,
                        service.to_uppercase()
                    )
                })?;
            let key = secret.expose_secret().to_string();

            // Resolve optional base_url override
            let url = match provider_enum {
                Provider::Anthropic => config.ai.anthropic.base_url.clone(),
                Provider::OpenAI => config.ai.openai.base_url.clone(),
                Provider::Google => config.ai.google.base_url.clone(),
                _ => None,
            };
            (key, url)
        }
    };

    let provider = create_provider(
        provider_enum,
        &api_key,
        &model,
        base_url.as_deref(),
    )
    .context("Failed to create AI provider")?;

    Ok(provider)
}

/// Send a single-turn completion request and return the response text.
pub async fn ask_ai(
    provider: &dyn AiProvider,
    system_prompt: &str,
    user_message: &str,
    model: &str,
    max_tokens: u32,
    temperature: f32,
) -> Result<String> {
    let request = CompletionRequest {
        system_prompt: system_prompt.to_string(),
        messages: vec![Message::user(user_message)],
        model: model.to_string(),
        max_tokens,
        temperature,
    };

    let response = provider
        .complete(request)
        .await
        .context("AI completion request failed")?;

    Ok(response.content)
}
