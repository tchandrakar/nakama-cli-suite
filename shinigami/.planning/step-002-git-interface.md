# Step 002: Git Interface

## Objective
Build a comprehensive git interface layer using `git2-rs` that provides diff parsing, log reading, branch management, and tag operations. This layer abstracts all direct git interactions and provides structured Rust types for the rest of the application.

## Tasks
- [ ] Add `git2` crate dependency and create `git.rs` module
- [ ] Implement `GitRepo` wrapper struct around `git2::Repository`:
  - `GitRepo::open(path)` — open repository at path or discover from cwd
  - `GitRepo::open_or_discover()` — discover repo from current directory
- [ ] Implement diff parsing:
  - `get_staged_diff()` — return structured diff of staged changes
  - Parse diff into `DiffFile` structs: file path, old/new path, status (added/modified/deleted/renamed)
  - Parse hunks: `DiffHunk` with context lines, added lines, deleted lines, line numbers
  - Track file types (detect language from extension)
  - Count insertions and deletions per file and total
- [ ] Implement log reading:
  - `get_log(range, limit)` — return commits between refs
  - `get_commits_since_tag(tag)` — commits since a specific tag
  - `get_commit(oid)` — single commit details
  - Parse each commit into `CommitInfo`: oid, subject, body, author, date, parents
- [ ] Implement branch management:
  - `current_branch()` — return current branch name
  - `list_branches()` — list local and remote branches
  - `create_branch(name)` — create a new branch
  - `branch_exists(name)` — check if branch exists
- [ ] Implement tag management:
  - `list_tags()` — list all tags (sorted by version if semver)
  - `latest_tag()` — return most recent tag
  - `create_tag(name, message)` — create annotated tag
  - `tag_exists(name)` — check if tag exists
- [ ] Implement status helpers:
  - `is_clean()` — check if working tree is clean
  - `has_staged_changes()` — check if there are staged changes
  - `staged_files()` — list staged file paths
- [ ] Handle edge cases: empty repo, detached HEAD, shallow clone, submodules
- [ ] Write unit tests with a temporary git repository (using `tempdir`)
- [ ] Write tests for diff parsing with various change types

## Acceptance Criteria
- `get_staged_diff()` returns correctly parsed hunks with line-level detail
- Log reader returns structured commits with full metadata
- Branch and tag operations work correctly
- All methods return `Result<T>` with descriptive error messages
- Edge cases (empty repo, no commits, detached HEAD) are handled gracefully
- Tests pass using temporary git repositories

## Dependencies
- Step 001 (CLI scaffold must exist to house this module)
