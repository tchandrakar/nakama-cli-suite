# Shared Logging Architecture — Nakama CLI Suite

> Beautiful, informative terminal output. Every tool looks and feels like Claude Code.

## Design Philosophy

Logging in Nakama serves two distinct purposes:
1. **User-facing output** — Beautiful, Claude-style terminal UX (what the user sees)
2. **Structured logs** — Machine-readable, append-only logs for debugging and auditing

These are separate systems. User output is never just "logging to stdout."

---

## 1. User-Facing Output (`nakama-ui`)

### Visual Language

Every Nakama tool shares the same visual language, so switching between tools feels seamless.

```
┌─────────────────────────────────────────────────────────┐
│                    Output Components                     │
│                                                          │
│  ╭─ Step Indicators ──────────────────────────────╮      │
│  │                                                │      │
│  │  ● Completed step                              │      │
│  │  ◐ In-progress step (with spinner)             │      │
│  │  ○ Pending step                                │      │
│  │  ✕ Failed step                                 │      │
│  │                                                │      │
│  ╰────────────────────────────────────────────────╯      │
│                                                          │
│  ╭─ Severity Badges ─────────────────────────────╮       │
│  │                                                │      │
│  │  INFO     dim white text                       │      │
│  │  SUCCESS  green with checkmark                 │      │
│  │  WARN     yellow with warning sign             │      │
│  │  ERROR    red with cross                       │      │
│  │  DEBUG    gray (only in verbose mode)          │      │
│  │                                                │      │
│  ╰────────────────────────────────────────────────╯      │
│                                                          │
│  ╭─ Progress Elements ───────────────────────────╮       │
│  │                                                │      │
│  │  Spinners: braille-style (⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏)      │      │
│  │  Progress bars: ████████░░░░ 67%               │      │
│  │  Timers: elapsed time on long operations       │      │
│  │                                                │      │
│  ╰────────────────────────────────────────────────╯      │
│                                                          │
│  ╭─ Content Panels ──────────────────────────────╮       │
│  │                                                │      │
│  │  Code blocks: syntax highlighted               │      │
│  │  Tables: aligned, bordered                     │      │
│  │  Diffs: red/green highlighted                  │      │
│  │  Tree views: for hierarchical data             │      │
│  │  Collapsible sections: for verbose output      │      │
│  │                                                │      │
│  ╰────────────────────────────────────────────────╯      │
└─────────────────────────────────────────────────────────┘
```

### Example Output (Claude-Style)

```
$ zangetsu run "find large files over 100mb"

  ◐ Analyzing request...
  ● Detected context: macOS, zsh, git repository
  ● Generated command:

    find . -type f -size +100M -exec ls -lh {} \;

  ◐ Executing... (timeout: 30s)

  ● Results:
    -rw-r--r--  1 user  staff   142M  Feb 28 10:15  ./data/dump.sql
    -rw-r--r--  1 user  staff   203M  Feb 27 09:42  ./assets/video.mp4

  ● Done in 1.2s — 2 files found

$ shinigami commit

  ◐ Analyzing staged changes...
  ● 3 files changed (+42, -18)

  ┌─ Commit Message ──────────────────────────────────┐
  │ feat(auth): add OAuth2 PKCE flow for Google login │
  │                                                    │
  │ Implement the authorization code flow with PKCE    │
  │ for secure Google OAuth2 authentication. Adds      │
  │ token refresh and secure storage via OS keychain.  │
  └────────────────────────────────────────────────────┘

  ? Accept this commit message? [Y/n/e(dit)]
```

### UI API

```rust
pub struct NakamaUI {
    verbosity: Verbosity,     // quiet | normal | verbose | debug
    color: ColorMode,         // auto | always | never
    format: OutputFormat,     // human | json | plain
}

impl NakamaUI {
    /// Step with spinner (in-progress)
    fn step_start(&self, message: &str) -> SpinnerHandle;

    /// Complete a step (checkmark)
    fn step_done(&self, message: &str);

    /// Fail a step (cross)
    fn step_fail(&self, message: &str);

    /// Warning
    fn warn(&self, message: &str);

    /// Error with context
    fn error(&self, message: &str, context: Option<&str>);

    /// Info (dim)
    fn info(&self, message: &str);

    /// Code block with syntax highlighting
    fn code_block(&self, code: &str, language: &str);

    /// Table
    fn table(&self, headers: &[&str], rows: &[Vec<String>]);

    /// Diff view
    fn diff(&self, old: &str, new: &str);

    /// Progress bar
    fn progress_bar(&self, total: u64) -> ProgressHandle;

    /// Confirmation prompt
    fn confirm(&self, message: &str) -> Result<bool>;

    /// Selection prompt
    fn select(&self, message: &str, options: &[&str]) -> Result<usize>;

    /// Panel (boxed content)
    fn panel(&self, title: &str, content: &str);
}
```

