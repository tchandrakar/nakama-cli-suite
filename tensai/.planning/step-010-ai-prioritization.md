# Step 010: AI Prioritization

## Objective

Build an AI-powered prioritization engine that scores tasks by urgency, importance, and context-switching cost. Consider meeting schedule to slot work into available focus blocks and minimize context switching. Use LLM reasoning to produce a prioritized plan for the day.

## Tasks

- [ ] Define `PrioritizedItem` struct: item, urgency_score, importance_score, context_cost, composite_score, slot_suggestion, reasoning
- [ ] Define scoring dimensions:
  - [ ] **Urgency**: deadline proximity, SLA, review age, CI failure impact
  - [ ] **Importance**: priority label, sprint goal alignment, dependency chain (blocks others), stakeholder visibility
  - [ ] **Context cost**: repo familiarity, task complexity, number of files, related to current branch
- [ ] Implement rule-based scoring:
  - [ ] Urgency: overdue=10, due today=8, due this week=5, no deadline=2
  - [ ] Importance: P0=10, P1=7, P2=5, P3=2, unlabeled=3
  - [ ] Context cost: same repo as current=1, recently touched=3, new repo=7
  - [ ] Composite: configurable weights (default: urgency=0.4, importance=0.4, context=0.2)
- [ ] Implement context-switching minimization:
  - [ ] Group tasks by repo/project
  - [ ] Prefer batching related tasks together
  - [ ] Schedule repo switches at natural break points (meetings)
- [ ] Implement meeting-aware scheduling:
  - [ ] Map focus blocks from calendar aggregator
  - [ ] Match task estimated duration to available blocks
  - [ ] Large tasks -> long focus blocks, small tasks -> short blocks
  - [ ] Schedule high-priority before high-context-cost
  - [ ] Leave buffer before meetings
- [ ] Implement LLM reasoning enhancement:
  - [ ] Feed all items + scores + schedule to nakama-ai
  - [ ] Ask for prioritization rationale
  - [ ] Let LLM adjust scores based on contextual reasoning
  - [ ] Generate natural language plan ("First, tackle PR #123 review during your 9-10am block because...")
- [ ] Implement `plan` subcommand:
  - [ ] Default: generate prioritized plan for today
  - [ ] `--items` flag: show only top N items
  - [ ] `--explain` flag: include LLM reasoning for each item
  - [ ] `--recalculate` flag: force fresh calculation
  - [ ] Output: ordered list with scores and time slot suggestions
- [ ] Implement plan persistence:
  - [ ] Save today's plan
  - [ ] Track plan completion through the day
  - [ ] Adjust plan based on completed items
- [ ] Unit tests for scoring algorithms
- [ ] Unit tests for scheduling with mock calendar data

## Acceptance Criteria

- Scoring correctly ranks urgent/important items higher
- Context switching cost is factored into ordering
- Tasks are mapped to available focus blocks based on estimated duration
- Related tasks from same repo are grouped together
- LLM reasoning provides actionable justification for priorities
- Plan accounts for meeting schedule and buffer time
- Composite score weights are configurable
- Plan can be regenerated during the day as context changes

## Dependencies

- Step 002 (Aggregator trait) for data collection
- Steps 003-006 (Aggregators) for task/PR/meeting data
- Step 005 (Calendar aggregator) for focus block calculation
- Step 007 (Briefing engine) for synthesis patterns
- nakama-ai for LLM reasoning
