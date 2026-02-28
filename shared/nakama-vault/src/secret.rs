use secrecy::{ExposeSecret, SecretString};
use std::fmt;

/// A wrapper around a secret string value that ensures the secret is not
/// accidentally logged or displayed. The inner memory is zeroized on drop
/// via the `secrecy` crate's `SecretString`.
pub struct SecretValue {
    inner: SecretString,
}

impl SecretValue {
    /// Create a new `SecretValue` from a plain string.
    pub fn new(value: String) -> Self {
        Self {
            inner: SecretString::from(value),
        }
    }

    /// Expose the secret value as a string slice.
    ///
    /// Use this only when you actually need the raw secret (e.g., to pass it
    /// to an API or write it to an encrypted store). Avoid logging the result.
    pub fn expose_secret(&self) -> &str {
        self.inner.expose_secret()
    }
}

impl Clone for SecretValue {
    fn clone(&self) -> Self {
        Self {
            inner: SecretString::from(self.inner.expose_secret().to_owned()),
        }
    }
}

impl fmt::Display for SecretValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl fmt::Debug for SecretValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretValue([REDACTED])")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expose_secret() {
        let secret = SecretValue::new("my-api-key".to_string());
        assert_eq!(secret.expose_secret(), "my-api-key");
    }

    #[test]
    fn test_display_redacted() {
        let secret = SecretValue::new("my-api-key".to_string());
        assert_eq!(format!("{}", secret), "[REDACTED]");
    }

    #[test]
    fn test_debug_redacted() {
        let secret = SecretValue::new("my-api-key".to_string());
        assert_eq!(format!("{:?}", secret), "SecretValue([REDACTED])");
    }

    #[test]
    fn test_clone() {
        let secret = SecretValue::new("clone-me".to_string());
        let cloned = secret.clone();
        assert_eq!(cloned.expose_secret(), "clone-me");
    }
}
