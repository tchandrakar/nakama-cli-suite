# Step 001: CLI Scaffold

## Objective

Set up the tensai binary crate with a fully structured CLI using clap, defining all subcommands (brief, standup, plan, status, review, focus) and integrating the shared nakama-ui, nakama-log, and nakama-audit crates from the workspace.

## Tasks

- [ ] Initialize `tensai` as a binary crate in the workspace Cargo.toml
- [ ] Add dependencies: clap (derive), tokio, serde, serde_json, anyhow/thiserror
- [ ] Add workspace dependencies: nakama-ui, nakama-log, nakama-audit
- [ ] Define top-level `TensaiCli` struct with clap derive
- [ ] Define `Commands` enum with subcommands:
  - `brief` — generate a morning dev briefing
  - `standup` — generate standup report from activity
  - `plan` — AI-assisted day/sprint planning
  - `status` — quick status dashboard of all data sources
  - `review` — review and summarize open items
  - `focus` — enter focus mode with timer and notification suppression
- [ ] Implement `main()` with tokio runtime, command dispatch, and graceful error handling
- [ ] Wire up nakama-log for structured logging (tracing subscriber)
- [ ] Wire up nakama-audit for audit trail initialization
- [ ] Wire up nakama-ui for consistent output formatting (panels, tables, spinners)
- [ ] Add `--config` global flag for config file path
- [ ] Add `--output` global flag (json, text, table)
- [ ] Add `--verbose` / `--quiet` global flags
- [ ] Add `--no-ai` global flag to disable LLM features
- [ ] Create stub handlers for each subcommand that return `todo!()`
- [ ] Verify `cargo build` succeeds and `tensai --help` prints usage

## Acceptance Criteria

- `tensai` binary compiles and runs
- `tensai --help` displays all subcommands with descriptions
- Each subcommand has a `--help` with its own flags/args
- nakama-ui, nakama-log, nakama-audit are initialized in main
- Audit trail records CLI invocation events
- All subcommand handlers exist as stubs

## Dependencies

- Workspace Cargo.toml must include tensai as a member
- nakama-ui crate must expose panel/table/spinner APIs
- nakama-log crate must expose tracing initialization
- nakama-audit crate must expose audit logging API
