# Step 012: Response Diff

## Objective

Build a response comparison engine that compares two API responses structurally, detects new and removed fields, identifies type changes, highlights value differences, and presents the diff in a clear, visual format.

## Tasks

- [ ] Define `ResponseDiff` struct: changes, added_fields, removed_fields, type_changes, value_changes, summary
- [ ] Define `DiffChange` struct: path (JSONPath), change_type, old_value, new_value, severity
- [ ] Define `DiffChangeType` enum: Added, Removed, TypeChanged, ValueChanged, OrderChanged
- [ ] Implement structural diff for JSON:
  - [ ] Deep comparison of JSON objects
  - [ ] Detect new fields (present in B, absent in A)
  - [ ] Detect removed fields (present in A, absent in B)
  - [ ] Detect type changes (string -> number, object -> array)
  - [ ] Detect value changes (same type, different value)
  - [ ] Handle array comparison (element-by-element, detect reordering)
  - [ ] Nested object comparison (recursive diff)
  - [ ] Report changes using JSONPath notation
- [ ] Implement structural diff for XML:
  - [ ] Element comparison (tag, attributes, text)
  - [ ] Detect added/removed elements
  - [ ] Attribute changes
  - [ ] Text content changes
- [ ] Implement header diff:
  - [ ] Compare response headers between two responses
  - [ ] Detect added/removed headers
  - [ ] Detect changed header values
  - [ ] Exclude volatile headers (Date, X-Request-Id) by default
- [ ] Implement status code diff:
  - [ ] Highlight status code changes
  - [ ] Classify change severity (2xx->4xx = breaking, 200->201 = minor)
- [ ] Implement timing diff:
  - [ ] Compare response times
  - [ ] Highlight significant performance changes (>2x slower/faster)
- [ ] Implement diff visualization:
  - [ ] Side-by-side view (terminal permitting)
  - [ ] Unified diff view (like git diff)
  - [ ] Color coding: green=added, red=removed, yellow=changed
  - [ ] Path annotations for nested changes
  - [ ] Summary statistics (N added, N removed, N changed)
- [ ] Implement diff sources:
  - [ ] Compare two history entries: `gate diff 42 43`
  - [ ] Compare current response with history: `gate diff --with-last`
  - [ ] Compare response with saved baseline: `gate diff --baseline baseline.json`
  - [ ] Compare two URLs: `gate diff GET /api/v1/users GET /api/v2/users`
  - [ ] Compare environments: `gate diff --env dev --env staging GET /api/users`
- [ ] Implement `diff` subcommand:
  - [ ] `gate diff <id1> <id2>` — compare two history entries
  - [ ] `gate diff --with-last` — compare latest with previous
  - [ ] `gate diff --baseline <file>` — compare with saved baseline
  - [ ] `--ignore-fields` flag: exclude specific fields from comparison
  - [ ] `--ignore-order` flag: ignore array element ordering
  - [ ] `--format` flag: side-by-side, unified, json
- [ ] Implement diff export:
  - [ ] Export diff as JSON (machine-readable)
  - [ ] Export diff as markdown (for documentation)
  - [ ] Export diff as HTML (for sharing)
- [ ] Unit tests for JSON structural diff
- [ ] Unit tests for XML diff
- [ ] Unit tests for header diff
- [ ] Integration test: diff two responses from mock server

## Acceptance Criteria

- JSON diff detects all structural and value changes
- New and removed fields are clearly identified with JSONPath
- Type changes are detected and reported with old/new types
- Array comparison handles both ordered and unordered comparisons
- Color-coded visualization clearly shows changes
- Side-by-side and unified views work correctly
- Diff across environments highlights API inconsistencies
- `--ignore-fields` correctly excludes specified fields
- Volatile headers are excluded by default
- Diff export produces valid JSON/markdown output

## Dependencies

- Step 004 (Response analyzer) for response parsing
- Step 006 (Request history) for history-based comparison
- Step 007 (Environment manager) for environment-based comparison
- serde_json for JSON comparison
- similar or diffy crate for text diffing algorithms
