# Step 005: Deep Analysis (LLM-Powered)

## Objective

Integrate with the nakama-ai shared crate to perform deep, LLM-powered analysis of log entries. This includes batch context assembly for efficient token usage, error explanation prompts, root cause suggestion generation, and configurable trigger modes (on_error, on_anomaly, manual).

## Tasks

- [ ] Add nakama-ai dependency and initialize AI client in sharingan
- [ ] Define `DeepAnalysisRequest` struct: log_entries, context, analysis_type, max_tokens
- [ ] Define `DeepAnalysisResult` struct: explanation, root_causes (ranked), suggested_fixes, confidence, related_docs
- [ ] Implement batch context assembly:
  - [ ] Group related log entries (same time window, same source, same error)
  - [ ] Build context window with surrounding healthy logs for contrast
  - [ ] Include system metadata (source type, environment, service name)
  - [ ] Truncate/summarize to fit token budget
  - [ ] Attach fast-analysis results as hints to the LLM
- [ ] Implement error explanation prompt templates:
  - [ ] Generic error explanation prompt
  - [ ] Stack trace analysis prompt
  - [ ] Performance degradation prompt
  - [ ] Security incident prompt
  - [ ] Custom prompt override via config
- [ ] Implement root cause suggestion engine:
  - [ ] Feed error context + system info to LLM
  - [ ] Parse structured response (cause, likelihood, fix)
  - [ ] Rank suggestions by confidence
  - [ ] Include relevant documentation links when available
- [ ] Implement trigger modes:
  - [ ] `on_error`: automatically trigger deep analysis on ERROR/FATAL entries
  - [ ] `on_anomaly`: trigger when fast-analysis detects an anomaly
  - [ ] `manual`: only when user explicitly requests via `explain` command
  - [ ] Configurable via CLI flag `--trigger` and config file
- [ ] Implement rate limiting for LLM calls:
  - [ ] Max calls per minute (configurable)
  - [ ] Debounce rapid-fire triggers
  - [ ] Queue with priority (manual > on_error > on_anomaly)
- [ ] Format deep analysis output using nakama-ui panels
- [ ] Cache recent analysis results (in-memory LRU) to avoid duplicate calls
- [ ] Add `--no-ai` flag to disable deep analysis entirely
- [ ] Unit tests with mocked nakama-ai responses
- [ ] Integration test: feed error log, verify explanation output

## Acceptance Criteria

- Deep analysis produces human-readable error explanations
- Root cause suggestions are ranked by confidence with actionable fixes
- Batch context stays within token budget (configurable, default 4096)
- Trigger modes correctly gate when LLM calls are made
- Rate limiting prevents runaway API costs
- `--no-ai` flag disables all LLM calls
- Cached results avoid duplicate LLM calls for identical errors
- Output is formatted with nakama-ui panels and syntax highlighting

## Dependencies

- Step 001 (CLI scaffold)
- Step 003 (Log parser) for structured LogEntry
- Step 004 (Fast analysis) for anomaly triggers and hints
- nakama-ai shared crate must expose async completion API
- nakama-vault for API key retrieval (see Step 006)
