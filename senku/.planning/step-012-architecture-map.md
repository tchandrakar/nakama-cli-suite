# Step 012: Architecture Map

## Objective
Build a visualization system that generates dependency graphs (DOT format convertible to SVG), module diagrams, and terminal-based tree views to provide visual representations of the codebase's architecture and relationships.

## Tasks
- [ ] Create `map.rs` module with `ArchitectureMapper` struct
- [ ] Implement dependency graph generation (DOT format):
  - File-level dependency graph: files as nodes, imports as edges
  - Module-level dependency graph: modules as nodes, inter-module deps as edges
  - Package-level dependency graph: packages/crates as nodes
  - Node attributes: name, language, size (line count), type (source/test/config)
  - Edge attributes: dependency type (import, calls, inherits), weight (reference count)
  - Color coding: by language, by module, or by custom grouping
- [ ] Implement DOT to SVG conversion:
  - Generate DOT source files
  - Invoke `dot` (graphviz) for SVG rendering if available
  - Fallback: output DOT file with instructions to install graphviz
  - Support PNG and PDF output via graphviz
- [ ] Implement module diagrams:
  - Show module hierarchy as nested boxes
  - Show public API surface for each module
  - Show inter-module communication (calls across module boundaries)
  - Highlight modules with most cross-dependencies (potential refactor targets)
- [ ] Implement terminal tree view:
  - Display project structure as an ASCII tree
  - Annotate directories with module info (file count, language, purpose)
  - Highlight entry points and key files
  - Collapsible depth control (`--depth` flag)
  - Color coding by file type
- [ ] Implement call graph visualization:
  - Generate DOT graph for function call relationships
  - Focus on a specific function: show callers and callees
  - Configurable depth for transitive calls
  - Highlight hot paths (most-called functions)
- [ ] Implement circular dependency visualization:
  - Detect circular dependencies in the graph
  - Highlight cycles with distinct coloring
  - Suggest cycle-breaking points
- [ ] Wire up `map` subcommand:
  - `senku map` — generate default architecture map (module-level)
  - `senku map --type file` — file-level dependency graph
  - `senku map --type module` — module-level diagram
  - `senku map --type calls --focus "main"` — call graph from main
  - `senku map --type tree` — terminal tree view
  - `--output` flag: file path for graph export
  - `--format` flag: `dot`, `svg`, `png`, `terminal`, `json`
- [ ] Add `--filter` flag to focus on specific directories or modules
- [ ] Add `--max-nodes` flag to limit graph complexity
- [ ] Write unit tests for DOT generation
- [ ] Write tests for tree view rendering
- [ ] Write tests for circular dependency detection

## Acceptance Criteria
- DOT graphs render correctly in graphviz
- SVG output produces readable diagrams for projects up to 500 files
- Module diagrams show clear inter-module relationships
- Terminal tree view is informative and visually clear
- Circular dependencies are detected and highlighted
- Large graphs are manageable with filtering and node limits
- Tests cover graph generation, tree rendering, and cycle detection

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (file discovery for project structure)
- Step 003 (AST parser for relationship data)
- Step 006 (graph store for relationship traversal)
- Step 009 (graph queries for dependency information)
