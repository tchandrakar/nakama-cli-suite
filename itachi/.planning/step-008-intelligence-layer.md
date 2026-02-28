# Step 008: Intelligence Layer

## Objective

Build the intelligence layer with a local embedding cache for Confluence content using LanceDB, incremental indexing that detects page changes, semantic search for finding relevant content beyond keyword matching, a context builder that assembles LLM context within token limits, and an LLM synthesizer that produces answers with source attribution.

## Tasks

- [ ] Add dependencies: `lancedb`, embedding API client (via nakama-ai)
- [ ] Implement the embedding cache:
  - Create LanceDB database at `~/.itachi/embeddings/`
  - Define schema: page_id, space_key, title, chunk_text, embedding_vector, page_url, last_modified
  - Chunk Confluence pages into segments (500-1000 tokens per chunk)
  - Preserve chunk context (include page title and section headers)
  - Support multiple embedding models (configurable, default: voyage-code-3)
- [ ] Implement incremental indexing:
  - On first run: index all pages in configured spaces
  - Track `lastModified` timestamp per page
  - On subsequent runs: only re-index pages that changed since last index
  - Use Confluence's `lastModified` field and CQL `lastModified > "date"` queries
  - Support manual re-index: `itachi index rebuild`
  - Configurable auto-update interval (default: 6 hours)
  - Display indexing progress with nakama-ui progress bar
- [ ] Implement semantic search:
  - Accept a natural language query
  - Generate query embedding via the same embedding model
  - Perform vector similarity search in LanceDB
  - Return top-K results with relevance scores
  - Support filtering by space, label, date range
  - Combine semantic results with CQL keyword results for hybrid search
- [ ] Implement the context builder:
  - Accept a user question and a set of source materials (issues, pages, sprint data)
  - Score and rank materials by relevance to the question
  - Assemble context within token budget:
    1. Most relevant Confluence page chunks
    2. Most relevant Jira issue details
    3. Cross-reference graph relationships
    4. Sprint/board data (if relevant)
  - Truncate individual items if needed to fit budget
  - Include source attribution markers for later reference
- [ ] Implement the LLM synthesizer:
  - Use nakama-ai to generate answers from assembled context
  - System prompt: "Answer the question based on the provided Jira and Confluence context. Always cite sources."
  - Parse response to extract:
    - Direct answer text
    - Links to relevant Jira tickets (with URLs)
    - Links to relevant Confluence pages (with URLs)
    - Suggested follow-up actions
  - Handle cases where context is insufficient ("I could not find enough information about...")
- [ ] Implement `itachi index` subcommand:
  - `itachi index status` -- show indexing status (pages indexed, last update, size)
  - `itachi index rebuild` -- full re-index
  - `itachi index update` -- incremental update
  - `itachi index search <query>` -- direct semantic search (for debugging)
- [ ] Write unit tests for chunking, context building, and synthesis
- [ ] Write integration tests for the embedding pipeline

## Acceptance Criteria

- Confluence pages are embedded and stored in LanceDB
- Incremental indexing correctly detects and re-indexes changed pages
- Semantic search returns relevant results for natural language queries
- Context builder assembles context within token limits, prioritized by relevance
- LLM synthesizer produces accurate answers with source attribution
- Source links point to correct Jira tickets and Confluence pages
- `itachi index` commands work correctly for status, rebuild, and update

## Dependencies

- Step 004 (Confluence Client) must be complete for page content access
- Step 007 (Cross-Reference Engine) must be complete for relationship data
- `lancedb` crate for vector storage
- `nakama-ai` shared crate for embeddings and LLM synthesis
- Embedding model API access (voyage-code-3 or similar)
