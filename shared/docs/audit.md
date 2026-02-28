# Shared Audit Logging Architecture — Nakama CLI Suite

> Every action tracked, every step logged, always available for inspection.

## Overview

The audit system provides a tamper-evident, queryable record of every significant action taken by any Nakama tool. It answers: "what happened, when, by which tool, and what was the outcome?" This is separate from operational logs — audit logs are for compliance, debugging, and user confidence.

---

## 1. What Gets Audited

### Audit Events by Category

| Category | Events | Example |
|----------|--------|---------|
| **Authentication** | Login, token refresh, token rotation, auth failure | `itachi` refreshes Atlassian OAuth token |
| **Credential Access** | Secret read, secret write, secret delete | `zangetsu` reads Anthropic API key from vault |
| **AI Interaction** | LLM request sent, response received, tokens used, cost | `byakugan` sends PR diff to Claude for review |
| **External API** | HTTP request to external service, response status | `gate` sends POST to api.example.com |
| **Data Modification** | Git commit, Jira ticket created, PR comment posted | `shinigami` creates commit with generated message |
| **Tool Execution** | Command started, command completed, command failed | `zangetsu run "find large files"` |
| **Configuration** | Config changed, provider switched, credential added | User switches AI provider from Claude to OpenAI |
| **IPC** | Tool-to-tool message sent, pipeline started | `itachi` output piped to `tensai` |

---

## 2. Audit Entry Structure

```json
{
  "id": "aud_7f8a9b2c3d4e",
  "timestamp": "2026-02-28T14:23:05.123456Z",
  "trace_id": "tr_abc123def456",
  "tool": "byakugan",
  "command": "review",
  "category": "ai_interaction",
  "action": "llm_request",
  "actor": "user",
  "detail": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-6",
    "input_tokens": 4250,
    "output_tokens": 1830,
    "cost_usd": 0.023,
    "purpose": "pr_review_correctness_pass",
    "pr": "#142"
  },
  "outcome": "success",
  "duration_ms": 3420,
  "checksum": "sha256:a1b2c3d4..."
}
```

### Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique audit entry ID |
| `timestamp` | ISO 8601 | Microsecond precision |
| `trace_id` | string | Correlation ID (links to operational logs and cross-tool pipelines) |
| `tool` | string | Which Nakama tool |
| `command` | string | Which subcommand |
| `category` | enum | Event category (see table above) |
| `action` | string | Specific action within category |
| `actor` | string | "user" or "system" (for automated/watch mode) |
| `detail` | object | Action-specific metadata (never contains secrets) |
| `outcome` | enum | "success", "failure", "denied", "skipped" |
| `duration_ms` | u64 | How long the action took |
| `checksum` | string | SHA-256 of the entry (tamper detection) |

---

## 3. Storage Architecture

```
┌───────────────────────────────────────────────────────┐
│                   Audit Storage                        │
│                                                        │
│  Primary: SQLite Database                              │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Location: ~/.nakama/audit/audit.db               │  │
│  │ Permissions: 0600 (owner read/write only)        │  │
│  │                                                  │  │
│  │ Tables:                                          │  │
│  │  audit_entries    — main audit log               │  │
│  │  audit_checksums  — chain of checksums           │  │
│  │  ai_usage         — token/cost tracking          │  │
│  │                                                  │  │
│  │ Indexes:                                         │  │
│  │  idx_timestamp    — time range queries           │  │
│  │  idx_tool         — per-tool queries             │  │
│  │  idx_trace_id     — cross-tool correlation       │  │
│  │  idx_category     — category filtering           │  │
│  │  idx_outcome      — failure analysis             │  │
│  └──────────────────────────────────────────────────┘  │
│                                                        │
│  Secondary: Append-Only Log Files (backup)             │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Location: ~/.nakama/audit/audit.jsonl            │  │
│  │ Format: JSON Lines (one entry per line)          │  │
│  │ Rotation: 10 MB max, 10 rotated files            │  │
│  │ Purpose: Backup if DB corrupts, easy grep        │  │
│  └──────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────┘
```

### Tamper Detection

Each audit entry includes a checksum. The chain works like this:

```
Entry 1: checksum = SHA256(entry_1_data)
Entry 2: checksum = SHA256(entry_2_data + entry_1_checksum)
Entry 3: checksum = SHA256(entry_3_data + entry_2_checksum)
```

This creates a hash chain — if any entry is modified or deleted, the chain breaks and `nakama audit verify` detects it.

---

## 4. Querying Audit Logs

### CLI Interface

```bash
# View recent audit entries
nakama audit

# Filter by tool
nakama audit --tool=byakugan

# Filter by category
nakama audit --category=ai_interaction

# Filter by time range
nakama audit --since="2026-02-27" --until="2026-02-28"

# Filter by trace (cross-tool pipeline)
nakama audit --trace=tr_abc123def456

# Show only failures
nakama audit --outcome=failure

# AI usage summary
nakama audit usage
nakama audit usage --period=week
nakama audit usage --tool=senku

# Full detail for a specific entry
nakama audit show aud_7f8a9b2c3d4e

# Verify audit chain integrity
nakama audit verify

# Export for compliance
nakama audit export --since="2026-02-01" --format=csv > feb_audit.csv
```

