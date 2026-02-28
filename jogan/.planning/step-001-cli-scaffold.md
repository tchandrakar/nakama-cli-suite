# Step 001: CLI Scaffold

## Objective
Set up the jogan binary crate with a fully-structured CLI using clap, defining all top-level subcommands (`scan`, `diagnose`, `trace`, `health`, `explain`, `watch`) and integrating shared crates (`nakama-ui`, `nakama-log`, `nakama-audit`) from the workspace.

## Tasks
- [ ] Create `jogan/Cargo.toml` with binary target and workspace dependencies
- [ ] Add `clap` (derive) as primary CLI framework with version, about, and help metadata
- [ ] Define top-level `Cli` struct with `#[command(subcommand)]` enum:
  - `Scan` — scan infrastructure for known issues and anomalies
  - `Diagnose` — diagnose a specific issue from natural language description
  - `Trace` — trace a request across services
  - `Health` — quick health check across all configured collectors
  - `Explain` — explain an error message or infrastructure concept
  - `Watch` — continuous monitoring TUI dashboard
- [ ] Wire up `nakama-ui` for styled terminal output (spinners, tables, colored severity)
- [ ] Wire up `nakama-log` for structured logging (tracing subscriber init)
- [ ] Wire up `nakama-audit` for audit event emission on startup/shutdown
- [ ] Create `main.rs` with async runtime (`tokio`) and subcommand dispatch skeleton
- [ ] Create module stubs: `collector.rs`, `kubernetes.rs`, `docker.rs`, `cloud.rs`, `symptom.rs`, `rules.rs`, `analysis.rs`, `report.rs`, `watch.rs`, `config.rs`
- [ ] Add `--verbose` / `--quiet` global flags for log-level control
- [ ] Add `--config` global flag for custom config file path
- [ ] Add `--target` global flag for specifying infrastructure target (k8s, docker, aws, etc.)
- [ ] Verify `cargo build` compiles cleanly and `jogan --help` produces correct output
- [ ] Add the crate to the workspace `Cargo.toml` members list

## Acceptance Criteria
- `cargo build -p jogan` succeeds with zero warnings
- `jogan --help` displays all subcommands with descriptions
- Each subcommand displays its own help text
- `nakama-ui`, `nakama-log`, and `nakama-audit` are initialized in `main()`
- Module stubs exist and are imported
- The crate is a valid workspace member

## Dependencies
- None (this is the first step)
