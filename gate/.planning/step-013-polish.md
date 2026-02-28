# Step 013: Polish and Integration

## Objective

Finalize gate with API documentation generation, shell completions, configuration file support, comprehensive integration tests, and IPC export capabilities for consumption by other tools in the nakama-cli-suite.

## Tasks

- [ ] Implement `doc` subcommand:
  - [ ] Generate API documentation from collections
  - [ ] Include: endpoints, methods, parameters, example requests/responses
  - [ ] Output formats: markdown, HTML, JSON (OpenAPI)
  - [ ] Auto-generate from request history (discover endpoints, infer params)
  - [ ] LLM option: generate human-readable endpoint descriptions
  - [ ] `gate doc generate --collection <name>` — from collection
  - [ ] `gate doc generate --history` — from request history
  - [ ] `gate doc serve` — serve generated docs on local HTTP server
- [ ] Implement shell completions:
  - [ ] Generate completions for bash, zsh, fish, PowerShell via clap_complete
  - [ ] Dynamic completion for URLs (from history), methods, environments
  - [ ] Dynamic completion for collection names and request names
  - [ ] Add `completions` subcommand to generate completion scripts
- [ ] Implement configuration file:
  - [ ] TOML config file at `~/.config/nakama/gate.toml`
  - [ ] Override with `--config` flag
  - [ ] Sections: http (timeout, retries, TLS), environments, collections, history, security, mock
  - [ ] Default config generation (`gate config init`)
  - [ ] Config validation on load with helpful error messages
- [ ] Implement IPC export:
  - [ ] Export API call results for tensai briefings (API health status)
  - [ ] Export mock server status for other tools
  - [ ] Define gate IPC message types: ApiCallResult, MockServerStatus, CollectionRunReport
  - [ ] Graceful handling when IPC consumers are unavailable
- [ ] Write integration tests:
  - [ ] Send requests to mock server (all HTTP methods)
  - [ ] Test NL request building with mocked LLM
  - [ ] Test import from cURL, OpenAPI, Postman, HAR
  - [ ] Test request history CRUD operations
  - [ ] Test environment variable substitution
  - [ ] Test collection execution
  - [ ] Test flow execution (multi-step with variables)
  - [ ] Test mock server (OpenAPI, record, replay)
  - [ ] Test response diff
  - [ ] End-to-end: build request -> send -> analyze -> save -> replay -> diff
- [ ] Performance testing:
  - [ ] Single request latency overhead (target: <5ms added)
  - [ ] History search performance (target: <100ms for 10k entries)
  - [ ] Mock server throughput (target: >1000 req/s)
- [ ] Documentation:
  - [ ] Man page generation
  - [ ] Usage examples for each subcommand
  - [ ] Configuration reference
  - [ ] Flow definition syntax reference
  - [ ] Mock definition syntax reference
- [ ] Final cleanup:
  - [ ] Remove all `todo!()` stubs
  - [ ] Error message review (user-friendly, actionable)
  - [ ] Clippy clean
  - [ ] cargo fmt

## Acceptance Criteria

- `doc` command generates useful API documentation from collections/history
- Shell completions work for bash, zsh, and fish
- Config file is loaded and validated, with sensible defaults
- IPC export sends structured messages consumable by suite tools
- All integration tests pass
- No `todo!()` stubs remain in codebase
- `cargo clippy` and `cargo fmt` pass with no warnings
- Performance targets are met for requests, history, and mock server
- Documentation covers all subcommands and configuration options

## Dependencies

- All previous steps (001-012) must be complete
- nakama-ai for documentation generation
- Shared IPC protocol definitions
- clap_complete crate for shell completions
