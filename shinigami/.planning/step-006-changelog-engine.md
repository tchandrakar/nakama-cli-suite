# Step 006: Changelog Engine

## Objective
Build a changelog generation engine that aggregates commits between two git refs, groups them by semantic type, renders them through configurable templates (using Tera), and outputs to CHANGELOG.md, stdout, or clipboard.

## Tasks
- [ ] Create `changelog.rs` module with `ChangelogGenerator` struct
- [ ] Implement commit aggregation:
  - `aggregate(from_ref, to_ref)` — collect all commits between two refs
  - Support tag-to-tag, tag-to-HEAD, commit-to-commit ranges
  - Auto-detect `from_ref` as latest tag if not specified
  - Parse conventional commit messages to extract type, scope, subject
  - Handle non-conventional commits (group as "Other")
- [ ] Implement grouping by type:
  - Features (feat), Bug Fixes (fix), Refactoring (refactor), Documentation (docs)
  - Tests (test), Chores (chore), Performance (perf), Style (style)
  - Breaking Changes (extracted from footers and `!` suffix)
  - Configurable group ordering
- [ ] Implement template rendering using `tera`:
  - Default template: Markdown changelog format (Keep a Changelog style)
  - Template variables: version, date, groups, commits, contributors
  - Support custom user templates via config
  - Include commit hash links (configurable remote URL)
  - Include author attribution (optional)
- [ ] Implement output targets:
  - `--output stdout` — print to terminal (default)
  - `--output file` — write/append to CHANGELOG.md
  - `--output clipboard` — copy to system clipboard
  - When writing to file: prepend to existing CHANGELOG.md (not overwrite)
- [ ] Wire up `reap` subcommand:
  - `shinigami reap` — generate changelog from latest tag to HEAD
  - `shinigami reap --from v1.0.0 --to v2.0.0` — specific range
  - `shinigami reap --unreleased` — show changes since last tag
  - `--format` flag: `markdown`, `plain`, `json`
- [ ] Add `--include-merge` flag to include/exclude merge commits
- [ ] Add `--contributors` flag to list unique contributors
- [ ] Write unit tests for commit parsing and grouping
- [ ] Write tests for template rendering with sample data
- [ ] Write integration tests with a real git repo (temp dir)

## Acceptance Criteria
- Changelog correctly groups commits by conventional commit type
- Breaking changes are prominently displayed
- Default Markdown template produces clean, readable output
- Custom templates render correctly with all variables
- CHANGELOG.md prepending works without losing existing content
- Clipboard output works on macOS and Linux
- Tests cover parsing, grouping, rendering, and output

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (git interface for log reading and tag listing)
- Step 003 (semantic analyzer for commit classification)
