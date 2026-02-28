# Step 004: Safety & Execution

## Objective
Implement a safe command execution engine with risk scoring, dangerous command detection, user confirmation flow, and sandboxed execution using `Command::new()` (never `sh -c`). Ensure no generated command runs without explicit user approval for medium/high risk operations.

## Tasks
- [ ] Create `execute.rs` module with `SafeExecutor` struct
- [ ] Implement risk scoring engine:
  - Parse command and arguments to assess risk independently of LLM assessment
  - Low risk: read-only commands (ls, cat, grep, find, wc, head, tail, df, du)
  - Medium risk: file modification (mv, cp, sed -i, chmod), network (curl POST, wget)
  - High risk: destructive commands (rm, mkfs, dd, kill -9), privilege escalation (sudo), system modification
- [ ] Build dangerous pattern blocklist:
  - `rm -rf /`, `rm -rf ~`, `rm -rf *` patterns
  - `:(){ :|:& };:` (fork bomb patterns)
  - `> /dev/sda`, `dd if=/dev/zero`
  - `chmod -R 777 /`, `chown -R` on system directories
  - Pipe to `sh`, `bash`, `eval` from untrusted sources
- [ ] Implement `Command::new()` execution (NEVER use `sh -c` or shell interpretation):
  - Parse command string into program + arguments
  - Handle pipes by chaining `Command` instances with `Stdio::piped()`
  - Handle redirections explicitly via `File::create()` / `File::open()`
- [ ] Add configurable timeout (default 30s, override via `--timeout`)
- [ ] Capture stdout and stderr separately, stream to terminal in real-time
- [ ] Implement user confirmation flow:
  - Low risk: execute immediately (configurable to always-confirm)
  - Medium risk: show command + explanation, prompt [Y/n]
  - High risk: show command + risk warning in red, prompt [y/N] (default no)
  - Blocked: refuse to execute, explain why
- [ ] Add `--yes` flag to skip confirmation (respects blocklist regardless)
- [ ] Add `--no-execute` flag to never execute (just display)
- [ ] Store execution result (exit code, stdout, stderr) for history
- [ ] Wire up the `run` subcommand to use `SafeExecutor`
- [ ] Add unit tests for risk scoring, blocklist detection, command parsing
- [ ] Add integration tests for simple command execution

## Acceptance Criteria
- Commands execute via `Command::new()`, never via shell invocation
- `rm -rf /` and other blocklisted patterns are always rejected
- Medium-risk commands require explicit user confirmation
- High-risk commands default to "no" and require explicit "yes"
- Timeout kills long-running commands and reports timeout error
- Stdout/stderr are captured and available for history storage
- All execution paths are covered by tests

## Dependencies
- Step 001 (CLI scaffold)
- Step 003 (translation engine produces commands to execute)
