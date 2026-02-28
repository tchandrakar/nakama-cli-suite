//! AI provider creation helper.
//!
//! Centralizes the logic for loading credentials from the vault and creating
//! an AI provider instance from the configuration.

use anyhow::Result;
use nakama_ai::{create_provider as ai_create_provider, AiProvider};
use nakama_core::config::Config;
use nakama_core::types::Provider;
use nakama_vault::{CredentialStore, Vault};

/// Create an AI provider from the global config and vault credentials.
///
/// Resolves the API key by looking up `<provider>/api_key` in the credential
/// vault. For Ollama (local), the API key is set to an empty string since
/// no authentication is required.
pub fn create_ai_provider(config: &Config) -> Result<Box<dyn AiProvider>> {
    let provider_type = config.ai.default_provider;

    let api_key = if provider_type == Provider::Ollama {
        String::new()
    } else {
        resolve_api_key(provider_type)?
    };

    let model = config.resolve_model(
        provider_type,
        nakama_core::types::ModelTier::Balanced,
    );

    let base_url = match provider_type {
        Provider::Anthropic => config.ai.anthropic.base_url.as_deref(),
        Provider::OpenAI => config.ai.openai.base_url.as_deref(),
        Provider::Google => config.ai.google.base_url.as_deref(),
        Provider::Ollama => Some(config.ai.ollama.base_url.as_str()),
    };

    let provider = ai_create_provider(provider_type, &api_key, &model, base_url)
        .map_err(|e| anyhow::anyhow!("Failed to create AI provider: {}", e))?;

    Ok(provider)
}

/// Resolve the API key for the given provider from the vault.
///
/// The vault lookup key follows the convention:
///   service = provider name (e.g., "anthropic", "openai", "google")
///   key     = "api_key"
///
/// Env var fallback is handled automatically by the vault's EnvBackend,
/// which checks for `NAKAMA_<PROVIDER>_API_KEY`.
fn resolve_api_key(provider: Provider) -> Result<String> {
    let vault = Vault::new()
        .map_err(|e| anyhow::anyhow!("Failed to initialize credential vault: {}", e))?;

    let service = provider.to_string();
    let secret = vault.retrieve(&service, "api_key").map_err(|_| {
        anyhow::anyhow!(
            "No API key found for provider '{}'. \
             Set it with one of:\n  \
             1. Store in vault: nakama vault set {} api_key <key>\n  \
             2. Environment variable: export NAKAMA_{}_API_KEY=<key>",
            service,
            service,
            service.to_uppercase()
        )
    })?;

    Ok(secret.expose_secret().to_string())
}