### Example Output

```
$ nakama audit --tool=byakugan --since="1 hour ago"

  Audit Trail — byakugan (last hour)

  ┌──────────────────┬──────────┬─────────────────────────┬─────────┐
  │ Time             │ Action   │ Detail                  │ Outcome │
  ├──────────────────┼──────────┼─────────────────────────┼─────────┤
  │ 14:23:05         │ llm_req  │ PR #142 correctness     │ success │
  │                  │          │ claude-sonnet, 6080 tok  │  3.4s   │
  │ 14:23:09         │ llm_req  │ PR #142 security pass   │ success │
  │                  │          │ claude-sonnet, 4200 tok  │  2.1s   │
  │ 14:23:11         │ api_call │ POST github comment     │ success │
  │                  │          │ github.com/org/repo      │  0.3s   │
  │ 14:20:00         │ cred_read│ Read github-token       │ success │
  │                  │          │ from OS keychain          │  0.01s  │
  └──────────────────┴──────────┴─────────────────────────┴─────────┘

  Total: 4 entries | AI cost: $0.023 | Trace: tr_abc123
```

---

## 5. AI Usage Dashboard

Special audit views for AI cost and usage tracking:

```
$ nakama audit usage --period=week

  AI Usage Report — Feb 22-28, 2026

  By Tool:
  ┌──────────────┬────────────┬─────────────┬──────────┐
  │ Tool         │ Provider   │ Tokens      │ Cost     │
  ├──────────────┼────────────┼─────────────┼──────────┤
  │ byakugan     │ Anthropic  │ 125,400     │ $0.47    │
  │ senku        │ Anthropic  │  98,200     │ $0.37    │
  │ shinigami    │ Anthropic  │  42,300     │ $0.08    │
  │ zangetsu     │ OpenAI     │  31,100     │ $0.06    │
  │ kami         │ Google     │  85,600     │ $0.21    │
  │ itachi       │ Anthropic  │  67,800     │ $0.25    │
  ├──────────────┼────────────┼─────────────┼──────────┤
  │ Total        │            │ 450,400     │ $1.44    │
  └──────────────┴────────────┴─────────────┴──────────┘

  Budget: $10.00/week — 14.4% used

  By Model:
  ┌────────────────────────┬─────────────┬──────────┐
  │ Model                  │ Tokens      │ Cost     │
  ├────────────────────────┼─────────────┼──────────┤
  │ claude-sonnet-4-6      │ 298,500     │ $1.04    │
  │ claude-haiku-4-5       │  35,200     │ $0.03    │
  │ gemini-2.5-pro         │  85,600     │ $0.21    │
  │ gpt-4.1-nano           │  31,100     │ $0.06    │
  └────────────────────────┴─────────────┴──────────┘

  Daily Trend:
  Mon: $0.12 ▏██
  Tue: $0.23 ▏████
  Wed: $0.31 ▏██████
  Thu: $0.18 ▏███
  Fri: $0.28 ▏█████
  Sat: $0.15 ▏███
  Sun: $0.17 ▏███
```

---

## 6. Retention Policy

```toml
# ~/.nakama/config.toml

[audit]
enabled = true
store = "~/.nakama/audit/audit.db"
jsonl_backup = true
jsonl_path = "~/.nakama/audit/audit.jsonl"

# Retention
retention_days = 90                # keep audit entries for 90 days
ai_usage_retention_days = 365      # keep AI usage data for 1 year
auto_cleanup = true                # clean up expired entries automatically

# Tamper detection
chain_verification = true          # maintain hash chain
verify_on_startup = false          # verify chain on every tool start (slow)

# Performance
batch_write = true                 # batch audit writes (better perf)
batch_flush_interval_ms = 1000     # flush batch every second
max_batch_size = 100               # or after 100 entries
```

---

## 7. Audit API

For tool developers:

```rust
use nakama_audit::*;

// Log an audit entry
audit::log(AuditEntry {
    tool: "byakugan",
    command: "review",
    category: Category::AiInteraction,
    action: "llm_request",
    detail: json!({
        "provider": "anthropic",
        "model": "claude-sonnet-4-6",
        "input_tokens": 4250,
        "output_tokens": 1830,
        "purpose": "pr_review_correctness",
    }),
    outcome: Outcome::Success,
    duration: elapsed,
    trace_id: &trace.id(),
})?;

// Convenience methods
audit::log_ai_call(&trace, &provider, &model, tokens, cost, purpose)?;
audit::log_credential_access(&trace, tool, key_name)?;
audit::log_external_api(&trace, method, url, status)?;
audit::log_tool_execution(&trace, tool, command, outcome)?;
```

## Tech Stack

- **Storage:** rusqlite (SQLite with WAL mode for concurrent reads)
- **Hashing:** sha2 crate (SHA-256)
- **Serialization:** serde_json (for detail field and JSONL backup)
- **Time:** chrono (microsecond timestamps)
- **UI:** shared nakama-ui components (tables, panels)
