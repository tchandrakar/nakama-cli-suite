# Step 011: Team Briefing

## Objective

Implement the `itachi brief <team>` command that generates a comprehensive team briefing including sprint health (completion percentage, burndown trajectory), active blockers and impediments, recent Confluence documentation updates, and suggested action items.

## Tasks

- [ ] Implement sprint health assessment:
  - Fetch current sprint for the team's board
  - Calculate completion percentage: completed story points / total committed story points
  - Calculate burndown trajectory: is the team on track to complete the sprint?
  - Detect scope creep: stories added after sprint start
  - Compare velocity against team average (from last N sprints)
  - Generate health status: Healthy, At Risk, Behind
- [ ] Implement blocker/impediment identification:
  - Query for issues flagged as impediment in the current sprint
  - Query for issues with "Blocker" priority
  - Query for issues that haven't moved in the status for > N days (stale)
  - Include blocker owner, duration, and any linked dependencies
  - Prioritize by impact (blocking other work vs. isolated)
- [ ] Implement recent Confluence updates:
  - Query for recently modified pages in the team's Confluence space(s)
  - Filter to relevant updates (not minor edits): check version increment
  - Categorize updates: new pages, significant edits, new ADRs/RFCs
  - Include page title, author, and summary of changes
  - Timeframe: last 7 days (configurable)
- [ ] Implement action item generation:
  - Use nakama-ai to analyze sprint health, blockers, and doc updates
  - Generate actionable suggestions:
    - "PROJ-151 has been blocked for 3 days - consider escalating"
    - "Sprint is at 40% completion with 3 days remaining - may need scope reduction"
    - "New ADR posted: Auth Service Migration - review recommended"
  - Prioritize action items by urgency
- [ ] Implement team identification:
  - Map team name to Jira board(s), project(s), and Confluence space(s)
  - Support configuration in `~/.itachi/config.toml`:
    ```toml
    [teams.payments]
    projects = ["PAY"]
    boards = ["PAY Board"]
    spaces = ["PAY"]
    members = ["alice", "bob"]
    ```
  - Auto-detect team from current project if possible
- [ ] Implement output formatting:
  - Terminal: rich formatted briefing with sections, color-coded health indicators
  - Markdown: shareable document suitable for Slack/email
  - JSON: structured data for IPC consumption
- [ ] Implement `--period` flag for different time ranges (daily, weekly)
- [ ] Write unit tests for health assessment and action item generation
- [ ] Write integration tests with mocked sprint and page data

## Acceptance Criteria

- Sprint health is accurately calculated with completion percentage and trajectory
- Blockers and impediments are identified with duration and impact
- Recent Confluence updates are listed with relevant categorization
- Action items are generated and prioritized by urgency
- Team mapping works from config and auto-detection
- Briefing provides actionable, relevant information for team leads
- All output formats are clean and well-structured

## Dependencies

- Step 003 (Jira Client) must be complete for sprint and issue access
- Step 004 (Confluence Client) must be complete for page updates
- Step 007 (Cross-Reference Engine) for linking docs to tickets
- `nakama-ai` for action item generation
