# Step 003: GitHub Aggregator

## Objective

Implement a GitHub aggregator using octocrab that fetches open pull requests (authored and review-requested), assigned issues, notifications, and CI/CD status from GitHub Actions. This provides the core code activity context for developer briefings.

## Tasks

- [ ] Add octocrab dependency
- [ ] Implement `GitHubAggregator` struct implementing `Aggregator` trait
- [ ] Implement GitHub authentication:
  - [ ] Personal access token from nakama-vault
  - [ ] Fallback to `GITHUB_TOKEN` environment variable
  - [ ] Fallback to `gh` CLI auth token
- [ ] Fetch open PRs authored by user:
  - [ ] Title, number, repo, created date, updated date
  - [ ] Review status (approved, changes requested, pending)
  - [ ] CI/CD status (passing, failing, pending)
  - [ ] Merge conflicts status
  - [ ] Comment count and last activity
- [ ] Fetch PRs requesting user's review:
  - [ ] Title, number, repo, author, requested date
  - [ ] Age of review request
  - [ ] PR size (files changed, lines added/removed)
- [ ] Fetch assigned issues:
  - [ ] Title, number, repo, labels, milestone
  - [ ] Priority labels detection
  - [ ] Due date (if set via milestone)
  - [ ] Linked PRs
- [ ] Fetch notifications:
  - [ ] Unread count and categories
  - [ ] Mention notifications (highest priority)
  - [ ] Review request notifications
  - [ ] CI failure notifications
- [ ] Fetch CI/CD status from GitHub Actions:
  - [ ] Latest workflow runs for user's repos/branches
  - [ ] Failed runs with failure reason summary
  - [ ] Currently running workflows
- [ ] Implement caching:
  - [ ] Cache responses with configurable TTL (default 5 minutes)
  - [ ] ETag-based conditional requests to reduce API usage
- [ ] Implement rate limit awareness:
  - [ ] Track remaining rate limit from response headers
  - [ ] Warn when approaching limit
  - [ ] Reduce fetch frequency when low on quota
- [ ] Add configuration options:
  - [ ] Repos to include/exclude
  - [ ] Organizations to watch
  - [ ] PR age threshold for urgency
- [ ] Unit tests with mocked octocrab responses
- [ ] Integration test with GitHub API (optional, requires token)

## Acceptance Criteria

- All authored PRs and review-requested PRs are fetched correctly
- CI/CD status is included with each PR
- Assigned issues include priority and due date information
- Notifications are categorized and prioritized
- Caching reduces API calls on repeated fetches
- Rate limiting is respected and surfaced to user
- Graceful handling when GitHub is unreachable

## Dependencies

- Step 001 (CLI scaffold)
- Step 002 (Aggregator trait) must be complete
- octocrab crate for GitHub API
- nakama-vault for GitHub token (Step 008)
