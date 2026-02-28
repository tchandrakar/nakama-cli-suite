# Step 007: Release Notes

## Objective
Build a release notes generation system that handles version argument processing (semver), auto-detects the appropriate version bump based on commit analysis, generates human-readable release notes via LLM, and creates annotated git tags.

## Tasks
- [ ] Create `release.rs` module with `ReleaseManager` struct
- [ ] Implement version argument handling:
  - Accept explicit version: `shinigami release v2.0.0`
  - Accept bump type: `shinigami release --bump major|minor|patch`
  - Parse and validate semver format
  - Derive next version from latest tag + bump type
- [ ] Implement auto-detect version bump:
  - Scan commits since last tag using semantic analysis
  - Breaking changes -> major bump
  - Features -> minor bump
  - Fixes/chores/refactors -> patch bump
  - Display suggested bump with reasoning
  - Allow user to override suggestion
- [ ] Implement release note generation via LLM:
  - Collect commits since last tag
  - Group by type (similar to changelog but more narrative)
  - Send to `nakama-ai` with system prompt for release notes style
  - Generate highlights section (most important changes)
  - Generate detailed changes section
  - Include migration notes for breaking changes
  - Include contributor acknowledgments
- [ ] Implement git tag creation:
  - Create annotated tag with release notes as message
  - Validate tag does not already exist
  - Support `--sign` flag for signed tags
  - Support `--prefix` flag for tag prefix (default: `v`)
- [ ] Wire up `release` subcommand:
  - `shinigami release` — auto-detect version and generate notes
  - `shinigami release v1.2.3` — use specified version
  - `shinigami release --bump minor` — bump minor version
  - `--draft` flag: show release notes without creating tag
  - `--no-tag` flag: generate notes without creating tag
  - `--edit` flag: open generated notes in $EDITOR before tagging
- [ ] Output release notes to stdout and optionally to file
- [ ] Add `--format` flag: `markdown`, `plain`, `json`
- [ ] Write unit tests for version detection and bump logic
- [ ] Write tests for release note generation with mock LLM
- [ ] Write integration tests for tag creation in temp repo

## Acceptance Criteria
- Version auto-detection correctly identifies major/minor/patch bumps
- Release notes are well-structured with highlights and detailed changes
- Annotated git tags are created with release notes as message
- Existing tags are not overwritten
- User can preview, edit, and confirm before tag creation
- All version formats and bump types are handled correctly
- Tests cover version logic, note generation, and tag creation

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (git interface for tags and log)
- Step 003 (semantic analyzer for bump detection)
- Step 006 (changelog engine for commit aggregation)
