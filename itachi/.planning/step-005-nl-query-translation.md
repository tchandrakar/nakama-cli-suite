# Step 005: Natural Language Query Translation

## Objective

Integrate with nakama-ai to translate natural language queries into JQL (Jira Query Language) and CQL (Confluence Query Language), injecting project-specific context (project keys, spaces, custom fields) to improve translation accuracy, and validating generated queries before execution.

## Tasks

- [ ] Add `nakama-ai` shared crate dependency
- [ ] Implement NL to JQL translation:
  - Build system prompt with JQL syntax reference and examples
  - Inject project-specific context:
    - Available project keys (e.g., PAY, ENG, PLATFORM)
    - Available issue types (Bug, Story, Epic, Task, Sub-task)
    - Available statuses and transitions
    - Custom field names and their `customfield_XXXXX` mappings
    - Available sprint names and board names
    - Available users/assignees
  - Send user's natural language query to LLM with context
  - Parse LLM response to extract JQL string
  - Examples:
    - "what's blocking the payments team?" -> `project = PAY AND status != Done AND flagged = impediment`
    - "my open bugs this sprint" -> `assignee = currentUser() AND type = Bug AND sprint in openSprints() AND status != Done`
- [ ] Implement NL to CQL translation:
  - Build system prompt with CQL syntax reference and examples
  - Inject context:
    - Available space keys and names
    - Common label taxonomies
    - Content types (page, blog, attachment)
  - Send user's natural language query to LLM with context
  - Parse LLM response to extract CQL string
  - Examples:
    - "find the auth service architecture doc" -> `space = ENG AND type = page AND (label = "adr" OR label = "architecture") AND text ~ "auth service"`
    - "recent updates in the payments space" -> `space = PAY AND type = page AND lastModified > now("-7d")`
- [ ] Implement query validation:
  - Validate generated JQL by calling Jira's JQL parse endpoint (`/rest/api/3/jql/parse`)
  - Validate generated CQL by attempting a search with `limit=0`
  - If validation fails, retry translation with error feedback to LLM
  - Maximum 3 retry attempts before falling back to user notification
- [ ] Implement context loading:
  - On first use, fetch and cache project metadata (projects, fields, statuses)
  - Store metadata cache in `~/.itachi/context_cache.json`
  - Refresh cache periodically or on `itachi context refresh`
  - Allow manual context additions in config file
- [ ] Implement query refinement loop:
  - Show generated query to user in verbose mode
  - Allow user to confirm, modify, or reject the generated query
  - Learn from user corrections (store as examples for future translations)
- [ ] Implement `--jql` and `--cql` flags for direct query input:
  - `itachi jira --jql="project = PAY AND status = Done"` -- bypass NL translation
  - `itachi wiki --cql="space = ENG AND label = adr"` -- bypass NL translation
- [ ] Write unit tests for NL to JQL/CQL translation with mock LLM
- [ ] Write integration tests for query validation

## Acceptance Criteria

- Natural language queries are correctly translated to JQL and CQL
- Project-specific context improves translation accuracy
- Generated queries are validated before execution
- Validation failures trigger automatic retry with error feedback
- Direct JQL/CQL input works via flags
- Context cache is maintained and refreshable
- Common query patterns produce correct results consistently

## Dependencies

- Step 003 (Jira Client) must be complete for JQL validation and context loading
- Step 004 (Confluence Client) must be complete for CQL validation and context loading
- `nakama-ai` shared crate for LLM-based translation
