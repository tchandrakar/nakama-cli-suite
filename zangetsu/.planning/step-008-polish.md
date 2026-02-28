# Step 008: Polish

## Objective
Add quality-of-life features including REPL mode for interactive sessions, shell completions for all major shells, a configuration file system, man page generation, comprehensive integration tests, and performance benchmarks.

## Tasks
- [ ] Implement REPL mode (`zangetsu repl` or `zangetsu -i`):
  - Interactive prompt loop using `rustyline` or similar
  - Command history within REPL session
  - Support all subcommands without the `zangetsu` prefix
  - Special REPL commands: `.quit`, `.clear`, `.history`, `.help`
  - Graceful Ctrl+C and Ctrl+D handling
- [ ] Generate shell completions:
  - `zangetsu completions zsh` — output zsh completions
  - `zangetsu completions bash` — output bash completions
  - `zangetsu completions fish` — output fish completions
  - Use `clap_complete` for generation
  - Include alias names in dynamic completions
- [ ] Implement configuration file system:
  - Location: `~/.config/zangetsu/config.toml` (XDG-aware)
  - Configurable options: default risk threshold, auto-execute for low risk, LLM model preference, timeout, output format
  - Config validation on load with helpful error messages
  - `zangetsu config show` — display current config
  - `zangetsu config edit` — open config in $EDITOR
  - `zangetsu config reset` — reset to defaults
- [ ] Generate man page using `clap_mangen`
- [ ] Write comprehensive integration tests:
  - End-to-end: ask -> confirm -> execute -> store in history
  - Alias lifecycle: create -> list -> run -> rename -> delete
  - History lifecycle: record -> search -> view -> delete
  - Config loading and validation
  - Error handling: network failure, invalid LLM response, missing tools
- [ ] Add performance benchmarks using `criterion`:
  - Context collection time
  - Prompt assembly time
  - Response parsing time
  - History search performance with 10K entries
- [ ] Add `--output-format` global flag: `text` (default), `json`, `plain`
- [ ] Ensure all error messages are user-friendly with actionable suggestions
- [ ] Add `--version` with build metadata (git hash, build date)
- [ ] Final code cleanup: rustfmt, clippy with all lints, documentation on public APIs
- [ ] Prepare IPC export: define JSON schema for inter-tool communication

## Acceptance Criteria
- REPL mode allows interactive multi-query sessions
- Shell completions install and work correctly for zsh, bash, and fish
- Config file is loaded on startup and respected by all subsystems
- Man page generates and renders correctly
- Integration test suite passes with mocked LLM backend
- Benchmarks establish baseline performance numbers
- `cargo clippy` reports zero warnings
- All public APIs have rustdoc comments

## Dependencies
- Step 001 through Step 007 (all prior steps must be complete)
