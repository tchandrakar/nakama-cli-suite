# Step 007: Environment Manager

## Objective

Build an environment management system that supports named environment definitions (dev, staging, prod), variable substitution in requests, an --env flag for environment selection, and secrets retrieval from nakama-vault for environment-specific credentials.

## Tasks

- [ ] Define `Environment` struct: name, base_url, variables (HashMap), auth_config, description
- [ ] Define `EnvironmentConfig` struct: environments (Vec<Environment>), active_default
- [ ] Implement environment file format:
  - [ ] YAML/TOML environment definitions
  - [ ] Support project-local (`.gate/environments.yaml`) and global environments
  - [ ] Example:
    ```yaml
    environments:
      dev:
        base_url: http://localhost:3000
        variables:
          api_version: v1
        auth: bearer:vault:dev-api-token
      staging:
        base_url: https://staging.example.com
        variables:
          api_version: v1
        auth: bearer:vault:staging-api-token
    ```
- [ ] Implement variable substitution:
  - [ ] Syntax: `{{variable_name}}` in URLs, headers, body
  - [ ] Environment variables: `{{env.base_url}}`, `{{env.api_version}}`
  - [ ] Request chain variables: `{{response.data.id}}` (from previous request in flow)
  - [ ] System variables: `{{$timestamp}}`, `{{$uuid}}`, `{{$random_int}}`
  - [ ] Vault references: `{{vault:secret-name}}`
  - [ ] Nested variable resolution
- [ ] Implement secrets via nakama-vault:
  - [ ] `vault:` prefix in variable values triggers vault lookup
  - [ ] Environment-specific vault keys
  - [ ] Cache vault lookups for session duration
  - [ ] Clear error when vault key not found
- [ ] Implement `--env` flag:
  - [ ] Select active environment per request
  - [ ] Override default environment
  - [ ] `gate send --env staging GET /api/users`
  - [ ] Show active environment in output
- [ ] Implement environment management commands:
  - [ ] `gate env list` — list all environments
  - [ ] `gate env show <name>` — show environment details (redact secrets)
  - [ ] `gate env add <name>` — add new environment interactively
  - [ ] `gate env edit <name>` — edit environment
  - [ ] `gate env delete <name>` — remove environment
  - [ ] `gate env set-default <name>` — set default environment
  - [ ] `gate env vars <name>` — list variables (redact vault refs)
- [ ] Implement environment inheritance:
  - [ ] Environments can extend a base environment
  - [ ] Override specific variables while inheriting rest
  - [ ] Common auth config shared across environments
- [ ] Implement environment validation:
  - [ ] Validate base_url is reachable (optional health check)
  - [ ] Validate vault references resolve
  - [ ] Warn about undefined variable references
- [ ] Unit tests for variable substitution
- [ ] Unit tests for environment loading and resolution
- [ ] Unit tests for vault reference handling

## Acceptance Criteria

- Environment definitions are loaded from YAML/TOML files
- Variable substitution works in URLs, headers, and request bodies
- `--env` flag correctly selects the active environment
- Vault references are resolved transparently
- System variables generate unique values per request
- Environment inheritance reduces configuration duplication
- Undefined variable references produce clear error messages
- Secrets are never displayed in plaintext (redacted in env show)

## Dependencies

- Step 001 (CLI scaffold)
- Step 005 (Security) for vault integration
- nakama-vault shared crate for secret resolution
- serde_yaml for environment file parsing
