use crate::secret::SecretValue;
use crate::vault::CredentialStore;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use argon2::Argon2;
use nakama_core::error::{NakamaError, NakamaResult};
use nakama_core::paths;
use nakama_core::permissions;
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::debug;

/// Encrypted file-based credential backend.
///
/// Stores credentials in `~/.nakama/vault/<service>/<key>.enc`, where each
/// file contains an AES-256-GCM encrypted payload. The encryption key is
/// derived from machine-specific data using Argon2id, providing a portable
/// fallback when the OS keychain is unavailable.
pub struct EncryptedFileBackend {
    vault_dir: PathBuf,
    machine_key: Vec<u8>,
}

/// On-disk format for an encrypted credential.
#[derive(Serialize, Deserialize)]
struct EncryptedEntry {
    /// Argon2id salt (16 bytes, hex-encoded)
    salt: String,
    /// AES-256-GCM nonce (12 bytes, hex-encoded)
    nonce: String,
    /// Ciphertext (hex-encoded)
    ciphertext: String,
}

impl EncryptedFileBackend {
    /// Create a new encrypted file backend.
    ///
    /// Ensures the vault directory exists and derives a machine-specific key.
    pub fn new() -> NakamaResult<Self> {
        let vault_dir = paths::vault_dir()?;
        paths::ensure_dir(&vault_dir)?;

        let machine_key = Self::derive_machine_identity()?;

        debug!("Encrypted file backend initialized at {}", vault_dir.display());
        Ok(Self {
            vault_dir,
            machine_key,
        })
    }

    /// Derive a stable machine-specific identity for use as Argon2id input.
    ///
    /// This combines several sources to create a key that is unique to the
    /// current machine and user, but stable across reboots:
    /// - Username
    /// - Home directory path
    /// - Hostname (if available)
    fn derive_machine_identity() -> NakamaResult<Vec<u8>> {
        let mut identity = Vec::new();

        // Username
        if let Ok(user) = std::env::var("USER").or_else(|_| std::env::var("USERNAME")) {
            identity.extend_from_slice(user.as_bytes());
        }

        // Home directory (use env var to avoid needing `dirs` as a direct dependency)
        if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
            identity.extend_from_slice(home.as_bytes());
        }

        // Hostname
        if let Ok(hostname) = hostname() {
            identity.extend_from_slice(hostname.as_bytes());
        }

        // Static salt to namespace our usage
        identity.extend_from_slice(b"nakama-vault-machine-key-v1");

        if identity.len() < 16 {
            return Err(NakamaError::Vault {
                message: "Could not gather sufficient machine identity data".to_string(),
                source: None,
            });
        }

        Ok(identity)
    }

    /// Derive an AES-256 key from the machine identity and a per-entry salt.
    fn derive_key(&self, salt: &[u8]) -> NakamaResult<[u8; 32]> {
        let mut key = [0u8; 32];
        let argon2 = Argon2::default();
        argon2
            .hash_password_into(&self.machine_key, salt, &mut key)
            .map_err(|e| NakamaError::Vault {
                message: format!("Argon2id key derivation failed: {}", e),
                source: None,
            })?;
        Ok(key)
    }

    /// Get the directory path for a service: `~/.nakama/vault/<service>/`
    fn service_dir(&self, service: &str) -> PathBuf {
        self.vault_dir.join(service)
    }

    /// Get the file path for a credential: `~/.nakama/vault/<service>/<key>.enc`
    fn credential_path(&self, service: &str, key: &str) -> PathBuf {
        self.service_dir(service).join(format!("{}.enc", key))
    }

    /// Encrypt a plaintext value and return the serialized entry.
    fn encrypt(&self, plaintext: &str) -> NakamaResult<EncryptedEntry> {
        // Generate random salt (16 bytes) and nonce (12 bytes)
        let mut salt = [0u8; 16];
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut salt);
        OsRng.fill_bytes(&mut nonce_bytes);

        // Derive key from machine identity + salt
        let key = self.derive_key(&salt)?;
        let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| NakamaError::Vault {
            message: format!("Failed to create cipher: {}", e),
            source: None,
        })?;

        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| NakamaError::Vault {
                message: format!("AES-256-GCM encryption failed: {}", e),
                source: None,
            })?;

        Ok(EncryptedEntry {
            salt: hex_encode(&salt),
            nonce: hex_encode(&nonce_bytes),
            ciphertext: hex_encode(&ciphertext),
        })
    }

    /// Decrypt an encrypted entry and return the plaintext.
    fn decrypt(&self, entry: &EncryptedEntry) -> NakamaResult<String> {
        let salt = hex_decode(&entry.salt)?;
        let nonce_bytes = hex_decode(&entry.nonce)?;
        let ciphertext = hex_decode(&entry.ciphertext)?;

        let key = self.derive_key(&salt)?;
        let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| NakamaError::Vault {
            message: format!("Failed to create cipher: {}", e),
            source: None,
        })?;

        let nonce = Nonce::from_slice(&nonce_bytes);
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| NakamaError::Vault {
                message: format!("AES-256-GCM decryption failed (wrong machine or corrupted file): {}", e),
                source: None,
            })?;

        String::from_utf8(plaintext).map_err(|e| NakamaError::Vault {
            message: format!("Decrypted value is not valid UTF-8: {}", e),
            source: Some(Box::new(e)),
        })
    }
}

