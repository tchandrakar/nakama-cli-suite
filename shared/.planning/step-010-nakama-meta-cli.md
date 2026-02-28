# Step 010: Build nakama Meta CLI

## Objective
Create the `nakama` meta command that provides suite-wide utilities.

## Tasks
- `nakama auth` — credential management (delegates to nakama-vault)
- `nakama logs` — log viewer (delegates to nakama-log)
- `nakama audit` — audit viewer and verifier (delegates to nakama-audit)
- `nakama usage` — AI usage dashboard with cost tracking
- `nakama config` — configuration editor and validator
- `nakama status` — show installed tools, versions, health
- `nakama update` — update tools (check for new versions)

## Acceptance Criteria
- All subcommands work as documented
- Beautiful Claude-style output via nakama-ui
- Configuration validated on edit

## Dependencies
- Steps 002-009 (all shared crates complete)
