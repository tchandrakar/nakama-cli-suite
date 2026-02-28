# Step 006: History Store

## Objective
Implement a persistent SQLite-backed history database that stores every query, generated command, risk assessment, execution result, and timestamp. Provide search, filtering, and a browsable TUI for reviewing past interactions.

## Tasks
- [ ] Create `history.rs` module with `HistoryStore` struct
- [ ] Set up SQLite database using `rusqlite` (or `sqlx` with SQLite):
  - Database location: `~/.config/zangetsu/history.db` (XDG-aware)
  - Auto-create database and tables on first use
  - Run migrations for schema changes
- [ ] Define history table schema:
  - `id` (INTEGER PRIMARY KEY)
  - `query` (TEXT) — original natural language query
  - `command` (TEXT) — generated shell command
  - `explanation` (TEXT) — LLM-provided explanation
  - `risk_level` (TEXT) — low/medium/high
  - `executed` (BOOLEAN) — whether the command was executed
  - `exit_code` (INTEGER, nullable) — execution exit code
  - `stdout_preview` (TEXT, nullable) — first 1000 chars of stdout
  - `stderr_preview` (TEXT, nullable) — first 1000 chars of stderr
  - `duration_ms` (INTEGER, nullable) — execution duration
  - `timestamp` (TEXT) — ISO 8601 timestamp
  - `cwd` (TEXT) — working directory at time of query
  - `tags` (TEXT, nullable) — comma-separated user tags
- [ ] Implement `HistoryStore::record()` — insert a new history entry
- [ ] Implement `HistoryStore::search(query: &str)` — full-text search across query and command fields
- [ ] Implement `HistoryStore::list(limit, offset, filter)` — paginated listing with filters (date range, risk level, executed status)
- [ ] Implement `HistoryStore::get(id)` — retrieve a single entry by ID
- [ ] Implement `HistoryStore::delete(id)` — delete a single entry
- [ ] Implement `HistoryStore::clear()` — delete all entries (with confirmation)
- [ ] Wire up `history` subcommand with sub-subcommands:
  - `history list` — show recent entries (default 20)
  - `history search <query>` — search entries
  - `history show <id>` — show full details of an entry
  - `history delete <id>` — delete an entry
  - `history clear` — clear all history
  - `history tui` — open browsable TUI
- [ ] Build TUI view using `ratatui`:
  - Scrollable list of history entries
  - Detail pane showing full entry
  - Search/filter bar
  - Key bindings: j/k scroll, Enter to view, / to search, q to quit
- [ ] Add `--json` output flag for programmatic access
- [ ] Write unit tests for all HistoryStore methods
- [ ] Write integration test for full record-search-retrieve cycle

## Acceptance Criteria
- History entries persist across sessions in SQLite database
- Search returns relevant results across query and command text
- TUI displays history in a browsable, searchable interface
- `history list` shows entries with formatted timestamps and risk colors
- Database auto-creates on first use without user intervention
- All CRUD operations work correctly with tests passing

## Dependencies
- Step 001 (CLI scaffold for subcommand wiring)
- Step 003 (translation engine produces entries to store)
- Step 004 (execution results to record)
