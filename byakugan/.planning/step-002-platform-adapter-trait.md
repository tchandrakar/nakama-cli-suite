# Step 002: Platform Adapter Trait

## Objective

Define the `PlatformAdapter` trait that abstracts all interactions with code hosting platforms (GitHub, GitLab, Bitbucket). This trait is the core of byakugan's platform-agnostic design. Also implement auto-detection logic that inspects git remote URLs to select the appropriate adapter at runtime.

## Tasks

- [ ] Define shared data types in a `models` module:
  - `PullRequest` struct (id, title, description, author, source_branch, target_branch, url, state, created_at, updated_at)
  - `UnifiedDiff` struct (files: Vec<DiffFile>, raw: String)
  - `DiffFile` struct (path, old_path, status: FileStatus, hunks: Vec<Hunk>)
  - `Hunk` struct (old_start, old_count, new_start, new_count, lines: Vec<DiffLine>)
  - `DiffLine` enum (Added, Removed, Context with content and line numbers)
  - `FileStatus` enum (Added, Modified, Deleted, Renamed, Copied)
  - `Comment` struct (id, author, body, path, line, created_at, updated_at)
  - `ReviewComment` struct (path, line, body, severity)
  - `Review` struct (body, comments: Vec<ReviewComment>, verdict: ReviewVerdict)
  - `ReviewVerdict` enum (Approve, RequestChanges, Comment)
- [ ] Define the `PlatformAdapter` async trait:
  - `async fn fetch_pr(&self, id: &str) -> Result<PullRequest>`
  - `async fn get_diff(&self, pr: &PullRequest) -> Result<UnifiedDiff>`
  - `async fn get_comments(&self, pr: &PullRequest) -> Result<Vec<Comment>>`
  - `async fn post_comment(&self, pr: &PullRequest, comment: &ReviewComment) -> Result<()>`
  - `async fn post_review(&self, pr: &PullRequest, review: &Review) -> Result<()>`
  - `async fn list_open_prs(&self) -> Result<Vec<PullRequest>>`
- [ ] Implement `PlatformDetector` module:
  - Parse git remote URLs (origin, upstream) using regex
  - Detect `github.com` -> GitHub adapter
  - Detect `gitlab.com` or configurable self-hosted patterns -> GitLab adapter
  - Detect `bitbucket.org` -> Bitbucket adapter
  - Extract owner/repo from the remote URL
  - Support SSH and HTTPS remote URL formats
- [ ] Implement `detect_platform()` function that returns a `Box<dyn PlatformAdapter>`
- [ ] Implement `detect_current_pr()` function that finds the PR for the current branch
- [ ] Add unit tests for URL parsing and platform detection
- [ ] Add unit tests for model serialization/deserialization

## Acceptance Criteria

- The `PlatformAdapter` trait compiles and is object-safe (usable as `dyn PlatformAdapter`)
- All model types derive `Debug`, `Clone`, `Serialize`, `Deserialize`
- Platform detection correctly identifies GitHub, GitLab, and Bitbucket from various URL formats (SSH, HTTPS, with/without `.git` suffix)
- `detect_platform()` returns an error with a helpful message for unrecognized remote URLs
- Unit tests pass for all URL parsing edge cases

## Dependencies

- Step 001 (CLI scaffold) must be complete
- `async-trait` crate for async trait definitions
- `regex` crate for URL parsing
- Git must be available on the system PATH for remote URL inspection
