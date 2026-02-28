# Step 013: Polish

## Objective

Final polish pass for byakugan: generate shell completions, finalize configuration management, write comprehensive integration tests with mock platform APIs, ensure IPC export schemas are stable, and prepare for release.

## Tasks

- [ ] Generate shell completions:
  - Use clap's built-in completion generation
  - Generate completions for bash, zsh, fish, and PowerShell
  - `byakugan completions --shell=zsh > _byakugan`
  - Include completions in the distribution package
  - Document installation instructions for each shell
- [ ] Finalize configuration management:
  - Ensure `~/.byakugan/config.toml` is fully documented with comments
  - Implement `byakugan config init` to generate a default config file
  - Implement `byakugan config show` to display current effective configuration
  - Implement `byakugan config set <key> <value>` for individual settings
  - Support environment variable overrides for all config values
  - Validate configuration on load and report clear errors
- [ ] Write comprehensive integration tests:
  - Mock GitHub API (PR fetch, diff, comments, review posting)
  - Mock GitLab API (MR fetch, diff, discussions, note posting)
  - Mock Bitbucket API (PR fetch, diff, comments, approve/request-changes)
  - End-to-end test: mock PR -> diff analysis -> LLM review (mocked) -> output
  - Test the watch daemon with simulated PR events
  - Test custom rules loading and application
  - Test all output formats (terminal, JSON, markdown)
  - Use `wiremock` or `mockito` for HTTP mocking
- [ ] Stabilize IPC export schemas:
  - Document the JSON output schema for consumption by mugen and tensai
  - Version the schema (include `schema_version` field)
  - Write schema validation tests
  - Create example JSON files for each export type
- [ ] Performance optimization:
  - Profile review latency and optimize bottlenecks
  - Implement caching for repeated platform API calls during a session
  - Optimize diff parsing for large PRs (streaming parser)
  - Measure and optimize memory usage for large diffs
- [ ] Error handling improvements:
  - Ensure all error messages are user-friendly and actionable
  - Include platform-specific troubleshooting hints (e.g., "Token may lack repo scope")
  - Implement `--debug` flag for full error chain output
  - Handle network errors gracefully with retry logic
- [ ] Documentation:
  - Write inline code documentation (rustdoc comments for all public items)
  - Ensure `--help` text is clear and complete for all commands
  - Create example workflows in help text
- [ ] CI/CD readiness:
  - Ensure `cargo test` runs all tests cleanly
  - Ensure `cargo clippy` produces zero warnings
  - Ensure `cargo fmt --check` passes
  - Add `cargo doc --no-deps` verification

## Acceptance Criteria

- Shell completions work correctly for bash, zsh, fish
- Configuration management commands work and produce helpful output
- All integration tests pass with mocked platform APIs
- IPC export schema is documented and versioned
- No clippy warnings or formatting issues
- All public APIs have rustdoc documentation
- Error messages are clear and actionable
- Performance is acceptable for large PRs (review completes within reasonable time)

## Dependencies

- All previous steps (001-012) must be complete
- `wiremock` or `mockito` crate for HTTP mocking in tests
- `clap_complete` for shell completion generation
