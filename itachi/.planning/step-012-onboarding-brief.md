# Step 012: Onboarding Brief

## Objective

Implement the `itachi onboard <project|service>` command that generates comprehensive onboarding documentation by aggregating Jira history, gathering Confluence docs, identifying key people, surfacing architecture documents, and highlighting open questions for a new team member.

## Tasks

- [ ] Implement Jira history aggregation:
  - Fetch all issues in the project from the last 90 days (configurable)
  - Categorize by type: features shipped, bugs fixed, tech debt addressed
  - Identify major milestones (epics completed, releases tagged)
  - Calculate project velocity and trend
  - Identify current active work (in-progress stories, current sprint)
  - Highlight recurring patterns (frequent bug areas, common issue types)
- [ ] Implement Confluence documentation gathering:
  - Search for key documentation types in the project's space:
    - Architecture docs (label: architecture, adr)
    - Onboarding guides (label: onboarding, getting-started)
    - API documentation (label: api, api-docs)
    - Runbooks (label: runbook, operations)
    - Design RFCs (label: rfc, design)
  - Sort by relevance and recency
  - Extract key sections from the most important pages
  - Identify documentation gaps (important topics without docs)
- [ ] Implement key people identification:
  - Analyze Jira activity: who has the most issue transitions in the project?
  - Analyze Confluence edits: who writes the most documentation?
  - Identify roles: tech lead (most reviews/approvals), domain expert (most edits in specific areas)
  - Include contact info where available
- [ ] Implement architecture documentation:
  - Search for ADRs (Architecture Decision Records) in the project space
  - Extract key architectural decisions and their rationale
  - Identify the tech stack from documentation and Jira labels
  - Build a component/service map if available
- [ ] Implement open questions identification:
  - Find recent Jira tickets with questions/discussion in comments
  - Find Confluence pages with unresolved inline comments
  - Find recently created "Question" type issues
  - Identify active RFCs or design discussions
- [ ] Implement brief generation with LLM synthesis:
  - Use nakama-ai to synthesize all gathered data into a coherent brief
  - Structure the brief:
    - Overview (project purpose, status)
    - Architecture (key decisions, tech stack)
    - Key People (roles and expertise areas)
    - Recent Activity (last 30-90 days summary)
    - Key Documents (with links)
    - Open Questions / Active Discussions
    - Getting Started Guide (suggested first steps)
  - Include links to all referenced Jira tickets and Confluence pages
- [ ] Implement output formatting:
  - Terminal: rich multi-section display with links
  - Markdown: complete onboarding document suitable for saving
  - JSON: structured data for further processing
- [ ] Implement `--depth` flag: quick (overview only), standard (default), comprehensive (everything)
- [ ] Write unit tests for data aggregation and brief generation
- [ ] Write integration tests with mocked Jira and Confluence data

## Acceptance Criteria

- Onboarding brief accurately reflects the project's current state
- Key people are correctly identified from activity data
- Architecture documents and ADRs are surfaced
- Documentation gaps are identified
- Open questions and active discussions are highlighted
- The brief is comprehensive enough for a new team member to get oriented
- All output formats produce well-structured, readable documents
- Links to Jira tickets and Confluence pages are included

## Dependencies

- Step 003 (Jira Client) must be complete for project history access
- Step 004 (Confluence Client) must be complete for documentation access
- Step 007 (Cross-Reference Engine) for linking docs to tickets
- Step 008 (Intelligence Layer) for semantic search and synthesis
- `nakama-ai` for brief synthesis
