# Step 011: Mock Server

## Objective

Build an axum-based mock API server that can be generated from OpenAPI specs, supports a record mode (proxy requests, save responses, replay later), and allows latency and error injection for testing resilience of API consumers.

## Tasks

- [ ] Add axum, tower, hyper dependencies
- [ ] Implement `MockServer` struct: routes, port, record_mode, config
- [ ] Implement mock server from OpenAPI spec:
  - [ ] Parse OpenAPI 3.0/3.1 spec
  - [ ] Generate routes for each endpoint
  - [ ] Generate response bodies from schema examples
  - [ ] Generate response bodies from schema definitions (when no example)
  - [ ] Support all HTTP methods
  - [ ] Respect content types (JSON, XML, plain text)
  - [ ] Return appropriate status codes per endpoint definition
- [ ] Implement static mock definitions:
  - [ ] YAML/JSON mock definition files
  - [ ] Match requests by: method, path (exact/glob/regex), headers, body
  - [ ] Define response: status, headers, body (inline or file reference)
  - [ ] Multiple responses per route (round-robin, conditional)
  - [ ] Example:
    ```yaml
    mocks:
      - match:
          method: GET
          path: /api/users
        respond:
          status: 200
          body:
            data: [{ id: 1, name: "Test User" }]
    ```
- [ ] Implement record mode (proxy + save):
  - [ ] Proxy requests to real API server
  - [ ] Record all request/response pairs
  - [ ] Save recordings to file (HAR-like format)
  - [ ] Replay recorded responses (no real API needed)
  - [ ] Match by URL + method for replay
  - [ ] Update recordings on re-record
- [ ] Implement latency injection:
  - [ ] Global latency (add Nms to all responses)
  - [ ] Per-route latency
  - [ ] Random latency (min-max range)
  - [ ] Latency distribution (normal, uniform)
  - [ ] Simulate slow connections
- [ ] Implement error injection:
  - [ ] Return specific error codes (500, 503, 429)
  - [ ] Error percentage (fail N% of requests)
  - [ ] Connection drops (close without response)
  - [ ] Timeout simulation (delay beyond client timeout)
  - [ ] Partial response (truncated body)
- [ ] Implement dynamic responses:
  - [ ] Template variables in response bodies (request params, timestamps)
  - [ ] Request counting (first request returns X, second returns Y)
  - [ ] Stateful mocks (create -> get returns created entity)
- [ ] Implement `mock` subcommand:
  - [ ] `gate mock start` — start mock server
  - [ ] `gate mock start --spec api.yaml` — from OpenAPI spec
  - [ ] `gate mock start --mocks mocks.yaml` — from mock definitions
  - [ ] `gate mock record --target https://api.example.com` — record mode
  - [ ] `gate mock replay --recordings recorded.json` — replay mode
  - [ ] `--port` flag (default: auto-assign)
  - [ ] `--latency` flag for global latency injection
  - [ ] `--error-rate` flag for error injection percentage
- [ ] Implement mock server management:
  - [ ] CORS headers (configurable)
  - [ ] Request logging (all received requests)
  - [ ] Admin endpoint for runtime configuration changes
  - [ ] Graceful shutdown
- [ ] Unit tests for route matching and response generation
- [ ] Integration test: mock server from OpenAPI spec serves valid responses
- [ ] Integration test: record mode captures and replays

## Acceptance Criteria

- Mock server starts and serves responses from OpenAPI spec
- Static mock definitions are matched and served correctly
- Record mode proxies to real API and saves request/response pairs
- Replay mode serves recorded responses without network calls
- Latency injection adds configurable delay to responses
- Error injection returns configured error responses at specified rates
- Dynamic response templates include request data in responses
- CORS is properly configured for browser-based API consumers
- Mock server handles concurrent requests without issues

## Dependencies

- Step 002 (Request builder) for OpenAPI parsing (shared)
- axum crate for HTTP server
- tower crate for middleware (latency injection, logging)
- serde_yaml for mock definition parsing
