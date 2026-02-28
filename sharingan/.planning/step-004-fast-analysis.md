# Step 004: Fast Analysis (Local, No LLM)

## Objective

Build a fast, local analysis engine that operates without LLM calls. This includes pattern matching against known error signatures, statistical anomaly detection using exponential moving averages (EMA) and HyperLogLog for cardinality, frequency shift detection, and rate change alerting. This layer runs in real-time on every log entry.

## Tasks

- [ ] Define `AnalysisResult` struct: severity, category, description, matched_pattern, confidence
- [ ] Define `Analyzer` trait with `fn analyze(&mut self, entry: &LogEntry) -> Vec<AnalysisResult>`
- [ ] Implement `PatternMatcher`:
  - [ ] Load known error signature database (YAML/JSON config)
  - [ ] Match against common patterns: OOM, connection refused, timeout, deadlock, segfault, panic, null pointer
  - [ ] Stack trace detection and classification (language-specific)
  - [ ] HTTP error status grouping (4xx client, 5xx server)
  - [ ] Database error patterns (connection pool exhausted, slow query, lock wait)
  - [ ] Support custom pattern definitions via config file
- [ ] Implement `StatisticalAnomalyDetector`:
  - [ ] Track error rate using Exponential Moving Average (EMA)
  - [ ] Configurable EMA alpha/window parameters
  - [ ] Detect rate spikes (current rate > N * EMA baseline)
  - [ ] Track unique value cardinality with HyperLogLog (e.g., unique error messages, unique IPs)
  - [ ] Detect cardinality explosions (new error types appearing)
  - [ ] Frequency shift detection: track distribution of log levels, detect shifts
- [ ] Implement `RateChangeAlerter`:
  - [ ] Sliding window counters per level (error, warn, info)
  - [ ] Configurable thresholds (e.g., >10 errors/minute)
  - [ ] Percentage-based alerts (error rate doubled in last 5 minutes)
  - [ ] Cooldown period to avoid alert storms
- [ ] Implement `AnalysisPipeline`:
  - [ ] Chain multiple analyzers
  - [ ] Deduplicate overlapping results
  - [ ] Priority-sort results by severity
- [ ] Store analysis state for time-window calculations
- [ ] Add CLI flags: `--threshold`, `--window`, `--patterns`
- [ ] Unit tests for each analyzer with synthetic log streams
- [ ] Benchmark: analysis must add <1ms per log entry

## Acceptance Criteria

- Known error patterns are detected with zero false negatives for defined signatures
- Statistical anomaly detection fires within 30 seconds of a rate spike
- HyperLogLog cardinality tracking is memory-bounded (<1KB per counter)
- Rate change alerts respect cooldown periods
- Analysis pipeline processes >50k entries/second
- Custom patterns can be added via configuration without code changes
- All analysis runs locally with no network calls

## Dependencies

- Step 001 (CLI scaffold)
- Step 003 (Log parser) for structured LogEntry input
- No external service dependencies (this is purely local computation)
