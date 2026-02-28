# Step 009: Standup Generation

## Objective

Build an automated standup report generator that pulls git activity and Jira transitions (via itachi IPC) to format Yesterday/Today/Blockers reports. Support export to markdown, Slack-formatted text, and plain text.

## Tasks

- [ ] Implement `StandupEngine` struct:
  - [ ] Accept data from LocalGitAggregator and JiraAggregator
  - [ ] Configurable time range (default: since last standup or yesterday)
  - [ ] Track last standup timestamp for accurate "since last standup"
- [ ] Implement "Yesterday" section generation:
  - [ ] Collect git commits since last standup
  - [ ] Collect Jira ticket transitions (Done, In Review)
  - [ ] Collect PR activity (opened, merged, reviewed)
  - [ ] Group by project/repo
  - [ ] Summarize into bullet points
  - [ ] LLM option: synthesize raw activity into coherent summary
- [ ] Implement "Today" section generation:
  - [ ] In-progress Jira tickets
  - [ ] Open PRs needing attention
  - [ ] Calendar context (key meetings affecting availability)
  - [ ] LLM option: suggest today's plan based on priorities
- [ ] Implement "Blockers" section generation:
  - [ ] Blocked Jira tickets (with blocker details)
  - [ ] PRs waiting on review for >24h
  - [ ] Failing CI blocking merges
  - [ ] LLM option: suggest unblocking strategies
- [ ] Implement IPC consumer for Jira transitions:
  - [ ] Connect to itachi IPC
  - [ ] Query ticket transitions in time range
  - [ ] Fallback to direct Jira API
- [ ] Implement output formats:
  - [ ] Plain text (terminal-friendly)
  - [ ] Markdown (for documentation/wiki)
  - [ ] Slack format (with mrkdwn syntax, @mentions, emoji)
  - [ ] JSON (for programmatic consumption)
- [ ] Implement `standup` subcommand:
  - [ ] Default: generate and print to terminal
  - [ ] `--copy` flag: copy to clipboard
  - [ ] `--post` flag: post to Slack channel (if configured)
  - [ ] `--format` flag: text, markdown, slack, json
  - [ ] `--since` flag: override time range
  - [ ] `--edit` flag: open in editor for manual tweaks before posting
- [ ] Implement standup history:
  - [ ] Save generated standups (local SQLite/JSON)
  - [ ] `tensai standup --history` to view past standups
  - [ ] Avoid duplicate content across standups
- [ ] Unit tests with mock git/Jira data
- [ ] Integration test: generate standup from sample activity

## Acceptance Criteria

- Yesterday section accurately reflects git and Jira activity
- Today section reflects current priorities and schedule
- Blockers section identifies blocked work with actionable context
- Slack-formatted output renders correctly in Slack
- Markdown output is valid and well-structured
- Clipboard copy works on macOS and Linux
- Standup history prevents duplicate content
- LLM synthesis produces natural, team-friendly language

## Dependencies

- Step 004 (Local git aggregator) for commit history
- Step 006 (Jira aggregator) for ticket transitions
- Step 007 (Briefing engine) for synthesis patterns
- Step 008 (Security) for Slack token via nakama-vault
- itachi IPC for Jira transition data
