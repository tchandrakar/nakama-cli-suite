# Step 008: Coverage Analysis

## Objective

Build a coverage analysis engine that parses coverage reports from multiple formats (lcov, cobertura, istanbul), identifies uncovered lines and branches, and targets test generation specifically at uncovered code paths to systematically improve coverage.

## Tasks

- [ ] Define `CoverageReport` struct: file_path, total_lines, covered_lines, total_branches, covered_branches, uncovered_ranges
- [ ] Define `UncoveredRegion` struct: file_path, start_line, end_line, region_type (line, branch, function), function_name, complexity
- [ ] Implement coverage report parsers:
  - [ ] **lcov** format (Rust tarpaulin, genhtml, gcov)
  - [ ] **cobertura** XML format (Python coverage, Java JaCoCo)
  - [ ] **istanbul/NYC** JSON format (JavaScript/TypeScript)
  - [ ] **Go cover** profile format
  - [ ] Auto-detect format from file content/extension
- [ ] Implement uncovered code identification:
  - [ ] Map uncovered lines to functions (using AST from code analyzer)
  - [ ] Identify uncovered branches (if-else, match arms, switch cases)
  - [ ] Identify completely untested functions
  - [ ] Identify partially tested functions (some paths covered)
  - [ ] Rank uncovered regions by importance (complexity, risk)
- [ ] Implement targeted test generation:
  - [ ] For each uncovered region, determine which input conditions reach it
  - [ ] Generate tests specifically targeting uncovered branches
  - [ ] Include branch condition in LLM prompt ("test the else branch when x < 0")
  - [ ] Prioritize by: uncovered function > uncovered branch > uncovered line
- [ ] Implement coverage tracking:
  - [ ] Run coverage before test generation (baseline)
  - [ ] Run coverage after adding generated tests (verify improvement)
  - [ ] Report coverage delta (lines gained, branches gained, percentage change)
  - [ ] Store coverage history for trend analysis
- [ ] Implement coverage commands:
  - [ ] Auto-detect coverage tool for the project
  - [ ] Run coverage tool and parse results
  - [ ] Support custom coverage command via config
- [ ] Implement `cover` subcommand:
  - [ ] `mugen cover` — analyze coverage and generate tests for gaps
  - [ ] `--target` flag: target coverage percentage (default: none, generate for all gaps)
  - [ ] `--file` flag: focus on specific file
  - [ ] `--function` flag: focus on specific function
  - [ ] `--report` flag: path to existing coverage report (skip running coverage)
  - [ ] `--diff-only` flag: only cover code changed in current branch
- [ ] Unit tests for each coverage report parser
- [ ] Integration test: generate tests that improve coverage on sample project

## Acceptance Criteria

- All four coverage report formats are parsed correctly
- Uncovered lines are correctly mapped to functions via AST
- Branch-level coverage gaps are identified and targeted
- Generated tests specifically target uncovered code paths
- Coverage improvement is measurable after test generation
- `--diff-only` mode focuses on changed code (useful for PRs)
- Coverage delta is reported clearly
- Coverage report auto-detection works for common project setups

## Dependencies

- Step 002 (Code analyzer) for AST-based line-to-function mapping
- Step 005 (LLM test generator) for targeted test generation
- Step 006 (Validation loop) for test validation
- Coverage tools must be installed (tarpaulin, coverage.py, nyc, go tool cover)
