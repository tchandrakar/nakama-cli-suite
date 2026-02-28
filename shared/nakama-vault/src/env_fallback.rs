use crate::secret::SecretValue;
use crate::vault::CredentialStore;
use nakama_core::error::{NakamaError, NakamaResult};
use tracing::warn;

/// Environment variable fallback credential backend.
///
/// Reads credentials from environment variables named
/// `NAKAMA_<SERVICE>_<KEY>` (uppercased, hyphens replaced with underscores).
///
/// This is a read-only backend: `store` and `delete` are no-ops since
/// environment variables cannot be persistently written from within a process.
pub struct EnvBackend {
    _private: (),
}

impl EnvBackend {
    /// Create a new environment variable backend.
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Convert a service/key pair to an environment variable name.
    ///
    /// Format: `NAKAMA_<SERVICE>_<KEY>` with hyphens replaced by underscores
    /// and everything uppercased.
    ///
    /// Examples:
    /// - `("zangetsu", "api-key")` -> `NAKAMA_ZANGETSU_API_KEY`
    /// - `("gate", "token")` -> `NAKAMA_GATE_TOKEN`
    fn env_var_name(service: &str, key: &str) -> String {
        format!(
            "NAKAMA_{}_{}",
            service.to_uppercase().replace('-', "_"),
            key.to_uppercase().replace('-', "_")
        )
    }
}

impl CredentialStore for EnvBackend {
    fn store(&self, service: &str, key: &str, _value: &SecretValue) -> NakamaResult<()> {
        warn!(
            "Cannot persistently store credentials via environment variables. \
             Set {} manually in your shell profile to persist.",
            Self::env_var_name(service, key)
        );
        // No-op: env vars are read-only from the application's perspective.
        Ok(())
    }

    fn retrieve(&self, service: &str, key: &str) -> NakamaResult<SecretValue> {
        let var_name = Self::env_var_name(service, key);
        match std::env::var(&var_name) {
            Ok(value) => {
                warn!(
                    "Reading credential {}/{} from environment variable {}. \
                     Consider using the keychain or encrypted file backend for better security.",
                    service, key, var_name
                );
                Ok(SecretValue::new(value))
            }
            Err(_) => Err(NakamaError::Vault {
                message: format!(
                    "Environment variable {} not set for credential {}/{}",
                    var_name, service, key
                ),
                source: None,
            }),
        }
    }

    fn delete(&self, service: &str, key: &str) -> NakamaResult<()> {
        warn!(
            "Cannot delete environment variable {}. \
             Remove it manually from your shell profile.",
            Self::env_var_name(service, key)
        );
        // No-op: env vars are read-only from the application's perspective.
        Ok(())
    }

    fn list_keys(&self, service: &str) -> NakamaResult<Vec<String>> {
        let prefix = format!(
            "NAKAMA_{}_",
            service.to_uppercase().replace('-', "_")
        );

        let keys: Vec<String> = std::env::vars()
            .filter_map(|(name, _)| {
                if name.starts_with(&prefix) {
                    // Strip the prefix and convert back to lowercase with hyphens
                    let key_part = &name[prefix.len()..];
                    Some(key_part.to_lowercase().replace('_', "-"))
                } else {
                    None
                }
            })
            .collect();

        if !keys.is_empty() {
            warn!(
                "Found {} credential(s) for service {} via environment variables. \
                 Consider migrating to the keychain for better security.",
                keys.len(),
                service
            );
        }

        Ok(keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_var_name() {
        assert_eq!(
            EnvBackend::env_var_name("zangetsu", "api-key"),
            "NAKAMA_ZANGETSU_API_KEY"
        );
        assert_eq!(
            EnvBackend::env_var_name("gate", "token"),
            "NAKAMA_GATE_TOKEN"
        );
        assert_eq!(
            EnvBackend::env_var_name("my-service", "my-secret-key"),
            "NAKAMA_MY_SERVICE_MY_SECRET_KEY"
        );
    }

    #[test]
    fn test_retrieve_from_env() {
        // Set a test env var
        std::env::set_var("NAKAMA_TEST_SVC_SECRET", "test-value-123");

        let backend = EnvBackend::new();
        let result = backend.retrieve("test-svc", "secret");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().expose_secret(), "test-value-123");

        // Clean up
        std::env::remove_var("NAKAMA_TEST_SVC_SECRET");
    }

    #[test]
    fn test_retrieve_missing() {
        let backend = EnvBackend::new();
        let result = backend.retrieve("nonexistent", "key");
        assert!(result.is_err());
    }

    #[test]
    fn test_store_is_noop() {
        let backend = EnvBackend::new();
        let secret = SecretValue::new("value".to_string());
        // Should succeed (no-op) without error
        assert!(backend.store("svc", "key", &secret).is_ok());
    }

    #[test]
    fn test_delete_is_noop() {
        let backend = EnvBackend::new();
        // Should succeed (no-op) without error
        assert!(backend.delete("svc", "key").is_ok());
    }

    #[test]
    fn test_list_keys_from_env() {
        std::env::set_var("NAKAMA_LISTSVC_KEY_ONE", "v1");
        std::env::set_var("NAKAMA_LISTSVC_KEY_TWO", "v2");

        let backend = EnvBackend::new();
        let keys = backend.list_keys("listsvc").unwrap();
        assert!(keys.contains(&"key-one".to_string()));
        assert!(keys.contains(&"key-two".to_string()));

        std::env::remove_var("NAKAMA_LISTSVC_KEY_ONE");
        std::env::remove_var("NAKAMA_LISTSVC_KEY_TWO");
    }
}
