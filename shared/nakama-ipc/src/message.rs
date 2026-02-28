//! The Nakama Message Protocol (NMP) message envelope.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// NMP protocol version.
pub const NMP_VERSION: &str = "1.0";

/// A single NMP message -- the universal envelope that flows between Nakama
/// tools over pipes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NmpMessage {
    /// Protocol version (currently `"1.0"`).
    pub nmp_version: String,

    /// Distributed trace identifier (`tr_<uuid>`).
    pub trace_id: String,

    /// Which tool/command produced this message.
    pub source: NmpSource,

    /// When the message was created.
    pub timestamp: DateTime<Utc>,

    /// Schema identifier for the payload (e.g. `"git.commit_list.v1"`).
    pub schema: String,

    /// Arbitrary JSON payload.
    pub data: serde_json::Value,
}

/// Identifies the tool and command that originated an NMP message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NmpSource {
    /// Tool name (e.g. `"zangetsu"`).
    pub tool: String,

    /// Sub-command that produced this message (e.g. `"log"`).
    pub command: String,

    /// Semver version of the tool.
    pub version: String,
}

impl NmpMessage {
    /// Create a new NMP message.
    ///
    /// A fresh `trace_id` is generated using `nakama_core::TraceContext`.
    pub fn new(
        tool: &str,
        command: &str,
        schema: &str,
        data: serde_json::Value,
    ) -> Self {
        let trace_ctx = nakama_core::TraceContext::new(tool, command);
        Self {
            nmp_version: NMP_VERSION.to_string(),
            trace_id: trace_ctx.trace_id,
            source: NmpSource {
                tool: tool.to_string(),
                command: command.to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            timestamp: Utc::now(),
            schema: schema.to_string(),
            data,
        }
    }

    /// Create an NMP message that continues an existing trace.
    pub fn with_trace_id(
        trace_id: &str,
        tool: &str,
        command: &str,
        schema: &str,
        data: serde_json::Value,
    ) -> Self {
        Self {
            nmp_version: NMP_VERSION.to_string(),
            trace_id: trace_id.to_string(),
            source: NmpSource {
                tool: tool.to_string(),
                command: command.to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            timestamp: Utc::now(),
            schema: schema.to_string(),
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_message_has_trace_id() {
        let msg = NmpMessage::new("zangetsu", "log", "git.log.v1", serde_json::json!({}));
        assert!(msg.trace_id.starts_with("tr_"));
        assert_eq!(msg.nmp_version, "1.0");
        assert_eq!(msg.source.tool, "zangetsu");
        assert_eq!(msg.source.command, "log");
        assert_eq!(msg.schema, "git.log.v1");
    }

    #[test]
    fn test_roundtrip_serialization() {
        let msg = NmpMessage::new("senku", "measure", "metrics.v1", serde_json::json!({"score": 42}));
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: NmpMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.trace_id, msg.trace_id);
        assert_eq!(parsed.data["score"], 42);
    }
}
