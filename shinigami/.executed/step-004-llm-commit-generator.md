# Step 004: LLM Commit Message Generator

## Status: NOT STARTED

| Task | Status | Date | Notes |
|------|--------|------|-------|
| Design system prompt for commit message generation | - | - | - |
| Create CommitStyle enum (Conventional, Gitmoji, Freeform, Custom) | - | - | - |
| Implement style-specific prompt variants | - | - | - |
| Create CommitMessage struct (subject, body, footer) | - | - | - |
| Implement generate_commit_message() via nakama-ai | - | - | - |
| Subject line enforcement (72 chars, imperative, no period) | - | - | - |
| Body generation (why not what, wrap at 72 chars) | - | - | - |
| Footer generation (BREAKING CHANGE, Refs, co-authors) | - | - | - |
| Implement commit subcommand handler (preview, accept, edit, regenerate) | - | - | - |
| Add --style, --amend, --no-body flags | - | - | - |
| Message regeneration support | - | - | - |
| Unit tests for each style with mock LLM | - | - | - |
| Tests for subject line enforcement rules | - | - | - |

Status legend: `-` Not started | `WIP` In progress | `DONE` Complete | `BLOCKED` Blocked
