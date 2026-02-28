# Step 001: CLI Scaffold

## Objective

Set up the gate binary crate with a fully structured CLI using clap, defining all subcommands (send, explore, import, replay, flow, mock, diff, doc) and integrating the shared nakama-ui, nakama-log, and nakama-audit crates from the workspace.

## Tasks

- [ ] Initialize `gate` as a binary crate in the workspace Cargo.toml
- [ ] Add dependencies: clap (derive), tokio, serde, serde_json, anyhow/thiserror
- [ ] Add workspace dependencies: nakama-ui, nakama-log, nakama-audit
- [ ] Define top-level `GateCli` struct with clap derive
- [ ] Define `Commands` enum with subcommands:
  - `send` — send an HTTP request (manual or natural language)
  - `explore` — interactive API explorer TUI
  - `import` — import requests from cURL/OpenAPI/Postman/HAR
  - `replay` — replay a request from history
  - `flow` — execute multi-step request flows
  - `mock` — start a mock API server
  - `diff` — compare two API responses
  - `doc` — generate API documentation from collections
- [ ] Implement `main()` with tokio runtime, command dispatch, and graceful error handling
- [ ] Wire up nakama-log for structured logging (tracing subscriber)
- [ ] Wire up nakama-audit for audit trail initialization
- [ ] Wire up nakama-ui for consistent output formatting (panels, tables, spinners)
- [ ] Add `--config` global flag for config file path
- [ ] Add `--output` global flag (json, text, table, raw)
- [ ] Add `--verbose` / `--quiet` global flags
- [ ] Add `--env` global flag for environment selection
- [ ] Create stub handlers for each subcommand that return `todo!()`
- [ ] Verify `cargo build` succeeds and `gate --help` prints usage

## Acceptance Criteria

- `gate` binary compiles and runs
- `gate --help` displays all subcommands with descriptions
- Each subcommand has a `--help` with its own flags/args
- nakama-ui, nakama-log, nakama-audit are initialized in main
- Audit trail records CLI invocation events
- All subcommand handlers exist as stubs
- `--env` flag is available globally

## Dependencies

- Workspace Cargo.toml must include gate as a member
- nakama-ui crate must expose panel/table/spinner APIs
- nakama-log crate must expose tracing initialization
- nakama-audit crate must expose audit logging API
