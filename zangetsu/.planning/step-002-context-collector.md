# Step 002: Context Collector

## Objective
Build a context collection module that gathers rich environmental information about the user's system, shell, working directory, installed tools, and git state. This context is passed to the LLM to produce accurate, platform-specific shell commands.

## Tasks
- [ ] Create `context.rs` module with a `ShellContext` struct containing all collected data
- [ ] OS detection: identify OS family (Linux, macOS, Windows/WSL), distro (if Linux), and architecture
- [ ] Shell detection: identify current shell (bash, zsh, fish, nushell, PowerShell) from `$SHELL` or parent process
- [ ] Shell version detection: parse `bash --version`, `zsh --version`, etc.
- [ ] CWD analysis: current directory path, directory contents summary (top-level files/dirs, count)
- [ ] Installed tools scan: check for common CLI tools (jq, curl, wget, ripgrep, fd, docker, kubectl, git, python, node, etc.)
- [ ] Git state collection (if in git repo): current branch, dirty/clean, last commit subject, remote URL
- [ ] Package manager detection: detect available package managers (brew, apt, dnf, pacman, cargo, npm, pip)
- [ ] Environment variable sampling: collect relevant env vars (PATH dirs count, EDITOR, LANG, TERM) without leaking secrets
- [ ] Implement `ShellContext::collect() -> Result<ShellContext>` as the main entry point
- [ ] Add serialization support (`serde::Serialize`) for passing context to LLM prompt
- [ ] Add caching: store collected context for duration of session (avoid re-scanning on every query)
- [ ] Handle errors gracefully: if any sub-collector fails, log warning and continue with partial context
- [ ] Add unit tests for each sub-collector with mock data

## Acceptance Criteria
- `ShellContext::collect()` returns a fully populated struct on macOS and Linux
- Context includes OS, shell, CWD, installed tools, git state (when applicable)
- Collection completes in under 500ms on a typical system
- Missing tools or information do not cause panics or errors
- Context serializes cleanly to a string suitable for LLM prompt injection
- Unit tests pass for individual collectors

## Dependencies
- Step 001 (CLI scaffold must exist to house this module)
