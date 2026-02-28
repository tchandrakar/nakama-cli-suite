# Step 007: Security Integration

## Objective

Integrate mugen with nakama-vault for secure API key management, ensure generated tests never expose secrets or sensitive data, and maintain an audit log of every test generation event.

## Tasks

- [ ] Integrate nakama-vault for credential retrieval:
  - [ ] LLM API keys for nakama-ai
  - [ ] Any service credentials for integration test generation
  - [ ] Fallback to environment variables when vault is unavailable
- [ ] Implement secret detection in generated tests:
  - [ ] Scan generated test code for hardcoded secrets
  - [ ] Detect API keys, passwords, tokens in test values
  - [ ] Detect real email addresses, phone numbers in test data
  - [ ] Detect actual database connection strings
  - [ ] Replace detected secrets with placeholder/mock values
  - [ ] Block test writing if secrets detected and not replaceable
- [ ] Implement source code secret awareness:
  - [ ] Do not send code containing secrets to LLM
  - [ ] Redact secrets in source code before prompt assembly
  - [ ] Detect .env file references and redact values
  - [ ] Detect hardcoded credentials in source under test
- [ ] Implement audit logging via nakama-audit:
  - [ ] Log every test generation request (function, file, strategy, timestamp)
  - [ ] Log every LLM API call (prompt hash, model, token count, cost estimate)
  - [ ] Log every validation attempt (stage, pass/fail, error type)
  - [ ] Log every file write (path, test count, validation status)
  - [ ] Log credential access events
  - [ ] No source code or test code in audit logs (only metadata)
- [ ] Implement test data safety:
  - [ ] Generate test data using faker-style generators, not real data
  - [ ] Ensure generated database URLs point to localhost/test
  - [ ] Ensure generated API URLs point to mock/localhost
  - [ ] Configurable allowed test domains (default: localhost, example.com)
- [ ] Add `--audit-log` flag for audit log location
- [ ] Add `--no-vault` flag to use only env vars
- [ ] Unit tests for secret detection in generated code
- [ ] Unit tests for source code redaction

## Acceptance Criteria

- All API keys are retrieved from nakama-vault
- Generated tests contain no real secrets, credentials, or PII
- Source code with secrets is redacted before LLM submission
- Audit log captures all generation events with metadata
- Audit log contains no actual source code or secrets
- Test data uses safe placeholder values (localhost, example.com)
- Secret detection catches common credential patterns

## Dependencies

- Step 005 (LLM test generator) for prompt assembly pipeline
- Step 006 (Validation loop) for file writing pipeline
- nakama-vault shared crate for credential retrieval
- nakama-audit shared crate for structured audit logging
