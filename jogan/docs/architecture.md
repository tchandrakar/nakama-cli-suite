# Jogan — Personal Infra Debugger

> "Seeing invisible threats across layers." — Inspired by the Jogan eye from Boruto.

## Overview

Jogan is a cross-layer infrastructure debugging tool that cuts through noise in logs, metrics, and cluster state to surface root causes. It sees what's hidden across containers, pods, networks, and cloud services.

## Core Commands

| Command | Description |
|---------|-------------|
| `jogan scan` | Scan current infra for common misconfigurations |
| `jogan diagnose <symptom>` | Describe a problem in plain English, get root cause analysis |
| `jogan trace <service>` | Trace request flow across services |
| `jogan health` | Quick health check across all connected infra |
| `jogan explain <resource>` | Explain the current state of a K8s resource, container, etc. |
| `jogan watch` | Live monitoring mode with anomaly alerts |

## Architecture

```
┌───────────────────────────────────────────────────┐
│                    CLI Layer                       │
│        (commands, interactive diagnostics)         │
├───────────────────────────────────────────────────┤
│               Symptom Interpreter                  │
│  ┌─────────────────────────────────────────────┐   │
│  │ Natural language → structured diagnosis     │   │
│  │ "pods keep crashing" → check: restarts,     │   │
│  │   OOM, image pull, probes, resource limits  │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│               Data Collectors                      │
│  ┌──────────┐ ┌──────────┐ ┌───────────────────┐  │
│  │Kubernetes│ │ Docker   │ │ Cloud Provider    │  │
│  │ (kubectl)│ │ (docker) │ │ (AWS/GCP/Azure)   │  │
│  ├──────────┤ ├──────────┤ ├───────────────────┤  │
│  │ Pods     │ │Containers│ │ IAM, Networking   │  │
│  │ Services │ │ Logs     │ │ Load Balancers    │  │
│  │ Events   │ │ Stats    │ │ DNS, Certificates │  │
│  │ Nodes    │ │ Networks │ │ Storage           │  │
│  └──────────┘ └──────────┘ └───────────────────┘  │
├───────────────────────────────────────────────────┤
│               Analysis Engine                      │
│  ┌────────────────┐ ┌────────────────────────┐     │
│  │ Rule-Based     │ │ LLM-Based              │     │
│  │ Checks         │ │ Reasoning              │     │
│  │ (known         │ │ (novel issues,         │     │
│  │  patterns)     │ │  correlation)          │     │
│  └────────────────┘ └────────────────────────┘     │
│  ┌─────────────────────────────────────────────┐   │
│  │ Correlation Engine                          │   │
│  │ (link events across layers: pod crash →     │   │
│  │  node pressure → disk full → log rotation)  │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│               Report Generator                     │
│  ┌──────────────────────────────────────────┐      │
│  │ Root Cause → Evidence → Remediation Steps│      │
│  │ Severity scoring, confidence levels      │      │
│  └──────────────────────────────────────────┘      │
└───────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Dual Analysis Engine
Jogan combines two approaches:
- **Rule-based checks:** Fast, deterministic checks for known issues (e.g., CrashLoopBackOff + OOMKilled = memory limit too low). These run first and catch ~80% of common problems.
- **LLM-based reasoning:** For novel or complex issues, collected data is sent to an LLM for correlation analysis and root cause inference.

### 2. Cross-Layer Correlation
The killer feature. Jogan doesn't just look at one layer:
```
Application Error (HTTP 503)
        ↓ correlate
Pod Status (CrashLoopBackOff)
        ↓ correlate
Node Events (MemoryPressure)
        ↓ correlate
Infrastructure (instance type too small for workload)
```
It builds a causal chain from symptom to root cause.

### 3. Plugin-Based Collectors
Each infrastructure provider is a plugin:
- `kubernetes` — talks to the K8s API via kubeconfig
- `docker` — talks to the Docker socket
- `aws` / `gcp` / `azure` — uses respective SDKs and CLI credentials

Users only enable the plugins relevant to their stack. Collectors implement a common trait/interface.

### 4. Remediation Suggestions
Every diagnosis includes:
- **Root cause** with confidence score
- **Evidence** (the specific logs, events, metrics that led to the conclusion)
- **Remediation steps** (concrete commands to fix the issue)
- **Prevention** (how to avoid this in the future)

## Data Flow — Diagnose

```
User describes symptom
        │
        ▼
Symptom Interpreter ──→ structured query (what to collect)
        │
        ▼
Data Collectors ──→ pods, events, logs, metrics, node status
        │
        ▼
Rule-Based Checks ──→ known pattern match? ──→ YES → Report
        │                                       NO
        ▼                                        │
LLM Correlation ←────────────────────────────────┘
        │
        ▼
Report: Root Cause + Evidence + Remediation
```

## Configuration

File: `~/.jogan/config.toml`

```toml
[llm]
provider = "anthropic"
model = "claude-sonnet-4-6"

[collectors]
enabled = ["kubernetes", "docker"]

[kubernetes]
kubeconfig = "~/.kube/config"
default_namespace = "default"

[docker]
socket = "unix:///var/run/docker.sock"

[analysis]
rule_checks_first = true
llm_fallback = true
max_events_to_collect = 500

[watch]
poll_interval_seconds = 10
anomaly_sensitivity = "medium"    # low | medium | high
```

## Tech Stack

- **Language:** Rust
- **CLI framework:** clap
- **Kubernetes:** kube-rs (Kubernetes client)
- **Docker:** bollard (Docker API client)
- **Cloud SDKs:** aws-sdk-rust, gcloud-sdk
- **LLM integration:** shared nakama LLM abstraction layer
- **TUI for watch mode:** ratatui
