# Step 005: Security Integration

## Objective

Integrate gate with nakama-vault for secure token and API key management, implement automatic auth header injection, ensure sensitive headers are never logged, enforce SSL by default, and maintain comprehensive audit logging of all HTTP requests.

## Tasks

- [ ] Integrate nakama-vault for credential retrieval:
  - [ ] Bearer tokens per service/environment
  - [ ] API keys per service
  - [ ] Basic auth credentials
  - [ ] OAuth2 client credentials
  - [ ] Client certificates for mTLS
  - [ ] Fallback to environment variables
- [ ] Implement auth header injection:
  - [ ] Auto-inject auth from vault based on request URL/environment
  - [ ] Match URL patterns to stored credentials
  - [ ] Support multiple auth strategies: Bearer, Basic, API-Key (header/query), OAuth2
  - [ ] `--auth` flag overrides auto-injection
  - [ ] `--no-auth` flag disables auto-injection
- [ ] Implement sensitive header protection:
  - [ ] Never log Authorization header values
  - [ ] Never log Cookie header values
  - [ ] Never log X-API-Key header values
  - [ ] Redact in history storage (show header name but mask value)
  - [ ] Configurable additional sensitive headers
- [ ] Implement SSL enforcement:
  - [ ] Default: require HTTPS for all requests
  - [ ] `--insecure` flag to allow HTTP (with warning)
  - [ ] Automatic HTTP -> HTTPS upgrade (optional)
  - [ ] Certificate validation errors are clear and actionable
  - [ ] TLS minimum version enforcement (see Step 003)
- [ ] Implement audit logging via nakama-audit:
  - [ ] Log every HTTP request (method, URL, timestamp, status, duration)
  - [ ] Log auth method used (type, not value)
  - [ ] Log retry attempts
  - [ ] Log redirect chains
  - [ ] Log WebSocket connections
  - [ ] Log import events (source format, request count)
  - [ ] No sensitive data in audit logs (no headers, no body content)
- [ ] Implement request body security:
  - [ ] Detect secrets in request body before sending (warn user)
  - [ ] Redact body in history for requests to sensitive endpoints
  - [ ] `--redact-body` flag for sensitive requests
- [ ] Add `--vault-key` flag to specify which vault credential to use
- [ ] Add `--audit-log` flag for audit log location
- [ ] Unit tests for auth injection (URL pattern matching)
- [ ] Unit tests for sensitive header redaction
- [ ] Unit tests for audit log format

## Acceptance Criteria

- All tokens/API keys are retrieved from nakama-vault
- Auth headers are automatically injected based on URL and environment
- Authorization, Cookie, and API key headers are never logged in plaintext
- SSL is enforced by default, HTTP requires explicit --insecure flag
- Audit log captures all HTTP requests with metadata but no sensitive data
- Body secrets are detected and user is warned
- Credential matching works across environments (dev/staging/prod)
- Fallback to environment variables works when vault is unavailable

## Dependencies

- Step 001 (CLI scaffold)
- Step 003 (HTTP engine) for request/response pipeline
- nakama-vault shared crate for credential retrieval
- nakama-audit shared crate for structured audit logging
