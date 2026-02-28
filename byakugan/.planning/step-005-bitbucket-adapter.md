# Step 005: Bitbucket Adapter

## Objective

Implement the `PlatformAdapter` trait for Bitbucket Cloud using `reqwest` with the Bitbucket REST API v2.0. This adapter handles fetching PR metadata, diffs, comments, posting review comments, and listing open PRs.

## Tasks

- [ ] Create `BitbucketAdapter` struct with fields: reqwest client, workspace, repo_slug, auth (username + app_password)
- [ ] Implement constructor that accepts Bitbucket credentials and workspace/repo_slug
- [ ] Implement `fetch_pr()`:
  - Use `GET /2.0/repositories/{workspace}/{repo_slug}/pullrequests/{pr_id}` endpoint
  - Map Bitbucket PR response to the shared `PullRequest` model
  - Extract participant and reviewer information
- [ ] Implement `get_diff()`:
  - Use `GET /2.0/repositories/{workspace}/{repo_slug}/pullrequests/{pr_id}/diff` for raw diff
  - Use `GET /2.0/repositories/{workspace}/{repo_slug}/pullrequests/{pr_id}/diffstat` for file-level summary
  - Parse into the `UnifiedDiff` model
  - Handle binary files and large diffs
- [ ] Implement `get_comments()`:
  - Use `GET /2.0/repositories/{workspace}/{repo_slug}/pullrequests/{pr_id}/comments` endpoint
  - Map to the shared `Comment` model
  - Handle nested/threaded comments (parent_id field)
  - Support pagination (Bitbucket uses cursor-based pagination)
- [ ] Implement `post_comment()`:
  - Use `POST /2.0/repositories/{workspace}/{repo_slug}/pullrequests/{pr_id}/comments`
  - Include `inline` object with `to`/`from` line numbers and `path` for inline comments
  - Handle the case where the line is outside the diff range
- [ ] Implement `post_review()`:
  - Bitbucket does not have a GitHub-style review object
  - Simulate by posting a top-level summary comment + individual inline comments
  - Use `POST /2.0/repositories/{workspace}/{repo_slug}/pullrequests/{pr_id}/approve` for approve
  - Use `POST /2.0/repositories/{workspace}/{repo_slug}/pullrequests/{pr_id}/request-changes` for request changes
- [ ] Implement `list_open_prs()`:
  - Use `GET /2.0/repositories/{workspace}/{repo_slug}/pullrequests?state=OPEN`
  - Handle pagination (Bitbucket uses `next` URL in response)
  - Map to Vec<PullRequest>
- [ ] Add Bitbucket Server/Data Center support (different API format) as optional
- [ ] Add rate limit handling (respect rate limit headers)
- [ ] Write integration tests with mock Bitbucket API responses

## Acceptance Criteria

- All 6 `PlatformAdapter` methods work correctly against the Bitbucket Cloud API
- PR metadata, diffs, and comments are correctly mapped to shared models
- Inline comments are correctly positioned on diff lines
- The approve/request-changes workflow functions correctly
- Pagination is handled for all list endpoints
- Rate limiting is respected
- Integration tests pass with mocked API responses

## Dependencies

- Step 002 (PlatformAdapter trait and models) must be complete
- `reqwest` crate with JSON support and basic auth
- `serde` / `serde_json` for response deserialization
- Bitbucket app password for authentication
- nakama-vault integration for credential retrieval
