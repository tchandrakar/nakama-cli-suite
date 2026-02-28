# Step 002: Code Analyzer

## Objective

Build a code analysis engine using tree-sitter (sharing grammars with senku where possible) that parses source code into ASTs, extracts function signatures, detects side effects, analyzes dependencies, and extracts type information. This forms the foundation for all test generation strategies.

## Tasks

- [ ] Add tree-sitter and language grammar dependencies
- [ ] Share tree-sitter grammar loading with senku crate (via shared utility or workspace dep)
- [ ] Define `FunctionInfo` struct: name, params (name, type, default), return_type, visibility, is_async, generics, doc_comments
- [ ] Define `FileAnalysis` struct: functions, classes/structs, imports, exports, module_path
- [ ] Define `SideEffect` enum: IO (file, network, db), Mutation (state, global), External (API call, system command), None
- [ ] Define `DependencyInfo` struct: imports, function_calls, type_references, external_crates/packages
- [ ] Implement AST parser for supported languages:
  - [ ] Rust: functions, impl blocks, traits, generics, lifetimes
  - [ ] TypeScript/JavaScript: functions, classes, arrow functions, exports
  - [ ] Python: functions, classes, decorators, type hints
  - [ ] Go: functions, methods, interfaces, struct types
  - [ ] Java: methods, classes, interfaces, annotations
- [ ] Implement function signature analyzer:
  - [ ] Extract parameter names and types
  - [ ] Extract return types
  - [ ] Detect generic/template parameters
  - [ ] Detect optional/default parameters
  - [ ] Detect variadic parameters
- [ ] Implement side effect detector:
  - [ ] Detect file I/O operations (read, write, open)
  - [ ] Detect network calls (HTTP, DB, socket)
  - [ ] Detect mutable state access (globals, class fields)
  - [ ] Detect system calls (exec, env access)
  - [ ] Classify functions as pure vs impure
- [ ] Implement dependency analyzer:
  - [ ] Track imports/requires/use statements
  - [ ] Track function call graph (within and across files)
  - [ ] Identify external dependencies vs internal modules
  - [ ] Detect circular dependencies
- [ ] Implement type extractor:
  - [ ] Primitive types
  - [ ] Collection types (Vec, Array, Map, Set)
  - [ ] Custom types (structs, classes, enums)
  - [ ] Generic type parameters
  - [ ] Nullable/Optional types
  - [ ] Union/sum types
- [ ] Implement language auto-detection from file extension and content
- [ ] Unit tests for each language parser
- [ ] Benchmark: analysis must complete in <500ms for files under 1000 lines

## Acceptance Criteria

- AST parsing works for Rust, TypeScript/JavaScript, Python, Go, and Java
- Function signatures are accurately extracted including generics and defaults
- Side effects are correctly classified (pure functions have no false positives)
- Dependencies are tracked at both file and function level
- Type information is extracted with enough detail for test value generation
- Language is auto-detected from file extension
- Tree-sitter grammars are shared with senku where applicable
- Analysis completes in <500ms for typical source files

## Dependencies

- Step 001 (CLI scaffold) must be complete
- tree-sitter and language-specific grammar crates
- senku crate for shared grammar loading (if available)
- No external service dependencies (local analysis only)