### Color Palette

```
Primary:    #7C6FE0  (purple — brand color, used for headers/emphasis)
Success:    #4ADE80  (green — completions, confirmations)
Warning:    #FBBF24  (amber — warnings, caution)
Error:      #F87171  (red — errors, failures)
Info:       #94A3B8  (slate — informational, dimmed)
Code:       #E2E8F0  (light gray — code blocks)
Background: terminal default (respects user theme)
```

### Pipe Detection
When stdout is not a TTY (piped to another command):
- No colors, no spinners, no interactive prompts
- Clean, parseable output
- `--format=json` for structured machine output

---

## 2. Structured Logging (`nakama-log`)

### Log Format

All tools write structured logs in JSON Lines format:

```json
{
  "timestamp": "2026-02-28T14:23:05.123Z",
  "level": "info",
  "tool": "shinigami",
  "event": "commit.generated",
  "message": "Generated commit message for 3 staged files",
  "context": {
    "files_changed": 3,
    "insertions": 42,
    "deletions": 18,
    "commit_type": "feat"
  },
  "duration_ms": 1250,
  "trace_id": "abc123def456"
}
```

### Log Levels

| Level | Purpose | Example |
|-------|---------|---------|
| `error` | Operation failed, user action needed | API call failed, auth expired |
| `warn` | Something unexpected but recoverable | Rate limit approaching, slow response |
| `info` | Normal operations, key milestones | Command executed, file indexed |
| `debug` | Detailed internals for troubleshooting | HTTP request/response details, cache hits |
| `trace` | Ultra-verbose, per-line processing | Each log line parsed, each AST node visited |

### Log Storage

```
~/.nakama/logs/
├── nakama.log              # Combined log (all tools)
├── zangetsu.log            # Per-tool logs
├── shinigami.log
├── jogan.log
├── senku.log
├── sharingan.log
├── tensai.log
├── mugen.log
├── gate.log
├── byakugan.log
├── kami.log
└── itachi.log
```

### Rotation Policy
- Max file size: 10 MB per log file
- Max files: 5 rotated files per tool (total ~50 MB per tool)
- Rotation: rename with `.1`, `.2`, etc. suffix
- Compression: gzip rotated files

### Log Viewer

```bash
# View recent logs for a tool
nakama logs shinigami

# Filter by level
nakama logs --level=error

# Filter by time range
nakama logs --since="1 hour ago"

# Follow mode (like tail -f)
nakama logs --follow zangetsu

# Search across all tools
nakama logs --grep="timeout"

# JSON output for piping
nakama logs --format=json | jq '.event'
```

---

## 3. Trace Context

Every operation gets a `trace_id` that flows across tool boundaries:

```
User runs: itachi standup | tensai ingest

itachi generates trace_id: "tr_abc123"
  → all itachi log entries tagged with trace_id
  → passes trace_id to tensai via JSON pipe header
tensai logs entries with same trace_id: "tr_abc123"
  → full operation is traceable end-to-end
```

This enables cross-tool debugging: "show me everything that happened in this pipeline."

---

## 4. Configuration

Global logging config in `~/.nakama/config.toml`:

```toml
[logging]
level = "info"                    # error | warn | info | debug | trace
format = "json"                   # json | pretty (for human-readable)
directory = "~/.nakama/logs"
max_file_size_mb = 10
max_rotated_files = 5
compress_rotated = true

[ui]
color = "auto"                    # auto | always | never
verbosity = "normal"              # quiet | normal | verbose | debug
spinners = true
unicode = true                    # false for basic ASCII
```

Per-tool override in tool config:
```toml
[logging]
level = "debug"    # override for this tool only
```

## Tech Stack

- **Terminal UI:** crossterm (cross-platform terminal control)
- **Colors:** owo-colors (compile-time color formatting)
- **Spinners/Progress:** indicatif
- **Tables:** comfy-table
- **Syntax highlighting:** syntect
- **Structured logging:** tracing + tracing-subscriber (JSON formatter)
- **Log rotation:** tracing-appender (rolling file appender)
