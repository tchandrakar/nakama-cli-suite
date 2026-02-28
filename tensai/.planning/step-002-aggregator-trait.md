# Step 002: Aggregator Trait

## Objective

Define the core Aggregator trait that all data source integrations will implement. This trait provides a uniform interface for fetching data, declaring data types, and setting priority. Implement a plugin registration system that discovers and manages available aggregators.

## Tasks

- [ ] Define `AggregatorData` enum: GitHubData, LocalGitData, CalendarData, JiraData, SlackData, CustomData
- [ ] Define `AggregatorMeta` struct: name, data_type, description, requires_auth, refresh_interval
- [ ] Define `Aggregator` trait:
  - [ ] `async fn fetch_data(&self) -> Result<AggregatorData>` — fetch latest data
  - [ ] `fn data_type(&self) -> DataType` — what kind of data this provides
  - [ ] `fn priority(&self) -> u8` — display/processing priority (lower = higher priority)
  - [ ] `fn meta(&self) -> AggregatorMeta` — metadata about the aggregator
  - [ ] `async fn health_check(&self) -> Result<bool>` — verify connectivity
- [ ] Define `AggregatorRegistry`:
  - [ ] Register aggregators by name
  - [ ] Enable/disable aggregators via config
  - [ ] Fetch all enabled aggregators
  - [ ] Parallel data fetching across all aggregators with timeout
  - [ ] Graceful degradation (continue if one aggregator fails)
- [ ] Implement `AggregatorResult` struct: data, source, fetch_duration, errors
- [ ] Implement parallel fetch orchestrator:
  - [ ] Spawn concurrent fetch tasks for all enabled aggregators
  - [ ] Configurable per-aggregator timeout (default 10s)
  - [ ] Collect results, log failures, continue with available data
  - [ ] Return combined results sorted by priority
- [ ] Define data type categories: CodeActivity, ProjectManagement, Communication, Calendar, SystemHealth
- [ ] Add `--aggregators` CLI flag to select/exclude specific aggregators
- [ ] Unit tests for registry (register, enable, disable)
- [ ] Unit tests for parallel fetch with mock aggregators

## Acceptance Criteria

- Aggregator trait is well-defined and ergonomic to implement
- Registry correctly manages aggregator lifecycle
- Parallel fetch completes within max(timeout) not sum(timeouts)
- Failed aggregators do not block others
- Aggregator health check can verify connectivity before full fetch
- Priority ordering is respected in result collection

## Dependencies

- Step 001 (CLI scaffold) must be complete
- tokio for async runtime and concurrent fetch
- No external service dependencies (this is the framework layer)
