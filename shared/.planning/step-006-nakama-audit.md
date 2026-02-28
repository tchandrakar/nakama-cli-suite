# Step 006: Build nakama-audit (Tamper-Evident Audit Trail)

## Objective
Implement the audit logging system — every sensitive action tracked, queryable, tamper-evident.

## Tasks
- AuditEntry struct and builder pattern
- Categories: authentication, credential_access, ai_interaction, external_api, data_modification, tool_execution, configuration, ipc
- SQLite storage: create tables (audit_entries, audit_checksums, ai_usage), indexes, WAL mode
- JSONL backup writer (append-only, one entry per line)
- Hash chain: SHA-256, each entry chains to previous for tamper detection
- Batch writing: buffer entries, flush every 1s or after 100 entries
- Query interface: by tool, category, time range, trace_id, outcome
- AI usage aggregation: by tool, by model, by time period, cost tracking
- Retention cleanup: auto-delete entries older than retention_days
- Chain verification: `nakama audit verify` validates hash chain integrity
- Export: CSV and JSON formats
- `nakama audit` CLI: view, filter, usage, verify, export
- Convenience API: audit::log_ai_call(), audit::log_credential_access(), audit::log_external_api()
- Unit tests: write/read round-trip, chain integrity after tampering, retention cleanup, concurrent writes

## Acceptance Criteria
- Every audited action produces an entry in both SQLite and JSONL
- Hash chain detects any tampering (modify, delete, insert)
- `nakama audit --tool=byakugan --since="1 hour ago"` shows results
- `nakama audit usage --period=week` shows AI cost breakdown
- `nakama audit verify` passes on clean data, fails on tampered data
- Batch writes perform well under concurrent tool usage

## Dependencies
- Step 002 (nakama-core), Step 004 (nakama-log)
