# Step 004: LLM Commit Message Generator

## Objective
Build the core commit message generation pipeline that takes a structured diff analysis, sends it to an LLM via `nakama-ai`, and produces a well-formatted commit message supporting multiple styles (conventional commits, gitmoji, freeform, custom templates).

## Tasks
- [ ] Design the system prompt for commit message generation:
  - Include diff hunks, file list, semantic analysis results
  - Instruct structured output: subject line, body, footer
  - Rules: imperative mood, max 72 chars subject, wrap body at 72 chars
  - Include repository style conventions (detected from recent commits)
- [ ] Create `CommitStyle` enum: `Conventional`, `Gitmoji`, `Freeform`, `Custom(String)`
- [ ] Implement style-specific prompt variants:
  - Conventional: `type(scope): subject` format
  - Gitmoji: emoji prefix matching change type
  - Freeform: natural prose subject line
  - Custom: user-provided template with placeholders
- [ ] Create `CommitMessage` struct: subject, body (optional), footer (optional), breaking change note
- [ ] Implement `generate_commit_message(diff, analysis, style) -> Result<CommitMessage>` via `nakama-ai`
- [ ] Subject line enforcement:
  - Max 72 characters (configurable)
  - Imperative mood (detect and warn if not)
  - No trailing period
  - Capitalize first word (after prefix)
- [ ] Body generation:
  - Explain the "why" not the "what"
  - List significant changes
  - Reference breaking changes
  - Wrap at 72 characters
- [ ] Footer generation:
  - `BREAKING CHANGE:` footer when applicable
  - `Refs:` for issue references (if detectable from branch name)
  - Co-author detection from git config
- [ ] Implement the `commit` subcommand handler:
  - Check for staged changes, abort if none
  - Collect diff and run analysis
  - Generate commit message
  - Display generated message with preview
  - Prompt user: accept, edit, regenerate, or cancel
  - Execute `git commit` with accepted message
- [ ] Add `--style` flag to override default style
- [ ] Add `--amend` flag to amend the last commit with regenerated message
- [ ] Add `--no-body` flag to generate subject-only messages
- [ ] Add message regeneration: allow user to request a new version
- [ ] Write unit tests with mock LLM responses for each style
- [ ] Write tests for subject line enforcement rules

## Acceptance Criteria
- Generated commit messages follow the selected style format
- Subject lines never exceed 72 characters
- Conventional commits include correct type and scope
- Gitmoji messages include appropriate emoji
- User can preview, accept, edit, or regenerate the message
- `git commit` executes successfully with the generated message
- Tests cover all styles and enforcement rules

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (git interface for staged diff)
- Step 003 (semantic analysis for type/scope/breaking change)
