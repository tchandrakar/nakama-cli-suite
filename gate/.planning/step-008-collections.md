# Step 008: Collections

## Objective

Build a request collection system that organizes requests into named, shareable groups defined in YAML or JSON format. Collections support collection-level variables, shared authentication, folder organization, and can be imported from or exported to Postman-compatible formats.

## Tasks

- [ ] Define `Collection` struct: name, description, requests, folders, variables, auth, version
- [ ] Define `CollectionRequest` struct: name, method, url, headers, body, params, pre_script, post_script, description
- [ ] Define `CollectionFolder` struct: name, description, requests, sub_folders, auth_override
- [ ] Implement collection file format:
  - [ ] YAML and JSON support
  - [ ] Store in project-local `.gate/collections/` directory
  - [ ] Support global collections at `~/.config/nakama/gate/collections/`
  - [ ] Example:
    ```yaml
    name: User API
    base_url: "{{env.base_url}}"
    auth: bearer:vault:api-token
    folders:
      - name: Users
        requests:
          - name: List Users
            method: GET
            url: /api/v1/users
          - name: Create User
            method: POST
            url: /api/v1/users
            body:
              name: "{{$random_name}}"
              email: "{{$random_email}}"
    ```
- [ ] Implement collection-level features:
  - [ ] Collection-level variables (applied to all requests)
  - [ ] Collection-level auth (inherited by all requests, overridable)
  - [ ] Collection-level headers (applied to all requests)
  - [ ] Request-level overrides for any collection-level setting
- [ ] Implement collection management commands:
  - [ ] `gate collection list` — list all collections
  - [ ] `gate collection show <name>` — show collection details
  - [ ] `gate collection run <name>` — run all requests in collection
  - [ ] `gate collection run <name> --request <req_name>` — run specific request
  - [ ] `gate collection create <name>` — create new collection interactively
  - [ ] `gate collection add <name>` — add request to collection (from last request or flags)
  - [ ] `gate collection delete <name>` — delete collection
  - [ ] `gate collection export <name> --format postman` — export to Postman format
- [ ] Implement collection execution:
  - [ ] Run individual requests within collection
  - [ ] Run all requests in folder
  - [ ] Run all requests in collection (sequential)
  - [ ] Variable substitution from collection + environment
  - [ ] Report results (pass/fail per request based on status code)
- [ ] Implement collection sharing:
  - [ ] Collections are plain YAML/JSON files (version-controllable)
  - [ ] No embedded secrets (use vault references)
  - [ ] Include/exclude patterns for sensitive requests
  - [ ] Export for sharing (strip vault references, add placeholders)
- [ ] Implement collection import:
  - [ ] Import from Postman collection format
  - [ ] Import from OpenAPI spec (generate collection from endpoints)
  - [ ] Import from HAR (group by domain)
  - [ ] Merge imported requests into existing collection
- [ ] Unit tests for collection loading and variable resolution
- [ ] Unit tests for collection execution with mock HTTP
- [ ] Unit tests for import/export format conversion

## Acceptance Criteria

- Collections organize requests into named, nested folder structures
- Collection-level variables and auth are inherited by all requests
- Requests can override collection-level settings
- Collections are stored as human-readable YAML/JSON files
- `collection run` executes requests and reports results
- Export to Postman format produces valid Postman collections
- Import from Postman preserves folder structure and variables
- Collections contain no embedded secrets (vault references only)

## Dependencies

- Step 002 (Request builder) for request definitions
- Step 003 (HTTP engine) for request execution
- Step 007 (Environment manager) for variable substitution
- serde_yaml and serde_json for file format handling
