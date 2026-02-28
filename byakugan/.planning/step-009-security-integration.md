# Step 009: Security Integration

## Objective

Integrate byakugan with `nakama-vault` for secure credential management, implement OAuth2 flows per platform, enforce confirmation before posting reviews/comments to platforms, and ensure comprehensive audit logging of all API calls and review actions.

## Tasks

- [ ] Add `nakama-vault` shared crate dependency
- [ ] Implement credential retrieval via nakama-vault:
  - GitHub token: `nakama-vault get github.token` with `GITHUB_TOKEN` env var fallback
  - GitLab token: `nakama-vault get gitlab.token` with `GITLAB_TOKEN` env var fallback
  - Bitbucket credentials: `nakama-vault get bitbucket.username` + `nakama-vault get bitbucket.app_password` with env var fallback
  - Support multiple tokens per platform (for different orgs/instances)
- [ ] Implement OAuth2 flows per platform:
  - **GitHub OAuth App**: Authorization code flow with PKCE, browser redirect, localhost callback
  - **GitLab OAuth2**: Authorization code flow, configurable for self-hosted instances
  - **Bitbucket OAuth2**: Authorization code grant with consumer key/secret
  - Token storage: save tokens in nakama-vault after OAuth flow completes
  - Token refresh: auto-refresh expired OAuth tokens
- [ ] Implement `byakugan auth` subcommand:
  - `byakugan auth login --platform=github` -- initiate OAuth flow
  - `byakugan auth status` -- show authentication status for all platforms
  - `byakugan auth logout --platform=github` -- revoke and remove tokens
- [ ] Implement confirmation gate for write operations:
  - Before `post_comment()`: display the comment content and ask for user confirmation
  - Before `post_review()`: display the full review summary and ask for confirmation
  - `--yes` / `-y` flag to skip confirmation (for automation)
  - `--dry-run` flag to show what would be posted without actually posting
- [ ] Implement comprehensive audit logging:
  - Log every platform API call (endpoint, method, response status, timestamp)
  - Log every review posted (platform, PR, findings count, verdict)
  - Log every comment posted (platform, PR, file, line)
  - Store audit log via `nakama-audit` shared crate
  - Never log token values or full credentials
- [ ] Implement token security best practices:
  - Never print tokens to terminal output
  - Mask tokens in debug/verbose log output
  - Use secure memory handling for token values where possible
  - Warn if tokens have overly broad scopes
- [ ] Write unit tests for credential retrieval and audit logging
- [ ] Write integration tests for the confirmation flow

## Acceptance Criteria

- Platform tokens are retrieved from nakama-vault with env var fallback
- OAuth2 flows work for all three platforms (GitHub, GitLab, Bitbucket)
- Users are prompted for confirmation before any write operation (unless `--yes` flag)
- `--dry-run` shows review content without posting
- All API calls are audit-logged with timestamps
- Tokens are never exposed in logs or terminal output
- `byakugan auth status` correctly shows authentication state per platform

## Dependencies

- Step 003, 004, 005 (platform adapters) must be complete
- `nakama-vault` shared crate must be available
- `nakama-audit` shared crate must be available
- `oauth2` crate for OAuth2 flows
- `tokio` for localhost callback server
