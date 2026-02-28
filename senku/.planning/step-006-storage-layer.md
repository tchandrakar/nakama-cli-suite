# Step 006: Storage Layer

## Objective
Build the storage layer consisting of a vector store (LanceDB or embedded qdrant) for embedding search, a graph store (petgraph + SQLite) for code relationships, and a metadata store for chunk and file information, all persisted in a `.senku/` directory.

## Tasks
- [ ] Create `storage.rs` module (or `storage/` sub-module) with storage abstractions
- [ ] Implement `.senku/` directory management:
  - Create `.senku/` in project root on first index
  - Sub-directories: `vectors/`, `graph/`, `metadata/`
  - Add `.senku/` to suggested .gitignore entries
  - Set directory permissions to 0700 (owner-only access)
- [ ] Implement vector store:
  - Evaluate and integrate LanceDB (embedded) or qdrant (embedded mode)
  - Store embeddings with chunk ID as key
  - Support similarity search (cosine distance, top-K)
  - Support filtered search (by language, file path, symbol type)
  - Handle store creation, insertion, deletion, and rebuilding
  - Optimize for search latency (<100ms for 100K embeddings)
- [ ] Implement graph store using petgraph + SQLite:
  - Nodes: files, functions, classes, modules
  - Edges: calls, imports, inherits, implements, contains
  - Persist graph to SQLite for durability
  - Load graph on startup, update incrementally
  - Support graph queries: "who calls function X?", "what does module Y depend on?"
- [ ] Implement metadata store using SQLite:
  - File metadata: path, hash, language, last indexed time, chunk count
  - Chunk metadata: chunk ID, file path, lines, symbol, type
  - Index statistics: total files, chunks, embeddings, last full index time
  - Schema migrations for version upgrades
- [ ] Implement `VectorStore` trait:
  - `insert(id: &str, embedding: &[f32], metadata: HashMap)`
  - `search(query: &[f32], top_k: usize, filter: Option<Filter>) -> Vec<SearchResult>`
  - `delete(id: &str)`
  - `delete_by_file(file_path: &str)` — remove all embeddings for a file
  - `count() -> usize`
- [ ] Implement `GraphStore` trait:
  - `add_node(node: GraphNode)`
  - `add_edge(from: NodeId, to: NodeId, edge_type: EdgeType)`
  - `query_callers(symbol: &str) -> Vec<GraphNode>`
  - `query_dependencies(file: &str) -> Vec<GraphNode>`
  - `query_dependents(file: &str) -> Vec<GraphNode>`
  - `remove_file(path: &str)` — remove all nodes/edges for a file
- [ ] Implement storage initialization and migration:
  - Auto-create stores on first use
  - Run schema migrations for upgrades
  - Validate store integrity on open
- [ ] Implement store compaction/optimization (periodic or manual)
- [ ] Write unit tests for vector store operations
- [ ] Write tests for graph store operations
- [ ] Write tests for metadata store CRUD

## Acceptance Criteria
- Vector store performs similarity search in <100ms for 100K embeddings
- Graph store supports all relationship queries (callers, dependencies, dependents)
- Metadata store tracks file and chunk information accurately
- `.senku/` directory is properly created and secured
- Incremental updates (add/remove/modify files) work without full rebuild
- Schema migrations handle store upgrades gracefully
- Tests cover all store operations

## Dependencies
- Step 001 (CLI scaffold)
- Step 003 (AST parser provides graph node data)
- Step 005 (embedding generation produces vectors to store)
