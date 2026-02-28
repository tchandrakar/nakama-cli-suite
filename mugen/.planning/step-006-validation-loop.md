# Step 006: Validation Loop

## Objective

Build a multi-stage validation loop that checks generated test code for syntax correctness, compilation/type-checking, actual execution, and auto-fixes common issues (imports, mocks, type mismatches). Only tests that pass all validation stages are written to disk.

## Tasks

- [ ] Define `ValidationResult` struct: passed, stage_failed, errors, fix_applied, attempts
- [ ] Define validation stages:
  - [ ] Stage 1: Syntax check (AST parse of generated code)
  - [ ] Stage 2: Compilation/type check (language-specific)
  - [ ] Stage 3: Execution (run the test)
  - [ ] Stage 4: Quality check (no trivial assertions, meaningful coverage)
- [ ] Implement syntax validation:
  - [ ] Parse generated code with tree-sitter
  - [ ] Detect syntax errors with line/column information
  - [ ] Detect incomplete code (unclosed brackets, missing semicolons)
- [ ] Implement compilation/type check:
  - [ ] Rust: `cargo check` on test file
  - [ ] TypeScript: `tsc --noEmit` on test file
  - [ ] Python: `mypy` check (if configured) or basic import check
  - [ ] Go: `go vet` on test file
  - [ ] Java: `javac` compilation check
  - [ ] Parse compiler errors for actionable feedback
- [ ] Implement test execution:
  - [ ] Run the generated test in isolation
  - [ ] Capture stdout, stderr, exit code
  - [ ] Detect test failures vs compilation errors
  - [ ] Timeout protection (configurable, default 30s per test)
  - [ ] Sandbox execution (prevent file system / network side effects)
- [ ] Implement auto-fix engine:
  - [ ] Missing imports: analyze errors, add required imports
  - [ ] Missing mock setup: detect unmocked dependencies, generate mocks
  - [ ] Type mismatches: adjust test values to match expected types
  - [ ] Assertion method fixes: correct assertion API usage
  - [ ] Missing test setup: add required beforeEach/setUp
  - [ ] Lifetime/ownership fixes (Rust-specific)
- [ ] Implement retry loop:
  - [ ] Max retry attempts (configurable, default 3)
  - [ ] Feed errors back to LLM for correction
  - [ ] Include error messages in follow-up prompt
  - [ ] Track which fixes were applied
- [ ] Implement quality gate:
  - [ ] Reject trivial tests (e.g., `assert(true)`, `expect(1).toBe(1)`)
  - [ ] Reject tests that don't call the target function
  - [ ] Reject tests that test only the mock, not the function
  - [ ] Minimum assertion count per test (configurable)
- [ ] Implement file writing:
  - [ ] Only write tests that pass all validation stages
  - [ ] Write to correct location per convention detection
  - [ ] Handle file creation vs appending to existing test files
  - [ ] Respect `--dry-run` flag (print but don't write)
- [ ] Add `--max-retries` flag
- [ ] Add `--skip-execution` flag (only syntax + compile check)
- [ ] Unit tests for each validation stage
- [ ] Integration test: generate, validate, and write tests for sample functions

## Acceptance Criteria

- Syntax validation catches all parse errors before compilation
- Compilation check runs successfully for each supported language
- Test execution correctly identifies passing vs failing tests
- Auto-fix resolves common issues (imports, types) in >70% of cases
- Retry loop with LLM feedback improves success rate per attempt
- Quality gate rejects trivial/meaningless tests
- Only validated tests are written to disk
- `--dry-run` shows tests without writing
- Validation completes within 60 seconds per function (including retries)

## Dependencies

- Step 002 (Code analyzer) for AST parsing in syntax validation
- Step 003 (Convention detector) for test file location
- Step 005 (LLM test generator) for test code to validate
- Language toolchains must be available (cargo, tsc, python, go, javac)
