# Step 003: HTTP Engine

## Objective

Build a robust HTTP execution engine using reqwest with connection pooling, TLS configuration (minimum TLS 1.2, optional certificate pinning), redirect following, retry with exponential backoff, configurable timeouts, response streaming, and WebSocket support via tokio-tungstenite.

## Tasks

- [ ] Add reqwest (with rustls-tls), tokio-tungstenite dependencies
- [ ] Implement `HttpEngine` struct with connection pool:
  - [ ] Shared reqwest::Client with configurable pool size
  - [ ] Connection reuse across requests
  - [ ] Per-host connection limits
- [ ] Implement TLS configuration:
  - [ ] Minimum TLS 1.2 (reject TLS 1.0, 1.1)
  - [ ] Custom CA certificate support (--ca-cert flag)
  - [ ] Client certificate support (--client-cert, --client-key)
  - [ ] Certificate pinning (optional, via config)
  - [ ] `--insecure` flag to skip TLS verification (with warning)
- [ ] Implement redirect handling:
  - [ ] Follow redirects (configurable max, default 10)
  - [ ] `--no-redirect` flag to disable
  - [ ] Log redirect chain
  - [ ] Handle same-origin vs cross-origin redirects
- [ ] Implement retry with backoff:
  - [ ] Configurable retry count (default 0, --retry N)
  - [ ] Exponential backoff with jitter
  - [ ] Retry on: connection error, 429 (respect Retry-After), 5xx
  - [ ] Do not retry on: 4xx (except 429), successful responses
  - [ ] Log retry attempts
- [ ] Implement timeout configuration:
  - [ ] Connection timeout (default 10s)
  - [ ] Request timeout (default 30s)
  - [ ] `--timeout` flag to override
  - [ ] Per-request timeout in flow definitions
- [ ] Implement response streaming:
  - [ ] Stream large responses to avoid OOM
  - [ ] Progress bar for large downloads
  - [ ] Save to file (`--output file.json`)
  - [ ] Stream to stdout for piping
- [ ] Implement WebSocket support:
  - [ ] Connect to WebSocket endpoint
  - [ ] Send/receive messages interactively
  - [ ] Support text and binary frames
  - [ ] Ping/pong handling
  - [ ] Auto-reconnect on disconnect (optional)
  - [ ] Message logging
- [ ] Implement request/response timing:
  - [ ] DNS resolution time
  - [ ] TCP connection time
  - [ ] TLS handshake time
  - [ ] Time to first byte (TTFB)
  - [ ] Total response time
  - [ ] Response size (bytes)
- [ ] Implement HTTP/2 support:
  - [ ] Enable HTTP/2 when available
  - [ ] Fallback to HTTP/1.1
  - [ ] Display protocol version in response
- [ ] Add `--timing` flag to show detailed timing breakdown
- [ ] Add `--raw` flag to show raw response (no formatting)
- [ ] Unit tests for retry logic, timeout handling
- [ ] Integration tests against httpbin or mock server

## Acceptance Criteria

- Connection pooling reuses connections for same-host requests
- TLS 1.2 minimum is enforced (TLS 1.0/1.1 rejected)
- Certificate pinning works when configured
- Retry with backoff handles transient failures correctly
- 429 responses respect Retry-After header
- Response streaming handles >100MB responses without OOM
- WebSocket connection works for text and binary messages
- Timing breakdown shows accurate per-phase durations
- HTTP/2 is used when the server supports it

## Dependencies

- Step 001 (CLI scaffold)
- Step 002 (Request builder) for HttpRequest input
- reqwest crate with rustls-tls feature
- tokio-tungstenite for WebSocket support
