//! Audit entry types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A single audit log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique entry identifier (`aud_<uuid>`).
    pub id: String,

    /// When the audited action occurred.
    pub timestamp: DateTime<Utc>,

    /// Distributed trace identifier for correlating across tools.
    pub trace_id: String,

    /// Which Nakama tool produced this entry.
    pub tool: String,

    /// The sub-command that was executed.
    pub command: String,

    /// High-level classification of the action.
    pub category: Category,

    /// Human-readable description of what happened.
    pub action: String,

    /// Arbitrary structured details about the action.
    pub detail: serde_json::Value,

    /// Whether the action succeeded, failed, was denied, or skipped.
    pub outcome: Outcome,

    /// Wall-clock duration of the action in milliseconds.
    pub duration_ms: u64,
}

impl AuditEntry {
    /// Create a new audit entry with a fresh `aud_<uuid>` identifier and the
    /// current UTC timestamp.
    pub fn new(
        trace_id: &str,
        tool: &str,
        command: &str,
        category: Category,
        action: &str,
        detail: serde_json::Value,
        outcome: Outcome,
        duration_ms: u64,
    ) -> Self {
        Self {
            id: format!("aud_{}", uuid::Uuid::new_v4().simple()),
            timestamp: Utc::now(),
            trace_id: trace_id.to_string(),
            tool: tool.to_string(),
            command: command.to_string(),
            category,
            action: action.to_string(),
            detail,
            outcome,
            duration_ms,
        }
    }
}

/// High-level classification of an audited action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    /// Login / token refresh / session management.
    Authentication,
    /// Reading or writing secrets from the vault.
    CredentialAccess,
    /// Calls to AI providers (Anthropic, OpenAI, etc.).
    AiInteraction,
    /// Calls to external HTTP APIs (GitHub, Jira, etc.).
    ExternalApi,
    /// Writes to files, databases, or other persistent stores.
    DataModification,
    /// Execution of a Nakama tool command.
    ToolExecution,
    /// Changes to configuration files.
    Configuration,
    /// Inter-process communication between tools.
    Ipc,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Authentication => write!(f, "authentication"),
            Category::CredentialAccess => write!(f, "credential_access"),
            Category::AiInteraction => write!(f, "ai_interaction"),
            Category::ExternalApi => write!(f, "external_api"),
            Category::DataModification => write!(f, "data_modification"),
            Category::ToolExecution => write!(f, "tool_execution"),
            Category::Configuration => write!(f, "configuration"),
            Category::Ipc => write!(f, "ipc"),
        }
    }
}

impl std::str::FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "authentication" => Ok(Category::Authentication),
            "credential_access" => Ok(Category::CredentialAccess),
            "ai_interaction" => Ok(Category::AiInteraction),
            "external_api" => Ok(Category::ExternalApi),
            "data_modification" => Ok(Category::DataModification),
            "tool_execution" => Ok(Category::ToolExecution),
            "configuration" => Ok(Category::Configuration),
            "ipc" => Ok(Category::Ipc),
            other => Err(format!("Unknown category: {other}")),
        }
    }
}

/// Outcome of an audited action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    /// The action completed successfully.
    Success,
    /// The action failed (error).
    Failure,
    /// The action was denied by a permission check or policy.
    Denied,
    /// The action was intentionally skipped (e.g. dry-run).
    Skipped,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Outcome::Success => write!(f, "success"),
            Outcome::Failure => write!(f, "failure"),
            Outcome::Denied => write!(f, "denied"),
            Outcome::Skipped => write!(f, "skipped"),
        }
    }
}

impl std::str::FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "success" => Ok(Outcome::Success),
            "failure" => Ok(Outcome::Failure),
            "denied" => Ok(Outcome::Denied),
            "skipped" => Ok(Outcome::Skipped),
            other => Err(format!("Unknown outcome: {other}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_id_prefix() {
        let entry = AuditEntry::new(
            "tr_abc123",
            "zangetsu",
            "log",
            Category::ToolExecution,
            "Executed git log",
            serde_json::json!({}),
            Outcome::Success,
            42,
        );
        assert!(entry.id.starts_with("aud_"));
    }

    #[test]
    fn test_category_display_roundtrip() {
        let cat = Category::AiInteraction;
        let s = cat.to_string();
        let parsed: Category = s.parse().unwrap();
        assert_eq!(parsed, cat);
    }

    #[test]
    fn test_outcome_display_roundtrip() {
        let out = Outcome::Denied;
        let s = out.to_string();
        let parsed: Outcome = s.parse().unwrap();
        assert_eq!(parsed, out);
    }

    #[test]
    fn test_category_serde_roundtrip() {
        let cat = Category::CredentialAccess;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, "\"credential_access\"");
        let parsed: Category = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, cat);
    }

    #[test]
    fn test_outcome_serde_roundtrip() {
        let out = Outcome::Skipped;
        let json = serde_json::to_string(&out).unwrap();
        assert_eq!(json, "\"skipped\"");
        let parsed: Outcome = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, out);
    }
}
