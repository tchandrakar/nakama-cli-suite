# Step 010: Polish

## Objective
Add quality-of-life features including pre-commit review, squash helper, shell completions, configuration file system, comprehensive integration tests, and IPC export for inter-tool communication.

## Tasks
- [ ] Implement pre-commit review (`shinigami review`):
  - Show staged diff with syntax highlighting
  - LLM-powered review: identify potential issues, suggest improvements
  - Display review as annotated diff or summary
  - `--check` flag: exit with non-zero if issues found (CI-friendly)
- [ ] Implement squash helper (`shinigami squash`):
  - Show commits eligible for squashing
  - Select range of commits to squash
  - Regenerate combined commit message from all squashed commits
  - Execute interactive rebase with pre-filled message
  - `--last N` flag to squash last N commits
- [ ] Generate shell completions:
  - `shinigami completions zsh` — output zsh completions
  - `shinigami completions bash` — output bash completions
  - `shinigami completions fish` — output fish completions
  - Use `clap_complete` for generation
- [ ] Implement configuration file system:
  - Global: `~/.config/shinigami/config.toml` (XDG-aware)
  - Per-repo: `.shinigami.toml` in repo root
  - Per-repo overrides global settings
  - Options: default style, max subject length, scopes list, template paths, LLM model
  - `shinigami config show` / `config edit` / `config reset`
- [ ] Write comprehensive integration tests:
  - Commit generation end-to-end (stage -> generate -> commit)
  - Changelog generation with various commit ranges
  - Release workflow (detect version -> generate notes -> tag)
  - Hook installation and invocation
  - Branch naming with various inputs
  - Multi-style commit generation
- [ ] Add performance benchmarks:
  - Diff parsing time for large diffs
  - Semantic analysis time
  - Prompt assembly time
- [ ] Implement IPC export:
  - Define JSON schema for shinigami data (commits, changelogs, releases)
  - `--ipc` flag to output structured JSON for other tools
  - Document IPC schema for consuming tools
- [ ] Final code cleanup: rustfmt, clippy, rustdoc on all public APIs
- [ ] Add `--version` with build metadata (git hash, build date)
- [ ] Ensure all error messages are user-friendly with suggestions

## Acceptance Criteria
- Pre-commit review identifies common issues in staged changes
- Squash helper correctly combines commits and regenerates messages
- Shell completions install and work for zsh, bash, and fish
- Config file system supports global and per-repo configuration
- Integration tests pass with mocked LLM and temp git repos
- IPC export produces well-documented JSON schema
- `cargo clippy` reports zero warnings
- All public APIs have rustdoc comments

## Dependencies
- Step 001 through Step 009 (all prior steps must be complete)
