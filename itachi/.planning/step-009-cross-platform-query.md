# Step 009: Cross-Platform Query

## Objective

Implement the `itachi ask` command that routes queries to both Jira and Confluence simultaneously, plans whether a query needs Jira data, Confluence data, or both, executes queries in parallel, matches cross-references, and produces a unified answer with links to both platforms.

## Tasks

- [ ] Implement query planning:
  - Classify user question intent:
    - Jira-only: "what's the status of PROJ-142?" -> route to Jira
    - Confluence-only: "find the auth service architecture doc" -> route to Confluence
    - Cross-platform: "what's the status of the migration project?" -> both
    - Analytics: "how is the sprint going?" -> Jira agile data
  - Use nakama-ai to classify intent and generate:
    - JQL query (if Jira needed)
    - CQL query (if Confluence needed)
    - Semantic search query (if embedding search needed)
  - Display query plan in verbose mode
- [ ] Implement parallel execution:
  - Execute Jira, Confluence, and embedding searches concurrently using tokio
  - Set per-query timeout (configurable, default: 15s)
  - Collect results as they arrive
  - Handle partial failures (one platform fails, still return results from the other)
- [ ] Implement cross-reference matching:
  - After collecting results from both platforms, use the cross-reference engine (step 007) to:
    - Link issues to their related documentation
    - Link pages to their associated tickets
    - Identify gaps (issues mentioned without docs, docs without implementation tickets)
  - Enrich results with cross-reference relationships
- [ ] Implement unified answer generation:
  - Feed all collected data to the context builder (step 008)
  - Generate a unified answer using the LLM synthesizer
  - Include:
    - Direct answer to the question
    - Relevant Jira tickets with status, assignee, and links
    - Relevant Confluence pages with titles and links
    - Cross-references between tickets and docs
    - Suggested actions (if applicable)
- [ ] Implement output formatting:
  - Terminal: structured output with Jira and Confluence sections, clickable links
  - Markdown: clean document suitable for sharing
  - JSON: structured data with all results and relationships
- [ ] Implement `--jira-only` and `--wiki-only` flags:
  - Force routing to a single platform even for cross-platform queries
  - Useful when user knows which platform has the answer
- [ ] Implement progressive output:
  - Show partial results as they arrive (Jira results first if faster)
  - Update display when all results are collected
  - Final synthesis displayed after all data is gathered
- [ ] Write unit tests for query planning and routing
- [ ] Write integration tests for cross-platform queries

## Acceptance Criteria

- `itachi ask` correctly routes queries to the appropriate platform(s)
- Parallel execution completes faster than sequential execution
- Cross-reference matching enriches results with platform links
- Unified answers include data from both Jira and Confluence with attribution
- Partial failures are handled gracefully (results from one platform still shown)
- `--jira-only` and `--wiki-only` flags work correctly
- Progressive output shows results as they arrive

## Dependencies

- Step 003 (Jira Client) must be complete
- Step 004 (Confluence Client) must be complete
- Step 005 (NL Query Translation) must be complete for JQL/CQL generation
- Step 007 (Cross-Reference Engine) must be complete for relationship matching
- Step 008 (Intelligence Layer) must be complete for semantic search and synthesis
