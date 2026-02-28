//! Query filters for the audit log.

use crate::entry::{Category, Outcome};
use chrono::{DateTime, Utc};

/// Filter criteria for querying audit entries.
///
/// All fields are optional -- `None` means "do not filter on this field".
#[derive(Debug, Clone, Default)]
pub struct AuditFilter {
    /// Filter by originating tool name.
    pub tool: Option<String>,

    /// Filter by action category.
    pub category: Option<Category>,

    /// Only entries at or after this timestamp.
    pub since: Option<DateTime<Utc>>,

    /// Only entries at or before this timestamp.
    pub until: Option<DateTime<Utc>>,

    /// Filter by distributed trace identifier.
    pub trace_id: Option<String>,

    /// Filter by outcome.
    pub outcome: Option<Outcome>,

    /// Maximum number of entries to return.
    pub limit: Option<u32>,
}

impl AuditFilter {
    /// Create an empty filter (matches everything).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the tool filter.
    pub fn with_tool(mut self, tool: impl Into<String>) -> Self {
        self.tool = Some(tool.into());
        self
    }

    /// Set the category filter.
    pub fn with_category(mut self, category: Category) -> Self {
        self.category = Some(category);
        self
    }

    /// Set the since filter.
    pub fn with_since(mut self, since: DateTime<Utc>) -> Self {
        self.since = Some(since);
        self
    }

    /// Set the until filter.
    pub fn with_until(mut self, until: DateTime<Utc>) -> Self {
        self.until = Some(until);
        self
    }

    /// Set the trace_id filter.
    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    /// Set the outcome filter.
    pub fn with_outcome(mut self, outcome: Outcome) -> Self {
        self.outcome = Some(outcome);
        self
    }

    /// Set the limit.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_pattern() {
        let filter = AuditFilter::new()
            .with_tool("zangetsu")
            .with_category(Category::ToolExecution)
            .with_outcome(Outcome::Success)
            .with_limit(50);

        assert_eq!(filter.tool.as_deref(), Some("zangetsu"));
        assert_eq!(filter.category, Some(Category::ToolExecution));
        assert_eq!(filter.outcome, Some(Outcome::Success));
        assert_eq!(filter.limit, Some(50));
        assert!(filter.since.is_none());
        assert!(filter.until.is_none());
        assert!(filter.trace_id.is_none());
    }
}
