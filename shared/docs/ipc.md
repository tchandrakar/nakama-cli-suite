# Shared Inter-Tool Communication (IPC) — Nakama CLI Suite

> Seamless communication between tools. Pipe them, chain them, orchestrate them.

## Overview

Nakama tools are designed to work together as naturally as Unix commands. Every tool can produce structured output that another tool can consume. Beyond simple piping, tools share context, trace IDs, and can invoke each other programmatically.

---

## 1. Communication Layers

```
┌───────────────────────────────────────────────────────────┐
│                    IPC Architecture                        │
│                                                            │
│  Layer 1: Unix Pipes (simplest, most composable)           │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ tool-a --format=json | tool-b --input=stdin          │  │
│  │                                                      │  │
│  │ Every tool supports:                                 │  │
│  │  --format=json    structured JSON output              │  │
│  │  --input=stdin    read structured input from pipe    │  │
│  │  --quiet          suppress UI, output data only      │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                            │
│  Layer 2: Nakama Message Protocol (NMP)                    │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ Structured JSON messages with metadata envelope      │  │
│  │ Carries: data + trace_id + source_tool + schema      │  │
│  │ Works over pipes, files, or Unix domain sockets      │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                            │
│  Layer 3: Direct Invocation (nakama-sdk)                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ Tools invoke each other as library calls             │  │
│  │ No process spawning, shared memory, fastest          │  │
│  │ Used by orchestrator tools (tensai, itachi)          │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                            │
│  Layer 4: Event Bus (for async / watch modes)              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ File-based event log at ~/.nakama/events/            │  │
│  │ Tools publish events, others subscribe               │  │
│  │ Used by daemon modes (byakugan watch, sharingan)     │  │
│  └──────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────┘
```

---

## 2. Nakama Message Protocol (NMP)

The standard envelope for tool-to-tool communication:

```json
{
  "nmp_version": "1.0",
  "trace_id": "tr_abc123def456",
  "source": {
    "tool": "itachi",
    "command": "jira",
    "version": "0.1.0"
  },
  "timestamp": "2026-02-28T14:23:05.123Z",
  "schema": "jira-issues",
  "data": {
    "issues": [
      {
        "key": "PAY-142",
        "summary": "Add retry logic to payment processor",
        "status": "In Review",
        "assignee": "alice"
      }
    ]
  }
}
```

### Fields

| Field | Required | Description |
|-------|----------|-------------|
| `nmp_version` | yes | Protocol version for compatibility |
| `trace_id` | yes | Correlation ID flowing across the pipeline |
| `source.tool` | yes | Which tool produced this message |
| `source.command` | yes | Which subcommand was run |
| `timestamp` | yes | ISO 8601 timestamp |
| `schema` | yes | Data schema name (for validation) |
| `data` | yes | The actual payload |

### Schema Registry

Each tool publishes its output schemas:

```
shared/schemas/
├── jira-issues.json          # itachi jira output
├── confluence-pages.json     # itachi wiki output
├── git-diff.json             # shinigami diff output
├── commit-message.json       # shinigami commit output
├── review-findings.json      # byakugan review output
├── search-results.json       # kami search output
├── test-results.json         # mugen gen output
├── log-analysis.json         # sharingan scan output
├── health-report.json        # jogan health output
├── code-query.json           # senku ask output
├── briefing.json             # tensai brief output
└── shell-command.json        # zangetsu ask output
```

Schemas are JSON Schema files. Consuming tools validate incoming data against schemas.

---

## 3. Common Pipeline Patterns

### Pattern 1: Linear Pipeline
```bash
# Review a PR with Jira context
itachi jira "PROJ-142" --format=json | byakugan review --context=stdin PR#42

# Generate tests for files changed in a PR
byakugan review --format=json PR#42 | mugen gen --from-review=stdin

# Morning briefing with Jira data
itachi brief backend --format=json | tensai ingest --source=jira
```

### Pattern 2: Fan-Out / Fan-In
```bash
# Parallel data collection, then synthesis
{
  itachi jira "sprint status" --format=json
  itachi wiki "recent updates" --format=json
  senku ask "recent architecture changes" --format=json
} | tensai synthesize --sources=stdin
```

### Pattern 3: Event-Driven
```bash
# Byakugan watches for new PRs, triggers review + test generation
byakugan watch --on-new-pr="byakugan review {pr} && mugen cover {pr_files}"

# Sharingan detects anomaly, triggers jogan diagnosis
sharingan watch --on-anomaly="jogan diagnose '{anomaly_description}'"
```

