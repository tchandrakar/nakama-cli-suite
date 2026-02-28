# Step 003: AST Parser

## Objective
Build a multi-language AST parsing layer using Tree-sitter that extracts function/class boundaries, symbol definitions, and import relationships from source files in Rust, TypeScript, Python, Go, and Java.

## Tasks
- [ ] Add `tree-sitter` and language grammar crate dependencies:
  - `tree-sitter` core
  - `tree-sitter-rust`
  - `tree-sitter-typescript` (covers TS and TSX)
  - `tree-sitter-python`
  - `tree-sitter-go`
  - `tree-sitter-java`
- [ ] Create `parser.rs` module with `AstParser` struct
- [ ] Implement grammar loading and parser initialization:
  - Load grammars for each supported language
  - Cache parser instances for reuse
  - Handle missing grammars gracefully (fallback to text-based parsing)
- [ ] Implement function boundary extraction:
  - Rust: `fn`, `pub fn`, `impl` blocks, trait methods
  - TypeScript/JavaScript: function declarations, arrow functions, class methods
  - Python: `def`, `async def`, class methods, decorators
  - Go: `func`, method receivers
  - Java: methods, constructors
  - Extract: name, start line, end line, visibility, parameters, return type
- [ ] Implement class/struct boundary extraction:
  - Rust: `struct`, `enum`, `trait`, `impl`
  - TypeScript/JavaScript: `class`, `interface`, `type`
  - Python: `class`
  - Go: `type struct`, `type interface`
  - Java: `class`, `interface`, `enum`
  - Extract: name, start line, end line, visibility, methods list, fields list
- [ ] Implement symbol extraction:
  - Constants, static variables, type aliases
  - Exported symbols (public API surface)
  - Module declarations and namespaces
  - Build symbol table per file
- [ ] Implement import/dependency detection:
  - Rust: `use`, `mod`, `extern crate`
  - TypeScript/JavaScript: `import`, `require`
  - Python: `import`, `from ... import`
  - Go: `import`
  - Java: `import`
  - Resolve relative imports to file paths where possible
- [ ] Build `ParsedFile` struct:
  - `functions`: list of function definitions with metadata
  - `classes`: list of class/struct/trait definitions
  - `symbols`: symbol table (name -> type, line, visibility)
  - `imports`: list of import statements with resolved paths
  - `exports`: list of exported/public symbols
  - `doc_comments`: extracted documentation comments
- [ ] Implement `AstParser::parse(file: &DiscoveredFile) -> Result<ParsedFile>`
- [ ] Handle parse errors gracefully (partial results from damaged files)
- [ ] Add fallback text-based parser for unsupported languages (regex-based)
- [ ] Write unit tests for each language parser
- [ ] Write tests for import resolution

## Acceptance Criteria
- Functions, classes, and symbols are correctly extracted for all 5 languages
- Import statements are parsed and resolved to file paths where possible
- Parse errors produce partial results rather than failures
- Unsupported languages fall back to text-based extraction
- Parser handles large files (>10K lines) without performance issues
- Tests cover each language with realistic code samples

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (file discovery provides files to parse)
