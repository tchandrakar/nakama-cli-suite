# Step 007: Briefing Engine

## Objective

Build the core briefing engine that collects data from all aggregators, synthesizes it using LLM via nakama-ai, produces formatted output using nakama-ui panels, and generates time-aware suggestions for the developer's day.

## Tasks

- [ ] Implement `BriefingEngine` struct:
  - [ ] Accept aggregator registry reference
  - [ ] Configurable briefing sections
  - [ ] Template system for different briefing types
- [ ] Implement data collection phase:
  - [ ] Trigger parallel fetch from all enabled aggregators
  - [ ] Collect results with error handling
  - [ ] Merge overlapping data (e.g., GitHub PR + Jira ticket linkage)
  - [ ] Build unified context object for LLM
- [ ] Implement LLM synthesis via nakama-ai:
  - [ ] Assemble prompt with all aggregated data
  - [ ] Prompt template: "Given this developer's current context, generate a morning briefing"
  - [ ] Include time-of-day awareness (morning vs afternoon vs evening)
  - [ ] Include day-of-week awareness (Monday = sprint planning, Friday = wrap-up)
  - [ ] Parse structured LLM response (sections, priorities, suggestions)
- [ ] Implement briefing sections:
  - [ ] **Priority items**: urgent PRs, overdue tickets, failing CI
  - [ ] **Code review**: PRs awaiting review with age and size
  - [ ] **My PRs**: authored PRs with review/CI status
  - [ ] **Schedule**: today's meetings with focus blocks
  - [ ] **Sprint context**: sprint progress, remaining work
  - [ ] **Suggestions**: AI-generated day plan recommendations
- [ ] Implement time-aware suggestions:
  - [ ] "Review PR #123 during your 10am focus block (30 min, 5 files)"
  - [ ] "Your PR #456 has been waiting for review for 2 days, consider pinging"
  - [ ] "Sprint ends Friday, 3 stories remaining - focus on story X today"
  - [ ] "You have a 1:1 at 2pm - prepare status update"
- [ ] Implement output formatting:
  - [ ] Use nakama-ui panels for each section
  - [ ] Color-coded urgency indicators
  - [ ] Summary counts (e.g., "3 PRs to review, 2 failing CI, 5 meetings")
  - [ ] Table format for lists (PRs, issues, meetings)
- [ ] Implement `brief` subcommand:
  - [ ] Default: full morning briefing
  - [ ] `--section` flag to show specific sections only
  - [ ] `--quick` flag for abbreviated output
  - [ ] Output format: text (default), json, markdown
- [ ] Implement offline mode:
  - [ ] Without AI: data-only briefing (no suggestions, no synthesis)
  - [ ] Respect `--no-ai` global flag
- [ ] Unit tests with mock aggregator data
- [ ] Integration test: full briefing generation with mocked services

## Acceptance Criteria

- Briefing collects data from all enabled aggregators in parallel
- LLM synthesis produces coherent, actionable briefing text
- Time-of-day and day-of-week context influence suggestions
- Suggestions reference specific items with actionable details
- Output is well-formatted with nakama-ui panels and tables
- Offline mode provides useful data-only briefing
- Briefing generates in <10 seconds with all aggregators
- Sections can be individually shown or hidden

## Dependencies

- Step 002 (Aggregator trait) for data collection
- Steps 003-006 (Aggregators) for actual data sources
- nakama-ai shared crate for LLM synthesis
- nakama-ui shared crate for formatted output
