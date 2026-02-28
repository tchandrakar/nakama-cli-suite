# Step 007: Correlation Engine

## Objective

Build a cross-source log correlation engine that aligns timestamps across multiple log sources, detects temporally proximate events, infers causal chains between related errors, and identifies cascading failure patterns across distributed systems.

## Tasks

- [ ] Define `CorrelationResult` struct: correlated_entries, chain_type, confidence, timeline, narrative
- [ ] Define `CorrelationConfig`: time_window, min_confidence, max_chain_depth
- [ ] Implement timestamp alignment:
  - [ ] Normalize all timestamps to UTC
  - [ ] Handle clock skew detection between sources (statistical offset estimation)
  - [ ] Support configurable time tolerance window (default 5s)
  - [ ] Handle missing timestamps (assign based on position/neighbor interpolation)
- [ ] Implement temporal proximity detection:
  - [ ] Index events by time bucket (configurable granularity)
  - [ ] Find events from different sources within the time window
  - [ ] Score proximity (closer in time = higher score)
  - [ ] Group clusters of related events
- [ ] Implement causal chain inference:
  - [ ] Rule-based patterns: upstream timeout -> downstream 503, DB slow -> API timeout
  - [ ] Service dependency graph (config or auto-detected from log content)
  - [ ] Request ID / trace ID correlation (extract common IDs across sources)
  - [ ] Error propagation tracking (same error message/code across services)
- [ ] Implement cascading failure pattern detection:
  - [ ] Detect error waves (errors appearing sequentially across services)
  - [ ] Identify the origin point (first error in the chain)
  - [ ] Track blast radius (how many services affected)
  - [ ] Pattern library of known cascading scenarios (circuit breaker trips, connection pool exhaustion)
- [ ] Implement `correlate` subcommand:
  - [ ] Accept multiple `--source` flags
  - [ ] Run correlation analysis across all sources
  - [ ] Output timeline view with causal annotations
  - [ ] Support `--trace-id` flag for request-level correlation
- [ ] Format correlation output:
  - [ ] Timeline view (chronological with source labels)
  - [ ] Causal chain view (tree/DAG format)
  - [ ] Summary narrative (LLM-generated if enabled)
- [ ] Unit tests with synthetic multi-source log scenarios
- [ ] Integration test: simulate cascading failure across 3 sources

## Acceptance Criteria

- Timestamps are correctly normalized across sources with different time zones
- Clock skew up to 30 seconds is automatically detected and compensated
- Temporally proximate events are grouped with configurable window
- Causal chains are identified for common patterns (timeout cascades, error propagation)
- Request/trace ID correlation works across sources
- Cascading failure origin is correctly identified in test scenarios
- Timeline output is human-readable with clear source attribution
- Correlation processes >10k entries from 5 sources in <5 seconds

## Dependencies

- Step 002 (Log ingestors) for multi-source input
- Step 003 (Log parser) for structured LogEntry with timestamps
- Step 004 (Fast analysis) for pattern identification
- Step 005 (Deep analysis) for LLM-generated narratives (optional)
