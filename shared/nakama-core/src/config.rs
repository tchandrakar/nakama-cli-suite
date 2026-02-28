use crate::error::{NakamaError, NakamaResult};
use crate::paths;
use crate::types::*;
use serde::{Deserialize, Serialize};

/// Global Nakama configuration (loaded from ~/.nakama/config.toml).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub ai: AiConfig,
    pub logging: LoggingConfig,
    pub ui: UiConfig,
    pub audit: AuditConfig,
    pub ipc: IpcConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AiConfig {
    pub default_provider: Provider,
    pub anthropic: ProviderModels,
    pub openai: ProviderModels,
    pub google: ProviderModels,
    pub ollama: OllamaConfig,
    pub retry: RetryConfig,
    pub budget: Option<BudgetConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProviderModels {
    pub model_fast: String,
    pub model_balanced: String,
    pub model_powerful: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OllamaConfig {
    pub base_url: String,
    pub model_fast: String,
    pub model_balanced: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
    pub max_backoff_ms: u64,
    pub backoff_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConfig {
    pub weekly_limit_usd: f64,
    pub alert_threshold_percent: u32,
    pub hard_limit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub directory: String,
    pub max_file_size_mb: u64,
    pub max_rotated_files: u32,
    pub compress_rotated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UiConfig {
    pub color: ColorMode,
    pub verbosity: Verbosity,
    pub spinners: bool,
    pub unicode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AuditConfig {
    pub enabled: bool,
    pub retention_days: u32,
    pub chain_verification: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IpcConfig {
    pub schema_validation: bool,
    pub trace_propagation: bool,
}

// --- Defaults ---

impl Default for Config {
    fn default() -> Self {
        Self {
            ai: AiConfig::default(),
            logging: LoggingConfig::default(),
            ui: UiConfig::default(),
            audit: AuditConfig::default(),
            ipc: IpcConfig::default(),
        }
    }
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            default_provider: Provider::Anthropic,
            anthropic: ProviderModels {
                model_fast: "claude-haiku-4-5-20251001".to_string(),
                model_balanced: "claude-sonnet-4-6".to_string(),
                model_powerful: "claude-opus-4-6".to_string(),
                base_url: None,
            },
            openai: ProviderModels {
                model_fast: "gpt-4.1-nano".to_string(),
                model_balanced: "gpt-4.1-mini".to_string(),
                model_powerful: "gpt-4.1".to_string(),
                base_url: None,
            },
            google: ProviderModels {
                model_fast: "gemini-2.5-flash".to_string(),
                model_balanced: "gemini-2.5-flash".to_string(),
                model_powerful: "gemini-2.5-pro".to_string(),
                base_url: None,
            },
            ollama: OllamaConfig::default(),
            retry: RetryConfig::default(),
            budget: None,
        }
    }
}

impl Default for ProviderModels {
    fn default() -> Self {
        Self {
            model_fast: String::new(),
            model_balanced: String::new(),
            model_powerful: String::new(),
            base_url: None,
        }
    }
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            model_fast: "llama3:8b".to_string(),
            model_balanced: "llama3:70b".to_string(),
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff_ms: 1000,
            max_backoff_ms: 30000,
            backoff_multiplier: 2.0,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            directory: "~/.nakama/logs".to_string(),
            max_file_size_mb: 10,
            max_rotated_files: 5,
            compress_rotated: true,
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            color: ColorMode::Auto,
            verbosity: Verbosity::Normal,
            spinners: true,
            unicode: true,
        }
    }
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            retention_days: 90,
            chain_verification: true,
        }
    }
}

impl Default for IpcConfig {
    fn default() -> Self {
        Self {
            schema_validation: true,
            trace_propagation: true,
        }
    }
}

