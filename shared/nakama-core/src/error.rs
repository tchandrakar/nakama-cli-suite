use thiserror::Error;

#[derive(Error, Debug)]
pub enum NakamaError {
    #[error("Configuration error: {message}")]
    Config { message: String, #[source] source: Option<Box<dyn std::error::Error + Send + Sync>> },

    #[error("Authentication error: {message}")]
    Auth { message: String },

    #[error("Credential vault error: {message}")]
    Vault { message: String, #[source] source: Option<Box<dyn std::error::Error + Send + Sync>> },

    #[error("AI provider error: {message}")]
    AiProvider { message: String, #[source] source: Option<Box<dyn std::error::Error + Send + Sync>> },

    #[error("Network error: {message}")]
    Network { message: String, #[source] source: Option<Box<dyn std::error::Error + Send + Sync>> },

    #[error("Audit error: {message}")]
    Audit { message: String },

    #[error("IPC error: {message}")]
    Ipc { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Permission denied: {message}")]
    Permission { message: String },

    #[error("{message}")]
    Tool { tool: String, message: String },
}

pub type NakamaResult<T> = Result<T, NakamaError>;
