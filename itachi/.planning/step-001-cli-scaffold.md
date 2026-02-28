# Step 001: CLI Scaffold

## Objective

Set up the itachi binary crate with a fully structured CLI using clap, defining all top-level commands (jira, wiki, ask, brief, onboard, standup, create, link, sprint) and integrating the shared nakama-ui, nakama-log, and nakama-audit crates for consistent output, logging, and audit trail.

## Tasks

- [ ] Initialize the `itachi` binary crate with `cargo init` inside the workspace
- [ ] Add workspace dependencies: `clap` (derive), `tokio`, `anyhow`, `serde`, `serde_json`
- [ ] Add shared crate dependencies: `nakama-ui`, `nakama-log`, `nakama-audit`
- [ ] Define the top-level `Cli` struct with clap derive macros
- [ ] Implement `Commands` enum with subcommands:
  - `jira <query>` -- natural language Jira query (translated to JQL)
  - `wiki <query>` -- natural language Confluence search (translated to CQL)
  - `ask <question>` -- cross-platform intelligence (queries both Jira and Confluence)
  - `brief <team>` -- team briefing (sprint health, blockers, relevant docs)
  - `onboard <project|service>` -- generate onboarding brief from Jira + Confluence
  - `standup` -- auto-generate standup from yesterday's Jira transitions
  - `create <type> <summary>` -- create a Jira ticket from the CLI
  - `link <issue> --doc <page>` -- link a Jira issue to a Confluence page
  - `sprint [board]` -- sprint dashboard (progress, velocity, burndown)
- [ ] Set up `main.rs` with tokio async runtime and clap parsing
- [ ] Wire up `nakama-log` for structured logging (tracing subscriber)
- [ ] Wire up `nakama-audit` to record CLI invocations
- [ ] Wire up `nakama-ui` for consistent terminal output (tables, panels, charts)
- [ ] Create stub handler functions for each subcommand
- [ ] Add `--format` global flag (terminal, json, markdown)
- [ ] Add `--verbose` / `--quiet` global flags
- [ ] Add `--config` global flag for custom config file path
- [ ] Add `--instance` flag for specifying Atlassian instance URL
- [ ] Verify `cargo build` compiles cleanly and `itachi --help` displays all commands

## Acceptance Criteria

- `itachi --help` shows all 9 subcommands with descriptions
- Each subcommand has its own `--help` with relevant flags and arguments
- Running any subcommand prints a placeholder message and exits cleanly
- nakama-log produces structured log output when `--verbose` is set
- nakama-audit records each CLI invocation
- The crate compiles with zero warnings

## Dependencies

- `nakama-ui` shared crate must be available in the workspace
- `nakama-log` shared crate must be available in the workspace
- `nakama-audit` shared crate must be available in the workspace
- Rust toolchain and cargo workspace must be configured
