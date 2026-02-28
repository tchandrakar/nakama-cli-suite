# Step 006: Security Integration

## Objective

Integrate sharingan with the nakama-vault shared crate for secure credential management, implement PII detection and redaction before any data is sent to an LLM, and ensure comprehensive audit logging of all sensitive operations.

## Tasks

- [ ] Integrate nakama-vault for credential retrieval:
  - [ ] AWS credentials for CloudWatch ingestor
  - [ ] Kubernetes kubeconfig/tokens for kube-rs
  - [ ] Docker socket/TLS credentials
  - [ ] LLM API keys for nakama-ai
  - [ ] Fallback to environment variables when vault is unavailable
- [ ] Implement PII detection engine:
  - [ ] Regex-based detectors for: email, phone, SSN, credit card, IP address
  - [ ] Custom PII pattern definitions via config
  - [ ] Named entity detection for names (optional, regex-based heuristic)
  - [ ] API key / secret pattern detection (AWS keys, JWT tokens, password fields)
  - [ ] Configurable sensitivity levels (strict, moderate, permissive)
- [ ] Implement PII redaction:
  - [ ] Replace detected PII with type-tagged placeholders (`[EMAIL_REDACTED]`, `[SSN_REDACTED]`)
  - [ ] Maintain redaction map for consistent replacement within a session
  - [ ] Apply redaction before any LLM submission (mandatory, non-bypassable)
  - [ ] Log redaction statistics (count per type) without logging actual PII
- [ ] Implement audit logging via nakama-audit:
  - [ ] Log every LLM API call (timestamp, prompt hash, model, token count)
  - [ ] Log every remote source connection (type, target, user)
  - [ ] Log every file access (path, read/watch)
  - [ ] Log PII redaction events (count, types)
  - [ ] Log credential access events (which vault key, when)
  - [ ] Audit log is append-only and tamper-evident (hashed chain)
- [ ] Add `--redact` flag (on by default, `--no-redact` to disable for local-only analysis)
- [ ] Add `--audit-log` flag to specify audit log location
- [ ] Unit tests for PII detection (known patterns)
- [ ] Unit tests for PII redaction (verify no leaks)
- [ ] Integration test: verify redacted content is sent to mock LLM

## Acceptance Criteria

- All cloud credentials are retrieved from nakama-vault (no hardcoded secrets)
- PII detection catches email, phone, SSN, credit card, IP, API keys
- PII is redacted before any data leaves the local machine (LLM calls)
- Audit log captures all sensitive operations with timestamps
- Audit log contains no actual PII or secrets
- `--no-redact` only disables redaction, never disables audit logging
- Custom PII patterns can be added via configuration

## Dependencies

- Step 002 (Log ingestors) for credential needs
- Step 005 (Deep analysis) for LLM submission pipeline
- nakama-vault shared crate must expose credential retrieval API
- nakama-audit shared crate must expose structured audit logging
