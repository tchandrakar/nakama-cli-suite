# Step 004: Code Chunking

## Objective
Build a semantic-aware code chunking system that splits source files into meaningful chunks at function/class boundaries, handles large functions by sub-splitting, groups related imports, and attaches rich metadata to each chunk for embedding and retrieval.

## Tasks
- [ ] Create `chunking.rs` module with `CodeChunker` struct
- [ ] Implement semantic boundary chunking:
  - Use AST parser results to identify natural split points
  - Each function/method becomes its own chunk
  - Each class/struct/trait becomes a chunk (with methods as sub-chunks)
  - Module-level code (imports, constants, type aliases) grouped as a header chunk
  - Respect logical groupings: keep related code together
- [ ] Implement large function splitting:
  - If a function exceeds max chunk size (configurable, default 500 lines), split it
  - Split at logical boundaries within functions: blank lines, comment blocks, nested blocks
  - Ensure each sub-chunk has function signature as context prefix
  - Maintain line number references across sub-chunks
- [ ] Implement import grouping:
  - Group all imports at file top into a single chunk
  - Associate import chunk with the file for context
  - Track which imports each function/class uses (for context attachment)
- [ ] Implement context attachment:
  - Each chunk gets a context header: file path, parent class/module, imports used
  - Include preceding doc comments as part of the chunk
  - Include function signature for method body chunks
  - Include class definition for method chunks
- [ ] Build `CodeChunk` struct:
  - `id`: unique chunk identifier (hash of content + path + line)
  - `file_path`: source file path
  - `start_line`: first line number
  - `end_line`: last line number
  - `content`: the actual code text
  - `context`: context header (file, parent, imports)
  - `chunk_type`: Function, Class, Module, Import, DocComment, Other
  - `language`: programming language
  - `symbol_name`: primary symbol defined in this chunk (if any)
  - `parent_symbol`: enclosing class/module name (if any)
  - `metadata`: additional key-value pairs (visibility, decorators, etc.)
- [ ] Implement chunk size configuration:
  - `max_chunk_size`: maximum lines per chunk (default 500)
  - `min_chunk_size`: minimum lines per chunk (default 5, merge smaller)
  - `overlap_lines`: lines to overlap between adjacent chunks (default 3)
- [ ] Implement chunk merging for very small items:
  - Merge adjacent constants, type aliases, and small functions
  - Ensure merged chunks do not exceed max size
- [ ] Implement `CodeChunker::chunk(parsed: &ParsedFile) -> Vec<CodeChunk>`
- [ ] Add statistics: total chunks, average size, size distribution
- [ ] Write unit tests for chunking various code structures
- [ ] Write tests for large function splitting
- [ ] Write tests for merge and overlap behavior

## Acceptance Criteria
- Functions and classes are chunked at natural boundaries
- Large functions are split at logical sub-boundaries
- Chunks include context (file path, parent, imports) for standalone comprehension
- Chunk sizes stay within configured min/max bounds
- Very small items are merged with neighbors
- Overlapping lines provide continuity between adjacent chunks
- Tests cover all chunk types and edge cases

## Dependencies
- Step 001 (CLI scaffold)
- Step 003 (AST parser provides parsed file data for chunking)