### Pattern 4: Enrichment
```bash
# Enrich a log error with search context
sharingan explain error.log --format=json | kami pipe "find solutions for these errors"

# Enrich PR review with codebase knowledge
byakugan review PR#42 --format=json | senku enrich --from-review=stdin
```

---

## 4. Direct Invocation (nakama-sdk)

For tools that deeply integrate (like Tensai orchestrating everything):

```rust
use nakama_sdk::prelude::*;

// Tensai morning briefing implementation
async fn morning_brief(config: &Config) -> Result<Briefing> {
    // Direct library calls — no process spawning
    let jira_data = itachi::jira::sprint_status(&config.jira).await?;
    let prs = byakugan::github::open_prs(&config.github).await?;
    let ci_status = shinigami::ci::status(&config.ci).await?;

    // All share the same trace_id
    let trace = TraceContext::new();

    // Synthesize with AI
    let briefing = tensai::synthesize(
        &[jira_data.into(), prs.into(), ci_status.into()],
        &trace,
    ).await?;

    Ok(briefing)
}
```

### SDK Structure

```
shared/nakama-sdk/
├── src/
│   ├── lib.rs              # Re-exports all tool interfaces
│   ├── prelude.rs          # Common imports
│   ├── trace.rs            # Trace context propagation
│   ├── message.rs          # NMP message types
│   └── tools/
│       ├── zangetsu.rs     # Zangetsu public API
│       ├── shinigami.rs    # Shinigami public API
│       ├── jogan.rs        # ... etc
│       ├── senku.rs
│       ├── sharingan.rs
│       ├── tensai.rs
│       ├── mugen.rs
│       ├── gate.rs
│       ├── byakugan.rs
│       ├── kami.rs
│       └── itachi.rs
└── Cargo.toml
```

---

## 5. Tool Discovery

Tools register themselves in a shared manifest:

```toml
# ~/.nakama/tools.toml (auto-generated on install)
[tools.zangetsu]
binary = "/usr/local/bin/zangetsu"
version = "0.1.0"
schemas = ["shell-command"]

[tools.shinigami]
binary = "/usr/local/bin/shinigami"
version = "0.1.0"
schemas = ["git-diff", "commit-message"]

# ... etc
```

Any tool can query what's available:
```rust
let tools = nakama_sdk::discover_tools()?;
if tools.has("itachi") {
    // Enrich with Jira context
}
```

---

## 6. Trace Context Propagation

Every operation gets a trace ID that flows across tool boundaries:

```
User runs: itachi standup | tensai ingest

Step 1: itachi starts, generates trace_id "tr_abc123"
Step 2: itachi outputs NMP message with trace_id in envelope
Step 3: tensai reads stdin, extracts trace_id from NMP envelope
Step 4: tensai uses same trace_id for its own logs and API calls
Step 5: All audit entries across both tools share trace_id

Result: `nakama logs --trace=tr_abc123` shows the complete pipeline execution
```

For direct invocation, trace context is passed as a parameter:
```rust
let trace = TraceContext::new();
let result = itachi::jira::query(&jql, &trace).await?;
// trace_id automatically attached to all logs and audit entries
```

---

## 7. Error Propagation

When a tool in a pipeline fails, errors propagate as NMP error messages:

```json
{
  "nmp_version": "1.0",
  "trace_id": "tr_abc123",
  "source": { "tool": "itachi", "command": "jira" },
  "timestamp": "2026-02-28T14:23:05.123Z",
  "schema": "error",
  "data": {
    "code": "AUTH_EXPIRED",
    "message": "Atlassian API token has expired",
    "recoverable": true,
    "suggestion": "Run 'nakama auth refresh atlassian' to re-authenticate"
  }
}
```

Downstream tools detect `"schema": "error"` and handle gracefully:
- Display the error to the user
- Skip the failed input and continue with others
- Trigger fallback behavior

---

## 8. Configuration

```toml
# ~/.nakama/config.toml

[ipc]
protocol = "nmp"                    # nmp | raw (raw = plain JSON, no envelope)
schema_validation = true            # validate incoming messages against schemas
trace_propagation = true            # auto-propagate trace IDs
event_dir = "~/.nakama/events"      # for event bus

[ipc.socket]
enabled = false                     # Unix domain socket for daemon communication
path = "~/.nakama/nakama.sock"
```
