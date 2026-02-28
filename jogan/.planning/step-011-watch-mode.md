# Step 011: Watch Mode

## Objective
Implement a continuous monitoring TUI dashboard using `ratatui` that polls infrastructure collectors at configurable intervals, detects anomalies by comparing against baselines, and alerts when thresholds are exceeded.

## Tasks
- [ ] Create `watch.rs` module with `WatchMode` struct
- [ ] Implement ratatui TUI dashboard layout:
  - Header: target infrastructure, uptime, last check time
  - Resource list pane: scrollable list of monitored resources with status indicators
  - Detail pane: selected resource details (metrics, events, status)
  - Findings pane: current active findings with severity colors
  - Log pane: recent events and status changes
  - Status bar: key bindings, collector health, next poll time
- [ ] Implement polling loop:
  - Configurable poll interval (default 30s, `--interval` flag)
  - Sequential or parallel collector execution
  - Incremental data collection (only fetch changed data where possible)
  - Handle collector timeouts without blocking the dashboard
  - Display spinner/progress during data collection
- [ ] Implement anomaly detection:
  - Establish baseline on first collection cycle (or from historical data)
  - Detect deviations from baseline:
    - CPU usage >2 standard deviations above baseline
    - Memory usage growth exceeding expected rate
    - Error rate increase beyond threshold
    - New resources appearing or disappearing
    - Status changes (Healthy -> Degraded)
  - Configurable sensitivity (low/medium/high)
- [ ] Implement alert thresholds:
  - Configurable per-metric thresholds
  - Default thresholds: CPU >80%, Memory >85%, Disk >90%, Error rate >5%
  - Alert levels: info, warning, critical
  - Visual alert: flash resource in red, show alert banner
  - Optional: terminal bell on critical alerts
- [ ] Implement TUI interactions:
  - `j/k` or arrow keys: navigate resource list
  - `Enter`: expand resource details
  - `f`: filter resources by type/status
  - `s`: sort by name/status/severity
  - `/`: search resources
  - `r`: force refresh
  - `p`: pause/resume polling
  - `q`: quit
  - `?`: show help overlay
- [ ] Implement watch configuration:
  - Specify which resources to watch (namespaces, labels, names)
  - Specify which collectors to use
  - Specify custom thresholds
  - Save watch profiles for repeated use
- [ ] Add `--headless` mode: run without TUI, output findings to stdout/file
- [ ] Add `--alert-command` flag: run custom command on alert (e.g., send notification)
- [ ] Handle terminal resize gracefully
- [ ] Write unit tests for anomaly detection logic
- [ ] Write tests for baseline calculation and deviation detection

## Acceptance Criteria
- TUI dashboard displays real-time infrastructure status
- Polling collects data at configured intervals without freezing the UI
- Anomaly detection identifies deviations from established baselines
- Alert thresholds trigger visual alerts on the dashboard
- TUI interactions (scroll, filter, search) work smoothly
- Headless mode produces machine-readable output
- Tests cover anomaly detection and baseline logic

## Dependencies
- Step 001 (CLI scaffold)
- Step 002-005 (collectors provide data to display)
- Step 007 (rule engine evaluates each poll cycle)
