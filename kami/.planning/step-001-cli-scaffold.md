# Step 001: CLI Scaffold

## Objective

Set up the kami binary crate with a fully structured CLI using clap, defining all top-level commands (search, deep, summarize, ask, grounded, compare, pipe) and integrating the shared nakama-ui, nakama-log, and nakama-audit crates for consistent output, logging, and audit trail.

## Tasks

- [ ] Initialize the `kami` binary crate with `cargo init` inside the workspace
- [ ] Add workspace dependencies: `clap` (derive), `tokio`, `anyhow`, `serde`, `serde_json`
- [ ] Add shared crate dependencies: `nakama-ui`, `nakama-log`, `nakama-audit`
- [ ] Define the top-level `Cli` struct with clap derive macros
- [ ] Implement `Commands` enum with subcommands:
  - `search <query>` -- quick grounded search with cited results
  - `deep <query>` -- multi-step research with exploration and synthesis
  - `summarize <url>` -- fetch a URL and summarize with Gemini
  - `ask [question]` -- conversational mode with session memory (REPL if no question)
  - `grounded <claim>` -- fact-check a claim with grounded citations
  - `compare <a> vs <b>` -- side-by-side comparison with sourced evidence
  - `pipe` -- read from stdin, enrich with search context, output to stdout
- [ ] Set up `main.rs` with tokio async runtime and clap parsing
- [ ] Wire up `nakama-log` for structured logging (tracing subscriber)
- [ ] Wire up `nakama-audit` to record CLI invocations
- [ ] Wire up `nakama-ui` for consistent terminal output (spinners, panels, citations)
- [ ] Create stub handler functions for each subcommand
- [ ] Add `--format` global flag (terminal, json, markdown)
- [ ] Add `--verbose` / `--quiet` global flags
- [ ] Add `--config` global flag for custom config file path
- [ ] Add `--no-cache` flag to bypass cache for any command
- [ ] Verify `cargo build` compiles cleanly and `kami --help` displays all commands

## Acceptance Criteria

- `kami --help` shows all 7 subcommands with descriptions
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
