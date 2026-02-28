//! Multi-provider AI abstraction for the Nakama CLI Suite.
//!
//! Provides a unified [`AiProvider`] trait with implementations for Anthropic,
//! OpenAI, Google Gemini, and local Ollama models.  Use [`create_provider`] to
//! obtain a provider instance from a [`Provider`] enum value.

pub mod anthropic;
pub mod google;
pub mod ollama;
pub mod openai;
pub mod provider;
pub mod types;

pub use provider::{create_provider, AiProvider};
pub use types::{CompletionRequest, CompletionResponse, Message, Role, TokenUsage};
