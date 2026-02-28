# Step 009: Branch Naming

## Objective
Build a branch name generator that converts natural language descriptions into well-formatted branch names following configurable naming patterns, with enforcement of maximum length and character restrictions.

## Tasks
- [ ] Create `branch.rs` module (or extend existing) with `BranchNamer` struct
- [ ] Implement NL description to branch name conversion:
  - Send description to LLM via `nakama-ai` for concise keyword extraction
  - Alternative: rule-based extraction (remove stop words, slugify remaining)
  - Produce a clean, lowercase, hyphen-separated name
- [ ] Implement configurable naming patterns:
  - Default: `type/short-description` (e.g., `feat/add-user-auth`)
  - With ticket: `type/TICKET-123-short-description`
  - Flat: `short-description`
  - Custom: user-defined template with placeholders (`{type}`, `{scope}`, `{description}`, `{ticket}`)
- [ ] Implement branch type detection from description:
  - "add", "new", "implement" -> `feat`
  - "fix", "bug", "resolve" -> `fix`
  - "refactor", "clean", "restructure" -> `refactor`
  - "docs", "readme", "documentation" -> `docs`
  - "test", "spec", "coverage" -> `test`
  - Allow explicit type via `--type` flag
- [ ] Max length enforcement:
  - Default max: 50 characters (configurable)
  - Truncate description intelligently (at word boundaries)
  - Warn if truncation occurs
- [ ] Character sanitization:
  - Replace spaces with hyphens
  - Remove special characters (keep alphanumeric, hyphens, slashes)
  - Collapse multiple hyphens
  - No leading/trailing hyphens
  - Lowercase everything
- [ ] Wire up `branch` subcommand:
  - `shinigami branch "add user authentication"` -> creates `feat/add-user-auth`
  - `--ticket PROJ-123` flag for ticket number inclusion
  - `--type fix` flag to override auto-detected type
  - `--pattern` flag to override naming pattern
  - `--create` flag to actually create the branch (default: just display)
  - `--checkout` flag to create and switch to the branch
- [ ] Add `--suggest` flag to show multiple name options
- [ ] Write unit tests for name generation, sanitization, and truncation
- [ ] Write integration tests for branch creation in temp repo

## Acceptance Criteria
- Natural language descriptions produce clean, conventional branch names
- Naming patterns are configurable and correctly applied
- Branch names never exceed configured maximum length
- Special characters are properly sanitized
- `--create` and `--checkout` flags work with the git interface
- Tests cover generation, patterns, sanitization, and truncation

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (git interface for branch creation)
