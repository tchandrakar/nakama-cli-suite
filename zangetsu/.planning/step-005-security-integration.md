# Step 005: Security Integration

## Objective
Integrate zangetsu with `nakama-vault` for secure API key management, implement command sanitization to prevent injection attacks, and wire up `nakama-audit` for comprehensive audit logging of every generated and executed command.

## Tasks
- [ ] Integrate `nakama-vault` for API key retrieval:
  - Fetch LLM API keys from vault at startup (not from env vars or config files)
  - Support key rotation: re-fetch on 401/403 errors
  - Cache keys in memory only (never write to disk)
- [ ] Implement command sanitization layer:
  - Strip or escape shell metacharacters in LLM-generated commands before execution
  - Detect and reject embedded shell injection patterns (backticks, `$()`, `$(())`)
  - Validate command binary exists in PATH before execution
  - Reject commands with null bytes or control characters
- [ ] Wire up `nakama-audit` for all significant events:
  - `query_received`: log the natural language query
  - `command_generated`: log the translated command + risk level
  - `command_approved`: log user confirmation (yes/no/blocked)
  - `command_executed`: log execution start, exit code, duration
  - `command_failed`: log execution errors with details
  - `alias_created`, `alias_deleted`: log alias operations
- [ ] Add audit context: user ID, session ID, timestamp, tool version
- [ ] Ensure no secrets (API keys, tokens) appear in logs or audit trails
- [ ] Add `--audit-level` flag: `minimal` (events only) vs `full` (events + command output)
- [ ] Write integration tests verifying vault key retrieval with mock vault
- [ ] Write tests verifying sanitization rejects known injection patterns
- [ ] Write tests verifying audit events are emitted for all operations

## Acceptance Criteria
- API keys are retrieved from `nakama-vault` and never stored on disk
- Command sanitization catches and rejects shell injection attempts
- Every query, generation, and execution is logged via `nakama-audit`
- Audit logs contain no secrets or API keys
- Tests cover vault integration, sanitization, and audit emission

## Dependencies
- Step 001 (CLI scaffold and nakama-audit wiring)
- Step 003 (LLM translation produces commands requiring sanitization)
- Step 004 (execution engine needs sanitized commands)
