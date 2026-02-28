# Step 011: Polish and Integration

## Objective

Finalize sharingan with summary generation, shell completions, configuration file support, comprehensive integration tests using mock log streams, and IPC export capabilities for integration with jogan and tensai tools in the nakama-cli-suite.

## Tasks

- [ ] Implement `summary` subcommand:
  - [ ] Collect statistics: total entries, entries per level, top error messages, anomaly count
  - [ ] Time range coverage
  - [ ] Source breakdown
  - [ ] LLM-generated narrative summary (optional, nakama-ai)
  - [ ] Output formats: text, json, markdown
- [ ] Implement shell completions:
  - [ ] Generate completions for bash, zsh, fish, PowerShell via clap_complete
  - [ ] Dynamic completion for `--source` (list available files, k8s pods, docker containers)
  - [ ] Add `completions` subcommand to generate completion scripts
- [ ] Implement configuration file:
  - [ ] TOML config file at `~/.config/nakama/sharingan.toml`
  - [ ] Override with `--config` flag
  - [ ] Sections: sources, analysis, ui, security, predictions
  - [ ] Default config generation (`sharingan config init`)
  - [ ] Config validation on load
- [ ] Implement IPC export for suite integration:
  - [ ] Export analysis results via shared IPC mechanism (Unix socket / named pipe)
  - [ ] Define sharingan IPC message types: LogAnalysis, Anomaly, Prediction, Summary
  - [ ] jogan integration: feed log summaries to daily journal
  - [ ] tensai integration: provide system health data for dev briefings
  - [ ] Export format: JSON messages over IPC channel
- [ ] Write integration tests:
  - [ ] Mock file log stream (write to temp file, verify analysis)
  - [ ] Mock stdin pipe (echo logs | sharingan scan)
  - [ ] Mock LLM responses for deep analysis tests
  - [ ] Mock k8s/docker APIs for remote ingestor tests
  - [ ] End-to-end: ingest -> parse -> analyze -> output
  - [ ] Regression tests for known log formats
- [ ] Performance benchmarks:
  - [ ] Throughput benchmark: lines/second for scan
  - [ ] Latency benchmark: time-to-first-output for tail
  - [ ] Memory benchmark: peak memory for large files
- [ ] Documentation:
  - [ ] Man page generation
  - [ ] Usage examples for each subcommand
  - [ ] Configuration reference
- [ ] Final cleanup:
  - [ ] Remove all `todo!()` stubs
  - [ ] Error message review (user-friendly)
  - [ ] Clippy clean
  - [ ] cargo fmt

## Acceptance Criteria

- `summary` subcommand produces accurate statistical and narrative summaries
- Shell completions work for bash, zsh, and fish
- Config file is loaded and validated, with sensible defaults
- IPC export sends structured messages consumable by jogan and tensai
- All integration tests pass
- No `todo!()` stubs remain in codebase
- `cargo clippy` and `cargo fmt` pass with no warnings
- Performance benchmarks meet targets from previous steps

## Dependencies

- All previous steps (001-010) must be complete
- nakama-ai for summary generation
- Shared IPC protocol definition (may need coordination with jogan/tensai teams)
- clap_complete crate for shell completions
