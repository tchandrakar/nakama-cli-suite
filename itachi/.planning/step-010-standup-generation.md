# Step 010: Standup Generation

## Objective

Implement the `itachi standup` command that auto-generates standup reports from Jira transition history (last 24 hours), optionally enriches with PR activity from shinigami/byakugan via IPC, formats output as Yesterday/Today/Blockers, and supports multiple export formats.

## Tasks

- [ ] Implement Jira activity retrieval:
  - Query current user's Jira activity from the last 24 hours (configurable lookback)
  - Fetch issue transition history: `GET /rest/api/3/issue/{key}/changelog`
  - Filter transitions by the current user and time window
  - Categorize transitions:
    - Completed: transitions to "Done", "Resolved", "Closed"
    - In Progress: transitions to "In Progress", "In Review"
    - Started: newly assigned or moved to sprint
  - Fetch issues the user commented on (optional, configurable)
  - Sort activities chronologically
- [ ] Implement "Yesterday" section generation:
  - List issues with completed transitions in the lookback period
  - Format: `- PROJ-142: Moved "Add retry logic" -> In Review`
  - Include issue summary and transition details
  - Group by project if multiple projects
- [ ] Implement "Today" section generation:
  - List issues currently assigned to the user with "In Progress" or "To Do" status
  - Prioritize by sprint commitment and priority
  - Format: `- PROJ-150: "Payment reconciliation batch job" (In Progress)`
  - Include carry-over items from yesterday
- [ ] Implement "Blockers" section generation:
  - Find issues flagged as impediment/blocked assigned to or affecting the user
  - Include blocking duration ("blocked since Feb 26")
  - Include blocker assignee/owner information
  - Query for issues with "Blocker" priority affecting the user's project
- [ ] Implement PR activity enrichment (IPC):
  - Accept PR activity data from shinigami/byakugan via stdin or IPC
  - Format: PRs reviewed, PRs submitted, PRs merged
  - Include in "Yesterday" section: `- Reviewed PAY-148: "Idempotency key implementation"`
  - Gracefully skip if IPC data is unavailable
- [ ] Implement output formatting:
  - Terminal: rich formatted standup with sections, project colors
  - Markdown: clean markdown suitable for pasting in Slack/Teams
  - Slack format: Slack-specific markdown with emoji
  - Plain text: minimal format for email
  - JSON: structured data for machine consumption
- [ ] Implement configuration:
  - `lookback_hours` (default: 24)
  - `include_reviews` (default: true)
  - `include_comments` (default: false)
  - `format` (default: markdown)
  - `projects` filter (optional: limit to specific projects)
- [ ] Implement `--date` flag for generating standups for past dates
- [ ] Implement `--team` flag for generating team standup (all team members)
- [ ] Write unit tests for activity categorization and standup formatting
- [ ] Write integration tests with mocked Jira changelog data

## Acceptance Criteria

- Standup accurately reflects the user's Jira activity from the last 24 hours
- Yesterday/Today/Blockers sections are correctly populated
- PR activity is included when IPC data is available
- All output formats produce clean, well-structured output
- `--date` flag generates historical standups
- `--team` flag generates standups for all team members
- Standup generation completes in under 10 seconds

## Dependencies

- Step 003 (Jira Client) must be complete for issue and changelog access
- IPC from shinigami/byakugan is optional (graceful degradation)
- `chrono` crate for date/time calculations
