# Step 002: File Discovery

## Objective
Build a .gitignore-aware file walker that discovers all relevant source files in a project, detects file types, applies size filtering, and respects configurable ignore patterns to produce a clean file list for indexing.

## Tasks
- [ ] Create `discovery.rs` module with `FileDiscovery` struct
- [ ] Implement .gitignore-aware file walking:
  - Use `ignore` crate (from ripgrep) for .gitignore parsing
  - Respect nested .gitignore files
  - Respect global gitignore (`~/.config/git/ignore`)
  - Walk directories recursively from project root
- [ ] Implement file type detection:
  - Detect programming language from file extension
  - Support: Rust (.rs), TypeScript (.ts/.tsx), JavaScript (.js/.jsx), Python (.py), Go (.go), Java (.java), C/C++ (.c/.cpp/.h/.hpp), Ruby (.rb), PHP (.php), Swift (.swift), Kotlin (.kt)
  - Detect configuration files: TOML, YAML, JSON, XML, INI
  - Detect documentation: Markdown (.md), reStructuredText (.rst), AsciiDoc (.adoc)
  - Detect build files: Makefile, Dockerfile, Cargo.toml, package.json, go.mod, pom.xml
  - Mark binary files for exclusion
- [ ] Implement size filtering:
  - Default max file size: 1MB (configurable)
  - Skip files exceeding max size with warning
  - Skip empty files
  - Report filtered file count and total size
- [ ] Implement configurable ignore patterns:
  - Default ignores: node_modules, target, .git, __pycache__, dist, build, vendor
  - User-configurable ignore patterns via `.senku/ignore` or config file
  - Support glob patterns for ignoring
  - Support language-specific ignores (e.g., ignore .pyc for Python projects)
- [ ] Build `DiscoveredFile` struct:
  - `path`: relative path from project root
  - `absolute_path`: full filesystem path
  - `language`: detected programming language
  - `file_type`: source, config, documentation, build, other
  - `size_bytes`: file size
  - `last_modified`: modification timestamp
  - `git_status`: tracked, untracked, modified (if in git repo)
- [ ] Implement `FileDiscovery::discover(root: &Path) -> Result<Vec<DiscoveredFile>>`
- [ ] Add progress reporting via `nakama-ui` for large projects
- [ ] Implement file change detection (for incremental re-indexing):
  - Compare against previous scan results
  - Report added, modified, and deleted files
- [ ] Write unit tests for file type detection
- [ ] Write tests for gitignore handling
- [ ] Write tests for size filtering and ignore patterns

## Acceptance Criteria
- File walker respects .gitignore at all levels
- Programming languages are correctly detected for all supported types
- Files exceeding size limit are skipped with warnings
- Custom ignore patterns work with glob syntax
- Discovery completes in under 5 seconds for typical projects (10K files)
- Change detection identifies added/modified/deleted files
- Tests cover detection, filtering, and ignore patterns

## Dependencies
- Step 001 (CLI scaffold must exist to house this module)
