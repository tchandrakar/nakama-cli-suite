# Step 015: Polish

## Objective

Final polish pass for itachi: generate shell completions, finalize configuration management, write comprehensive integration tests with mock Atlassian APIs, stabilize IPC export schemas for tensai/byakugan/shinigami/senku consumption, and prepare for release.

## Tasks

- [ ] Generate shell completions:
  - Use clap's built-in completion generation
  - Generate completions for bash, zsh, fish, PowerShell
  - `itachi completions --shell=zsh > _itachi`
  - Include dynamic completions for project keys and board names
  - Document installation instructions for each shell
- [ ] Finalize configuration management:
  - Ensure `~/.itachi/config.toml` is fully documented with comments
  - Implement `itachi config init` to generate a default config file
  - Implement `itachi config show` to display current effective configuration
  - Implement `itachi config set <key> <value>` for individual settings
  - Support environment variable overrides for all config values
  - Validate configuration on load with clear error messages
- [ ] Write comprehensive integration tests:
  - Mock Jira REST API (issue CRUD, JQL search, sprint/board data, changelog)
  - Mock Confluence REST API (CQL search, page content, space navigation)
  - Mock OAuth2 flow for authentication testing
  - Test NL to JQL/CQL translation with mock LLM responses
  - Test cross-reference engine with mock data
  - Test intelligence layer (embedding, semantic search, synthesis)
  - Test standup generation with mock activity data
  - Test team briefing with mock sprint data
  - Test onboarding brief with mock project data
  - Test sprint analytics calculations
  - Test ticket creation and linking
  - Test all output formats (terminal, JSON, markdown)
  - Use `wiremock` or `mockito` for HTTP mocking
- [ ] Stabilize IPC export schemas:
  - Document JSON output schema for tensai (Jira sprint data, Confluence updates)
  - Document JSON output schema for byakugan (Jira issue context for PR review)
  - Document JSON output schema for shinigami (Jira issue keys for commit linking)
  - Document JSON output schema for senku (organizational knowledge context)
  - Version all schemas (include `schema_version` field)
  - Write schema validation tests
  - Create example JSON files for each export type
- [ ] Performance optimization:
  - Profile API call latency and optimize batching
  - Optimize embedding index queries
  - Implement connection pooling for reqwest clients
  - Cache frequently accessed data (project metadata, user lists)
- [ ] Error handling improvements:
  - Ensure all error messages are user-friendly and actionable
  - Include Atlassian-specific troubleshooting hints
  - Handle network errors gracefully with retry logic
  - Handle Atlassian Cloud vs. Data Center API differences
- [ ] Documentation:
  - Write inline code documentation (rustdoc)
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
- All integration tests pass with mocked Atlassian APIs
- IPC export schemas are documented and versioned for all consuming tools
- No clippy warnings or formatting issues
- All public APIs have rustdoc documentation
- Error messages are clear and actionable
- Performance is acceptable for all command types

## Dependencies

- All previous steps (001-014) must be complete
- `wiremock` or `mockito` crate for HTTP mocking
- `clap_complete` for shell completion generation
