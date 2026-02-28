# Step 007: Cross-Reference Engine

## Objective

Build the cross-reference engine that discovers and maintains links between Jira issues and Confluence pages through explicit links (Jira remote links), implicit links (PROJ-XXX patterns in Confluence content), and semantic link inference (LLM-based relationship detection), producing a navigable relationship graph.

## Tasks

- [ ] Implement explicit link discovery:
  - Fetch Jira remote links for issues (`GET /rest/api/3/issue/{key}/remotelink`)
  - Filter for links pointing to Confluence pages
  - Fetch Confluence page links that reference Jira issues
  - Map link types: "references", "implements", "documents", "relates to"
  - Build bidirectional link records
- [ ] Implement implicit link detection:
  - Scan Confluence page content for Jira issue key patterns (`[A-Z]+-\d+`)
  - Scan Jira issue descriptions and comments for Confluence page URLs
  - Validate detected keys/URLs actually exist (resolve against APIs)
  - Handle false positives (e.g., "HTTP-200" is not a Jira key)
  - Configurable project key allowlist for pattern matching
- [ ] Implement semantic link inference:
  - Use nakama-ai to identify conceptual relationships between:
    - Jira epic/story descriptions and Confluence design docs
    - Jira bug reports and Confluence runbooks/incident reports
    - Jira feature requests and Confluence RFC/ADR documents
  - Generate candidate relationships with confidence scores
  - Only surface high-confidence relationships (configurable threshold)
  - Semantic matching uses text similarity + LLM reasoning
- [ ] Build the relationship graph:
  - Define `Relationship` struct: source (Jira/Confluence), target (Jira/Confluence), type, confidence, discovered_via (explicit/implicit/semantic)
  - Store relationships in a local graph structure
  - Support queries: "what documents relate to PROJ-142?", "what tickets reference this page?"
  - Persist graph in `~/.itachi/cross_references.json`
  - Incremental updates (only process changed content since last scan)
- [ ] Implement graph query API:
  - `find_related_docs(issue_key) -> Vec<(Page, RelationshipType)>`
  - `find_related_issues(page_id) -> Vec<(Issue, RelationshipType)>`
  - `find_gaps() -> GapReport` -- issues without docs, docs without tickets
  - `get_graph_stats() -> GraphStats` -- total links, by type, coverage
- [ ] Implement `itachi xref` subcommand:
  - `itachi xref scan` -- full scan to build/rebuild the relationship graph
  - `itachi xref show <key>` -- show all relationships for an issue/page
  - `itachi xref gaps` -- report gaps (undocumented work, orphaned docs)
  - `itachi xref stats` -- display graph statistics
- [ ] Write unit tests for link pattern matching and graph queries
- [ ] Write integration tests for cross-reference discovery

## Acceptance Criteria

- Explicit links between Jira and Confluence are discovered correctly
- Implicit links (PROJ-XXX in Confluence) are detected with validation
- Semantic link inference produces high-confidence relationships
- The relationship graph is navigable and queryable
- Gap detection identifies undocumented tickets and orphaned docs
- Incremental updates process only changed content
- False positives in implicit link detection are minimized

## Dependencies

- Step 003 (Jira Client) must be complete for issue and link access
- Step 004 (Confluence Client) must be complete for page content access
- `nakama-ai` for semantic link inference
- `regex` crate for pattern matching
