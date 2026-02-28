# Step 003: Log Parser

## Objective

Build a log parsing layer with automatic format detection that can parse JSON, Common/Combined Log Format, syslog (RFC 3164 and RFC 5424), framework-specific formats (Rails, Spring Boot, Django), and unstructured text. The parser extracts structured fields from raw log lines into the `LogEntry` model.

## Tasks

- [ ] Define `LogFormat` enum: Json, Clf, Combined, Syslog3164, Syslog5424, Rails, SpringBoot, Django, Unstructured
- [ ] Define `LogParser` trait with `fn parse(&self, raw: &str) -> Result<LogEntry>`
- [ ] Implement format auto-detection:
  - [ ] Sample first N lines (configurable, default 20)
  - [ ] Score each format parser on sample lines
  - [ ] Select highest-confidence format
  - [ ] Support `--format` override flag to skip auto-detection
- [ ] Implement `JsonParser`:
  - [ ] Parse JSON objects, extract common field names (timestamp, level, msg, message, severity, etc.)
  - [ ] Support nested JSON structures
  - [ ] Handle JSON Lines (newline-delimited JSON)
  - [ ] Map various timestamp formats to chrono DateTime
- [ ] Implement `ClfParser`:
  - [ ] Parse Common Log Format (host ident authuser date request status bytes)
  - [ ] Parse Combined Log Format (CLF + referer + user-agent)
  - [ ] Extract HTTP method, path, status code as metadata
- [ ] Implement `SyslogParser`:
  - [ ] RFC 3164: `<priority>timestamp hostname app[pid]: message`
  - [ ] RFC 5424: `<priority>version timestamp hostname app-name procid msgid structured-data msg`
  - [ ] Extract facility and severity from priority value
  - [ ] Parse structured data elements (RFC 5424)
- [ ] Implement `RailsParser`:
  - [ ] Parse Rails log format with request blocks
  - [ ] Extract controller, action, duration, status
  - [ ] Handle multi-line log entries (stack traces)
- [ ] Implement `SpringBootParser`:
  - [ ] Parse `timestamp level [thread] logger - message`
  - [ ] Extract thread name, logger class
  - [ ] Handle multi-line stack traces
- [ ] Implement `DjangoParser`:
  - [ ] Parse Django default and verbose formats
  - [ ] Extract request method, path, status
- [ ] Implement `UnstructuredParser`:
  - [ ] Best-effort timestamp extraction (regex library of common formats)
  - [ ] Best-effort level extraction (ERROR, WARN, INFO patterns)
  - [ ] Store full line as message
- [ ] Implement multi-line log entry handling:
  - [ ] Stack trace continuation detection
  - [ ] Indentation-based grouping
  - [ ] Configurable multi-line patterns
- [ ] Add `--format` CLI flag to each relevant subcommand
- [ ] Unit tests for each parser with sample log lines
- [ ] Benchmark parsing throughput

## Acceptance Criteria

- Auto-detection correctly identifies format from sample lines (>90% accuracy on test corpus)
- Each parser extracts timestamp, level, source, and message fields
- Multi-line entries (stack traces) are grouped into single LogEntry
- JSON parser handles various timestamp field names and formats
- Syslog parser handles both RFC variants
- Unstructured parser gracefully handles unknown formats
- Parsing throughput exceeds 100k lines/second for structured formats

## Dependencies

- Step 001 (CLI scaffold) for command structure
- Step 002 (Log ingestors) for raw line stream input
- chrono crate for timestamp parsing
- regex crate for pattern matching
- serde_json for JSON parsing
