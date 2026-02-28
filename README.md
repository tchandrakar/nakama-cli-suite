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

After installation, edit `~/.nakama/config.toml` to set your AI provider and API key:

```toml
[ai]
default_provider = "anthropic"  # or: openai, google, ollama
```

Store your API key securely:

```bash
# Via environment variable
export ANTHROPIC_API_KEY="your-key"

# Or via the vault (OS keychain)
nakama vault store nakama anthropic_api_key your-key
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
