# Step 012: Polish

## Objective
Add quality-of-life features including request tracing, shell completions, configuration file system, comprehensive integration tests with mock APIs, and IPC export for inter-tool communication.

## Tasks
- [ ] Implement request tracing (`jogan trace`):
  - Trace a request path across services (Kubernetes services, containers)
  - Identify service-to-service call chain
  - Detect where in the chain a failure occurs
  - Integrate with distributed tracing if available (Jaeger, Zipkin headers)
  - Display trace as a visual service graph
- [ ] Generate shell completions:
  - `jogan completions zsh` — output zsh completions
  - `jogan completions bash` — output bash completions
  - `jogan completions fish` — output fish completions
  - Use `clap_complete` for generation
- [ ] Implement configuration file system:
  - Location: `~/.config/jogan/config.toml` (XDG-aware)
  - Options: default collectors, poll interval, alert thresholds, output format, namespaces
  - Per-environment profiles (staging, production, etc.)
  - `jogan config show` / `config edit` / `config reset`
- [ ] Write comprehensive integration tests:
  - Full scan workflow: discover -> collect -> analyze -> report
  - Diagnosis workflow: symptom -> plan -> collect -> diagnose -> report
  - Watch mode: start -> poll -> detect anomaly -> alert
  - Each collector with mock API responses
  - Error handling: unreachable collector, timeout, auth failure
- [ ] Mock API infrastructure:
  - Mock Kubernetes API server for testing
  - Mock Docker API for testing
  - Mock AWS API responses for testing
  - Configurable mock responses for various failure scenarios
- [ ] Add performance benchmarks:
  - Collection time per collector
  - Rule evaluation time with varying data sizes
  - TUI render time
  - Report generation time
- [ ] Implement IPC export:
  - Define JSON schema for jogan data (findings, reports, metrics)
  - `--ipc` flag to output structured JSON for other tools
  - Document IPC schema for consuming tools
- [ ] Implement `health` subcommand:
  - Quick health check across all configured collectors
  - Display collector status table (name, status, latency)
  - Exit with non-zero code if any collector is unhealthy (CI-friendly)
- [ ] Final code cleanup: rustfmt, clippy, rustdoc on all public APIs
- [ ] Add `--version` with build metadata
- [ ] Ensure all error messages are user-friendly with actionable suggestions

## Acceptance Criteria
- Request tracing identifies failure points in service chains
- Shell completions install and work for zsh, bash, and fish
- Config file system supports environment profiles
- Integration tests pass with mock API infrastructure
- IPC export produces well-documented JSON schema
- Health subcommand provides quick infrastructure status
- `cargo clippy` reports zero warnings
- All public APIs have rustdoc comments

## Dependencies
- Step 001 through Step 011 (all prior steps must be complete)
