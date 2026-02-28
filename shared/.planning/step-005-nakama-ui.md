# Step 005: Build nakama-ui (Claude-Style Terminal Output)

## Objective
Implement beautiful, consistent terminal output matching Claude Code's visual style.

## Tasks
- NakamaUI struct with verbosity (quiet/normal/verbose/debug), color (auto/always/never), format (human/json/plain)
- Step indicators: step_start (spinner), step_done (checkmark), step_fail (cross)
- Spinners: braille-style (⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏) via indicatif
- Progress bars: ████████░░░░ 67% style
- Severity badges: info (dim), success (green), warn (yellow), error (red), debug (gray)
- Code blocks with syntax highlighting via syntect
- Table rendering via comfy-table (aligned, bordered)
- Diff view: red/green highlighted
- Panel rendering (boxed content with title)
- Confirmation prompt (Y/n) and selection prompt
- Color palette constants (brand purple, success green, warning amber, error red, info slate)
- TTY detection: disable rich output when piped (no colors, no spinners, clean parseable output)
- --format=json mode: structured JSON output for machine consumption
- Unit tests: output formatting, TTY detection behavior, color mode switching

## Acceptance Criteria
- All output consistent across all 11 tools
- Spinners animate smoothly during long operations
- Piped output is clean and parseable
- Code blocks are syntax-highlighted for common languages
- Tables render with proper alignment
- Looks and feels like Claude Code terminal output

## Dependencies
- Step 002 (nakama-core)
