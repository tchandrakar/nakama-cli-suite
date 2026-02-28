# Step 011: Watch Daemon

## Objective

Implement the `byakugan watch` daemon mode that continuously monitors configured repositories for new PRs, auto-triggers reviews, supports both polling and webhook listener modes, respects per-platform rate limits, and sends notifications when reviews are completed.

## Tasks

- [ ] Implement the polling loop:
  - Periodically call `list_open_prs()` on each configured repository
  - Track which PRs have already been reviewed (persist state in `~/.byakugan/watch_state.json`)
  - Detect new PRs that have not been reviewed yet
  - Detect updated PRs (new commits pushed since last review)
  - Configurable poll interval per platform (default 300 seconds)
  - Stagger polling across repos to distribute API load
- [ ] Implement webhook listener mode:
  - Start an HTTP server (using `axum` or `warp`) on a configurable port
  - Accept GitHub webhook payloads (`pull_request` events: opened, synchronize, reopened)
  - Accept GitLab webhook payloads (merge request events)
  - Accept Bitbucket webhook payloads (pullrequest events)
  - Verify webhook signatures per platform (HMAC-SHA256 for GitHub, token for GitLab)
  - Parse webhook payloads into platform-agnostic PR event structs
- [ ] Implement auto-review trigger:
  - On detecting a new/updated PR, queue a review job
  - Run reviews with a configurable concurrency limit (default: 1 concurrent review)
  - Use tokio tasks for concurrent review execution
  - Handle review failures gracefully (log error, mark for retry)
  - Configurable retry policy (max retries, backoff interval)
- [ ] Implement rate limit management:
  - Track API call counts per platform
  - Respect platform-specific rate limits:
    - GitHub: 5000 requests/hour for authenticated users
    - GitLab: varies by instance configuration
    - Bitbucket: 1000 requests/hour
  - Back off when approaching limits (pause polling until rate limit resets)
  - Log rate limit status in verbose mode
- [ ] Implement notification system:
  - Terminal notifications (print to stdout when running in foreground)
  - Desktop notifications via `notify-rust` crate (optional)
  - Future: Slack webhook integration
  - Future: Email notification support
  - Notification content: PR title, review verdict, finding count, link
- [ ] Implement daemon lifecycle management:
  - `byakugan watch start` -- start the daemon (foreground or background with `--daemon`)
  - `byakugan watch stop` -- stop the daemon (signal handling)
  - `byakugan watch status` -- show daemon status, watched repos, last poll times
  - Graceful shutdown on SIGINT/SIGTERM
  - PID file management for background mode
- [ ] Implement configuration for watch mode:
  - List of repos to watch (per platform)
  - Poll interval, auto-post settings, notification preferences
  - Read from `~/.byakugan/config.toml` [watch] section
- [ ] Write integration tests with mock platform APIs and simulated PR events

## Acceptance Criteria

- Polling mode correctly detects new and updated PRs at configured intervals
- Webhook listener correctly receives and processes events from all three platforms
- Auto-review triggers and completes reviews for new PRs
- Rate limits are respected and the daemon backs off when approaching limits
- Notifications are sent when reviews complete
- The daemon starts, stops, and reports status correctly
- State persistence survives daemon restarts
- Graceful shutdown completes in-progress reviews before exiting

## Dependencies

- Step 003, 004, 005 (platform adapters) must be complete
- Step 007 (Review Engine) must be complete
- Step 010 (Output Layer) must be complete for posting reviews
- `tokio` for async runtime and task management
- `axum` or `warp` for webhook HTTP server
- `notify-rust` for desktop notifications (optional)
