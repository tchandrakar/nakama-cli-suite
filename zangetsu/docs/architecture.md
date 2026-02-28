# Zangetsu — Smart Shell Companion

> "Instant, precise power on command." — Inspired by Ichigo's zanpakuto from Bleach.

## Overview

Zangetsu is a natural-language-to-shell-command translator that lives in your terminal. You describe what you want in plain English, and Zangetsu produces the exact command, explains it, and optionally executes it.

## Core Commands

| Command | Description |
|---------|-------------|
| `zangetsu ask <query>` | Translate natural language to a shell command |
| `zangetsu run <query>` | Translate and immediately execute |
| `zangetsu explain <command>` | Explain what a given command does |
| `zangetsu history` | Browse and re-run past translations |
| `zangetsu alias <name> <query>` | Save a natural language query as a reusable alias |

## Architecture

```
┌─────────────────────────────────────────────────┐
│                   CLI Layer                      │
│         (argument parsing, REPL mode)            │
├─────────────────────────────────────────────────┤
│                Intent Parser                     │
│   (classify: ask / run / explain / alias)        │
├─────────────────────────────────────────────────┤
│              Context Collector                   │
│  ┌───────────┐ ┌──────────┐ ┌────────────────┐  │
│  │ OS / Shell│ │ cwd info │ │ installed tools│  │
│  │  detection│ │ git state│ │ (brew, apt...) │  │
│  └───────────┘ └──────────┘ └────────────────┘  │
├─────────────────────────────────────────────────┤
│              LLM Translation Engine              │
│  ┌───────────────────────────────────────────┐   │
│  │  System prompt: OS context + constraints  │   │
│  │  User prompt: natural language query      │   │
│  │  Response: { command, explanation, risk }  │   │
│  └───────────────────────────────────────────┘   │
├─────────────────────────────────────────────────┤
│              Safety & Execution Layer             │
│  ┌──────────┐ ┌────────────┐ ┌──────────────┐   │
│  │ Risk     │ │ Dry-run /  │ │ Sandboxed    │   │
│  │ Scoring  │ │ Confirm    │ │ Execution    │   │
│  └──────────┘ └────────────┘ └──────────────┘   │
├─────────────────────────────────────────────────┤
│              History & Alias Store               │
│          (SQLite ~/.zangetsu/history.db)          │
└─────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Context-Aware Translation
Before sending a query to the LLM, Zangetsu gathers:
- **OS and shell type** (macOS/zsh, Linux/bash, etc.)
- **Current directory context** (is it a git repo? node project? python venv?)
- **Available tools** (checks PATH for common utilities)

This context is injected into the system prompt so the LLM generates commands appropriate for the user's actual environment.

### 2. Risk Scoring
Every generated command receives a risk score:
- **Low (green):** Read-only commands (`ls`, `cat`, `grep`)
- **Medium (yellow):** Writes to files, installs packages
- **High (red):** Destructive operations (`rm -rf`, `DROP TABLE`, `--force`)

High-risk commands always require explicit confirmation. Users can configure their risk threshold.

### 3. Execution Sandbox
When `zangetsu run` is used:
- Commands are executed in a child process with configurable timeouts
- stdout/stderr are captured and streamed to the terminal
- Exit codes are tracked for history

### 4. History and Aliases
- All translations are stored in a local SQLite database
- Users can search history by natural language query or generated command
- Frequently used queries can be saved as aliases for instant recall

## Data Flow

```
User Input (natural language)
        │
        ▼
  Context Collection ──→ OS, shell, cwd, tools
        │
        ▼
  LLM API Call ──→ { command, explanation, risk_level }
        │
        ▼
  Risk Assessment ──→ auto-execute / prompt / block
        │
        ▼
  Execute or Display
        │
        ▼
  Store in History
```

## Configuration

File: `~/.zangetsu/config.toml`

```toml
[llm]
provider = "anthropic"           # anthropic | openai | ollama
model = "claude-sonnet-4-6"

[safety]
auto_execute_threshold = "low"   # low | medium | never
confirm_destructive = true
timeout_seconds = 30

[shell]
default_shell = "zsh"
```

## Tech Stack

- **Language:** Rust
- **CLI framework:** clap
- **LLM integration:** shared nakama LLM abstraction layer
- **Storage:** SQLite via rusqlite
- **Output:** colored terminal output via crossterm
