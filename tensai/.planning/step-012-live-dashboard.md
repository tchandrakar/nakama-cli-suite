# Step 012: Live Dashboard (TUI)

## Objective

Build a ratatui-based terminal user interface that provides a live, auto-refreshing dashboard showing PR status, CI/CD status, issue status, sprint progress, and upcoming meetings. This serves as a persistent "mission control" for the developer's workflow.

## Tasks

- [ ] Add ratatui and crossterm dependencies
- [ ] Design TUI layout:
  - [ ] Top bar: current time, focus mode status, next meeting countdown
  - [ ] Left panel: PR status (authored + review-requested)
  - [ ] Center panel: active tasks/issues with priority
  - [ ] Right panel: CI/CD status and recent builds
  - [ ] Bottom panel: today's schedule timeline
  - [ ] Status bar: aggregator status, last refresh time, keybindings
- [ ] Implement PR status panel:
  - [ ] List of authored PRs with review/CI status icons
  - [ ] List of PRs awaiting my review with age
  - [ ] Color coding: green=approved, red=failing, yellow=pending
  - [ ] Compact view with expandable details
- [ ] Implement CI/CD status panel:
  - [ ] Recent workflow runs with pass/fail indicators
  - [ ] Currently running builds with progress
  - [ ] Failed build summary (test name, error snippet)
- [ ] Implement issues/tasks panel:
  - [ ] Active Jira tickets sorted by priority
  - [ ] Sprint progress bar
  - [ ] Ticket status indicators
  - [ ] Quick status transitions (if supported)
- [ ] Implement schedule timeline:
  - [ ] Horizontal timeline of today's events
  - [ ] Current time marker
  - [ ] Focus blocks highlighted
  - [ ] Meeting details on hover/select
- [ ] Implement auto-refresh:
  - [ ] Configurable refresh interval (default 60s)
  - [ ] Per-aggregator refresh rates
  - [ ] Visual indicator during refresh (spinner)
  - [ ] Manual refresh with `r` key
- [ ] Implement interactive controls:
  - [ ] Tab key to switch between panels
  - [ ] Arrow keys to navigate within panels
  - [ ] Enter to expand/view details
  - [ ] `o` to open item in browser (PR, issue, etc.)
  - [ ] `f` to start focus mode
  - [ ] `b` to generate quick briefing
  - [ ] `s` to generate standup
  - [ ] `q` to quit
- [ ] Implement responsive layout:
  - [ ] Adapt to terminal size
  - [ ] Collapse panels on narrow terminals
  - [ ] Handle resize events smoothly
- [ ] Implement notification overlay:
  - [ ] Show toast-style notifications for important changes
  - [ ] CI failure, new review request, mention
  - [ ] Auto-dismiss after configurable duration
- [ ] Unit tests for panel rendering with mock data
- [ ] Manual test checklist for TUI interactions

## Acceptance Criteria

- Dashboard displays all aggregated data in a clear layout
- Auto-refresh updates data without flickering or losing scroll position
- PR/CI/issue status is visually clear with color coding and icons
- Schedule timeline accurately shows today's events and focus blocks
- Interactive controls allow navigation and action on items
- Browser open works for PRs and issues
- Layout adapts to different terminal sizes
- Notification overlay appears for important changes

## Dependencies

- Step 002 (Aggregator trait) for data sources
- Steps 003-006 (Aggregators) for live data
- Step 011 (Focus mode) for focus mode integration
- ratatui and crossterm crates
