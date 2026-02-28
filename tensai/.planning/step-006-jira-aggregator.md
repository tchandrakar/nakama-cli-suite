# Step 006: Jira Aggregator

## Objective

Implement a Jira aggregator that fetches sprint status, assigned tickets, and project context either via IPC from the itachi tool or by directly calling the Jira REST API. This provides project management context for briefings and standup generation.

## Tasks

- [ ] Implement `JiraAggregator` struct implementing `Aggregator` trait
- [ ] Implement dual data source strategy:
  - [ ] Primary: IPC from itachi tool (if available and running)
  - [ ] Fallback: Direct Jira REST API calls
  - [ ] Auto-detect which source is available
- [ ] Implement Jira REST API client:
  - [ ] Authentication: API token + email from nakama-vault
  - [ ] Base URL from config (support Jira Cloud and Server)
  - [ ] Pagination handling for large result sets
- [ ] Fetch assigned tickets:
  - [ ] JQL: `assignee = currentUser() AND resolution = Unresolved`
  - [ ] Key, summary, status, priority, type (bug/story/task)
  - [ ] Sprint assignment
  - [ ] Due date and SLA tracking
  - [ ] Story points / estimate
  - [ ] Labels and components
  - [ ] Subtask progress
- [ ] Fetch sprint status:
  - [ ] Active sprint name, goal, start/end dates
  - [ ] Sprint progress (completed vs remaining points)
  - [ ] Sprint burndown position (ahead/behind)
  - [ ] Remaining days in sprint
- [ ] Fetch recent transitions:
  - [ ] Tickets moved today/yesterday (for standup)
  - [ ] Status changes by current user
  - [ ] Blocked tickets and blocker reasons
- [ ] Implement IPC consumer:
  - [ ] Connect to itachi IPC channel
  - [ ] Subscribe to relevant data updates
  - [ ] Cache IPC data with TTL
  - [ ] Graceful fallback when itachi is not running
- [ ] Implement ticket prioritization:
  - [ ] Sort by: priority, due date, sprint goal alignment
  - [ ] Flag overdue tickets
  - [ ] Flag tickets approaching SLA breach
- [ ] Add configuration:
  - [ ] Jira instance URL
  - [ ] Project keys to track
  - [ ] Board/sprint IDs
  - [ ] Custom JQL for additional queries
  - [ ] IPC preference (prefer itachi vs direct)
- [ ] Unit tests with mocked Jira API responses
- [ ] Unit tests with mocked IPC messages

## Acceptance Criteria

- Assigned tickets are fetched with full context (priority, sprint, due date)
- Sprint status shows progress and remaining work
- Recent transitions provide standup-ready data
- IPC from itachi works when available
- Direct API fallback works when itachi is not running
- Overdue and SLA-breach tickets are flagged
- Pagination handles large ticket sets correctly

## Dependencies

- Step 001 (CLI scaffold)
- Step 002 (Aggregator trait) must be complete
- Step 008 (Security) for Jira API token via nakama-vault
- itachi tool IPC protocol (optional, for IPC mode)
- reqwest for direct Jira API calls