impl Config {
    /// Load configuration with merge: global config → tool config → defaults.
    pub fn load(tool_name: &str) -> NakamaResult<Self> {
        // Ensure directory structure exists
        paths::ensure_nakama_dirs()?;

        let mut config = Config::default();

        // Load global config if it exists
        let global_path = paths::global_config_path()?;
        if global_path.exists() {
            let contents = std::fs::read_to_string(&global_path).map_err(|e| {
                NakamaError::Config {
                    message: format!("Failed to read {}", global_path.display()),
                    source: Some(Box::new(e)),
                }
            })?;
            config = toml::from_str(&contents).map_err(|e| NakamaError::Config {
                message: format!("Failed to parse {}", global_path.display()),
                source: Some(Box::new(e)),
            })?;
        }

        // Load tool-specific config if it exists (overrides global)
        let tool_config_path = paths::tool_config_dir(tool_name)?.join("config.toml");
        if tool_config_path.exists() {
            let contents = std::fs::read_to_string(&tool_config_path).map_err(|e| {
                NakamaError::Config {
                    message: format!("Failed to read {}", tool_config_path.display()),
                    source: Some(Box::new(e)),
                }
            })?;
            // Merge tool config over global (tool config takes priority)
            let tool_config: toml::Value = toml::from_str(&contents).map_err(|e| {
                NakamaError::Config {
                    message: format!("Failed to parse {}", tool_config_path.display()),
                    source: Some(Box::new(e)),
                }
            })?;
            let mut global_value = toml::Value::try_from(&config).unwrap_or(toml::Value::Table(toml::map::Map::new()));
            merge_toml(&mut global_value, &tool_config);
            config = global_value.try_into().unwrap_or_default();
        }

        Ok(config)
    }

    /// Resolve a model ID for a given provider and tier.
    pub fn resolve_model(&self, provider: Provider, tier: ModelTier) -> String {
        match provider {
            Provider::Anthropic => match tier {
                ModelTier::Fast => self.ai.anthropic.model_fast.clone(),
                ModelTier::Balanced => self.ai.anthropic.model_balanced.clone(),
                ModelTier::Powerful => self.ai.anthropic.model_powerful.clone(),
            },
            Provider::OpenAI => match tier {
                ModelTier::Fast => self.ai.openai.model_fast.clone(),
                ModelTier::Balanced => self.ai.openai.model_balanced.clone(),
                ModelTier::Powerful => self.ai.openai.model_powerful.clone(),
            },
            Provider::Google => match tier {
                ModelTier::Fast => self.ai.google.model_fast.clone(),
                ModelTier::Balanced => self.ai.google.model_balanced.clone(),
                ModelTier::Powerful => self.ai.google.model_powerful.clone(),
            },
            Provider::Ollama => match tier {
                ModelTier::Fast => self.ai.ollama.model_fast.clone(),
                ModelTier::Balanced => self.ai.ollama.model_balanced.clone(),
                ModelTier::Powerful => self.ai.ollama.model_balanced.clone(),
            },
        }
    }
}

/// Deep merge two TOML values (source overrides target).
fn merge_toml(target: &mut toml::Value, source: &toml::Value) {
    match (target, source) {
        (toml::Value::Table(target_map), toml::Value::Table(source_map)) => {
            for (key, value) in source_map {
                if let Some(existing) = target_map.get_mut(key) {
                    merge_toml(existing, value);
                } else {
                    target_map.insert(key.clone(), value.clone());
                }
            }
        }
        (target, _) => {
            *target = source.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.ai.default_provider, Provider::Anthropic);
        assert_eq!(config.ai.anthropic.model_balanced, "claude-sonnet-4-6");
        assert!(config.audit.enabled);
    }

    #[test]
    fn test_resolve_model() {
        let config = Config::default();
        assert_eq!(
            config.resolve_model(Provider::Anthropic, ModelTier::Fast),
            "claude-haiku-4-5-20251001"
        );
        assert_eq!(
            config.resolve_model(Provider::OpenAI, ModelTier::Powerful),
            "gpt-4.1"
        );
    }

    #[test]
    fn test_merge_toml() {
        let mut target: toml::Value = toml::from_str(r#"
            [ai]
            default_provider = "anthropic"
            [ai.anthropic]
            model_fast = "haiku"
        "#).unwrap();

        let source: toml::Value = toml::from_str(r#"
            [ai]
            default_provider = "openai"
        "#).unwrap();

        merge_toml(&mut target, &source);

        let table = target.as_table().unwrap();
        let ai = table["ai"].as_table().unwrap();
        assert_eq!(ai["default_provider"].as_str().unwrap(), "openai");
        // Anthropic config should still be there
        assert!(ai.contains_key("anthropic"));
    }
}
