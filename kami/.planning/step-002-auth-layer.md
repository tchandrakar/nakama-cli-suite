# Step 002: Auth Layer

## Objective

Implement the authentication layer for Google APIs, supporting OAuth2 with PKCE for browser-based consent, localhost redirect callback for token capture, secure token storage in nakama-vault, automatic token refresh, API key fallback for simpler setups, and proper scope management for Gemini and Custom Search APIs.

## Tasks

- [ ] Add dependencies: `oauth2` (oauth2-rs), `reqwest`, `tokio`, `serde`
- [ ] Add `nakama-vault` shared crate dependency
- [ ] Implement the OAuth2 + PKCE flow for Google:
  - Configure Google OAuth2 client (client_id, client_secret from config/env)
  - Generate authorization URL with PKCE code verifier
  - Request scopes: `https://www.googleapis.com/auth/generative-language` (Gemini), `https://www.googleapis.com/auth/cse` (Custom Search)
  - Open browser to Google consent page (use `open` crate for cross-platform)
- [ ] Implement localhost redirect callback:
  - Start a temporary HTTP server on `localhost:PORT` (random available port)
  - Listen for the OAuth2 redirect with authorization code
  - Exchange authorization code for access token and refresh token
  - Shut down the temporary server after token exchange
  - Handle timeout if user doesn't complete consent
- [ ] Implement token storage via nakama-vault:
  - Store access token, refresh token, and expiry timestamp
  - Key namespace: `kami.google.access_token`, `kami.google.refresh_token`, `kami.google.token_expiry`
  - Encrypt tokens at rest via nakama-vault's encryption
- [ ] Implement auto-refresh:
  - Check token expiry before each API call
  - If expired or within 5-minute window, use refresh token to obtain new access token
  - Update stored tokens after refresh
  - Handle refresh token revocation (prompt re-authentication)
- [ ] Implement API key fallback:
  - Support `GEMINI_API_KEY` environment variable
  - Support `api_key` field in `~/.kami/config.toml`
  - Store API key in nakama-vault
  - Use API key when OAuth2 is not configured
- [ ] Implement scope management:
  - Track which scopes are granted
  - Request additional scopes if a command needs them
  - Display current scopes in `kami auth status`
- [ ] Implement `kami auth` subcommand:
  - `kami auth login` -- initiate OAuth2 flow in browser
  - `kami auth login --api-key` -- prompt for API key and store it
  - `kami auth status` -- show current auth method, token expiry, granted scopes
  - `kami auth logout` -- revoke tokens and remove from vault
  - `kami auth refresh` -- manually force token refresh
- [ ] Write unit tests for token management and refresh logic
- [ ] Write integration tests for the OAuth2 flow (with mocked Google endpoints)

## Acceptance Criteria

- `kami auth login` opens browser, completes OAuth2 flow, and stores tokens
- Tokens are securely stored in nakama-vault (not plain text on disk)
- Auto-refresh works transparently before API calls
- API key fallback works when OAuth2 is not configured
- `kami auth status` correctly reports authentication state
- `kami auth logout` revokes and removes tokens
- OAuth2 flow handles errors gracefully (timeout, denied consent, network errors)

## Dependencies

- Step 001 (CLI scaffold) must be complete
- `nakama-vault` shared crate must be available
- `oauth2` crate for OAuth2 flows
- `open` crate for browser launching
- Google Cloud project with OAuth2 credentials configured
