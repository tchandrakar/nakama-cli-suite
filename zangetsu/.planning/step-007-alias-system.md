# Step 007: Alias System

## Objective
Implement an alias system that allows users to save natural language query-to-command mappings for frequently used operations. Aliases can be executed by name, listed, renamed, and deleted, providing a personal command library.

## Tasks
- [ ] Create `alias.rs` module with `AliasStore` struct
- [ ] Define alias storage in SQLite (same database as history or separate):
  - `id` (INTEGER PRIMARY KEY)
  - `name` (TEXT UNIQUE) — user-defined alias name
  - `query` (TEXT) — original natural language query
  - `command` (TEXT) — the shell command to execute
  - `description` (TEXT, nullable) — optional user description
  - `risk_level` (TEXT) — risk level at time of creation
  - `use_count` (INTEGER) — number of times executed
  - `created_at` (TEXT) — ISO 8601 timestamp
  - `updated_at` (TEXT) — ISO 8601 timestamp
- [ ] Implement `AliasStore::save(name, query, command, risk_level)` — create or update alias
- [ ] Implement `AliasStore::get(name)` — retrieve alias by name
- [ ] Implement `AliasStore::list()` — list all aliases
- [ ] Implement `AliasStore::delete(name)` — remove an alias
- [ ] Implement `AliasStore::rename(old_name, new_name)` — rename an alias
- [ ] Implement `AliasStore::increment_use(name)` — bump use counter on execution
- [ ] Wire up `alias` subcommand with sub-subcommands:
  - `alias save <name>` — save last generated command as alias (or `--command` flag)
  - `alias run <name>` — execute an alias (goes through SafeExecutor)
  - `alias list` — show all aliases in a table
  - `alias show <name>` — show full alias details
  - `alias delete <name>` — delete an alias (with confirmation)
  - `alias rename <old> <new>` — rename an alias
- [ ] Add `--save-as <name>` flag to `ask` command for inline alias creation
- [ ] Add alias name auto-completion support (prepare completions data)
- [ ] Display alias table with columns: name, command (truncated), risk, use count
- [ ] Add `--json` output flag for programmatic access
- [ ] Write unit tests for all AliasStore methods
- [ ] Write integration test for save-list-run-delete cycle

## Acceptance Criteria
- Users can save a query/command pair as a named alias
- `alias run <name>` executes the stored command through the safety system
- `alias list` displays all aliases in a formatted table
- Alias names are unique; saving with existing name updates it
- Rename and delete operations work correctly
- Use count increments on each execution
- Tests cover all CRUD operations

## Dependencies
- Step 001 (CLI scaffold)
- Step 004 (execution engine for running aliases)
- Step 006 (history store for shared database infrastructure)
