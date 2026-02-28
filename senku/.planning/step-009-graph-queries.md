# Step 009: Graph Queries

## Objective
Build a graph query system that traverses the code knowledge graph to answer structural questions like "who calls X?", explore dependency trees, and navigate module relationships, complementing the semantic search with precise structural navigation.

## Tasks
- [ ] Create `graph.rs` module with `GraphQuerier` struct
- [ ] Implement "who calls X?" traversal:
  - Find all callers of a function/method by name
  - Support qualified names (e.g., `auth::validate_token`)
  - Return caller list with file paths and line numbers
  - Support transitive callers (callers of callers, with depth limit)
  - Display as a tree or flat list
- [ ] Implement "what does X call?" traversal:
  - Find all functions called by a given function
  - Support transitive callees (with depth limit)
  - Useful for understanding function behavior
- [ ] Implement dependency tree queries:
  - File-level: what files does file X import?
  - Module-level: what modules does module X depend on?
  - Crate/package-level: what external dependencies are used?
  - Reverse dependencies: what depends on file/module X?
  - Support depth-limited traversal
- [ ] Implement module relationship navigation:
  - List all modules/packages in the project
  - Show module hierarchy (parent-child relationships)
  - Show cross-module dependencies
  - Identify circular dependencies
  - List public API surface of a module
- [ ] Implement class hierarchy queries:
  - Find implementations of a trait/interface
  - Find subclasses of a class
  - Show inheritance chain
  - List all methods of a class (including inherited)
- [ ] Build query result types:
  - `CallGraph`: directed graph of function calls
  - `DependencyTree`: tree of file/module dependencies
  - `ModuleMap`: module hierarchy with relationships
  - `SymbolInfo`: detailed information about a symbol
- [ ] Wire up structural queries to the `search` subcommand:
  - `senku search --callers "parse_config"` — who calls parse_config?
  - `senku search --deps "src/auth.rs"` — what does auth.rs depend on?
  - `senku search --rdeps "src/utils.rs"` — what depends on utils.rs?
  - `senku search --implementations "Collector"` — who implements Collector?
- [ ] Add visualization output:
  - Tree view for hierarchical results
  - DOT format for graph visualization
  - JSON format for programmatic access
- [ ] Write unit tests for each query type
- [ ] Write tests for circular dependency detection
- [ ] Write integration tests with realistic code graphs

## Acceptance Criteria
- "Who calls X?" returns accurate caller lists with file references
- Dependency trees correctly trace file and module dependencies
- Circular dependencies are detected and reported
- Class hierarchy queries work for traits/interfaces
- Transitive traversals respect depth limits
- Graph queries complete in under 200ms for typical codebases
- Tests cover all query types and edge cases

## Dependencies
- Step 001 (CLI scaffold)
- Step 003 (AST parser provides relationship data)
- Step 006 (graph store provides the graph to query)
