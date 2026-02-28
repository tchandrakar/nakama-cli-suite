# Step 013: Polish

## Objective
Add quality-of-life features including diff explanation, incremental re-indexing optimization, shell completions, configuration file system, comprehensive integration tests, and IPC export for inter-tool communication.

## Tasks
- [ ] Implement diff explanation (`senku diff-explain`):
  - Parse a git diff (staged, unstaged, or between refs)
  - Identify changed functions and their purposes (from index)
  - Use LLM to explain what the changes do and why (inferred from context)
  - Highlight breaking changes or API modifications
  - `senku diff-explain` — explain staged changes
  - `senku diff-explain HEAD~3..HEAD` — explain recent commits
  - `senku diff-explain --pr 123` — explain a PR diff (if git remote available)
- [ ] Optimize incremental re-indexing:
  - File-watcher integration (notify crate) for real-time index updates
  - Background re-indexing mode: update index without blocking queries
  - Batch file changes to avoid excessive re-embedding
  - `senku index --watch` — continuous re-indexing mode
  - Performance target: <5s for incremental update of 10 changed files
- [ ] Generate shell completions:
  - `senku completions zsh` — output zsh completions
  - `senku completions bash` — output bash completions
  - `senku completions fish` — output fish completions
  - Use `clap_complete` for generation
- [ ] Implement configuration file system:
  - Location: `~/.config/senku/config.toml` (XDG-aware)
  - Per-project: `.senku/config.toml` in project root
  - Options: embedding model, chunk size, ignore patterns, search defaults, LLM model
  - `senku config show` / `config edit` / `config reset`
- [ ] Write comprehensive integration tests:
  - Full index workflow: discover -> parse -> chunk -> embed -> store
  - Search workflow: query -> retrieve -> display
  - Ask workflow: question -> retrieve -> generate -> display
  - Onboard workflow: analyze -> generate -> export
  - Map workflow: analyze -> generate graph -> render
  - Incremental index: modify file -> re-index -> verify update
  - Error handling: missing files, corrupt index, API failures
- [ ] Create mock infrastructure for testing:
  - Sample project with multiple languages
  - Mock embedding API
  - Mock LLM API
  - Pre-built test indexes
- [ ] Add performance benchmarks:
  - Indexing time by project size (100, 1K, 10K files)
  - Search latency by index size
  - Memory usage during indexing
  - Chunk generation throughput
- [ ] Implement IPC export:
  - Define JSON schema for senku data (chunks, search results, graphs)
  - `--ipc` flag to output structured JSON for other tools
  - Document IPC schema for consuming tools (especially shinigami for commit context)
- [ ] Final code cleanup: rustfmt, clippy, rustdoc on all public APIs
- [ ] Add `--version` with build metadata (git hash, build date)
- [ ] Ensure all error messages are user-friendly with actionable suggestions

## Acceptance Criteria
- Diff explanation provides clear, contextual explanations of code changes
- Incremental re-indexing updates the index in under 5 seconds for small changes
- Shell completions install and work for zsh, bash, and fish
- Config file system supports global and per-project configuration
- Integration tests pass with mock APIs and sample projects
- IPC export produces well-documented JSON schema
- `cargo clippy` reports zero warnings
- All public APIs have rustdoc comments

## Dependencies
- Step 001 through Step 012 (all prior steps must be complete)
