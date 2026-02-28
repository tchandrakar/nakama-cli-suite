# Step 008: Prediction Engine

## Objective

Build a predictive analysis engine that identifies trends in log data, matches patterns against historical incidents, projects resource exhaustion timelines, and generates confidence-scored alerts for potential future issues.

## Tasks

- [ ] Define `Prediction` struct: prediction_type, description, confidence (0.0-1.0), time_horizon, recommended_action, supporting_evidence
- [ ] Define `PredictionConfig`: lookback_window, min_confidence, alert_threshold
- [ ] Implement trend analysis:
  - [ ] Track error rate acceleration (first and second derivatives)
  - [ ] Detect monotonic increase patterns in error/warn counts
  - [ ] Detect periodic patterns (hourly, daily spikes) to filter known cycles
  - [ ] Linear regression on rate metrics for projection
  - [ ] Exponential growth detection
- [ ] Implement historical pattern matching:
  - [ ] Store incident fingerprints (error signature + sequence + timing)
  - [ ] Match current log patterns against stored incidents
  - [ ] Similarity scoring (Jaccard on error types, DTW on time series)
  - [ ] Report: "This looks like incident X from <date>"
  - [ ] Incident fingerprint database (local SQLite or JSON)
- [ ] Implement resource exhaustion projection:
  - [ ] Parse resource-related log messages (disk, memory, connections, queue depth)
  - [ ] Track resource usage trends from log messages
  - [ ] Project time-to-exhaustion based on current rate
  - [ ] Alert when projected exhaustion is within configurable horizon
  - [ ] Common patterns: "disk 80% full", "connection pool 90/100", "queue depth growing"
- [ ] Implement confidence-scored alerts:
  - [ ] Combine multiple signals (trend + pattern + resource) into composite score
  - [ ] Calibrate confidence thresholds to minimize false positives
  - [ ] Alert levels: info, warning, critical (based on confidence + severity)
  - [ ] Include supporting evidence (which log entries, what trend)
  - [ ] Cooldown and deduplication for repeated predictions
- [ ] Implement `predict` subcommand:
  - [ ] Accept log source(s) and lookback period
  - [ ] Run all prediction engines
  - [ ] Output ranked predictions with evidence
  - [ ] Support `--min-confidence` filter
- [ ] Format prediction output with nakama-ui:
  - [ ] Color-coded severity
  - [ ] Trend graphs (ASCII/Unicode sparklines)
  - [ ] Evidence sections with relevant log excerpts
- [ ] Unit tests with synthetic trending data
- [ ] Integration test: simulate gradual degradation, verify prediction fires

## Acceptance Criteria

- Error rate acceleration is detected within 2 minutes of trend start
- Historical pattern matching identifies similar past incidents with >70% accuracy
- Resource exhaustion projection estimates are within 20% of actual time
- Confidence scores are calibrated (>0.8 confidence = >80% true positive rate)
- Periodic patterns (known spikes) are filtered to reduce false positives
- Predictions include actionable recommendations
- Sparkline trend visualization works in terminal

## Dependencies

- Step 003 (Log parser) for structured log entries
- Step 004 (Fast analysis) for statistical tracking infrastructure
- Step 005 (Deep analysis) for LLM-enhanced prediction narratives (optional)
- SQLite or JSON storage for historical incident database
