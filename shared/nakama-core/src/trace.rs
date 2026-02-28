use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Trace context that flows across tool boundaries for end-to-end observability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceContext {
    pub trace_id: String,
    pub tool: String,
    pub command: String,
    pub started_at: DateTime<Utc>,
}

impl TraceContext {
    /// Create a new trace context for a tool invocation.
    pub fn new(tool: &str, command: &str) -> Self {
        Self {
            trace_id: format!("tr_{}", Uuid::new_v4().simple()),
            tool: tool.to_string(),
            command: command.to_string(),
            started_at: Utc::now(),
        }
    }

    /// Create a child trace context (preserves trace_id, changes tool/command).
    pub fn child(&self, tool: &str, command: &str) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            tool: tool.to_string(),
            command: command.to_string(),
            started_at: Utc::now(),
        }
    }

    /// Parse a trace context from an incoming NMP message trace_id.
    pub fn from_trace_id(trace_id: &str, tool: &str, command: &str) -> Self {
        Self {
            trace_id: trace_id.to_string(),
            tool: tool.to_string(),
            command: command.to_string(),
            started_at: Utc::now(),
        }
    }

    /// Elapsed time since trace started.
    pub fn elapsed_ms(&self) -> i64 {
        Utc::now()
            .signed_duration_since(self.started_at)
            .num_milliseconds()
    }
}
