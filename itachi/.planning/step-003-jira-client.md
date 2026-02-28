# Step 003: Jira Client

## Objective

Build the Jira REST API v3 client using reqwest, supporting issue CRUD operations, JQL query execution, sprint and board data retrieval, comment management, and attachment handling.

## Tasks

- [ ] Create `JiraClient` struct with fields: reqwest client, base_url, auth (from step 002)
- [ ] Implement the base request builder:
  - Construct request URLs: `{base_url}/rest/api/3/{endpoint}`
  - Attach authentication headers (Basic auth or Bearer token)
  - Handle pagination (startAt, maxResults, total) for all list endpoints
  - Handle rate limiting (429 responses with retry)
- [ ] Implement issue operations:
  - `get_issue(key: &str) -> Result<Issue>` -- fetch single issue by key (e.g., PROJ-142)
  - `search_issues(jql: &str) -> Result<Vec<Issue>>` -- execute JQL query with pagination
  - `create_issue(project, type, summary, description, fields) -> Result<Issue>` -- create new issue
  - `update_issue(key, fields) -> Result<()>` -- update issue fields
  - `transition_issue(key, transition_id) -> Result<()>` -- change issue status
  - `get_transitions(key) -> Result<Vec<Transition>>` -- list available transitions
- [ ] Define Jira data models:
  - `Issue` struct: key, id, summary, description, status, priority, assignee, reporter, labels, components, created, updated, custom_fields
  - `IssueType` struct: id, name, description
  - `Status` struct: id, name, category
  - `Priority` struct: id, name
  - `User` struct: accountId, displayName, emailAddress
  - `Transition` struct: id, name, to_status
  - `Sprint` struct: id, name, state, startDate, endDate, goal
  - `Board` struct: id, name, type (scrum/kanban)
- [ ] Implement comment operations:
  - `get_comments(issue_key) -> Result<Vec<Comment>>` -- list comments on an issue
  - `add_comment(issue_key, body) -> Result<Comment>` -- add a comment
  - Handle ADF (Atlassian Document Format) for rich text comments
- [ ] Implement attachment operations:
  - `get_attachments(issue_key) -> Result<Vec<Attachment>>` -- list attachments
  - `add_attachment(issue_key, file_path) -> Result<Attachment>` -- upload attachment
  - `download_attachment(attachment_id, dest_path) -> Result<()>` -- download attachment
- [ ] Implement sprint and board operations:
  - `get_boards() -> Result<Vec<Board>>` -- list all boards
  - `get_sprints(board_id) -> Result<Vec<Sprint>>` -- list sprints for a board
  - `get_sprint_issues(sprint_id) -> Result<Vec<Issue>>` -- list issues in a sprint
  - `get_board_backlog(board_id) -> Result<Vec<Issue>>` -- list backlog items
- [ ] Implement Agile API integration:
  - Use `/rest/agile/1.0/` endpoints for sprint/board data
  - Fetch velocity data (story points completed per sprint)
  - Fetch burndown data (remaining work over time)
- [ ] Implement JQL helper utilities:
  - Validate JQL syntax before executing
  - Provide JQL auto-complete suggestions for field names
  - Support custom field mapping (human-readable names to customfield_XXXXX)
- [ ] Write unit tests with mocked Jira API responses
- [ ] Write integration tests for issue CRUD and JQL queries

## Acceptance Criteria

- Issue fetch, create, update, and transition operations work correctly
- JQL queries execute and return paginated results
- Sprint and board data is correctly retrieved
- Comments and attachments are handled (create, read)
- ADF (Atlassian Document Format) is handled for rich text
- Pagination works for all list endpoints
- Rate limiting is respected (429 retry with backoff)
- Custom field mapping works for project-specific fields

## Dependencies

- Step 002 (Auth Layer) must be complete for authentication
- `reqwest` crate with JSON support
- `serde` / `serde_json` for response deserialization
- `chrono` for date/time handling
