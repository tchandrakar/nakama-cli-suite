# Step 001: CLI Scaffold

## Objective

Set up the sharingan binary crate with a fully structured CLI using clap, defining all subcommands (tail, explain, scan, correlate, predict, filter, summary) and integrating the shared nakama-ui, nakama-log, and nakama-audit crates from the workspace.

## Tasks

- [ ] Initialize `sharingan` as a binary crate in the workspace Cargo.toml
- [ ] Add dependencies: clap (derive), tokio, serde, serde_json, anyhow/thiserror
- [ ] Add workspace dependencies: nakama-ui, nakama-log, nakama-audit
- [ ] Define top-level `SharinganCli` struct with clap derive
- [ ] Define `Commands` enum with subcommands:
  - `tail` — live log streaming with real-time analysis
  - `explain` — deep explanation of a specific log entry or error
  - `scan` — batch scan of log file(s) for issues
  - `correlate` — cross-source log correlation
  - `predict` — predictive analysis from log trends
  - `filter` — natural language log filtering
  - `summary` — generate a summary of log activity
- [ ] Implement `main()` with tokio runtime, command dispatch, and graceful error handling
- [ ] Wire up nakama-log for structured logging (tracing subscriber)
- [ ] Wire up nakama-audit for audit trail initialization
- [ ] Wire up nakama-ui for consistent output formatting (panels, tables, spinners)
- [ ] Add `--config` global flag for config file path
- [ ] Add `--output` global flag (json, text, table)
- [ ] Add `--verbose` / `--quiet` global flags
- [ ] Create stub handlers for each subcommand that return `todo!()`
- [ ] Verify `cargo build` succeeds and `sharingan --help` prints usage

## Acceptance Criteria

- `sharingan` binary compiles and runs
- `sharingan --help` displays all subcommands with descriptions
- Each subcommand has a `--help` with its own flags/args
- nakama-ui, nakama-log, nakama-audit are initialized in main
- Audit trail records CLI invocation events
- All subcommand handlers exist as stubs

## Dependencies

- Workspace Cargo.toml must include sharingan as a member
- nakama-ui crate must expose panel/table/spinner APIs
- nakama-log crate must expose tracing initialization
- nakama-audit crate must expose audit logging API
