# Step 008: Security Integration

## Objective

Integrate tensai with nakama-vault for secure management of GitHub tokens, Google Calendar OAuth credentials, Jira API tokens, and Slack tokens. Implement audit logging for all data fetches and AI calls to ensure transparency and compliance.

## Tasks

- [ ] Integrate nakama-vault for all credential retrieval:
  - [ ] GitHub personal access token / OAuth token
  - [ ] Google Calendar OAuth2 client ID, client secret, refresh token
  - [ ] Jira API token + email
  - [ ] Slack bot token (for focus mode status updates)
  - [ ] LLM API keys (for nakama-ai)
- [ ] Implement credential lifecycle management:
  - [ ] Store credentials via `tensai auth <service>` commands
  - [ ] Token refresh for OAuth services (Google Calendar)
  - [ ] Credential validation on startup (health check)
  - [ ] Clear credentials (`tensai auth revoke <service>`)
- [ ] Implement fallback credential sources:
  - [ ] Environment variables (GITHUB_TOKEN, JIRA_TOKEN, etc.)
  - [ ] System keychain (via nakama-vault keychain backend)
  - [ ] Config file (warn if plaintext secrets detected)
- [ ] Implement audit logging via nakama-audit:
  - [ ] Log every aggregator data fetch (source, timestamp, data types, success/failure)
  - [ ] Log every AI/LLM call (timestamp, purpose, token count, model)
  - [ ] Log every credential access (which service, when, by which aggregator)
  - [ ] Log every IPC communication (direction, message type)
  - [ ] No sensitive data in audit logs (hash tokens, redact responses)
- [ ] Implement data sensitivity handling:
  - [ ] Redact meeting details before LLM submission (optional)
  - [ ] Redact ticket descriptions with sensitive content
  - [ ] Configurable redaction levels (none, moderate, strict)
- [ ] Add security-related CLI flags:
  - [ ] `--redact-level` (none, moderate, strict)
  - [ ] `--audit-log` path override
  - [ ] `--no-vault` to use only env vars
- [ ] Add `tensai auth` subcommand group:
  - [ ] `tensai auth github` — set up GitHub token
  - [ ] `tensai auth calendar` — run Google OAuth flow
  - [ ] `tensai auth jira` — set up Jira credentials
  - [ ] `tensai auth status` — show which services are authenticated
  - [ ] `tensai auth revoke <service>` — remove stored credentials
- [ ] Unit tests for credential retrieval patterns
- [ ] Unit tests for audit log entry format

## Acceptance Criteria

- All service credentials are retrieved from nakama-vault
- OAuth token refresh works automatically for Google Calendar
- Audit log captures all data fetches and AI calls
- Audit log contains no sensitive data (tokens, meeting content)
- `tensai auth status` shows authentication state for all services
- Fallback to environment variables works when vault is unavailable
- Plaintext secrets in config files generate warnings

## Dependencies

- Step 001 (CLI scaffold)
- nakama-vault shared crate must expose credential CRUD API
- nakama-audit shared crate must expose structured audit logging
- Google OAuth2 flow requires user interaction (browser-based)
