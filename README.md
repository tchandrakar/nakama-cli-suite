# Nakama CLI Suite

> Your personal dev crew — 11 anime-inspired CLI tools that form a complete developer toolkit.

"Nakama" means crewmates/companions. Like the Straw Hat crew where each member has a unique specialty but they work as one unit, these 11 tools each have a distinct role but form your complete developer toolkit — all sailing under one flag.

## The Crew

| Tool | Name | Source | Description |
|------|------|--------|-------------|
| Smart Shell Companion | [zangetsu](./zangetsu/) | Bleach | Natural language to shell commands |
| Git Workflow Automator | [shinigami](./shinigami/) | Death Note | Intelligent git workflows and changelogs |
| Infra Debugger | [jogan](./jogan/) | Boruto | Cross-layer infrastructure debugging |
| Codebase Knowledge Base | [senku](./senku/) | Dr. Stone | Codebase indexing, search, and Q&A |
| Log Analyzer | [sharingan](./sharingan/) | Naruto | AI-powered log pattern detection |
| Dev Briefing Dashboard | [tensai](./tensai/) | Kuroko no Basket | Daily developer briefing and task planning |
| Test Generator | [mugen](./mugen/) | Samurai Champloo | AI test generation and mutation testing |
| API Explorer | [gate](./gate/) | Steins;Gate | Interactive API exploration and testing |
| PR Reviewer | [byakugan](./byakugan/) | Naruto | Platform-agnostic PR review (GitHub, GitLab, Bitbucket) |
| Google/Gemini Search | [kami](./kami/) | Dragon Ball | Grounded search and research via Gemini |
| Atlassian Hub | [itachi](./itachi/) | Naruto | Jira + Confluence cross-platform intelligence |

## Repository Structure

```
nakama-cli-suite/
├── zangetsu/          # Smart Shell Companion
├── shinigami/         # Git Workflow Automator
├── jogan/             # Infra Debugger
├── senku/             # Codebase Knowledge Base
├── sharingan/         # Log Analyzer
├── tensai/            # Dev Briefing Dashboard
├── mugen/             # Test Generator
├── gate/              # API Explorer
├── byakugan/          # PR Reviewer
├── kami/              # Google/Gemini Search
├── itachi/            # Atlassian Jira & Confluence Hub
└── shared/            # Shared libraries and utilities
```

## Installation

### Quick Install (Pre-built Binaries)

No Rust toolchain required. Downloads pre-built binaries from the latest GitHub release:

```bash
curl -fsSL https://raw.githubusercontent.com/tchandrakar/nakama-cli-suite/main/install-release.sh | bash
```

Supports: Linux (x86_64), macOS (x86_64, Apple Silicon).

### Build from Source

Requires Rust >= 1.75:

```bash
git clone https://github.com/tchandrakar/nakama-cli-suite.git
cd nakama-cli-suite
./install.sh
```

This builds all 11 tools in release mode and installs them to `~/.cargo/bin/`.

### Configuration

After installation, you need to configure two things: your **AI provider** and your **API keys/tokens**.

#### 1. Choose Your AI Provider

Edit `~/.nakama/config.toml` to set your default AI provider:

```toml
[ai]
default_provider = "anthropic"  # anthropic | openai | google | ollama
```

You can override per-command with `--ai-provider`:

```bash
zangetsu ask "find large files" --ai-provider=openai --ai-model=gpt-4.1
```

#### 2. Store API Keys

API keys are managed by `nakama-vault`, which tries storage backends in priority order:

##### Option A: OS Keychain (Recommended)

The most secure option. Uses macOS Keychain, GNOME Keyring (Linux), or Windows Credential Manager:

```bash
# AI provider keys — store whichever provider(s) you use
nakama-vault store anthropic api_key sk-ant-...
nakama-vault store openai api_key sk-...
nakama-vault store google api_key AIza...

# Platform tokens — needed by tools that interact with code platforms
nakama-vault store github api_key ghp_...       # byakugan, shinigami
nakama-vault store gitlab api_key glpat-...     # byakugan
nakama-vault store bitbucket api_key ...        # byakugan
```

##### Option B: Environment Variables (CI/CD or Fallback)

When no keychain is available (e.g., CI/CD, containers, SSH sessions), set environment variables using the `NAKAMA_<SERVICE>_<KEY>` pattern:

```bash
# AI providers
export NAKAMA_ANTHROPIC_API_KEY="sk-ant-..."
export NAKAMA_OPENAI_API_KEY="sk-..."
export NAKAMA_GOOGLE_API_KEY="AIza..."

# Platform tokens
export NAKAMA_GITHUB_API_KEY="ghp_..."
export NAKAMA_GITLAB_API_KEY="glpat-..."
export NAKAMA_BITBUCKET_API_KEY="..."
```

> A warning is logged when env vars are used instead of the keychain.

##### Option C: Encrypted File Store (Automatic Fallback)

If the OS keychain is unavailable, `nakama-vault` automatically falls back to an AES-256-GCM encrypted file store at `~/.nakama/vault/`. Key derivation uses Argon2id from a master password. No setup needed — it activates transparently.

#### 3. Platform Tokens (Alternative)

