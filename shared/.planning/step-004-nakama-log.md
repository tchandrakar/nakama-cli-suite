# Step 004: Build nakama-log (Structured Logging)

## Objective
Implement structured JSON logging with per-tool files, rotation, and secret redaction.

## Tasks
- tracing subscriber setup with JSON formatter
- Per-tool log files (~/.nakama/logs/<tool>.log) via tracing-appender
- Combined log file (~/.nakama/logs/nakama.log)
- Rolling file appender: 10 MB max size, 5 rotated files, gzip compression
- Log level configuration: global default + per-tool override from config
- Trace ID injection: every log entry tagged with current TraceContext
- Secret redaction filter: scrub patterns matching API keys, tokens, passwords
- `nakama logs` CLI viewer: filter by tool, level, time range, grep, follow mode
- Unit tests: log output format validation, rotation triggers, redaction verification

## Acceptance Criteria
- Each tool writes structured JSON logs to its own file
- Logs rotate at 10 MB, keeping 5 backups
- Secrets are never present in log output
- `nakama logs shinigami --level=error --since="1 hour ago"` works
- Trace IDs present in every log entry

## Dependencies
- Step 002 (nakama-core)
