# Step 009: Security Integration

## Objective
Integrate jogan with `nakama-vault` for secure cloud credential management, implement read-only kubeconfig handling, validate IAM scope to ensure minimal required permissions, and wire up `nakama-audit` for comprehensive audit logging of all diagnostic operations.

## Tasks
- [ ] Integrate `nakama-vault` for credential retrieval:
  - Fetch cloud API keys/tokens from vault (AWS, GCP, Azure)
  - Fetch LLM API keys from vault
  - Support credential rotation with automatic re-fetch on auth failures
  - Cache credentials in memory only (never write to disk)
  - Support multiple credential profiles for different environments
- [ ] Implement read-only kubeconfig handling:
  - Open kubeconfig as read-only (never modify)
  - Validate kubeconfig permissions before access
  - Support kubeconfig from vault (instead of filesystem)
  - Detect and warn about admin-level kubeconfig contexts
  - Recommend read-only service account for production use
- [ ] Implement IAM scope validation:
  - Before collecting, verify required permissions are available
  - AWS: use `sts:GetCallerIdentity` and IAM policy simulation
  - Kubernetes: use SelfSubjectAccessReview
  - Report missing permissions with exact policy statements needed
  - Operate in degraded mode (skip what cannot be accessed)
- [ ] Wire up `nakama-audit` for all operations:
  - `diagnosis_started`: log symptoms, target infrastructure
  - `data_collected`: log collector name, resource types, data volume
  - `rules_evaluated`: log rules run, findings count, severities
  - `llm_analysis_run`: log prompt size, response size, model used
  - `diagnosis_complete`: log total findings, root cause, confidence
  - `watch_started`: log monitored resources, interval
  - `credential_accessed`: log which credentials were used (not the credentials themselves)
- [ ] Add audit context: user identity, target cluster/environment, session ID
- [ ] Ensure no credentials, tokens, or sensitive infrastructure details appear in audit logs
- [ ] Implement data redaction for audit: strip IPs, hostnames, and secrets from logged data
- [ ] Write tests for vault credential retrieval with mock vault
- [ ] Write tests for IAM scope validation
- [ ] Write tests for audit event emission and redaction

## Acceptance Criteria
- Cloud credentials are retrieved from `nakama-vault` and never persisted to disk
- Kubeconfig is accessed read-only with appropriate warnings for admin contexts
- IAM scope validation reports missing permissions before collection begins
- All diagnostic operations emit appropriate audit events
- Audit logs contain no credentials, tokens, or sensitive infrastructure details
- Tests validate vault integration, IAM checks, and audit emission

## Dependencies
- Step 001 (CLI scaffold and audit wiring)
- Step 003-005 (collectors need credentials)
- Step 006-008 (analysis operations need audit logging)
