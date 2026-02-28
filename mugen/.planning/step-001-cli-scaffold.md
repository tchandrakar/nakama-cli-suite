# Step 001: CLI Scaffold

## Objective

Set up the mugen binary crate with a fully structured CLI using clap, defining all subcommands (gen, cover, mutate, edge, fuzz, review, watch) and integrating the shared nakama-ui, nakama-log, and nakama-audit crates from the workspace.

## Tasks

- [ ] Initialize `mugen` as a binary crate in the workspace Cargo.toml
- [ ] Add dependencies: clap (derive), tokio, serde, serde_json, anyhow/thiserror
- [ ] Add workspace dependencies: nakama-ui, nakama-log, nakama-audit
- [ ] Define top-level `MugenCli` struct with clap derive
- [ ] Define `Commands` enum with subcommands:
  - `gen` — generate tests for specified functions/files
  - `cover` — analyze coverage and generate tests for uncovered code
  - `mutate` — run mutation testing and generate killing tests
  - `edge` — generate edge case tests
  - `fuzz` — generate property-based and fuzz tests
  - `review` — review and assess existing test quality
  - `watch` — continuous test generation on file changes
- [ ] Implement `main()` with tokio runtime, command dispatch, and graceful error handling
- [ ] Wire up nakama-log for structured logging (tracing subscriber)
- [ ] Wire up nakama-audit for audit trail initialization
- [ ] Wire up nakama-ui for consistent output formatting (panels, tables, spinners)
- [ ] Add `--config` global flag for config file path
- [ ] Add `--output` global flag (json, text, table)
- [ ] Add `--verbose` / `--quiet` global flags
- [ ] Add `--language` global flag for target language override
- [ ] Add `--dry-run` global flag (show tests without writing)
- [ ] Create stub handlers for each subcommand that return `todo!()`
- [ ] Verify `cargo build` succeeds and `mugen --help` prints usage

## Acceptance Criteria

- `mugen` binary compiles and runs
- `mugen --help` displays all subcommands with descriptions
- Each subcommand has a `--help` with its own flags/args
- nakama-ui, nakama-log, nakama-audit are initialized in main
- Audit trail records CLI invocation events
- All subcommand handlers exist as stubs
- `--dry-run` flag is available globally

## Dependencies

- Workspace Cargo.toml must include mugen as a member
- nakama-ui crate must expose panel/table/spinner APIs
- nakama-log crate must expose tracing initialization
- nakama-audit crate must expose audit logging API