impl CredentialStore for EncryptedFileBackend {
    fn store(&self, service: &str, key: &str, value: &SecretValue) -> NakamaResult<()> {
        let svc_dir = self.service_dir(service);
        paths::ensure_dir(&svc_dir)?;

        let entry = self.encrypt(value.expose_secret())?;
        let json = serde_json::to_string_pretty(&entry).map_err(|e| NakamaError::Vault {
            message: format!("Failed to serialize encrypted entry: {}", e),
            source: Some(Box::new(e)),
        })?;

        let path = self.credential_path(service, key);
        std::fs::write(&path, json)?;
        permissions::set_file_permissions(&path)?;

        debug!("Stored {}/{} to encrypted file at {}", service, key, path.display());
        Ok(())
    }

    fn retrieve(&self, service: &str, key: &str) -> NakamaResult<SecretValue> {
        let path = self.credential_path(service, key);
        if !path.exists() {
            return Err(NakamaError::Vault {
                message: format!("No encrypted file found for {}/{}", service, key),
                source: None,
            });
        }

        let json = std::fs::read_to_string(&path)?;
        let entry: EncryptedEntry = serde_json::from_str(&json).map_err(|e| NakamaError::Vault {
            message: format!("Failed to parse encrypted entry at {}: {}", path.display(), e),
            source: Some(Box::new(e)),
        })?;

        let plaintext = self.decrypt(&entry)?;
        debug!("Retrieved {}/{} from encrypted file", service, key);
        Ok(SecretValue::new(plaintext))
    }

    fn delete(&self, service: &str, key: &str) -> NakamaResult<()> {
        let path = self.credential_path(service, key);
        if path.exists() {
            std::fs::remove_file(&path)?;
            debug!("Deleted encrypted file for {}/{}", service, key);
        }
        Ok(())
    }

    fn list_keys(&self, service: &str) -> NakamaResult<Vec<String>> {
        let svc_dir = self.service_dir(service);
        if !svc_dir.exists() {
            return Ok(Vec::new());
        }

        let mut keys = Vec::new();
        for entry in std::fs::read_dir(&svc_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("enc") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    keys.push(stem.to_string());
                }
            }
        }

        keys.sort();
        debug!("Listed {} keys for service {} in encrypted file store", keys.len(), service);
        Ok(keys)
    }
}

/// Hex-encode bytes to a lowercase hex string.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Hex-decode a hex string to bytes.
fn hex_decode(hex: &str) -> NakamaResult<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return Err(NakamaError::Vault {
            message: "Invalid hex string (odd length)".to_string(),
            source: None,
        });
    }

    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| NakamaError::Vault {
                message: format!("Invalid hex character: {}", e),
                source: Some(Box::new(e)),
            })
        })
        .collect()
}

/// Get the system hostname.
fn hostname() -> Result<String, std::io::Error> {
    #[cfg(unix)]
    {
        use std::process::Command;
        let output = Command::new("hostname").output()?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    #[cfg(not(unix))]
    {
        std::env::var("COMPUTERNAME")
            .or_else(|_| std::env::var("HOSTNAME"))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_roundtrip() {
        let original = b"hello world";
        let encoded = hex_encode(original);
        let decoded = hex_decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_hex_decode_invalid() {
        assert!(hex_decode("zz").is_err());
        assert!(hex_decode("abc").is_err()); // odd length
    }

    #[test]
    fn test_credential_path() {
        let backend = EncryptedFileBackend::new().unwrap();
        let path = backend.credential_path("zangetsu", "api-key");
        assert!(path.to_string_lossy().contains("vault"));
        assert!(path.to_string_lossy().contains("zangetsu"));
        assert!(path.to_string_lossy().ends_with("api-key.enc"));
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let backend = EncryptedFileBackend::new().unwrap();
        let plaintext = "super-secret-api-key-12345";
        let entry = backend.encrypt(plaintext).unwrap();

        // Ciphertext should not contain plaintext
        assert!(!entry.ciphertext.contains("super-secret"));

        let decrypted = backend.decrypt(&entry).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
