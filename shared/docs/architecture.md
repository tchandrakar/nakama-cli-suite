# Shared Infrastructure — Architecture Overview

> The foundation that every Nakama tool is built on.

## Crate Structure

```
shared/
├── nakama-core/          # Configuration, errors, common types
├── nakama-vault/         # Credential storage (OS keychain, encrypted fallback)
├── nakama-ui/            # Claude-style terminal output
├── nakama-ai/            # Multi-provider AI abstraction
├── nakama-ipc/           # Inter-tool communication protocol
├── nakama-audit/         # Tamper-evident audit logging
├── nakama-log/           # Structured logging (tracing + rotation)
├── nakama-sdk/           # Tool-to-tool SDK (direct invocation)
└── docs/
    ├── architecture.md   # This file
    ├── security.md       # Credential vault, TLS, input validation, SaaS standards
    ├── logging.md        # Claude-style UI, structured logs, rotation
    ├── ai-providers.md   # Multi-provider AI (Claude, OpenAI, Gemini, Ollama)
    ├── ipc.md            # Inter-tool communication, NMP protocol, pipes
    └── audit.md          # Audit trail, AI usage tracking, compliance
```

## How Tools Integrate

Every Nakama tool binary follows the same pattern:

```rust
use nakama_core::config::Config;
use nakama_vault::CredentialVault;
use nakama_ui::NakamaUI;
use nakama_ai::AIProvider;
use nakama_log::init_logging;
use nakama_audit::Audit;
use nakama_ipc::NmpMessage;

fn main() -> Result<()> {
    // 1. Load configuration (global → tool → CLI flags)
    let config = Config::load("zangetsu")?;

    // 2. Initialize logging (structured JSON + per-tool file)
    init_logging(&config)?;

    // 3. Initialize UI (respects verbosity, color, TTY detection)
    let ui = NakamaUI::from_config(&config);

    // 4. Initialize credential vault (OS keychain → encrypted file → env var)
    let vault = CredentialVault::new(&config)?;

    // 5. Initialize AI provider (user's choice: Claude, OpenAI, Gemini, Ollama)
    let ai = AIProvider::from_config(&config, &vault)?;

    // 6. Initialize audit logger
    let audit = Audit::new(&config)?;

    // 7. Run the tool's actual logic
    zangetsu::run(&config, &ui, &ai, &vault, &audit)?;

    Ok(())
}
```

## Dependency Graph

```
nakama-core ──────────────────────────────────────┐
    │                                              │
    ├── nakama-vault (core)                        │
    ├── nakama-log (core)                          │
    │       │                                      │
    │       ├── nakama-ui (core)                   │
    │       ├── nakama-audit (core, log)           │
    │       │                                      │
    │       └── nakama-ai (core, vault, log, audit)│
    │                                              │
    ├── nakama-ipc (core, log)                     │
    │                                              │
    └── nakama-sdk (all of the above) ─────────────┘
             │
             ▼
    Individual tool binaries
```

## Cross-Cutting Concerns Summary

| Concern | Handled By | Key Principle |
|---------|------------|---------------|
| Credentials | nakama-vault | Zero plaintext, OS keychain first |
| Terminal UI | nakama-ui | Claude-style output, pipe-aware |
| AI calls | nakama-ai | Provider-agnostic, user-selectable |
| Tool communication | nakama-ipc | NMP protocol, Unix pipes, schemas |
| Audit trail | nakama-audit | Tamper-evident, every action logged |
| Structured logs | nakama-log | JSON Lines, per-tool + combined, rotated |
| Configuration | nakama-core | Global → tool → CLI flag merge |
| Error handling | nakama-core | Typed errors, context chain, no secret leaks |

## Global Configuration

File: `~/.nakama/config.toml`

```toml
# AI Provider (all tools use this unless overridden)
[ai]
default_provider = "anthropic"     # anthropic | openai | google | ollama

[ai.anthropic]
model_fast = "claude-haiku-4-5-20251001"
model_balanced = "claude-sonnet-4-6"
model_powerful = "claude-opus-4-6"

[ai.openai]
model_fast = "gpt-4.1-nano"
model_balanced = "gpt-4.1-mini"
model_powerful = "gpt-4.1"

[ai.google]
model_fast = "gemini-2.5-flash"
model_balanced = "gemini-2.5-flash"
model_powerful = "gemini-2.5-pro"

[ai.ollama]
base_url = "http://localhost:11434"
model_fast = "llama3:8b"
model_balanced = "llama3:70b"

[ai.budget]
weekly_limit_usd = 10.00
alert_threshold_percent = 80

# Logging
[logging]
level = "info"
directory = "~/.nakama/logs"

# UI
[ui]
color = "auto"
verbosity = "normal"
spinners = true

# Audit
[audit]
enabled = true
retention_days = 90

# IPC
[ipc]
schema_validation = true
trace_propagation = true
```

## File System Layout

```
~/.nakama/                        # Global config and data
├── config.toml                   # Global configuration
├── vault/                        # Encrypted credential fallback store
├── logs/                         # Structured log files
│   ├── nakama.log                # Combined log
│   ├── zangetsu.log              # Per-tool logs
│   ├── shinigami.log
│   └── ...
├── audit/                        # Audit database and backups
│   ├── audit.db                  # SQLite audit store
│   └── audit.jsonl               # JSONL backup
├── events/                       # Event bus (for async IPC)
└── tools.toml                    # Installed tool manifest
```
