# Step 011: Onboarding Guide

## Objective
Build an automated onboarding guide generator that analyzes a project's structure, architecture, entry points, and coding conventions to produce a comprehensive markdown guide for new developers joining the project.

## Tasks
- [ ] Create `onboard.rs` module with `OnboardingGenerator` struct
- [ ] Implement project overview generation:
  - Project name (from Cargo.toml, package.json, go.mod, etc.)
  - Primary language(s) and framework(s) detected
  - Project type: library, CLI, web server, monorepo, etc.
  - Brief description (from manifest files or README if available)
  - File count, line count, and language breakdown
- [ ] Implement architecture summary:
  - Top-level directory structure with descriptions
  - Module/package organization
  - Layer identification (API, business logic, data, infrastructure)
  - Key abstractions (traits, interfaces, base classes)
  - Use LLM to synthesize architecture narrative from code structure
- [ ] Implement entry point detection:
  - Main functions and binary entry points
  - HTTP/API route handlers (common frameworks)
  - CLI command handlers
  - Event/message handlers
  - Test entry points and test suites
  - Ranked by importance
- [ ] Implement pattern/convention detection:
  - Error handling patterns (Result types, exceptions, error codes)
  - Logging patterns (structured, printf-style, framework)
  - Testing patterns (unit, integration, e2e, test frameworks used)
  - Naming conventions (camelCase, snake_case, file naming)
  - Code organization patterns (MVC, clean architecture, hexagonal, etc.)
  - Dependency injection patterns
- [ ] Implement key dependency analysis:
  - List critical dependencies with brief descriptions
  - Identify framework dependencies (web, ORM, testing)
  - Note internal workspace dependencies (for monorepos)
- [ ] Implement build and run guide:
  - Detect build commands (cargo build, npm build, go build, etc.)
  - Detect test commands
  - Detect development server commands
  - List required environment variables (names only, from .env.example)
  - Prerequisites (language runtimes, tools)
- [ ] Wire up `onboard` subcommand:
  - `senku onboard` — generate full onboarding guide
  - `senku onboard --section architecture` — specific section only
  - `senku onboard --output guide.md` — write to file
  - `--format` flag: `terminal`, `markdown`, `json`
  - `--depth` flag: `brief`, `standard`, `comprehensive`
- [ ] Implement markdown export:
  - Clean markdown with headings, code blocks, and lists
  - Table of contents
  - Mermaid diagrams for architecture (optional)
- [ ] Write unit tests for entry point detection
- [ ] Write tests for convention detection
- [ ] Write integration tests with sample projects

## Acceptance Criteria
- Onboarding guide covers project overview, architecture, entry points, and conventions
- Entry points are correctly identified for various project types
- Coding conventions are accurately detected
- Build and run instructions are correct for the project type
- Markdown output is clean and well-structured
- Guide is useful for a developer new to the project
- Tests cover detection logic for each guide section

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (file discovery for project structure)
- Step 003 (AST parser for code analysis)
- Step 006 (storage for cached analysis data)
- Step 010 (LLM for narrative generation)
