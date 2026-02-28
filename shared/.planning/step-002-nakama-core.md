# Step 002: Build nakama-core

## Objective
Implement the foundational crate that all others depend on.

## Tasks
- Configuration loader: TOML parsing, merge hierarchy (global → tool → CLI flags)
- NakamaError enum with error context chain (thiserror)
- Common types: TraceContext, ToolManifest, Provider enum, ModelTier enum
- File permission utilities: set_secure_permissions(path, mode), verify_permissions(path)
- Path resolution: ~/.nakama/ directory structure creation and validation
- Version constants and build info

## Acceptance Criteria
- Config::load("tool_name") loads and merges config correctly
- Errors propagate with full context chain
- TraceContext generates unique IDs and propagates
- File permissions set to 0700/0600 correctly on macOS and Linux
- ~/.nakama/ directory created with correct permissions on first run

## Dependencies
- Step 001 (workspace exists)
