# Step 008: Git Hooks

## Objective
Implement a git hook management system that can install a `prepare-commit-msg` hook to auto-generate commit messages, a `commit-msg` validation hook to enforce message conventions, and provide configuration for hook behavior.

## Tasks
- [ ] Create `hooks.rs` module with `HookManager` struct
- [ ] Implement `prepare-commit-msg` hook:
  - Generate hook script that invokes `shinigami commit --hook-mode`
  - Hook script detects if running interactively vs automated
  - In hook mode: generate message, write to commit message file, allow git to proceed
  - Handle merge commits (skip generation, keep default message)
  - Handle amend commits (regenerate or keep existing)
  - Pass through if message already provided via `-m` flag
- [ ] Implement `commit-msg` validation hook:
  - Validate commit message against configured style (conventional, gitmoji, etc.)
  - Check subject line length (max 72 chars)
  - Check for imperative mood (basic heuristic)
  - Check for blank line between subject and body
  - Report validation errors with suggestions
  - `--strict` mode: reject invalid messages; `--warn` mode: warn but allow
- [ ] Implement hook installer:
  - `shinigami hook install prepare-commit-msg` — install generation hook
  - `shinigami hook install commit-msg` — install validation hook
  - `shinigami hook install all` — install both hooks
  - Backup existing hooks before overwriting
  - Set correct permissions (chmod +x)
- [ ] Implement hook management:
  - `shinigami hook list` — show installed shinigami hooks
  - `shinigami hook remove <type>` — remove a hook (restore backup if exists)
  - `shinigami hook status` — show hook configuration and status
- [ ] Hook configuration:
  - Configure via `.shinigami.toml` in repo root or global config
  - Options: style, max-subject-length, require-body, require-scope, strict mode
  - Per-repo overrides for global settings
- [ ] Handle hook environment:
  - Detect PATH for shinigami binary in hook scripts
  - Handle virtual environments and shell differences
  - Graceful fallback if shinigami binary not found (proceed without generation)
- [ ] Write unit tests for hook script generation
- [ ] Write tests for commit message validation
- [ ] Write integration tests for hook installation and removal

## Acceptance Criteria
- `prepare-commit-msg` hook auto-generates messages on `git commit`
- `commit-msg` hook validates messages against configured conventions
- Hook installer backs up existing hooks and sets correct permissions
- Hooks work correctly in interactive and non-interactive contexts
- Hooks gracefully degrade if shinigami binary is not available
- Tests cover installation, validation, and edge cases

## Dependencies
- Step 001 (CLI scaffold)
- Step 004 (commit generator for hook-mode generation)
