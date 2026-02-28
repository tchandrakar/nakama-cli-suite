# Step 008: Semantic Retrieval

## Objective
Build a semantic retrieval system that performs vector similarity search to find relevant code chunks, classifies queries as semantic or structural, assembles rich context from search results, and provides the foundation for RAG-based answer generation.

## Tasks
- [ ] Create `retrieval.rs` module with `SemanticRetriever` struct
- [ ] Implement vector similarity search:
  - Embed the query text using the same embedding model
  - Search vector store for top-K most similar chunks (default K=10)
  - Apply relevance threshold (minimum similarity score, configurable)
  - Return ranked results with similarity scores
- [ ] Implement query classification:
  - Classify query as semantic ("how does authentication work?") or structural ("who calls parse_config?")
  - Semantic queries -> vector search
  - Structural queries -> graph queries (delegated to step 009)
  - Hybrid queries -> combine both approaches
  - Classification via keyword detection and/or LLM-based classifier
- [ ] Implement search filters:
  - Filter by language: `--lang rust`
  - Filter by file path pattern: `--path "src/auth/**"`
  - Filter by symbol type: `--type function` or `--type class`
  - Filter by date: `--since "2024-01-01"` (based on file modification)
- [ ] Implement context assembly:
  - Take top-K search results
  - Expand each result with surrounding context (preceding/following chunks)
  - Include file-level context (imports, module description)
  - Include parent class/struct context for method chunks
  - Deduplicate overlapping context
  - Rank and trim to fit within LLM context window
- [ ] Build `RetrievalResult` struct:
  - `chunks`: ranked list of relevant chunks with scores
  - `context`: assembled context string for LLM
  - `source_files`: list of source files referenced
  - `query_type`: semantic, structural, or hybrid
  - `total_matches`: total matches before filtering
- [ ] Implement `search` subcommand:
  - `senku search "error handling patterns"` — semantic search
  - Display results with file paths, line numbers, and code previews
  - Syntax highlighting for code previews
  - `--limit` flag to control result count
  - `--format` flag: `terminal`, `json`, `plain`
- [ ] Implement result caching:
  - Cache recent query results (LRU cache, configurable size)
  - Invalidate cache on index update
- [ ] Write unit tests for query classification
- [ ] Write tests for context assembly logic
- [ ] Write integration tests for end-to-end search

## Acceptance Criteria
- Semantic search returns relevant code chunks for natural language queries
- Query classification correctly routes semantic vs structural queries
- Context assembly produces coherent, deduplicated context for LLM consumption
- Search filters narrow results correctly by language, path, and type
- Search results display with syntax highlighting and file references
- Response time is under 500ms for typical queries on indexed codebases
- Tests cover classification, assembly, and search operations

## Dependencies
- Step 001 (CLI scaffold)
- Step 005 (embedding generation for query embedding)
- Step 006 (vector store for similarity search)
