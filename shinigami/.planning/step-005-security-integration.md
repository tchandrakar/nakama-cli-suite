# Step 005: Security Integration

## Objective
Integrate shinigami with `nakama-vault` for secure credential management, implement secrets scanning to prevent accidental credential commits, support signed commits (GPG/SSH), and wire up `nakama-audit` for comprehensive audit logging of all git operations.

## Tasks
- [ ] Integrate `nakama-vault` for credential retrieval:
  - Fetch LLM API keys from vault (not env vars or config files)
  - Fetch git signing keys if stored in vault
  - Support key rotation with automatic re-fetch on auth failures
  - Cache keys in memory only
- [ ] Implement secrets scanning in diffs:
  - Scan staged changes before commit for potential secrets
  - Pattern detection: API keys, tokens, passwords, private keys, connection strings
  - Regex patterns for common secret formats (AWS keys, GitHub tokens, JWT, etc.)
  - High-entropy string detection for unknown secret formats
  - Report findings with file path and line number
  - Block commit if secrets detected (override with `--allow-secrets` flag)
- [ ] Support signed commits:
  - Detect GPG/SSH signing configuration from git config
  - Pass signing flags to git commit when configured
  - Support GPG key ID specification via `--sign-key` flag
  - Support SSH signing via `--ssh-sign` flag
- [ ] Wire up `nakama-audit` for all operations:
  - `commit_generated`: log generated message, style, analysis results
  - `commit_executed`: log commit hash, author, timestamp
  - `commit_amended`: log old and new commit hashes
  - `secrets_detected`: log file paths (not the secrets themselves)
  - `changelog_generated`: log ref range, output path
  - `release_created`: log version, tag name
  - `hook_installed`: log hook type, path
- [ ] Add audit context: repository path, branch, user identity
- [ ] Ensure no secrets or diff content appears in audit logs
- [ ] Write tests for secrets scanning patterns
- [ ] Write tests for vault integration with mock vault
- [ ] Write tests for audit event emission

## Acceptance Criteria
- API keys are retrieved from `nakama-vault` and never persisted to disk
- Secrets scanner catches common credential patterns in staged diffs
- Commits are blocked when secrets are detected (unless explicitly overridden)
- Signed commits work when GPG/SSH is configured
- All git operations emit appropriate audit events
- Audit logs never contain actual secrets or sensitive diff content
- Tests validate scanning patterns and audit emissions

## Dependencies
- Step 001 (CLI scaffold and audit wiring)
- Step 002 (git interface for diff access)
- Step 004 (commit generator for integration)
