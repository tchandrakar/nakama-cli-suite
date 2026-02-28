# Step 009: Natural Language Filtering

## Objective

Implement natural language log filtering that translates human-readable filter descriptions into structured filter operations via LLM, including time range parsing, service/source filtering, log level filtering, and content matching. Users can say "show me errors from the auth service in the last hour" instead of writing complex filter expressions.

## Tasks

- [ ] Define `FilterExpression` struct: time_range, levels, sources, services, content_pattern, limit, sort_order
- [ ] Define `FilterResult`: matching_entries, total_scanned, filter_applied (show parsed filter)
- [ ] Implement NL-to-filter translation:
  - [ ] Send user's natural language query to nakama-ai
  - [ ] Prompt template that maps NL to structured FilterExpression JSON
  - [ ] Parse LLM response into FilterExpression
  - [ ] Validate parsed filter (reject nonsensical filters)
  - [ ] Show user the interpreted filter for confirmation (optional, `--confirm` flag)
- [ ] Implement time range parsing:
  - [ ] Relative: "last hour", "past 30 minutes", "since yesterday", "today"
  - [ ] Absolute: "between 2pm and 3pm", "on January 5th"
  - [ ] Open-ended: "since the deployment", "after the restart"
  - [ ] Combine LLM parsing with local chrono validation
- [ ] Implement service/source filtering:
  - [ ] Auto-discover available services/sources from log data
  - [ ] Fuzzy matching on service names
  - [ ] Support negation ("not from the scheduler")
- [ ] Implement level filtering:
  - [ ] Map natural language to levels ("errors" -> ERROR, "warnings and above" -> WARN+ERROR+FATAL)
  - [ ] Support compound levels ("errors and warnings")
- [ ] Implement content filtering:
  - [ ] Keyword matching ("containing timeout")
  - [ ] Regex generation from description ("messages about database connections")
  - [ ] Semantic search (if embedding support available)
- [ ] Implement structured filter fallback:
  - [ ] Support direct structured filter syntax for power users
  - [ ] `--level ERROR --source auth --since 1h` direct flags
  - [ ] NL is default, structured available via `--filter` flag
- [ ] Implement `filter` subcommand:
  - [ ] Accept NL query as positional argument
  - [ ] Apply filter to source logs
  - [ ] Output matching entries with highlighting on match reason
  - [ ] Show filter interpretation ("I understood: errors from auth-service since 14:00")
- [ ] Cache recent filter translations to avoid redundant LLM calls
- [ ] Unit tests: NL queries mapped to expected FilterExpression
- [ ] Integration test: filter mock log data with NL query

## Acceptance Criteria

- Natural language queries correctly translate to structured filters (>90% for common queries)
- Time range parsing handles relative, absolute, and open-ended expressions
- Service names are fuzzy-matched against discovered services
- Level filtering supports compound expressions and natural language
- Filter interpretation is displayed to user for transparency
- Structured filter flags work without LLM (offline mode)
- Cache prevents duplicate LLM calls for same/similar queries
- Filter processes 100k entries in <2 seconds after filter compilation

## Dependencies

- Step 003 (Log parser) for LogEntry to filter against
- Step 005 (Deep analysis) for nakama-ai integration patterns
- Step 006 (Security) for PII redaction in filter queries
- nakama-ai shared crate for LLM completion
