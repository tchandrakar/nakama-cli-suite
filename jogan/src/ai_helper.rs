use nakama_ai::{create_provider, AiProvider, CompletionRequest, Message};
use nakama_core::config::Config;
use nakama_core::error::{NakamaError, NakamaResult};
use nakama_core::types::{ModelTier, Provider};
use nakama_vault::{CredentialStore, Vault};

/// Resolve the provider name to env var key prefix for API keys.
fn provider_env_key(provider: &Provider) -> &'static str {
    match provider {
        Provider::Anthropic => "anthropic",
        Provider::OpenAI => "openai",
        Provider::Google => "google",
        Provider::Ollama => "ollama",
    }
}

/// Create an AI provider from config, resolving credentials from the vault.
pub fn make_provider(config: &Config, tier: ModelTier) -> NakamaResult<(Box<dyn AiProvider>, String)> {
    let provider = config.ai.default_provider.clone();
    let model = config.resolve_model(provider.clone(), tier);

    // Ollama doesn't need an API key
    if provider == Provider::Ollama {
        let p = create_provider(provider, "", &model, Some(&config.ai.ollama.base_url))?;
        return Ok((p, model));
    }

    let service = provider_env_key(&provider);

    // Try vault first, fall back to env var
    let api_key = match Vault::new() {
        Ok(vault) => match vault.retrieve(service, "api_key") {
            Ok(secret) => secret.expose_secret().to_string(),
            Err(_) => std::env::var(format!("{}_API_KEY", service.to_uppercase()))
                .map_err(|_| NakamaError::Auth {
                    message: format!(
                        "No API key found for {}. Set {}_API_KEY or run: nakama auth add --service {} --key YOUR_KEY",
                        service,
                        service.to_uppercase(),
                        service,
                    ),
                })?,
        },
        Err(_) => std::env::var(format!("{}_API_KEY", service.to_uppercase()))
            .map_err(|_| NakamaError::Auth {
                message: format!(
                    "No API key found. Set {}_API_KEY environment variable.",
                    service.to_uppercase(),
                ),
            })?,
    };

    // Resolve base URL override
    let base_url = match &provider {
        Provider::Anthropic => config.ai.anthropic.base_url.as_deref(),
        Provider::OpenAI => config.ai.openai.base_url.as_deref(),
        Provider::Google => config.ai.google.base_url.as_deref(),
        Provider::Ollama => Some(config.ai.ollama.base_url.as_str()),
    };

    let p = create_provider(provider, &api_key, &model, base_url)?;
    Ok((p, model))
}

/// Send a single-turn completion request.
pub async fn ask_ai(
    provider: &dyn AiProvider,
    system_prompt: &str,
    user_message: &str,
    model: &str,
    max_tokens: u32,
    temperature: f32,
) -> NakamaResult<String> {
    let request = CompletionRequest {
        system_prompt: system_prompt.to_string(),
        messages: vec![Message::user(user_message)],
        model: model.to_string(),
        max_tokens,
        temperature,
    };
    let response = provider.complete(request).await?;
    Ok(response.content)
}
