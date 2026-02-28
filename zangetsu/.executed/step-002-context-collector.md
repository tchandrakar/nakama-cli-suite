# Step 002: Context Collector

## Status: NOT STARTED

| Task | Status | Date | Notes |
|------|--------|------|-------|
| Create context.rs module with ShellContext struct | - | - | - |
| OS detection (family, distro, architecture) | - | - | - |
| Shell detection from $SHELL or parent process | - | - | - |
| Shell version detection | - | - | - |
| CWD analysis (path, contents summary) | - | - | - |
| Installed tools scan (jq, curl, docker, kubectl, git, etc.) | - | - | - |
| Git state collection (branch, dirty/clean, last commit) | - | - | - |
| Package manager detection | - | - | - |
| Environment variable sampling (non-secret) | - | - | - |
| Implement ShellContext::collect() entry point | - | - | - |
| Add serde::Serialize for LLM prompt serialization | - | - | - |
| Add session-level caching | - | - | - |
| Graceful error handling for failed sub-collectors | - | - | - |
| Unit tests for each sub-collector | - | - | - |

Status legend: `-` Not started | `WIP` In progress | `DONE` Complete | `BLOCKED` Blocked