For GitHub, GitLab, and Bitbucket, you can also configure tokens directly in `~/.nakama/config.toml`:

```toml
[platforms.github]
token = "ghp_..."
api_url = "https://api.github.com"          # default; override for GitHub Enterprise

[platforms.gitlab]
token = "glpat-..."
api_url = "https://gitlab.com/api/v4"       # default; override for self-hosted

[platforms.bitbucket]
username = "your-username"
app_password = "..."
api_url = "https://api.bitbucket.org/2.0"   # default; override for Data Center
```

> **Note:** AI provider API keys are **never** stored in config files — they must go through the vault (keychain or env vars).

#### Credential Resolution Order

```
OS Keychain  →  Encrypted File Store  →  Environment Variables
```

For platform tokens specifically:

```
Vault (any backend)  →  config.toml [platforms.*] section
```

#### 4. Optional: AI Model Overrides

Customize which models each provider uses per tier:

```toml
[ai.anthropic]
model_fast = "claude-haiku-4-5-20251001"
model_balanced = "claude-sonnet-4-6"
model_powerful = "claude-opus-4-6"
# base_url = "https://your-proxy.example.com"  # optional proxy

[ai.openai]
model_fast = "gpt-4.1-nano"
model_balanced = "gpt-4.1-mini"
model_powerful = "gpt-4.1"

[ai.google]
model_fast = "gemini-2.5-flash"
model_balanced = "gemini-2.5-flash"
model_powerful = "gemini-2.5-pro"

[ai.ollama]
base_url = "http://localhost:11434"     # no API key needed
model_fast = "llama3:8b"
model_balanced = "llama3:70b"
```

#### 5. Optional: Spending Limits

Track and cap your AI API costs:

```toml
[ai.budget]
weekly_limit_usd = 10.00
alert_threshold_percent = 80
hard_limit = true    # true = block requests at limit; false = warn only
```

#### 6. Optional: Per-Tool Overrides

Any tool can override the global AI config in its own config file (e.g., `~/.nakama/byakugan/config.toml`):

```toml
[ai]
default_provider = "openai"    # use OpenAI just for this tool
```

### Auto-Update

All tools automatically check for new releases once every 24 hours. When a new version is available, you'll see a notice after the command finishes. To disable:

```toml
[updates]
enabled = false
```

## Architecture

Each tool is a standalone CLI binary built in its own directory. They share common libraries from `shared/` for configuration, LLM provider abstraction, and output formatting.

### Shared Infrastructure

All tools are built on a common foundation that enforces security, consistency, and interoperability:

| Layer | Docs | Purpose |
|-------|------|---------|
| **nakama-vault** | [security.md](./shared/docs/security.md) | OS keychain credential storage, zero plaintext secrets, AES-256 fallback |
| **nakama-ui** | [logging.md](./shared/docs/logging.md) | Claude-style terminal output, spinners, syntax highlighting, tables |
| **nakama-ai** | [ai-providers.md](./shared/docs/ai-providers.md) | Multi-provider AI abstraction (Claude, OpenAI, Gemini, Ollama) |
| **nakama-ipc** | [ipc.md](./shared/docs/ipc.md) | Inter-tool communication, Nakama Message Protocol, pipe support |
| **nakama-audit** | [audit.md](./shared/docs/audit.md) | Tamper-evident audit trail, AI usage tracking, compliance |

### Security Standards

This is a SaaS-grade application. Every tool follows:
- All credentials stored in OS keychain (never plaintext on disk)
- All HTTP traffic over TLS 1.2+ (HTTPS only)
- All user input validated, all LLM output treated as untrusted
- All sensitive operations logged to tamper-evident audit trail
- All secrets auto-zeroed in memory, redacted from logs and errors
- Full compliance: OWASP Top 10, SOC 2, GDPR-aware

### AI Provider Support

Users choose their preferred AI provider once — all tools use it:
- **Anthropic** (Claude) — opus, sonnet, haiku
- **OpenAI** (GPT) — gpt-4.1, gpt-4.1-mini, gpt-4.1-nano
- **Google** (Gemini) — gemini-2.5-pro, gemini-2.5-flash
- **Ollama** (Local) — llama, mistral, codellama

Override per-tool or per-command. See [ai-providers.md](./shared/docs/ai-providers.md).

See each tool's `docs/architecture.md` for detailed design documents.

## Suite Synergies

Some tools become exponentially more powerful when combined:

| Combination | Synergy |
|-------------|---------|
| **Itachi + Tensai** | Morning briefing auto-includes Jira sprint data + Confluence updates |
| **Itachi + Shinigami** | Commit messages auto-link to Jira tickets, PR descriptions pull Confluence context |
| **Itachi + Byakugan** | PR reviews reference Jira requirements and Confluence design docs for completeness |
| **Itachi + Senku** | Codebase queries enriched with org context — pulls code history AND the Confluence RFC |
| **Sharingan + Jogan** | Log anomalies feed directly into infra diagnosis for full-stack root cause analysis |
| **Mugen + Byakugan** | Test coverage gaps found during PR review trigger targeted test generation |
| **Kami + Senku** | External research grounded against your own codebase knowledge |

## License

MIT
