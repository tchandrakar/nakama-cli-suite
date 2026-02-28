//! Common types shared by all AI provider implementations.

use serde::{Deserialize, Serialize};

/// A completion request sent to any AI provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    /// System-level prompt that sets context/behaviour.
    pub system_prompt: String,

    /// Conversation messages (user and assistant turns).
    pub messages: Vec<Message>,

    /// The model identifier to use (e.g. `"claude-sonnet-4-6"`).
    pub model: String,

    /// Maximum number of tokens to generate.
    pub max_tokens: u32,

    /// Sampling temperature (0.0 = deterministic, 1.0 = creative).
    pub temperature: f32,
}

/// A single message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Who sent this message.
    pub role: Role,

    /// The textual content of the message.
    pub content: String,
}

/// Participant role in a conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// System instructions (not always sent as a message to every provider).
    System,
    /// The human user.
    User,
    /// The AI assistant.
    Assistant,
}

/// The response returned by an AI provider after a completion request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    /// The generated text content.
    pub content: String,

    /// The model that actually served the request (may differ from requested
    /// model if the provider remapped it).
    pub model: String,

    /// Token usage statistics.
    pub usage: TokenUsage,
}

/// Token usage statistics for a single completion.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Number of tokens in the prompt.
    pub input_tokens: u32,

    /// Number of tokens generated.
    pub output_tokens: u32,
}

impl Message {
    /// Convenience constructor.
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    /// Create a user message.
    pub fn user(content: impl Into<String>) -> Self {
        Self::new(Role::User, content)
    }

    /// Create an assistant message.
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(Role::Assistant, content)
    }

    /// Create a system message.
    pub fn system(content: impl Into<String>) -> Self {
        Self::new(Role::System, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_constructors() {
        let msg = Message::user("Hello");
        assert_eq!(msg.role, Role::User);
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_role_serde() {
        let json = serde_json::to_string(&Role::Assistant).unwrap();
        assert_eq!(json, "\"assistant\"");
        let parsed: Role = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Role::Assistant);
    }
}
