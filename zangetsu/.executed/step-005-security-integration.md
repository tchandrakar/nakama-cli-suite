# Step 005: Security Integration

## Status: NOT STARTED

| Task | Status | Date | Notes |
|------|--------|------|-------|
| Integrate nakama-vault for API key retrieval | - | - | - |
| Support key rotation on 401/403 errors | - | - | - |
| Cache keys in memory only (never disk) | - | - | - |
| Implement command sanitization layer | - | - | - |
| Detect and reject shell injection patterns | - | - | - |
| Validate command binary exists in PATH | - | - | - |
| Reject commands with null bytes or control characters | - | - | - |
| Wire up nakama-audit for all events (query, generate, approve, execute, fail) | - | - | - |
| Add audit context (user ID, session ID, timestamp, version) | - | - | - |
| Ensure no secrets in logs or audit trails | - | - | - |
| Add --audit-level flag (minimal vs full) | - | - | - |
| Integration tests for vault key retrieval | - | - | - |
| Tests for sanitization rejection patterns | - | - | - |
| Tests for audit event emission | - | - | - |

Status legend: `-` Not started | `WIP` In progress | `DONE` Complete | `BLOCKED` Blocked
