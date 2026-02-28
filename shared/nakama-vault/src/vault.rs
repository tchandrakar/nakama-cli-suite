use crate::encrypted_file::EncryptedFileBackend;
use crate::env_fallback::EnvBackend;
use crate::keychain::KeychainBackend;
use crate::secret::SecretValue;
use nakama_core::error::{NakamaError, NakamaResult};
use tracing::{debug, warn};

/// Trait for credential storage backends.
///
/// Implementations must be able to store, retrieve, delete, and list
/// credentials organized by service and key.
pub trait CredentialStore {
    /// Store a secret value under the given service and key.
    fn store(&self, service: &str, key: &str, value: &SecretValue) -> NakamaResult<()>;

    /// Retrieve a secret value for the given service and key.
    fn retrieve(&self, service: &str, key: &str) -> NakamaResult<SecretValue>;

    /// Delete a stored secret for the given service and key.
    fn delete(&self, service: &str, key: &str) -> NakamaResult<()>;

    /// List all keys stored under the given service.
    fn list_keys(&self, service: &str) -> NakamaResult<Vec<String>>;
}

/// The primary credential vault that tries backends in priority order:
/// 1. OS keychain (most secure)
/// 2. Encrypted file store (portable fallback)
/// 3. Environment variables (read-only, last resort)
pub struct Vault {
    backends: Vec<(&'static str, Box<dyn CredentialStore>)>,
}

impl Vault {
    /// Create a new `Vault`, auto-detecting the best available backend.
    ///
    /// Backends are tried in priority order: keychain, encrypted file, env vars.
    /// The vault always includes all backends so that retrieval can fall through
    /// to env vars even when the keychain is available.
    pub fn new() -> NakamaResult<Self> {
        let mut backends: Vec<(&'static str, Box<dyn CredentialStore>)> = Vec::new();

        // Try keychain backend (may fail if no keychain daemon is running)
        match KeychainBackend::new() {
            Ok(kb) => {
                debug!("Keychain backend available");
                backends.push(("keychain", Box::new(kb)));
            }
            Err(e) => {
                debug!("Keychain backend not available: {}", e);
            }
        }

        // Encrypted file backend (should always work)
        match EncryptedFileBackend::new() {
            Ok(fb) => {
                debug!("Encrypted file backend available");
                backends.push(("encrypted_file", Box::new(fb)));
            }
            Err(e) => {
                warn!("Encrypted file backend not available: {}", e);
            }
        }

        // Env fallback is always available
        backends.push(("env", Box::new(EnvBackend::new())));
        debug!("Environment variable fallback backend registered");

        Ok(Self { backends })
    }

    /// Return the name of the highest-priority backend currently available.
    pub fn primary_backend_name(&self) -> &'static str {
        self.backends
            .first()
            .map(|(name, _)| *name)
            .unwrap_or("none")
    }
}

impl CredentialStore for Vault {
    fn store(&self, service: &str, key: &str, value: &SecretValue) -> NakamaResult<()> {
        // Store in the highest-priority writable backend
        for (name, backend) in &self.backends {
            match backend.store(service, key, value) {
                Ok(()) => {
                    debug!("Stored credential {}/{} in {} backend", service, key, name);
                    return Ok(());
                }
                Err(e) => {
                    debug!(
                        "Failed to store in {} backend, trying next: {}",
                        name, e
                    );
                }
            }
        }

        Err(NakamaError::Vault {
            message: format!(
                "Failed to store credential {}/{} in any backend",
                service, key
            ),
            source: None,
        })
    }

    fn retrieve(&self, service: &str, key: &str) -> NakamaResult<SecretValue> {
        // Try each backend in priority order
        for (name, backend) in &self.backends {
            match backend.retrieve(service, key) {
                Ok(value) => {
                    debug!(
                        "Retrieved credential {}/{} from {} backend",
                        service, key, name
                    );
                    return Ok(value);
                }
                Err(e) => {
                    debug!(
                        "Could not retrieve from {} backend, trying next: {}",
                        name, e
                    );
                }
            }
        }

        Err(NakamaError::Vault {
            message: format!(
                "Credential {}/{} not found in any backend",
                service, key
            ),
            source: None,
        })
    }

    fn delete(&self, service: &str, key: &str) -> NakamaResult<()> {
        let mut deleted = false;

        // Delete from all backends that have it
        for (name, backend) in &self.backends {
            match backend.delete(service, key) {
                Ok(()) => {
                    debug!(
                        "Deleted credential {}/{} from {} backend",
                        service, key, name
                    );
                    deleted = true;
                }
                Err(e) => {
                    debug!(
                        "Could not delete from {} backend: {}",
                        name, e
                    );
                }
            }
        }

        if deleted {
            Ok(())
        } else {
            Err(NakamaError::Vault {
                message: format!(
                    "Credential {}/{} not found in any backend for deletion",
                    service, key
                ),
                source: None,
            })
        }
    }

    fn list_keys(&self, service: &str) -> NakamaResult<Vec<String>> {
        let mut all_keys = Vec::new();

        for (name, backend) in &self.backends {
            match backend.list_keys(service) {
                Ok(keys) => {
                    debug!(
                        "Listed {} keys from {} backend for service {}",
                        keys.len(),
                        name,
                        service
                    );
                    for key in keys {
                        if !all_keys.contains(&key) {
                            all_keys.push(key);
                        }
                    }
                }
                Err(e) => {
                    debug!(
                        "Could not list keys from {} backend: {}",
                        name, e
                    );
                }
            }
        }

        Ok(all_keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_creation() {
        // Vault::new() should succeed even if keychain is unavailable
        let vault = Vault::new();
        assert!(vault.is_ok());
    }

    #[test]
    fn test_vault_has_env_backend() {
        let vault = Vault::new().unwrap();
        // At minimum, the env backend should always be present
        assert!(!vault.backends.is_empty());
    }
}
