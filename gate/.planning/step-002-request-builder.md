# Step 002: Request Builder

## Objective

Build a flexible request builder that supports three input modes: manual construction via CLI flags, natural language request building via nakama-ai, and import from existing formats (cURL commands, OpenAPI specs, Postman collections, HAR files).

## Tasks

- [ ] Define `HttpRequest` struct: method, url, headers (Vec<(String,String)>), body, query_params, auth, timeout, follow_redirects
- [ ] Define `RequestSource` enum: Manual, NaturalLanguage, CurlImport, OpenApiImport, PostmanImport, HarImport
- [ ] Implement manual request builder:
  - [ ] Method flag: `--method GET/POST/PUT/PATCH/DELETE/HEAD/OPTIONS`
  - [ ] URL as positional argument
  - [ ] Headers: `--header "Content-Type: application/json"` (repeatable)
  - [ ] Body: `--body '{"key": "value"}'`, `--body @file.json`
  - [ ] Query params: `--query "key=value"` (repeatable)
  - [ ] Auth: `--auth bearer:token`, `--auth basic:user:pass`
  - [ ] Shorthand: `gate send GET https://api.example.com/users`
  - [ ] Content-Type inference from body format
- [ ] Implement natural language request builder:
  - [ ] Accept NL description: `gate send "Get all users from the API"`
  - [ ] Send description to nakama-ai for translation
  - [ ] Prompt includes available environment context (base URL, auth)
  - [ ] Parse LLM response into HttpRequest struct
  - [ ] Show interpreted request for user confirmation
  - [ ] `--confirm` flag to require confirmation before sending
- [ ] Implement cURL import:
  - [ ] Parse cURL command string
  - [ ] Extract method, URL, headers, body, auth
  - [ ] Handle cURL flags: `-H`, `-d`, `-X`, `-u`, `-b`, `--data-raw`
  - [ ] Handle multiline cURL commands (backslash continuation)
- [ ] Implement OpenAPI import:
  - [ ] Parse OpenAPI 3.0/3.1 YAML/JSON specs
  - [ ] Extract endpoints, methods, parameters, request bodies
  - [ ] Generate sample requests from schema definitions
  - [ ] Store as collection for reuse
- [ ] Implement Postman collection import:
  - [ ] Parse Postman Collection v2.1 JSON format
  - [ ] Extract requests with headers, body, auth
  - [ ] Preserve folder structure as collection groups
  - [ ] Map Postman variables to gate environment variables
- [ ] Implement HAR import:
  - [ ] Parse HAR 1.2 JSON format
  - [ ] Extract request entries (method, URL, headers, body)
  - [ ] Filter by domain, status code, content type
  - [ ] Convert to gate request format
- [ ] Implement `import` subcommand:
  - [ ] `gate import curl "curl -X POST ..."` — from cURL command
  - [ ] `gate import openapi api.yaml` — from OpenAPI spec
  - [ ] `gate import postman collection.json` — from Postman
  - [ ] `gate import har recording.har` — from HAR file
  - [ ] Auto-detect format when possible
- [ ] Unit tests for each import format parser
- [ ] Unit tests for NL request building with mock LLM

## Acceptance Criteria

- Manual request building supports all common HTTP methods and options
- NL request building translates natural language to valid HTTP requests
- cURL import handles common cURL patterns (headers, body, auth)
- OpenAPI import generates requests from spec definitions
- Postman import preserves collection structure and variables
- HAR import extracts requests from browser recordings
- All importers produce valid HttpRequest structs
- Content-Type is inferred when not explicitly set

## Dependencies

- Step 001 (CLI scaffold) must be complete
- nakama-ai shared crate for NL request building
- serde_yaml for OpenAPI parsing
- serde_json for Postman and HAR parsing
