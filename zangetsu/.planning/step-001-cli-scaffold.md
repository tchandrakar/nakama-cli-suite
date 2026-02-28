# Step 001: CLI Scaffold

## Objective
Set up the zangetsu binary crate with a fully-structured CLI using clap, defining all top-level subcommands (`ask`, `run`, `explain`, `history`, `alias`) and integrating shared crates (`nakama-ui`, `nakama-log`, `nakama-audit`) from the workspace.

## Tasks
- [ ] Create `zangetsu/Cargo.toml` with binary target and workspace dependencies
- [ ] Add `clap` (derive) as primary CLI framework with version, about, and help metadata
- [ ] Define top-level `Cli` struct with `#[command(subcommand)]` enum:
  - `Ask` — translate natural language to shell command
  - `Run` — execute a previously generated or stored command
  - `Explain` — explain what a given command does
  - `History` — browse/search past queries and generated commands
  - `Alias` — manage saved query-to-command aliases
- [ ] Wire up `nakama-ui` for styled terminal output (spinners, prompts, colored text)
- [ ] Wire up `nakama-log` for structured logging (tracing subscriber init)
- [ ] Wire up `nakama-audit` for audit event emission on startup/shutdown
- [ ] Create `main.rs` with async runtime (`tokio`) and subcommand dispatch skeleton
- [ ] Create module stubs: `context.rs`, `translate.rs`, `execute.rs`, `history.rs`, `alias.rs`, `config.rs`
- [ ] Add `--verbose` / `--quiet` global flags for log-level control
- [ ] Add `--config` global flag for custom config file path
- [ ] Verify `cargo build` compiles cleanly and `zangetsu --help` produces correct output
- [ ] Add the crate to the workspace `Cargo.toml` members list

## Acceptance Criteria
- `cargo build -p zangetsu` succeeds with zero warnings
- `zangetsu --help` displays all subcommands with descriptions
- `zangetsu ask --help`, `zangetsu run --help`, etc. each display their own help text
- `nakama-ui`, `nakama-log`, and `nakama-audit` are initialized in `main()`
- Module stubs exist and are imported but can be empty
- The crate is a valid workspace member

## Dependencies
- None (this is the first step)
