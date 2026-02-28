# Step 008: Build nakama-ipc (Inter-Tool Communication)

## Objective
Implement the Nakama Message Protocol (NMP) for seamless tool-to-tool communication.

## Tasks
- NMP message envelope types (version, trace_id, source, timestamp, schema, data)
- Schema registry: load JSON Schema files from shared/schemas/
- Schema validation: validate incoming NMP messages against registered schemas
- Pipe I/O helpers: read_nmp_stdin(), write_nmp_stdout()
- Trace context propagation: extract trace_id from incoming NMP, inject into outgoing
- Error message propagation: structured error schema for pipeline failures
- Tool discovery: read/write ~/.nakama/tools.toml manifest
- Event bus: file-based event log at ~/.nakama/events/ for async/daemon communication
- Initial schemas: shell-command, git-diff, commit-message, review-findings, search-results, jira-issues, confluence-pages, error
- Unit tests: serialization round-trip, schema validation, trace propagation, pipe I/O

## Acceptance Criteria
- `tool-a --format=json | tool-b --input=stdin` works with NMP envelope
- Trace IDs flow across piped tools
- Schema validation catches malformed messages
- Tool discovery lists installed tools and their capabilities
- Error messages propagate gracefully through pipelines

## Dependencies
- Step 002 (nakama-core), Step 004 (nakama-log)
