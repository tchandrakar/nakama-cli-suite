# Step 012: Watch Mode

## Objective

Build a file watcher that monitors source file changes, automatically generates tests for modified functions (with debouncing to avoid rapid re-generation), and continuously validates that generated tests pass.

## Tasks

- [ ] Add notify crate for file system watching
- [ ] Implement file watcher:
  - [ ] Watch source directories (configurable, auto-detect from project structure)
  - [ ] Exclude test files, build artifacts, node_modules, target/
  - [ ] Configurable file extensions per language
  - [ ] Recursive directory watching
- [ ] Implement debouncing:
  - [ ] Debounce rapid file changes (configurable, default 500ms)
  - [ ] Batch changes within debounce window
  - [ ] Deduplicate: multiple saves of same file trigger one generation
- [ ] Implement change detection:
  - [ ] Detect which functions changed (AST diff between old and new version)
  - [ ] Detect new functions (added since last scan)
  - [ ] Detect deleted functions (remove orphaned tests? flag only?)
  - [ ] Detect signature changes (regenerate affected tests)
- [ ] Implement auto-generation pipeline:
  - [ ] Analyze changed functions
  - [ ] Select strategy per changed function
  - [ ] Generate tests via LLM
  - [ ] Validate generated tests
  - [ ] Write passing tests to disk
  - [ ] Run existing tests to verify no regressions
- [ ] Implement continuous validation:
  - [ ] After writing new tests, run full relevant test suite
  - [ ] Detect if generated tests break existing tests
  - [ ] Rollback generated tests if they cause regressions
  - [ ] Report test generation results in real-time
- [ ] Implement watch mode UI:
  - [ ] Terminal output showing watched files, changes, generation status
  - [ ] Spinner during generation/validation
  - [ ] Color-coded results (green=pass, red=fail, yellow=skipped)
  - [ ] Summary of tests generated in current session
- [ ] Implement `watch` subcommand:
  - [ ] `mugen watch` — start watching current directory
  - [ ] `--path` flag: specify directory to watch
  - [ ] `--strategy` flag: override strategy for all generations
  - [ ] `--no-validate` flag: skip execution validation
  - [ ] `--exclude` flag: additional exclude patterns
  - [ ] Ctrl-C: graceful shutdown with session summary
- [ ] Implement session persistence:
  - [ ] Track what was generated during the session
  - [ ] Session summary on exit (files changed, tests generated, tests passed)
  - [ ] Log file for session history
- [ ] Unit tests for debouncing logic
- [ ] Unit tests for change detection (AST diff)
- [ ] Integration test: modify file, verify test generation triggers

## Acceptance Criteria

- File changes trigger test generation within 1 second (after debounce)
- Only changed functions get new tests (not entire file)
- Debouncing prevents duplicate generation on rapid saves
- Generated tests pass validation before writing
- Existing tests are not broken by generated tests
- Regression detection rolls back problematic tests
- Session summary accurately reflects all generation activity
- Watch mode handles file deletions and renames gracefully
- Graceful shutdown saves state and prints summary

## Dependencies

- Step 002 (Code analyzer) for AST diffing
- Step 004 (Test strategy) for strategy selection
- Step 005 (LLM test generator) for test generation
- Step 006 (Validation loop) for test validation
- notify crate for file system events
