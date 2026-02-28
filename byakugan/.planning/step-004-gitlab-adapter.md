# Step 004: GitLab Adapter

## Objective

Implement the `PlatformAdapter` trait for GitLab using `reqwest` with the GitLab REST and GraphQL APIs. This adapter handles fetching merge request metadata, diffs, discussions, posting review notes, and listing open MRs.

## Tasks

- [ ] Create `GitLabAdapter` struct with fields: reqwest client, base_url, project_id, token
- [ ] Implement constructor that accepts a GitLab token, base URL, and project path
- [ ] Implement project ID resolution (convert `owner/repo` path to numeric project ID via API)
- [ ] Implement `fetch_pr()` (maps to GitLab MR):
  - Use `GET /projects/:id/merge_requests/:mr_iid` REST endpoint
  - Map GitLab MR response to the shared `PullRequest` model
  - Handle the terminology difference (MR vs PR) transparently
- [ ] Implement `get_diff()`:
  - Use `GET /projects/:id/merge_requests/:mr_iid/changes` for file-level diffs
  - Alternatively use `GET /projects/:id/merge_requests/:mr_iid/diffs` for version-based diffs
  - Parse the diff content into the `UnifiedDiff` model
  - Handle renamed/moved files
- [ ] Implement `get_comments()` (maps to GitLab Discussions/Notes):
  - Use `GET /projects/:id/merge_requests/:mr_iid/discussions` for threaded discussions
  - Use `GET /projects/:id/merge_requests/:mr_iid/notes` for flat notes
  - Map to the shared `Comment` model
  - Preserve thread structure for context
- [ ] Implement `post_comment()`:
  - Use `POST /projects/:id/merge_requests/:mr_iid/discussions` for new inline discussions
  - Position comments on the correct diff line using `position` object (new_path, new_line, base_sha, head_sha, start_sha)
- [ ] Implement `post_review()`:
  - GitLab does not have a single "review" concept like GitHub
  - Simulate by posting a summary note + individual inline discussion threads
  - Use approval API (`POST /projects/:id/merge_requests/:mr_iid/approve`) for approve verdict
- [ ] Implement `list_open_prs()`:
  - Use `GET /projects/:id/merge_requests?state=opened`
  - Support pagination via Link headers
  - Map to Vec<PullRequest>
- [ ] Add self-hosted GitLab support (configurable base URL, default to gitlab.com)
- [ ] Add rate limit handling (respect RateLimit headers)
- [ ] Implement GraphQL queries for efficient batch data fetching where beneficial
- [ ] Write integration tests with mock GitLab API responses

## Acceptance Criteria

- All 6 `PlatformAdapter` methods work correctly against the GitLab API
- MR metadata, diffs, and discussions are correctly mapped to shared models
- Inline discussion comments are positioned correctly on diff lines
- The approval workflow is handled for the approve verdict
- Self-hosted GitLab instances (custom base URL) work correctly
- Rate limiting is respected
- Integration tests pass with mocked API responses

## Dependencies

- Step 002 (PlatformAdapter trait and models) must be complete
- `reqwest` crate with JSON support
- `serde` / `serde_json` for response deserialization
- GitLab personal access token for authentication
- nakama-vault integration for token retrieval
