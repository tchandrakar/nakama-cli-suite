//! Tamper-evident audit logging for the Nakama CLI Suite.
//!
//! Every significant action taken by a Nakama tool is recorded as an
//! [`AuditEntry`] in a local SQLite database with SHA-256 hash chaining to
//! detect retroactive tampering.

pub mod entry;
pub mod query;
pub mod store;

pub use entry::{AuditEntry, Category, Outcome};
pub use query::AuditFilter;
pub use store::AuditLog;
