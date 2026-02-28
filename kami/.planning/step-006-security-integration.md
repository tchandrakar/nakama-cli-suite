# Step 006: Security Integration

## Objective

Integrate kami with nakama-vault for secure credential management, ensure Google OAuth tokens and API keys are stored securely, prevent sensitive queries from being logged, and implement comprehensive audit logging for every search, URL fetch, and AI call.

## Tasks

- [ ] Add `nakama-vault` shared crate dependency (if not already added in step 002)
- [ ] Implement credential storage via nakama-vault:
  - Google OAuth2 access token: `kami.google.access_token`
  - Google OAuth2 refresh token: `kami.google.refresh_token`
  - Gemini API key: `kami.gemini.api_key`
  - Custom Search API key: `kami.google.search_api_key`
  - Custom Search engine ID: `kami.google.search_cx`
  - Environment variable fallbacks: `GEMINI_API_KEY`, `GOOGLE_SEARCH_API_KEY`, `GOOGLE_SEARCH_CX`
- [ ] Implement sensitive query protection:
  - Never log full query text in audit logs (hash instead)
  - Never include query text in error reports
  - Implement `--sensitive` flag that further restricts logging
  - Warn if query appears to contain personal information (PII regex heuristics)
- [ ] Implement comprehensive audit logging via nakama-audit:
  - Log every Gemini API call (endpoint, model, token count, latency, success/failure)
  - Log every Google Search API call (query hash, result count, latency)
  - Log every URL fetch (URL, status code, content size, latency)
  - Log session starts/ends for conversational mode
  - Never log response content (only metadata)
  - Include request ID for correlation
- [ ] Implement token security best practices:
  - Never print API keys or OAuth tokens to terminal
  - Mask credentials in debug/verbose log output
  - Use secure memory handling where possible
  - Clear tokens from memory after use in requests
- [ ] Implement API quota monitoring:
  - Track Gemini API usage (requests per minute, tokens per day)
  - Track Custom Search API usage (queries per day, quota limit)
  - Warn when approaching quota limits
  - Display quota status in `kami auth status`
- [ ] Write unit tests for credential retrieval and audit logging
- [ ] Write tests to verify sensitive data is never logged

## Acceptance Criteria

- All credentials are stored securely in nakama-vault
- Environment variable fallbacks work when vault is unavailable
- Sensitive queries are never logged in plain text
- Every API call is audit-logged with appropriate metadata
- API keys and tokens are never visible in terminal output or logs
- Quota monitoring warns when approaching limits
- `--sensitive` flag provides additional privacy protection

## Dependencies

- Step 002 (Auth Layer) must be complete for token management
- `nakama-vault` shared crate must be available
- `nakama-audit` shared crate must be available
