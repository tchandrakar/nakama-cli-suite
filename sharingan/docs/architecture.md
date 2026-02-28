# Sharingan — AI-Powered Log Analyzer

> "Tracking high-speed patterns, prediction." — Inspired by the Sharingan eye from Naruto.

## Overview

Sharingan watches your log streams in real time, detects patterns invisible to the human eye, identifies anomalies, explains errors, and predicts cascading failures before they happen.

## Core Commands

| Command | Description |
|---------|-------------|
| `sharingan tail <source>` | Tail logs with real-time AI annotation |
| `sharingan explain <logfile>` | Explain errors and patterns in a log file |
| `sharingan scan <logfile>` | Scan for anomalies, errors, and warnings |
| `sharingan correlate <source1> <source2>` | Find correlated events across log sources |
| `sharingan predict <source>` | Predict potential issues from current patterns |
| `sharingan filter <query> <source>` | Natural language log filtering |
| `sharingan summary <source>` | Generate an executive summary of log activity |

## Architecture

```
┌───────────────────────────────────────────────────┐
│                    CLI Layer                       │
│       (commands, TUI for tail mode)               │
├───────────────────────────────────────────────────┤
│                 Log Ingestors                      │
│  ┌──────────┐ ┌──────────┐ ┌───────────────────┐  │
│  │ File     │ │ stdin /  │ │ Remote            │  │
│  │ (tail -f │ │ pipe     │ │ (kubectl logs,    │  │
│  │  equiv.) │ │          │ │  docker logs,     │  │
│  │          │ │          │ │  CloudWatch, etc.) │  │
│  └──────────┘ └──────────┘ └───────────────────┘  │
├───────────────────────────────────────────────────┤
│                 Log Parser                         │
│  ┌─────────────────────────────────────────────┐   │
│  │ Format auto-detection:                      │   │
│  │  - JSON structured logs                     │   │
│  │  - Common Log Format (Apache/Nginx)         │   │
│  │  - syslog                                   │   │
│  │  - Application-specific (Rails, Spring...)  │   │
│  │  - Unstructured (regex + heuristics)        │   │
│  └─────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────┐   │
│  │ Parsed Log Entry:                           │   │
│  │  { timestamp, level, source, message,       │   │
│  │    structured_fields, raw_line }            │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│                 Analysis Engines                   │
│  ┌────────────────┐ ┌──────────────────────────┐   │
│  │ Pattern        │ │ Anomaly Detector         │   │
│  │ Matcher        │ │ (statistical:            │   │
│  │ (known error   │ │  rate changes,           │   │
│  │  signatures,   │ │  new error types,        │   │
│  │  stack traces) │ │  frequency shifts)       │   │
│  └────────────────┘ └──────────────────────────┘   │
│  ┌────────────────┐ ┌──────────────────────────┐   │
│  │ Correlation    │ │ LLM Explainer            │   │
│  │ Engine         │ │ (error context →         │   │
│  │ (temporal      │ │  plain English           │   │
│  │  proximity,    │ │  explanation +           │   │
│  │  causal links) │ │  fix suggestions)        │   │
│  └────────────────┘ └──────────────────────────┘   │
├───────────────────────────────────────────────────┤
│                 Prediction Engine                   │
│  ┌─────────────────────────────────────────────┐   │
│  │ Trend analysis: error rate acceleration     │   │
│  │ Pattern matching: "last time X happened,    │   │
│  │   Y followed within N minutes"              │   │
│  │ Resource exhaustion: disk, memory, conns    │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│                 Output Layer                       │
│  ┌──────────┐ ┌────────────┐ ┌────────────────┐   │
│  │ Annotated│ │ Summary    │ │ Alert /        │   │
│  │ Stream   │ │ Reports    │ │ Webhook        │   │
│  └──────────┘ └────────────┘ └────────────────┘   │
└───────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Auto-Detect Log Format
Sharingan samples the first N lines of a log source and automatically detects the format. It supports:
- JSON structured logs (most modern apps)
- Common Log Format / Combined Log Format (web servers)
- Syslog (RFC 3164, RFC 5424)
- Framework-specific formats (Rails, Spring Boot, Django)
- Unstructured plain text (falls back to regex + heuristic timestamp extraction)

No configuration needed in most cases.

### 2. Two-Tier Analysis
- **Fast tier (local):** Pattern matching, regex-based error detection, statistical anomaly detection (rate changes, new error classes). Runs on every log line with no API calls.
- **Deep tier (LLM):** Triggered on-demand or when the fast tier detects something interesting. Sends batched context to the LLM for explanation and root cause analysis.

This keeps `sharingan tail` fast and responsive while still offering deep insights.

### 3. Temporal Correlation
When analyzing multiple log sources, Sharingan aligns events by timestamp and looks for:
- Events that consistently co-occur within a time window
- Causal chains (A happens, then B happens within T seconds)
- Cascading failure patterns (error in service A → timeout in service B → 503 in service C)

### 4. Natural Language Filtering
Instead of writing complex grep/awk pipelines:
```
sharingan filter "show me all timeout errors from the payment service in the last hour" app.log
```
The LLM translates this to the appropriate filter criteria.

### 5. Prediction
Sharingan tracks trends and warns about:
- Accelerating error rates ("error rate doubled in the last 5 minutes")
- Resource exhaustion projections ("at current rate, disk fills in 2 hours")
- Known failure precursors ("this pattern of warnings preceded an outage last time")

## Data Flow — Tail Mode

```
Log Source (file / stdin / remote)
        │
        ▼
  Log Ingestor ──→ raw lines
        │
        ▼
  Log Parser ──→ structured entries
        │
        ├──→ Fast Analysis (every line)
        │       │
        │       ├── pattern match → annotate
        │       ├── anomaly detect → alert
        │       └── stats update → trend tracking
        │
        ├──→ Deep Analysis (on trigger)
        │       │
        │       └── LLM explain → inline annotation
        │
        ▼
  Annotated Output Stream (terminal)
```

## Configuration

File: `~/.sharingan/config.toml`

```toml
[llm]
provider = "anthropic"
model = "claude-haiku-4-5-20251001"      # fast model for real-time

[ingest]
buffer_size = 1000
auto_detect_format = true

[analysis]
fast_tier_enabled = true
deep_tier_trigger = "on_error"    # on_error | on_anomaly | manual | all
batch_size = 50                   # lines to send to LLM at once

[anomaly]
window_seconds = 300
rate_change_threshold = 2.0       # 2x normal rate triggers alert
new_error_alert = true

[prediction]
enabled = true
lookback_minutes = 60

[output]
color = true
annotations_inline = true
```

## Tech Stack

- **Language:** Rust
- **CLI framework:** clap
- **TUI:** ratatui (for tail mode with live annotations)
- **Log parsing:** custom parser with regex fallbacks
- **Statistics:** streaming algorithms (HyperLogLog, exponential moving average)
- **LLM integration:** shared nakama LLM abstraction layer
- **Remote logs:** kube-rs, bollard, aws-sdk
