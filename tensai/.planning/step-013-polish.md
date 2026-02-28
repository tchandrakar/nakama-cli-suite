# Step 013: Polish and Integration

## Objective

Finalize tensai with the review command, shell completions, configuration file support, comprehensive integration tests, and IPC consumption from other nakama-cli-suite tools.

## Tasks

- [ ] Implement `review` subcommand:
  - [ ] Summarize all open items across services
  - [ ] Highlight items needing immediate attention
  - [ ] Show items that have been stale (no activity in N days)
  - [ ] Suggest actions for each item
  - [ ] LLM option: prioritized review with reasoning
- [ ] Implement shell completions:
  - [ ] Generate completions for bash, zsh, fish, PowerShell via clap_complete
  - [ ] Dynamic completion for `--format`, `--section`
  - [ ] Add `completions` subcommand to generate completion scripts
- [ ] Implement configuration file:
  - [ ] TOML config file at `~/.config/nakama/tensai.toml`
  - [ ] Override with `--config` flag
  - [ ] Sections: aggregators, briefing, standup, focus, dashboard, security
  - [ ] Default config generation (`tensai config init`)
  - [ ] Config validation on load with helpful error messages
- [ ] Implement IPC consumption:
  - [ ] Consume sharingan log summaries for system health in briefings
  - [ ] Consume itachi Jira data for sprint context
  - [ ] Consume byakugan code analysis insights
  - [ ] Define consumed IPC message types
  - [ ] Graceful handling when IPC sources are unavailable
- [ ] Write integration tests:
  - [ ] Mock all aggregator responses
  - [ ] Test full briefing generation pipeline
  - [ ] Test standup generation with sample data
  - [ ] Test prioritization scoring with known inputs
  - [ ] Test focus mode timer logic
  - [ ] Test dashboard rendering with mock data
  - [ ] End-to-end: aggregation -> synthesis -> output
- [ ] Performance testing:
  - [ ] Measure aggregation latency (target: <5s for all sources)
  - [ ] Measure LLM synthesis latency (target: <10s)
  - [ ] Measure dashboard refresh performance
- [ ] Documentation:
  - [ ] Man page generation
  - [ ] Usage examples for each subcommand
  - [ ] Configuration reference
  - [ ] Aggregator setup guides
- [ ] Final cleanup:
  - [ ] Remove all `todo!()` stubs
  - [ ] Error message review (user-friendly, actionable)
  - [ ] Clippy clean
  - [ ] cargo fmt

## Acceptance Criteria

- `review` command produces comprehensive overview of open items
- Shell completions work for bash, zsh, and fish
- Config file is loaded and validated, with sensible defaults
- IPC consumption works when source tools are running
- All integration tests pass
- No `todo!()` stubs remain in codebase
- `cargo clippy` and `cargo fmt` pass with no warnings
- Performance targets are met for aggregation and synthesis

## Dependencies

- All previous steps (001-012) must be complete
- nakama-ai for review synthesis
- Shared IPC protocol definitions from sharingan, itachi, byakugan
- clap_complete crate for shell completions
