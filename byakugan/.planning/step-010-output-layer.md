# Step 010: Output Layer

## Objective

Implement the output layer that presents review findings in multiple formats: rich terminal summary using `nakama-ui`, inline comments posted to the platform, markdown report for export, and JSON output for inter-process communication with other nakama tools.

## Tasks

- [ ] Implement terminal summary output (via nakama-ui):
  - Display review header: PR title, author, platform, branch info
  - Show findings grouped by severity with color coding:
    - Critical: red bold
    - Error: red
    - Warning: yellow
    - Info: blue
    - Suggestion: dim/gray
  - For each finding: severity badge, file:line, title, description, suggestion
  - Show code snippets with syntax highlighting for each finding
  - Display summary statistics: total findings, breakdown by severity, breakdown by pass
  - Show overall verdict with visual indicator (checkmark/cross)
  - Use nakama-ui tables for structured data display
  - Use nakama-ui panels for grouped findings
  - Paginate output for large reviews (or use a pager)
- [ ] Implement inline comment posting:
  - Map each `ReviewFinding` to a platform-specific inline comment
  - Format comment body with severity badge, description, and suggestion
  - Post comments via the platform adapter's `post_comment()` method
  - Post the review summary via `post_review()` method
  - Respect the `max_comments` configuration to avoid flooding PRs
  - Prioritize comments by severity when capping
- [ ] Implement markdown report output:
  - Generate a well-structured markdown document with:
    - Review metadata (PR, platform, date, reviewer)
    - Executive summary
    - Findings table (sortable by severity)
    - Detailed findings with code snippets
    - Statistics section
  - Write to file (`--output=report.md`) or stdout
  - Support Slack-compatible markdown variant
- [ ] Implement JSON output for IPC:
  - Serialize `ReviewResult` to JSON with all findings
  - Include metadata (PR info, platform, timestamp, review config)
  - Support `--format=json` flag
  - Design schema to be consumable by mugen (test generation) and tensai (daily brief)
  - Support streaming JSON (one finding per line) for large reviews
- [ ] Implement output format selection:
  - `--format=terminal` (default): rich terminal output
  - `--format=json`: structured JSON
  - `--format=markdown`: markdown report
  - `--post`: post inline comments to platform (combinable with other formats)
  - Auto-detect: if stdout is not a TTY, default to JSON
- [ ] Write unit tests for each output format
- [ ] Write visual regression tests for terminal output (snapshot testing)

## Acceptance Criteria

- Terminal output is readable, color-coded, and well-structured
- Inline comments are correctly posted to the platform with proper formatting
- Markdown report is well-formatted and contains all review information
- JSON output is valid and contains all review data in a documented schema
- `max_comments` cap is enforced with severity-based prioritization
- Output format selection works correctly via flags
- Non-TTY detection auto-selects JSON output

## Dependencies

- Step 007 (Review Engine) must be complete for ReviewResult data
- Step 003, 004, 005 (platform adapters) must be complete for posting comments
- `nakama-ui` shared crate for terminal output
- `serde_json` for JSON serialization
- `syntect` or similar for code syntax highlighting in terminal
