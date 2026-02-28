# Step 003: Semantic Analyzer

## Objective
Build a diff classification engine that analyzes staged changes to determine the semantic type (feat, fix, refactor, docs, test, chore), detect scope, identify breaking changes, and recognize renames/moves. This analysis feeds into the LLM prompt for commit message generation.

## Tasks
- [ ] Create `analyzer.rs` module with `SemanticAnalysis` result struct
- [ ] Implement change type classification based on file patterns and diff content:
  - `feat`: new files, new public functions/exports, new modules, new API endpoints
  - `fix`: modifications in existing code, changes near error handling, test fixes
  - `refactor`: renames, moves, restructuring without behavior change
  - `docs`: changes to .md files, doc comments, README, CHANGELOG
  - `test`: changes in test files, test modules, test fixtures
  - `chore`: changes to CI config, Cargo.toml deps, .gitignore, build scripts
  - `style`: formatting-only changes (whitespace, semicolons, imports ordering)
  - `perf`: changes in hot paths, algorithm changes, caching additions
- [ ] Implement scope detection:
  - Identify affected module/package from file paths
  - Common scope: directory name, crate name, or feature area
  - Support monorepo scope detection (workspace member names)
- [ ] Implement breaking change detection:
  - Public API removals or signature changes
  - Removed or renamed exports
  - Major version dependency bumps
  - Detection of `BREAKING CHANGE` in existing commit messages
- [ ] Implement rename/move detection:
  - Use git2 rename detection (similarity threshold)
  - Track file renames and report old -> new paths
  - Detect function/class renames within files (basic heuristic)
- [ ] Build `AnalysisReport` struct containing:
  - Primary change type (most significant)
  - Secondary change types (if multiple)
  - Detected scope(s)
  - Breaking change flag with description
  - Rename/move list
  - File-level summary (per file: type, insertions, deletions)
  - Overall complexity score (simple/moderate/complex)
- [ ] Add configurable classification rules (allow override via config)
- [ ] Write unit tests with sample diffs for each change type
- [ ] Write tests for scope detection across different project structures

## Acceptance Criteria
- Analyzer correctly classifies common change patterns (new feature, bug fix, docs update)
- Scope detection identifies the affected module or package
- Breaking changes are flagged when public APIs change
- Renames are detected and reported
- Analysis completes in under 100ms for typical diffs
- Classification rules are tested for each change type

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (git interface provides the diff data to analyze)
