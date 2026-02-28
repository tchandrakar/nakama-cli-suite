use serde::{Deserialize, Serialize};

/// AI provider identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Anthropic,
    OpenAI,
    Google,
    Ollama,
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::Anthropic => write!(f, "anthropic"),
            Provider::OpenAI => write!(f, "openai"),
            Provider::Google => write!(f, "google"),
            Provider::Ollama => write!(f, "ollama"),
        }
    }
}

/// Model capability tier — tools request a tier, the config resolves it to a concrete model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelTier {
    /// Fast, cheap: haiku / gpt-4.1-nano / gemini-flash
    Fast,
    /// Balanced: sonnet / gpt-4.1-mini / gemini-flash
    Balanced,
    /// Most capable: opus / gpt-4.1 / gemini-pro
    Powerful,
}

/// Output format for tool responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Human,
    Json,
    Plain,
}

/// Verbosity level for terminal output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
    Debug,
}

/// Color mode for terminal output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

/// Nakama tool identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tool {
    Zangetsu,
    Shinigami,
    Jogan,
    Senku,
    Sharingan,
    Tensai,
    Mugen,
    Gate,
    Byakugan,
    Kami,
    Itachi,
}

impl std::fmt::Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tool::Zangetsu => write!(f, "zangetsu"),
            Tool::Shinigami => write!(f, "shinigami"),
            Tool::Jogan => write!(f, "jogan"),
            Tool::Senku => write!(f, "senku"),
            Tool::Sharingan => write!(f, "sharingan"),
            Tool::Tensai => write!(f, "tensai"),
            Tool::Mugen => write!(f, "mugen"),
            Tool::Gate => write!(f, "gate"),
            Tool::Byakugan => write!(f, "byakugan"),
            Tool::Kami => write!(f, "kami"),
            Tool::Itachi => write!(f, "itachi"),
        }
    }
}
