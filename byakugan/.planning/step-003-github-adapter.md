# Step 003: GitHub Adapter

## Objective

Implement the `PlatformAdapter` trait for GitHub using the `octocrab` crate. This adapter handles fetching PR metadata, diffs, comments, posting review comments, submitting reviews, and listing open PRs via the GitHub REST and GraphQL APIs.

## Tasks

- [ ] Add `octocrab` dependency to Cargo.toml
- [ ] Create `GitHubAdapter` struct with fields: octocrab client instance, owner, repo
- [ ] Implement constructor that accepts a GitHub token and owner/repo pair
- [ ] Implement `fetch_pr()`:
  - Use `octocrab.pulls(owner, repo).get(pr_number)` to fetch PR metadata
  - Map GitHub PR response to the shared `PullRequest` model
  - Fetch linked issues from PR body (parse `#NNN` references)
- [ ] Implement `get_diff()`:
  - Fetch the diff via GitHub's media type `application/vnd.github.v3.diff`
  - Parse the raw unified diff into the `UnifiedDiff` model
  - Handle large diffs (paginate file list if needed)
- [ ] Implement `get_comments()`:
  - Fetch both PR review comments and issue comments
  - Map to the shared `Comment` model
  - Sort by creation date
- [ ] Implement `post_comment()`:
  - Post an inline review comment to a specific file and line
  - Handle the case where the line is not part of the diff (fall back to general comment)
- [ ] Implement `post_review()`:
  - Submit a pull request review with verdict (APPROVE, REQUEST_CHANGES, COMMENT)
  - Include inline comments as part of the review submission
  - Handle the review body (summary)
- [ ] Implement `list_open_prs()`:
  - Fetch all open PRs for the repository
  - Map to Vec<PullRequest>
  - Support pagination for repos with many open PRs
- [ ] Implement webhook listener for `pull_request` events (used by watch daemon):
  - Parse webhook payload to extract PR details
  - Verify webhook signature (HMAC-SHA256)
- [ ] Add GitHub Enterprise support (configurable API base URL)
- [ ] Add rate limit handling (respect X-RateLimit headers, back off when near limit)
- [ ] Write integration tests with mock GitHub API responses (using wiremock or similar)

## Acceptance Criteria

- All 6 `PlatformAdapter` methods work correctly against the GitHub API
- PR metadata, diffs, and comments are correctly mapped to shared models
- Reviews and comments are posted successfully to GitHub
- Rate limiting is respected and the adapter backs off gracefully
- GitHub Enterprise (custom API URL) works
- Webhook payloads are correctly parsed and signature-verified
- Integration tests pass with mocked API responses

## Dependencies

- Step 002 (PlatformAdapter trait and models) must be complete
- `octocrab` crate for GitHub API interaction
- GitHub personal access token for authentication
- nakama-vault integration for token retrieval (can use env var fallback initially)
