# Step 006: Security Integration

## Objective

Integrate itachi with nakama-vault for secure credential management, implement scope validation for Atlassian API calls, ensure PII awareness to prevent sending employee data to the LLM, and implement comprehensive audit logging for every API call.

## Tasks

- [ ] Add `nakama-vault` shared crate dependency (if not already added in step 002)
- [ ] Implement credential storage via nakama-vault:
  - Atlassian API token: `itachi.atlassian.email`, `itachi.atlassian.api_token`
  - OAuth tokens: `itachi.atlassian.oauth_access_token`, `itachi.atlassian.oauth_refresh_token`
  - PAT: `itachi.atlassian.pat`
  - Instance URL: `itachi.atlassian.instance_url`
  - Environment variable fallbacks for all credentials
- [ ] Implement scope validation:
  - Before making API calls, verify the auth method has required scopes
  - For OAuth: check granted scopes match required scopes for the operation
  - For API tokens: warn if certain operations may fail due to permissions
  - Provide clear error messages when permission is insufficient
- [ ] Implement PII awareness:
  - Identify PII fields in Jira/Confluence data:
    - User email addresses
    - User display names (when not the querying user)
    - Phone numbers, employee IDs in custom fields
  - Before sending data to LLM (nakama-ai):
    - Anonymize user data (replace names with `User-1`, `User-2`, etc.)
    - Strip email addresses
    - Remove custom fields marked as PII in configuration
  - Maintain a de-anonymization map to restore names in output
  - Configuration: `[security] pii_fields = ["customfield_10050", "phone"]`
- [ ] Implement comprehensive audit logging via nakama-audit:
  - Log every Jira API call (endpoint, method, JQL query, result count, latency)
  - Log every Confluence API call (endpoint, CQL query, result count, latency)
  - Log every NL query translation (input hash, generated JQL/CQL, validation result)
  - Log every LLM call (purpose, token count, latency)
  - Never log full issue descriptions or page content (only metadata)
  - Include request correlation ID
- [ ] Implement credential security:
  - Never print credentials to terminal
  - Mask tokens in debug/verbose log output
  - Warn about insecure auth methods (basic auth over HTTP)
- [ ] Write unit tests for PII anonymization
- [ ] Write tests to verify credential security (no leaks in logs)

## Acceptance Criteria

- All credentials are stored securely in nakama-vault
- Scope validation catches insufficient permissions before API calls
- PII is anonymized before being sent to the LLM
- De-anonymization correctly restores names in final output
- Every API call is audit-logged with appropriate metadata
- Credentials never appear in terminal output or logs
- PII fields are configurable per project

## Dependencies

- Step 002 (Auth Layer) must be complete for credential management
- Step 003 (Jira Client) must be complete for API call interception
- Step 004 (Confluence Client) must be complete for API call interception
- `nakama-vault` shared crate must be available
- `nakama-audit` shared crate must be available
