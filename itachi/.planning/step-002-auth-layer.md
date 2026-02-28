# Step 002: Auth Layer

## Objective

Implement the authentication layer for Atlassian APIs, supporting API token auth (email + token) for Cloud, OAuth 2.0 (3LO) for Cloud apps, Personal Access Tokens (PAT) for Data Center, all stored securely via nakama-vault with automatic token refresh for OAuth flows.

## Tasks

- [ ] Add dependencies: `oauth2` (oauth2-rs), `reqwest`, `tokio`, `serde`
- [ ] Add `nakama-vault` shared crate dependency
- [ ] Implement API token authentication (Atlassian Cloud):
  - Accept email + API token pair
  - Encode as HTTP Basic Auth: `base64(email:token)`
  - Store email and token in nakama-vault
  - Key namespace: `itachi.atlassian.email`, `itachi.atlassian.api_token`
  - Environment variable fallback: `ATLASSIAN_EMAIL`, `ATLASSIAN_API_TOKEN`
- [ ] Implement OAuth 2.0 (3LO) for Atlassian Cloud apps:
  - Authorization code grant flow
  - Configure OAuth client (client_id, client_secret, callback URL)
  - Open browser to Atlassian consent page
  - Localhost redirect callback to capture authorization code
  - Exchange code for access token and refresh token
  - Store tokens in nakama-vault
  - Auto-refresh expired tokens using refresh token
  - Handle scope management (read:jira-work, write:jira-work, read:confluence-content.all, etc.)
- [ ] Implement Personal Access Token (PAT) for Data Center:
  - Accept PAT directly (used as Bearer token)
  - Store PAT in nakama-vault
  - Key namespace: `itachi.atlassian.pat`
  - Environment variable fallback: `ATLASSIAN_PAT`
- [ ] Implement auth method auto-detection:
  - Check config for `auth_method` setting
  - If not configured, detect available credentials:
    1. Check for OAuth tokens in vault
    2. Check for API token pair in vault/env
    3. Check for PAT in vault/env
  - Fall back with clear error message if no credentials found
- [ ] Implement `itachi auth` subcommand:
  - `itachi auth login` -- interactive credential setup (prompts for method choice)
  - `itachi auth login --method=api-token` -- prompt for email and API token
  - `itachi auth login --method=oauth` -- initiate OAuth 3LO flow
  - `itachi auth login --method=pat` -- prompt for personal access token
  - `itachi auth status` -- show auth method, instance URL, token expiry (if OAuth)
  - `itachi auth logout` -- remove stored credentials
  - `itachi auth test` -- verify credentials by making a test API call
- [ ] Implement instance URL management:
  - Store Atlassian instance URL (e.g., `https://your-org.atlassian.net`)
  - Support multiple instances with named profiles
  - Default instance from config, override with `--instance` flag
- [ ] Write unit tests for auth encoding and token management
- [ ] Write integration tests for OAuth flow with mocked Atlassian endpoints

## Acceptance Criteria

- API token auth works for Atlassian Cloud (email + token)
- OAuth 2.0 (3LO) flow works with browser consent and token storage
- PAT auth works for Data Center/Server instances
- Credentials are securely stored in nakama-vault
- Auto-refresh works for OAuth tokens
- `itachi auth test` successfully validates credentials
- Auth method auto-detection works correctly
- Multiple Atlassian instances are supported

## Dependencies

- Step 001 (CLI scaffold) must be complete
- `nakama-vault` shared crate must be available
- `oauth2` crate for OAuth 2.0 flows
- `open` crate for browser launching
- Atlassian Cloud instance with API token or OAuth app configured
