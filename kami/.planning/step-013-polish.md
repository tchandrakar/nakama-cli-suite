# Step 013: Polish

## Objective

Final polish pass for kami: implement pipe mode for Unix pipeline integration, generate shell completions, finalize configuration management, write comprehensive integration tests, and stabilize IPC export schemas for senku and tensai consumption.

## Tasks

- [ ] Implement pipe mode (`kami pipe`):
  - Read content from stdin
  - Accept a prompt argument: `echo "error text" | kami pipe "explain this error"`
  - Enrich stdin content with grounded search context
  - Output enriched result to stdout
  - Support `--format=json` for structured pipe output
  - Auto-detect stdin availability (if no stdin, show help)
  - Design for Unix pipeline chaining: `jogan diagnose "crash" | kami pipe "search for known issues"`
- [ ] Generate shell completions:
  - Use clap's built-in completion generation
  - Generate completions for bash, zsh, fish, PowerShell
  - `kami completions --shell=zsh > _kami`
  - Include completions in the distribution package
  - Document installation instructions for each shell
- [ ] Finalize configuration management:
  - Ensure `~/.kami/config.toml` is fully documented with comments
  - Implement `kami config init` to generate a default config file
  - Implement `kami config show` to display current effective configuration
  - Support environment variable overrides for all config values
  - Validate configuration on load with clear error messages
- [ ] Write comprehensive integration tests:
  - Mock Gemini API (generateContent, streamGenerateContent, grounded search)
  - Mock Google Custom Search API
  - Mock URL fetching (various content types)
  - End-to-end test: search query -> grounded search -> synthesis -> output
  - Test conversational mode with simulated multi-turn input
  - Test deep research with mocked multi-round searches
  - Test fact-checking with mocked evidence
  - Test comparison mode
  - Test cache layer (hit/miss/expiry/eviction)
  - Use `wiremock` or `mockito` for HTTP mocking
- [ ] Stabilize IPC export schemas:
  - Document JSON output schema for consumption by senku (knowledge enrichment)
  - Document JSON output schema for consumption by tensai (daily brief)
  - Version the schema (include `schema_version` field)
  - Write schema validation tests
  - Create example JSON files for each export type
- [ ] Performance optimization:
  - Profile search latency and optimize bottlenecks
  - Optimize streaming response display
  - Measure and optimize memory usage for deep research
- [ ] Error handling improvements:
  - Ensure all error messages are user-friendly and actionable
  - Include API-specific troubleshooting hints
  - Handle network errors gracefully with retry logic
- [ ] Documentation:
  - Write inline code documentation (rustdoc)
  - Ensure `--help` text is clear and complete for all commands
- [ ] CI/CD readiness:
  - Ensure `cargo test` runs all tests cleanly
  - Ensure `cargo clippy` produces zero warnings
  - Ensure `cargo fmt --check` passes

## Acceptance Criteria

- Pipe mode reads from stdin, enriches with search, and outputs to stdout
- Shell completions work correctly for bash, zsh, fish
- Configuration management commands work and produce helpful output
- All integration tests pass with mocked APIs
- IPC export schema is documented and versioned
- No clippy warnings or formatting issues
- Error messages are clear and actionable
- Performance is acceptable for all command types

## Dependencies

- All previous steps (001-012) must be complete
- `wiremock` or `mockito` crate for HTTP mocking
- `clap_complete` for shell completion generation
