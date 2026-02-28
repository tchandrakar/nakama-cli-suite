# Step 006: Diff Analysis Engine

## Objective

Build the diff analysis engine that parses unified diffs into structured representations, performs semantic analysis (function-level changes, import changes, type signature changes, control flow changes, API surface changes), and enriches context with full file content, PR description, previous comments, and CI status.

## Tasks

- [ ] Implement the unified diff parser:
  - Parse raw unified diff format into `UnifiedDiff` with `DiffFile` and `Hunk` structures
  - Handle file headers (`---`, `+++`, `@@` hunk headers)
  - Parse added, removed, and context lines with correct line numbers
  - Detect file renames (from `rename from`/`rename to` or `similarity index`)
  - Detect file type from extension and shebang lines
  - Handle binary file markers
  - Handle edge cases: empty files, new files, deleted files, mode changes
- [ ] Implement semantic diff analyzer:
  - **Function-level changes**: Identify which functions/methods were modified, added, or removed (use tree-sitter or regex-based heuristics per language)
  - **Import/dependency changes**: Detect added/removed imports, dependency declarations
  - **Type signature changes**: Detect changes to struct definitions, interfaces, type aliases, function signatures
  - **Control flow changes**: Identify changes to if/else, loops, match/switch, error handling
  - **API surface changes**: Detect changes to public functions, exported types, endpoint definitions
- [ ] Implement language-specific analysis hints:
  - Rust: detect `pub fn`, `impl`, `struct`, `enum`, `trait`, `use`, `mod`
  - TypeScript/JavaScript: detect `export`, `function`, `class`, `interface`, `import`
  - Python: detect `def`, `class`, `import`, `from`
  - Go: detect `func`, `type`, `struct`, `interface`, `import`
  - Generic fallback for unsupported languages
- [ ] Implement context enrichment module:
  - Fetch full file content (not just diff) from the platform adapter for surrounding context
  - Extract PR description and linked issue information
  - Gather previous review comments to avoid redundant feedback
  - Query CI/CD status if available via platform adapter
  - Identify file ownership (who typically maintains this file, via git blame or CODEOWNERS)
- [ ] Create `DiffAnalysis` output struct that combines:
  - Parsed diff (structured hunks and lines)
  - Semantic annotations (what changed at a semantic level)
  - Enriched context (surrounding code, PR info, previous comments)
  - File metadata (language, type, ownership)
- [ ] Write comprehensive unit tests for diff parsing (various formats and edge cases)
- [ ] Write unit tests for semantic analysis per language

## Acceptance Criteria

- Unified diffs from GitHub, GitLab, and Bitbucket are all parsed correctly
- Semantic analysis correctly identifies function-level, import, type, and control flow changes
- Context enrichment includes full file content and PR metadata
- The analyzer handles diffs of any size (streaming for very large diffs)
- All language-specific analysis hints produce correct results for test cases
- Unit tests cover edge cases: empty diffs, binary files, renames, mode changes, large diffs

## Dependencies

- Step 002 (models and PlatformAdapter trait) must be complete
- At least one platform adapter (Step 003, 004, or 005) should be complete for integration testing
- Optional: `tree-sitter` crates for precise AST-based analysis
- `regex` crate for pattern matching
