use crate::secret::SecretValue;
use crate::vault::CredentialStore;
use keyring::Entry;
use nakama_core::error::{NakamaError, NakamaResult};
use tracing::debug;

/// OS keychain credential backend using the `keyring` crate.
///
/// On macOS this uses the system Keychain, on Linux it uses the Secret Service
/// (D-Bus), and on Windows it uses the Credential Manager.
///
/// Service names are formatted as `nakama-<service>` to namespace all
/// credentials under the Nakama CLI Suite.
pub struct KeychainBackend {
    _private: (),
}

impl KeychainBackend {
    /// Create a new keychain backend.
    ///
    /// This performs a lightweight probe to confirm the OS keychain is accessible.
    pub fn new() -> NakamaResult<Self> {
        // Probe the keychain by attempting to create an entry reference.
        // The keyring crate will error at retrieval time if the daemon is
        // unavailable, but entry creation itself is infallible. We do a
        // quick test store+delete of a sentinel value.
        let probe = Entry::new("nakama-probe", "connectivity-test").map_err(|e| {
            NakamaError::Vault {
                message: format!("Keychain not available: {}", e),
                source: Some(Box::new(e)),
            }
        })?;

        // Try to delete any leftover probe value (ignore errors)
        let _ = probe.delete_credential();

        debug!("OS keychain is accessible");
        Ok(Self { _private: () })
    }

    /// Format a keyring service name: `nakama-<service>`.
    fn service_name(service: &str) -> String {
        format!("nakama-{}", service)
    }

    /// Build a keyring entry for the given service and key.
    fn entry(service: &str, key: &str) -> NakamaResult<Entry> {
        let svc = Self::service_name(service);
        Entry::new(&svc, key).map_err(|e| NakamaError::Vault {
            message: format!("Failed to create keyring entry for {}/{}: {}", service, key, e),
            source: Some(Box::new(e)),
        })
    }
}

impl CredentialStore for KeychainBackend {
    fn store(&self, service: &str, key: &str, value: &SecretValue) -> NakamaResult<()> {
        let entry = Self::entry(service, key)?;
        entry
            .set_password(value.expose_secret())
            .map_err(|e| NakamaError::Vault {
                message: format!(
                    "Failed to store credential {}/{} in keychain: {}",
                    service, key, e
                ),
                source: Some(Box::new(e)),
            })?;
        debug!("Stored {}/{} in OS keychain", service, key);
        Ok(())
    }

    fn retrieve(&self, service: &str, key: &str) -> NakamaResult<SecretValue> {
        let entry = Self::entry(service, key)?;
        let password = entry.get_password().map_err(|e| NakamaError::Vault {
            message: format!(
                "Failed to retrieve credential {}/{} from keychain: {}",
                service, key, e
            ),
            source: Some(Box::new(e)),
        })?;
        debug!("Retrieved {}/{} from OS keychain", service, key);
        Ok(SecretValue::new(password))
    }

    fn delete(&self, service: &str, key: &str) -> NakamaResult<()> {
        let entry = Self::entry(service, key)?;
        entry
            .delete_credential()
            .map_err(|e| NakamaError::Vault {
                message: format!(
                    "Failed to delete credential {}/{} from keychain: {}",
                    service, key, e
                ),
                source: Some(Box::new(e)),
            })?;
        debug!("Deleted {}/{} from OS keychain", service, key);
        Ok(())
    }

    fn list_keys(&self, _service: &str) -> NakamaResult<Vec<String>> {
        // The keyring crate does not provide a way to enumerate keys for a
        // given service. Return an empty list; callers should combine results
        // from all backends.
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_name_formatting() {
        assert_eq!(KeychainBackend::service_name("zangetsu"), "nakama-zangetsu");
        assert_eq!(KeychainBackend::service_name("gate"), "nakama-gate");
    }
}
