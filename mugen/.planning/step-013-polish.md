# Step 013: Polish and Integration

## Objective

Finalize mugen with the test review command, shell completions, configuration file support, comprehensive integration tests, and IPC integration to consume code analysis from byakugan.

## Tasks

- [ ] Implement `review` subcommand:
  - [ ] Analyze existing test suite quality
  - [ ] Identify test smells (empty tests, commented-out assertions, magic numbers)
  - [ ] Identify untested functions
  - [ ] Identify tests with poor assertions (too many mocks, trivial checks)
  - [ ] Calculate test-to-code ratio
  - [ ] LLM option: generate improvement suggestions
  - [ ] Output quality report with actionable recommendations
- [ ] Implement shell completions:
  - [ ] Generate completions for bash, zsh, fish, PowerShell via clap_complete
  - [ ] Dynamic completion for `--strategy`, `--language`, `--framework`
  - [ ] File path completion for target files
  - [ ] Add `completions` subcommand to generate completion scripts
- [ ] Implement configuration file:
  - [ ] TOML config file at `~/.config/nakama/mugen.toml`
  - [ ] Override with `--config` flag
  - [ ] Sections: generation, validation, coverage, mutation, watch, security
  - [ ] Per-language settings (framework, conventions, file patterns)
  - [ ] Default config generation (`mugen config init`)
  - [ ] Config validation on load with helpful error messages
- [ ] Implement IPC integration:
  - [ ] Consume byakugan code analysis data (dependency graphs, complexity metrics)
  - [ ] Use byakugan insights to improve test strategy selection
  - [ ] Export test generation results for tensai briefings
  - [ ] Define consumed/produced IPC message types
  - [ ] Graceful handling when IPC sources are unavailable
- [ ] Write integration tests:
  - [ ] End-to-end: analyze source -> select strategy -> generate -> validate -> write
  - [ ] Test generation for each supported language
  - [ ] Coverage analysis with sample projects
  - [ ] Mutation testing with sample functions
  - [ ] Edge case generation for various types
  - [ ] Watch mode with simulated file changes
  - [ ] Convention detection on real project structures
- [ ] Performance testing:
  - [ ] Single function generation time (target: <30s including LLM)
  - [ ] Batch generation time (target: <5min for 20 functions)
  - [ ] Watch mode response time (target: <2s from change to generation start)
- [ ] Documentation:
  - [ ] Man page generation
  - [ ] Usage examples for each subcommand
  - [ ] Configuration reference
  - [ ] Supported language/framework matrix
- [ ] Final cleanup:
  - [ ] Remove all `todo!()` stubs
  - [ ] Error message review (user-friendly, actionable)
  - [ ] Clippy clean
  - [ ] cargo fmt

## Acceptance Criteria

- `review` command identifies test quality issues and generates recommendations
- Shell completions work for bash, zsh, and fish
- Config file is loaded and validated, with sensible defaults
- IPC consumption from byakugan enhances strategy selection
- All integration tests pass for each supported language
- No `todo!()` stubs remain in codebase
- `cargo clippy` and `cargo fmt` pass with no warnings
- Performance targets are met for generation and validation

## Dependencies

- All previous steps (001-012) must be complete
- nakama-ai for review analysis
- byakugan IPC protocol for code analysis data
- clap_complete crate for shell completions
