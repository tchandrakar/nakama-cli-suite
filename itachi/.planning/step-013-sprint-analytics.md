# Step 013: Sprint Analytics

## Objective

Implement the `itachi sprint` command that provides comprehensive sprint analytics including progress/burndown visualization, velocity trend across sprints, scope creep detection, cycle time distribution, and workload balance across team members.

## Tasks

- [ ] Implement sprint progress tracking:
  - Fetch current sprint issues and their statuses
  - Calculate story point completion: done / total committed
  - Calculate issue count completion: done / total
  - Group by status category: To Do, In Progress, Done
  - Display as a progress bar and percentage
- [ ] Implement burndown chart data:
  - Fetch sprint start date and end date
  - Calculate ideal burndown line (linear from total to zero)
  - Calculate actual burndown from daily issue transitions
  - Identify days where actual > ideal (behind schedule)
  - Terminal chart using nakama-ui (ASCII chart or Unicode block characters)
  - Configurable resolution: daily (default), hourly
- [ ] Implement velocity trend:
  - Fetch completed story points for the last N sprints (configurable, default: 5)
  - Calculate average velocity
  - Calculate velocity trend (increasing, stable, decreasing)
  - Display as a bar chart or sparkline in terminal
  - Predict sprint capacity based on average velocity
- [ ] Implement scope creep detection:
  - Compare sprint scope at start vs. current scope
  - Identify issues added after sprint start (with timestamps)
  - Identify issues removed from sprint (with reasons if available)
  - Calculate scope change percentage
  - Flag sprints with > 20% scope change as "high scope creep"
- [ ] Implement cycle time distribution:
  - For completed issues, calculate time from "In Progress" to "Done"
  - Generate distribution: min, max, average, median, P90
  - Break down by issue type (bug fix vs. feature vs. task)
  - Compare against team historical averages
  - Identify outliers (issues that took much longer than average)
- [ ] Implement workload balance:
  - Fetch story points or issue counts assigned per team member
  - Calculate distribution and identify imbalances
  - Display as a horizontal bar chart per team member
  - Flag overloaded team members (> 1.5x average)
  - Flag underloaded team members (< 0.5x average)
- [ ] Implement the sprint dashboard:
  - Combine all analytics into a single dashboard view
  - Use nakama-ui for rich terminal display (panels, tables, charts)
  - Support `--board` flag to select a specific board
  - Support `--sprint` flag to analyze a past sprint
  - Default to current active sprint
- [ ] Implement output formatting:
  - Terminal: rich dashboard with charts and tables
  - Markdown: report format with embedded tables
  - JSON: structured analytics data for further processing
- [ ] Write unit tests for analytics calculations
- [ ] Write integration tests with mocked sprint data

## Acceptance Criteria

- Sprint progress is accurately calculated and displayed
- Burndown chart correctly shows ideal vs. actual trajectories
- Velocity trend is calculated from historical sprint data
- Scope creep is detected and quantified
- Cycle time distribution provides meaningful insights
- Workload balance identifies imbalances across team members
- The dashboard view combines all analytics in a readable format
- All calculations handle edge cases (empty sprints, no story points, etc.)

## Dependencies

- Step 003 (Jira Client) must be complete for sprint and issue access
- Jira Agile API access for board/sprint data
- `nakama-ui` for terminal chart rendering
- `chrono` for date/time calculations
