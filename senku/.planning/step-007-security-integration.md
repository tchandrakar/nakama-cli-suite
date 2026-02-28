# Step 007: Security Integration

## Objective
Integrate senku with `nakama-vault` for secure embedding API key management, enforce restrictive permissions on the `.senku/` data directory, and wire up `nakama-audit` for comprehensive audit logging of all indexing and query operations.

## Tasks
- [ ] Integrate `nakama-vault` for API key retrieval:
  - Fetch embedding API keys from vault (not env vars or config files)
  - Fetch LLM API keys for answer generation from vault
  - Support key rotation with automatic re-fetch on auth failures
  - Cache keys in memory only (never write to disk)
- [ ] Implement `.senku/` directory security:
  - Set directory permissions to 0700 (owner read/write/execute only)
  - Set file permissions to 0600 within `.senku/`
  - Validate permissions on startup, warn if too permissive
  - Lock index files during write operations (file locking)
  - Prevent concurrent indexing (PID file or file lock)
- [ ] Implement sensitive file handling:
  - Detect and skip sensitive files (.env, credentials.json, private keys)
  - Configurable sensitive file patterns
  - Warn when sensitive files are encountered during indexing
  - Never embed content from sensitive files
- [ ] Wire up `nakama-audit` for all operations:
  - `index_started`: log project path, file count, language breakdown
  - `index_completed`: log chunk count, embedding count, duration
  - `index_incremental`: log files added/modified/deleted
  - `query_received`: log query text (sanitized)
  - `query_results`: log result count, top result files, response time
  - `graph_query`: log query type, target symbol
  - `onboard_generated`: log project path, guide sections
  - `map_generated`: log map type, node/edge count
- [ ] Add audit context: project path, user identity, session ID, tool version
- [ ] Ensure no source code content appears in audit logs (only metadata)
- [ ] Implement data retention policy:
  - Configurable max index age
  - Auto-cleanup of stale index data
  - Manual purge command: `senku purge`
- [ ] Write tests for vault integration with mock vault
- [ ] Write tests for permission enforcement
- [ ] Write tests for sensitive file detection
- [ ] Write tests for audit event emission

## Acceptance Criteria
- API keys are retrieved from `nakama-vault` and never persisted to disk
- `.senku/` directory has restrictive permissions (0700)
- Sensitive files (.env, credentials) are never indexed
- All indexing and query operations emit appropriate audit events
- Audit logs contain no source code, only metadata
- Concurrent indexing is prevented via locking
- Tests validate vault integration, permissions, and audit emission

## Dependencies
- Step 001 (CLI scaffold and audit wiring)
- Step 005 (embedding generation needs API keys)
- Step 006 (storage layer manages `.senku/` directory)
