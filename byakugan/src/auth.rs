//! Platform authentication helpers.
//!
//! Convenience functions for storing and verifying platform tokens
//! using the nakama-vault credential store.

use anyhow::{Context, Result};
use nakama_vault::{CredentialStore, SecretValue, Vault};

/// Store a platform token in the vault.
pub fn store_token(platform: &str, token: &str) -> Result<()> {
    let vault = Vault::new().context("Failed to initialize vault")?;
    let key = format!("{}_token", platform);
    let secret = SecretValue::new(token.to_string());
    vault
        .store("nakama", &key, &secret)
        .context(format!("Failed to store {} token in vault", platform))?;
    Ok(())
}

/// Retrieve a platform token from the vault.
pub fn retrieve_token(platform: &str) -> Result<String> {
    let vault = Vault::new().context("Failed to initialize vault")?;
    let key = format!("{}_token", platform);
    let secret = vault
        .retrieve("nakama", &key)
        .context(format!("No {} token found in vault", platform))?;
    Ok(secret.expose_secret().to_string())
}

/// Check if a platform token exists in the vault.
pub fn has_token(platform: &str) -> bool {
    if let Ok(vault) = Vault::new() {
        let key = format!("{}_token", platform);
        vault.retrieve("nakama", &key).is_ok()
    } else {
        false
    }
}

/// Verify a platform token is valid by checking it's non-empty.
pub fn verify_token(platform: &str) -> Result<bool> {
    let token = retrieve_token(platform)?;
    Ok(!token.is_empty())
}

/// Get a summary of configured authentication.
pub fn auth_status() -> Vec<(String, bool)> {
    let platforms = ["github", "gitlab", "bitbucket"];
    platforms
        .iter()
        .map(|p| (p.to_string(), has_token(p)))
        .collect()
}
