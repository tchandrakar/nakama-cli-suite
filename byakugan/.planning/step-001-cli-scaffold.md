# Step 001: CLI Scaffold

## Objective

Set up the byakugan binary crate with a fully structured CLI using clap, defining all top-level commands (review, scan, report, watch, rules, comment) and integrating the shared nakama-ui, nakama-log, and nakama-audit crates for consistent output, logging, and audit trail across the suite.

## Tasks

- [ ] Initialize the `byakugan` binary crate with `cargo init` inside the workspace
- [ ] Add workspace dependencies: `clap` (derive), `tokio`, `anyhow`, `serde`, `serde_json`
- [ ] Add shared crate dependencies: `nakama-ui`, `nakama-log`, `nakama-audit`
- [ ] Define the top-level `Cli` struct with clap derive macros
- [ ] Implement `Commands` enum with subcommands:
  - `review` -- auto-detect platform from git remote and review current PR/MR
  - `scan` -- review a specific PR with `--platform`, `--pr`/`--mr` flags
  - `report` -- generate unified review summary (terminal + markdown)
  - `watch` -- daemon mode for auto-reviewing new PRs across repos
  - `rules` -- list, add, or edit custom review rules
  - `comment` -- post a review comment to a PR/MR with positional args `<pr>` and `<message>`
- [ ] Set up `main.rs` with tokio async runtime and clap parsing
- [ ] Wire up `nakama-log` for structured logging (tracing subscriber)
- [ ] Wire up `nakama-audit` to record CLI invocations
- [ ] Wire up `nakama-ui` for consistent terminal output (spinners, tables, panels)
- [ ] Create stub handler functions for each subcommand that print placeholder messages
- [ ] Add `--format` global flag (terminal, json, markdown) for output mode selection
- [ ] Add `--verbose` / `--quiet` global flags for log level control
- [ ] Add `--config` global flag for custom config file path
- [ ] Verify `cargo build` compiles cleanly and `byakugan --help` displays all commands

## Acceptance Criteria

- `byakugan --help` shows all 6 subcommands with descriptions
- Each subcommand has its own `--help` with relevant flags
- Running any subcommand prints a placeholder message and exits cleanly
- nakama-log produces structured log output when `--verbose` is set
- nakama-audit records each CLI invocation with timestamp, command, and args
- The crate compiles with zero warnings

## Dependencies

- `nakama-ui` shared crate must be available in the workspace
- `nakama-log` shared crate must be available in the workspace
- `nakama-audit` shared crate must be available in the workspace
- Rust toolchain and cargo workspace must be configured
