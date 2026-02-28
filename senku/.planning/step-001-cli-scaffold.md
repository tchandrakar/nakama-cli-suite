# Step 001: CLI Scaffold

## Objective
Set up the senku binary crate with a fully-structured CLI using clap, defining all top-level subcommands (`index`, `ask`, `explain`, `map`, `onboard`, `search`, `diff-explain`) and integrating shared crates (`nakama-ui`, `nakama-log`, `nakama-audit`) from the workspace.

## Tasks
- [ ] Create `senku/Cargo.toml` with binary target and workspace dependencies
- [ ] Add `clap` (derive) as primary CLI framework with version, about, and help metadata
- [ ] Define top-level `Cli` struct with `#[command(subcommand)]` enum:
  - `Index` — index a codebase for semantic search and knowledge graph
  - `Ask` — ask questions about the codebase using RAG
  - `Explain` — explain a specific file, function, or module
  - `Map` — generate architecture and dependency maps
  - `Onboard` — generate onboarding guide for the project
  - `Search` — semantic search across the codebase
  - `DiffExplain` — explain changes in a git diff
- [ ] Wire up `nakama-ui` for styled terminal output (spinners, progress bars, code blocks)
- [ ] Wire up `nakama-log` for structured logging (tracing subscriber init)
- [ ] Wire up `nakama-audit` for audit event emission on startup/shutdown
- [ ] Create `main.rs` with async runtime (`tokio`) and subcommand dispatch skeleton
- [ ] Create module stubs: `discovery.rs`, `parser.rs`, `chunking.rs`, `embedding.rs`, `storage.rs`, `retrieval.rs`, `graph.rs`, `answer.rs`, `onboard.rs`, `map.rs`, `config.rs`
- [ ] Add `--verbose` / `--quiet` global flags for log-level control
- [ ] Add `--config` global flag for custom config file path
- [ ] Add `--project` global flag for specifying project root (default: cwd)
- [ ] Verify `cargo build` compiles cleanly and `senku --help` produces correct output
- [ ] Add the crate to the workspace `Cargo.toml` members list

## Acceptance Criteria
- `cargo build -p senku` succeeds with zero warnings
- `senku --help` displays all subcommands with descriptions
- Each subcommand displays its own help text
- `nakama-ui`, `nakama-log`, and `nakama-audit` are initialized in `main()`
- Module stubs exist and are imported
- The crate is a valid workspace member

## Dependencies
- None (this is the first step)
