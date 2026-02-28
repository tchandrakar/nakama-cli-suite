# Step 006: Request History

## Objective

Build a request history system backed by SQLite that stores every request and response with timestamps and duration, supports search and filtering, and enables replay of historical requests.

## Tasks

- [ ] Add rusqlite (or sqlx with SQLite) dependency
- [ ] Define database schema:
  - [ ] `requests` table: id, method, url, headers_hash, body_hash, timestamp, environment
  - [ ] `responses` table: id, request_id, status, headers, body (compressed), duration_ms, size_bytes
  - [ ] `tags` table: id, request_id, tag_name
  - [ ] Indexes on timestamp, url, method, status
- [ ] Implement history storage:
  - [ ] Auto-save every request/response pair
  - [ ] Compress large response bodies (zstd)
  - [ ] Redact sensitive headers before storage
  - [ ] Configurable max history size (entries or disk space)
  - [ ] Auto-cleanup old entries (configurable retention period)
- [ ] Implement history search:
  - [ ] Search by URL pattern (glob or regex)
  - [ ] Filter by method (GET, POST, etc.)
  - [ ] Filter by status code (2xx, 4xx, 5xx, specific code)
  - [ ] Filter by time range (today, last week, custom range)
  - [ ] Filter by environment
  - [ ] Filter by tag
  - [ ] Full-text search in response bodies (optional)
  - [ ] Sort by: time (default), duration, status, url
- [ ] Implement history display:
  - [ ] List view: method, URL, status, duration, timestamp (one line per entry)
  - [ ] Detail view: full request + response for a specific entry
  - [ ] Table format with nakama-ui
  - [ ] Pagination for large result sets
- [ ] Implement `replay` subcommand:
  - [ ] Replay a request by history ID: `gate replay 42`
  - [ ] Replay last request: `gate replay --last`
  - [ ] Replay with modifications: `gate replay 42 --header "X-New: value"`
  - [ ] Replay all matching a filter: `gate replay --url "*/users" --last 5`
  - [ ] Compare replay response with original (auto-diff)
- [ ] Implement history management:
  - [ ] `gate history` — list recent requests
  - [ ] `gate history search <query>` — search history
  - [ ] `gate history show <id>` — show full details
  - [ ] `gate history clear` — clear all history
  - [ ] `gate history export` — export as HAR or JSON
  - [ ] `gate history tag <id> <tag>` — tag a request
- [ ] Implement history statistics:
  - [ ] Most frequently called endpoints
  - [ ] Average response times per endpoint
  - [ ] Error rate per endpoint
  - [ ] Total requests over time
- [ ] Database migrations for schema changes
- [ ] Unit tests for storage and retrieval
- [ ] Unit tests for search/filter queries

## Acceptance Criteria

- Every request/response is automatically saved to history
- Large responses are compressed to save disk space
- Sensitive headers are redacted in storage
- Search supports URL patterns, methods, status codes, and time ranges
- Replay reproduces the original request accurately
- Replay with modifications allows header/body changes
- History export produces valid HAR format
- Auto-cleanup prevents unbounded disk usage
- Database operations are fast (< 10ms for common queries)

## Dependencies

- Step 001 (CLI scaffold)
- Step 003 (HTTP engine) for request/response data
- Step 005 (Security) for header redaction before storage
- rusqlite or sqlx with SQLite
- zstd for response compression
