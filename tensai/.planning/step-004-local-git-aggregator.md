# Step 004: Local Git Aggregator

## Objective

Implement a local git repository aggregator that inspects the current git repo (and optionally configured repos) to extract current branch, uncommitted changes, stashes, and recent commit history. This provides developer-local context for briefings and standup generation.

## Tasks

- [ ] Add git2 (libgit2 bindings) dependency
- [ ] Implement `LocalGitAggregator` struct implementing `Aggregator` trait
- [ ] Implement repository discovery:
  - [ ] Current working directory repo
  - [ ] Configured list of repos to watch (config file)
  - [ ] Auto-discover repos under configured parent directories
- [ ] Extract current branch info:
  - [ ] Branch name
  - [ ] Tracking branch and ahead/behind counts
  - [ ] Last push time (if available from reflog)
- [ ] Extract uncommitted changes:
  - [ ] Staged files (count, list)
  - [ ] Unstaged modifications (count, list)
  - [ ] Untracked files (count)
  - [ ] Conflict markers detection
- [ ] Extract stash information:
  - [ ] Stash count
  - [ ] Stash messages and dates
  - [ ] Age of oldest stash
- [ ] Extract recent commits:
  - [ ] Last N commits (configurable, default 10)
  - [ ] Commits since last push
  - [ ] Commits today / since yesterday (for standup)
  - [ ] Commit messages, authors, timestamps
  - [ ] Files changed per commit (summary)
- [ ] Extract branch overview:
  - [ ] Active branches (recently updated)
  - [ ] Branches with unpushed commits
  - [ ] Stale branches (no activity in N days)
- [ ] Implement multi-repo aggregation:
  - [ ] Iterate configured repos
  - [ ] Combine results with repo labels
  - [ ] Handle missing/inaccessible repos gracefully
- [ ] Add configuration:
  - [ ] `repos` list in config file
  - [ ] `scan_dirs` for auto-discovery
  - [ ] `recent_commits_count` setting
  - [ ] `stale_branch_days` threshold
- [ ] Unit tests with temp git repos (git2 init/commit)
- [ ] Integration test with real repo data

## Acceptance Criteria

- Current branch and tracking status are correctly reported
- Uncommitted changes are accurately counted and categorized
- Stash count and details are extracted
- Recent commits include messages, authors, and timestamps
- Multi-repo aggregation works with configured repo list
- Missing repos are handled gracefully (skip with warning)
- Data extraction completes in <1 second per repo

## Dependencies

- Step 001 (CLI scaffold)
- Step 002 (Aggregator trait) must be complete
- git2 crate for git repository access
- No network dependencies (local only)
