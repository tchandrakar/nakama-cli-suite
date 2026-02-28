# Step 001: CLI Scaffold

## Objective
Set up the shinigami binary crate with a fully-structured CLI using clap, defining all top-level subcommands (`commit`, `reap`, `branch`, `squash`, `release`, `review`, `hook`) and integrating shared crates (`nakama-ui`, `nakama-log`, `nakama-audit`) from the workspace.

## Tasks
- [ ] Create `shinigami/Cargo.toml` with binary target and workspace dependencies
- [ ] Add `clap` (derive) as primary CLI framework with version, about, and help metadata
- [ ] Define top-level `Cli` struct with `#[command(subcommand)]` enum:
  - `Commit` — generate semantic commit messages from staged changes
  - `Reap` — generate changelog from commit history between refs
  - `Branch` — generate branch names from natural language descriptions
  - `Squash` — interactive squash helper with message regeneration
  - `Release` — generate release notes and create git tags
  - `Review` — pre-commit review of staged changes
  - `Hook` — install/manage git hooks
- [ ] Wire up `nakama-ui` for styled terminal output (spinners, prompts, colored diffs)
- [ ] Wire up `nakama-log` for structured logging (tracing subscriber init)
- [ ] Wire up `nakama-audit` for audit event emission on startup/shutdown
- [ ] Create `main.rs` with async runtime (`tokio`) and subcommand dispatch skeleton
- [ ] Create module stubs: `git.rs`, `analyzer.rs`, `commit_gen.rs`, `changelog.rs`, `release.rs`, `hooks.rs`, `branch.rs`, `config.rs`
- [ ] Add `--verbose` / `--quiet` global flags for log-level control
- [ ] Add `--config` global flag for custom config file path
- [ ] Add `--repo` global flag for specifying git repository path (default: cwd)
- [ ] Verify `cargo build` compiles cleanly and `shinigami --help` produces correct output
- [ ] Add the crate to the workspace `Cargo.toml` members list

## Acceptance Criteria
- `cargo build -p shinigami` succeeds with zero warnings
- `shinigami --help` displays all subcommands with descriptions
- Each subcommand displays its own help text
- `nakama-ui`, `nakama-log`, and `nakama-audit` are initialized in `main()`
- Module stubs exist and are imported
- The crate is a valid workspace member

## Dependencies
- None (this is the first step)
