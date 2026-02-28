//! SQLite-backed audit log with SHA-256 hash chaining.

use crate::entry::{AuditEntry, Category, Outcome};
use crate::query::AuditFilter;
use chrono::{DateTime, Utc};
use nakama_core::config::AuditConfig;
use nakama_core::error::{NakamaError, NakamaResult};
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::sync::Mutex;

/// The tamper-evident audit log backed by SQLite.
///
/// Each entry's checksum is computed over the entry fields *plus* the previous
/// entry's checksum, forming a hash chain that makes retroactive modification
/// detectable.
pub struct AuditLog {
    conn: Mutex<Connection>,
}

impl AuditLog {
    /// Open (or create) the audit database.
    ///
    /// The database is stored at `~/.nakama/audit/audit.db` and uses WAL mode
    /// for concurrent read access.
    pub fn new(_config: &AuditConfig) -> NakamaResult<Self> {
        let db_path = Self::db_path()?;

        // Ensure the parent directory exists.
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| NakamaError::Audit {
                message: format!("Failed to create audit directory: {e}"),
            })?;
        }

        let conn = Connection::open(&db_path).map_err(|e| NakamaError::Audit {
            message: format!("Failed to open audit database at {}: {e}", db_path.display()),
        })?;

        // Enable WAL mode for better concurrent read performance.
        conn.execute_batch("PRAGMA journal_mode = WAL;")
            .map_err(|e| NakamaError::Audit {
                message: format!("Failed to enable WAL mode: {e}"),
            })?;

        // Create the audit_entries table if it does not exist.
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS audit_entries (
                id          TEXT PRIMARY KEY,
                timestamp   TEXT NOT NULL,
                trace_id    TEXT NOT NULL,
                tool        TEXT NOT NULL,
                command     TEXT NOT NULL,
                category    TEXT NOT NULL,
                action      TEXT NOT NULL,
                detail      TEXT NOT NULL,
                outcome     TEXT NOT NULL,
                duration_ms INTEGER NOT NULL,
                checksum    TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_audit_tool      ON audit_entries(tool);
            CREATE INDEX IF NOT EXISTS idx_audit_category   ON audit_entries(category);
            CREATE INDEX IF NOT EXISTS idx_audit_timestamp  ON audit_entries(timestamp);
            CREATE INDEX IF NOT EXISTS idx_audit_trace_id   ON audit_entries(trace_id);
            CREATE INDEX IF NOT EXISTS idx_audit_outcome    ON audit_entries(outcome);",
        )
        .map_err(|e| NakamaError::Audit {
            message: format!("Failed to initialize audit tables: {e}"),
        })?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Open an audit database at a custom path (useful for testing).
    pub fn open_at(path: &std::path::Path) -> NakamaResult<Self> {
        let conn = Connection::open(path).map_err(|e| NakamaError::Audit {
            message: format!("Failed to open audit database at {}: {e}", path.display()),
        })?;

        conn.execute_batch("PRAGMA journal_mode = WAL;")
            .map_err(|e| NakamaError::Audit {
                message: format!("Failed to enable WAL mode: {e}"),
            })?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS audit_entries (
                id          TEXT PRIMARY KEY,
                timestamp   TEXT NOT NULL,
                trace_id    TEXT NOT NULL,
                tool        TEXT NOT NULL,
                command     TEXT NOT NULL,
                category    TEXT NOT NULL,
                action      TEXT NOT NULL,
                detail      TEXT NOT NULL,
                outcome     TEXT NOT NULL,
                duration_ms INTEGER NOT NULL,
                checksum    TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_audit_tool      ON audit_entries(tool);
            CREATE INDEX IF NOT EXISTS idx_audit_category   ON audit_entries(category);
            CREATE INDEX IF NOT EXISTS idx_audit_timestamp  ON audit_entries(timestamp);
            CREATE INDEX IF NOT EXISTS idx_audit_trace_id   ON audit_entries(trace_id);
            CREATE INDEX IF NOT EXISTS idx_audit_outcome    ON audit_entries(outcome);",
        )
        .map_err(|e| NakamaError::Audit {
            message: format!("Failed to initialize audit tables: {e}"),
        })?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Insert a new audit entry and compute the SHA-256 chain link.
    pub fn log(&self, entry: AuditEntry) -> NakamaResult<()> {
        let conn = self.conn.lock().map_err(|e| NakamaError::Audit {
            message: format!("Failed to acquire audit database lock: {e}"),
        })?;

        // Fetch the previous checksum to chain.
        let prev_checksum: String = conn
            .query_row(
                "SELECT checksum FROM audit_entries ORDER BY rowid DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| "GENESIS".to_string());

        let detail_str = serde_json::to_string(&entry.detail).unwrap_or_default();
        let timestamp_str = entry.timestamp.to_rfc3339();

        // Compute the checksum: SHA-256(prev_checksum || id || timestamp || tool || action || outcome).
        let checksum = Self::compute_checksum(
            &prev_checksum,
            &entry.id,
            &timestamp_str,
            &entry.tool,
            &entry.action,
            &entry.outcome.to_string(),
        );

        conn.execute(
            "INSERT INTO audit_entries (id, timestamp, trace_id, tool, command, category, action, detail, outcome, duration_ms, checksum)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                entry.id,
                timestamp_str,
                entry.trace_id,
                entry.tool,
                entry.command,
                entry.category.to_string(),
                entry.action,
                detail_str,
                entry.outcome.to_string(),
                entry.duration_ms as i64,
                checksum,
            ],
        )
        .map_err(|e| NakamaError::Audit {
            message: format!("Failed to insert audit entry: {e}"),
        })?;

        tracing::debug!(
            audit_id = %entry.id,
            checksum = %checksum,
            "Audit entry recorded"
        );

        Ok(())
    }

    /// Query audit entries matching the given filter.
    pub fn query(&self, filter: &AuditFilter) -> NakamaResult<Vec<AuditEntry>> {
        let conn = self.conn.lock().map_err(|e| NakamaError::Audit {
            message: format!("Failed to acquire audit database lock: {e}"),
        })?;

        // Build a dynamic WHERE clause.
        let mut clauses: Vec<String> = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(ref tool) = filter.tool {
            clauses.push(format!("tool = ?{}", param_values.len() + 1));
            param_values.push(Box::new(tool.clone()));
        }
        if let Some(ref category) = filter.category {
            clauses.push(format!("category = ?{}", param_values.len() + 1));
            param_values.push(Box::new(category.to_string()));
        }
        if let Some(ref since) = filter.since {
            clauses.push(format!("timestamp >= ?{}", param_values.len() + 1));
            param_values.push(Box::new(since.to_rfc3339()));
        }
        if let Some(ref until) = filter.until {
            clauses.push(format!("timestamp <= ?{}", param_values.len() + 1));
            param_values.push(Box::new(until.to_rfc3339()));
        }
        if let Some(ref trace_id) = filter.trace_id {
            clauses.push(format!("trace_id = ?{}", param_values.len() + 1));
            param_values.push(Box::new(trace_id.clone()));
        }
        if let Some(ref outcome) = filter.outcome {
            clauses.push(format!("outcome = ?{}", param_values.len() + 1));
            param_values.push(Box::new(outcome.to_string()));
        }

        let where_clause = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let limit_clause = match filter.limit {
            Some(limit) => format!("LIMIT {limit}"),
            None => String::new(),
        };

        let sql = format!(
            "SELECT id, timestamp, trace_id, tool, command, category, action, detail, outcome, duration_ms
             FROM audit_entries
             {where_clause}
             ORDER BY timestamp DESC
             {limit_clause}"
        );

        let params_refs: Vec<&dyn rusqlite::types::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&sql).map_err(|e| NakamaError::Audit {
            message: format!("Failed to prepare audit query: {e}"),
        })?;

        let entries = stmt
            .query_map(params_refs.as_slice(), |row| {
                let timestamp_str: String = row.get(1)?;
                let category_str: String = row.get(5)?;
                let detail_str: String = row.get(7)?;
                let outcome_str: String = row.get(8)?;
                let duration: i64 = row.get(9)?;

                Ok(AuditEntry {
                    id: row.get(0)?,
                    timestamp: DateTime::parse_from_rfc3339(&timestamp_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    trace_id: row.get(2)?,
                    tool: row.get(3)?,
                    command: row.get(4)?,
                    category: category_str
                        .parse::<Category>()
                        .unwrap_or(Category::ToolExecution),
                    action: row.get(6)?,
                    detail: serde_json::from_str(&detail_str).unwrap_or(serde_json::Value::Null),
                    outcome: outcome_str
                        .parse::<Outcome>()
                        .unwrap_or(Outcome::Failure),
                    duration_ms: duration as u64,
                })
            })
            .map_err(|e| NakamaError::Audit {
                message: format!("Failed to execute audit query: {e}"),
            })?;

        let mut results = Vec::new();
        for entry_result in entries {
            let entry = entry_result.map_err(|e| NakamaError::Audit {
                message: format!("Failed to read audit row: {e}"),
            })?;
            results.push(entry);
        }

        Ok(results)
    }

    /// Verify the integrity of the hash chain.
    ///
    /// Returns `Ok(true)` if the chain is valid, `Ok(false)` if tampering
    /// is detected, or an error if the database cannot be read.
    pub fn verify_chain(&self) -> NakamaResult<bool> {
        let conn = self.conn.lock().map_err(|e| NakamaError::Audit {
            message: format!("Failed to acquire audit database lock: {e}"),
        })?;

        let mut stmt = conn
            .prepare(
                "SELECT id, timestamp, tool, action, outcome, checksum
                 FROM audit_entries ORDER BY rowid ASC",
            )
            .map_err(|e| NakamaError::Audit {
                message: format!("Failed to prepare verification query: {e}"),
            })?;

        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                ))
            })
            .map_err(|e| NakamaError::Audit {
                message: format!("Failed to query for verification: {e}"),
            })?;

        let mut prev_checksum = "GENESIS".to_string();
        for row_result in rows {
            let (id, timestamp, tool, action, outcome, stored_checksum) =
                row_result.map_err(|e| NakamaError::Audit {
                    message: format!("Failed to read verification row: {e}"),
                })?;

            let expected =
                Self::compute_checksum(&prev_checksum, &id, &timestamp, &tool, &action, &outcome);

            if expected != stored_checksum {
                tracing::warn!(
                    entry_id = %id,
                    expected = %expected,
                    stored = %stored_checksum,
                    "Audit chain integrity violation detected"
                );
                return Ok(false);
            }

            prev_checksum = stored_checksum;
        }

        Ok(true)
    }

    // --- Private helpers ---

    fn db_path() -> NakamaResult<PathBuf> {
        let audit_dir = nakama_core::paths::audit_dir()?;
        Ok(audit_dir.join("audit.db"))
    }

    fn compute_checksum(
        prev_checksum: &str,
        id: &str,
        timestamp: &str,
        tool: &str,
        action: &str,
        outcome: &str,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(prev_checksum.as_bytes());
        hasher.update(b"|");
        hasher.update(id.as_bytes());
        hasher.update(b"|");
        hasher.update(timestamp.as_bytes());
        hasher.update(b"|");
        hasher.update(tool.as_bytes());
        hasher.update(b"|");
        hasher.update(action.as_bytes());
        hasher.update(b"|");
        hasher.update(outcome.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::{AuditEntry, Category, Outcome};
    use crate::query::AuditFilter;

    fn test_log() -> AuditLog {
        AuditLog::open_at(std::path::Path::new(":memory:")).unwrap()
    }

    #[test]
    fn test_insert_and_query() {
        let log = test_log();
        let entry = AuditEntry::new(
            "tr_test123",
            "zangetsu",
            "log",
            Category::ToolExecution,
            "Ran git log",
            serde_json::json!({"branch": "main"}),
            Outcome::Success,
            150,
        );
        log.log(entry).unwrap();

        let results = log.query(&AuditFilter::new()).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].tool, "zangetsu");
        assert_eq!(results[0].outcome, Outcome::Success);
    }

    #[test]
    fn test_query_with_filter() {
        let log = test_log();

        // Insert two entries for different tools.
        log.log(AuditEntry::new(
            "tr_1",
            "zangetsu",
            "log",
            Category::ToolExecution,
            "action1",
            serde_json::json!({}),
            Outcome::Success,
            10,
        ))
        .unwrap();

        log.log(AuditEntry::new(
            "tr_2",
            "shinigami",
            "review",
            Category::AiInteraction,
            "action2",
            serde_json::json!({}),
            Outcome::Failure,
            20,
        ))
        .unwrap();

        let filter = AuditFilter::new().with_tool("zangetsu");
        let results = log.query(&filter).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].tool, "zangetsu");

        let filter = AuditFilter::new().with_outcome(Outcome::Failure);
        let results = log.query(&filter).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].tool, "shinigami");
    }

    #[test]
    fn test_chain_verification_valid() {
        let log = test_log();
        for i in 0..5 {
            log.log(AuditEntry::new(
                &format!("tr_{i}"),
                "test",
                "cmd",
                Category::ToolExecution,
                &format!("action {i}"),
                serde_json::json!({}),
                Outcome::Success,
                i as u64,
            ))
            .unwrap();
        }
        assert!(log.verify_chain().unwrap());
    }

    #[test]
    fn test_limit_filter() {
        let log = test_log();
        for i in 0..10 {
            log.log(AuditEntry::new(
                &format!("tr_{i}"),
                "test",
                "cmd",
                Category::ToolExecution,
                &format!("action {i}"),
                serde_json::json!({}),
                Outcome::Success,
                0,
            ))
            .unwrap();
        }
        let filter = AuditFilter::new().with_limit(3);
        let results = log.query(&filter).unwrap();
        assert_eq!(results.len(), 3);
    }
}
