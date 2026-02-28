# Step 005: Embedding Generation

## Objective
Build an embedding generation pipeline that uses `nakama-ai` to generate vector embeddings for code chunks, supports batch processing for efficiency, and implements incremental updates so only changed files are re-embedded.

## Tasks
- [ ] Create `embedding.rs` module with `EmbeddingGenerator` struct
- [ ] Integrate with `nakama-ai` for embedding generation:
  - Use the embedding endpoint/model from nakama-ai
  - Configure embedding model (e.g., text-embedding-ada-002 or equivalent)
  - Handle API rate limiting with backoff
- [ ] Implement chunk-to-embedding-text preparation:
  - Combine chunk content with context header for richer embeddings
  - Format: "File: {path}\nLanguage: {lang}\nSymbol: {name}\n\n{content}"
  - Truncate to embedding model's max token limit
  - Handle multi-line formatting consistently
- [ ] Implement batch embedding:
  - Group chunks into batches (configurable batch size, default 100)
  - Send batches to embedding API in parallel (configurable concurrency)
  - Collect results and associate embeddings with chunk IDs
  - Progress reporting via `nakama-ui` (progress bar with ETA)
- [ ] Implement incremental updates:
  - Compare file modification times against last index time
  - Only re-embed chunks from files that changed
  - Delete embeddings for removed files
  - Update embeddings for modified files (delete old, create new)
  - Track file hashes for reliable change detection
- [ ] Build `EmbeddedChunk` struct:
  - `chunk_id`: reference to the CodeChunk
  - `embedding`: Vec<f32> (embedding vector)
  - `model`: embedding model identifier
  - `created_at`: timestamp
  - `file_hash`: hash of source file at embedding time
- [ ] Implement embedding cache:
  - Cache embeddings on disk (in `.senku/` directory)
  - Validate cache freshness against file hashes
  - Invalidate cache on model change
- [ ] Handle embedding failures:
  - Retry failed chunks (configurable retry count)
  - Skip permanently failed chunks with warning
  - Report success/failure statistics
- [ ] Add `--force` flag to re-embed everything (ignore incremental)
- [ ] Add `--dry-run` flag to show what would be embedded without doing it
- [ ] Write unit tests for chunk-to-text preparation
- [ ] Write tests for incremental update logic
- [ ] Write integration tests with mock embedding API

## Acceptance Criteria
- Embeddings are generated for all code chunks via nakama-ai
- Batch processing achieves reasonable throughput (>100 chunks/second with API)
- Incremental updates only process changed files (no unnecessary re-embedding)
- Progress bar shows real-time status for large indexing jobs
- Embedding failures are retried and reported without crashing
- Cache invalidation works correctly on file changes and model changes
- Tests cover preparation, incremental logic, and error handling

## Dependencies
- Step 001 (CLI scaffold)
- Step 004 (code chunking produces chunks to embed)
