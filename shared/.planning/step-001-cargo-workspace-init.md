# Step 001: Initialize Cargo Workspace

## Objective
Set up the root Cargo workspace with all shared crates and tool binaries.

## Tasks
- Create root Cargo.toml with workspace members
- Create shared crate stubs: nakama-core, nakama-vault, nakama-ui, nakama-ai, nakama-ipc, nakama-audit, nakama-log, nakama-sdk
- Create binary crate stubs for all 11 tools
- Set up common dependencies in workspace Cargo.toml
- Set up CI/CD: GitHub Actions for build, test, clippy, cargo-audit

## Acceptance Criteria
- `cargo build --workspace` compiles successfully
- `cargo test --workspace` runs (even if no tests yet)
- `cargo clippy --workspace` passes
- CI pipeline runs on push

## Dependencies
- None (first step)
