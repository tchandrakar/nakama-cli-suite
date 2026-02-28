# Step 009: Mutation Testing

## Objective

Build a mutation testing engine that applies AST-based mutation operators to source code, runs the test suite against mutants, reports surviving mutants, and automatically generates tests to kill surviving mutants. This validates test suite quality beyond coverage metrics.

## Tasks

- [ ] Define `Mutant` struct: original_code, mutated_code, mutation_type, file_path, line, function_name, status
- [ ] Define `MutantStatus` enum: Killed, Survived, Timeout, Error, Skipped
- [ ] Define `MutationReport` struct: total_mutants, killed, survived, timeout, mutation_score, surviving_mutants
- [ ] Implement AST-based mutation operators:
  - [ ] **Arithmetic**: replace +/-/*// with each other
  - [ ] **Conditional**: replace >, <, >=, <=, ==, != with each other
  - [ ] **Boolean**: replace && with ||, negate conditions
  - [ ] **Boundary**: off-by-one changes (< to <=, > to >=)
  - [ ] **Return value**: replace return with default/empty/null
  - [ ] **Statement deletion**: remove statements (especially assignments)
  - [ ] **Constant replacement**: change numeric literals (0, 1, -1, max)
  - [ ] **Negate conditional**: if(x) -> if(!x)
- [ ] Implement mutation engine:
  - [ ] Parse source with tree-sitter
  - [ ] Identify mutation points in AST
  - [ ] Generate mutated source code
  - [ ] Write mutant to temporary file
  - [ ] Filter trivially equivalent mutants
- [ ] Implement mutant execution pipeline:
  - [ ] Run existing test suite against each mutant
  - [ ] Detect killed mutants (test failure = mutant killed)
  - [ ] Detect surviving mutants (all tests pass = mutant survived)
  - [ ] Timeout detection (mutant causes infinite loop)
  - [ ] Parallel execution of mutants (configurable concurrency)
  - [ ] Incremental: only test mutants in changed functions
- [ ] Implement surviving mutant analysis:
  - [ ] Group surviving mutants by mutation type
  - [ ] Group by function (which functions have weak tests)
  - [ ] Identify most critical survivors (based on mutation type severity)
  - [ ] Generate human-readable report of what each survivor means
- [ ] Implement automatic killing test generation:
  - [ ] For each surviving mutant, generate a test that detects the mutation
  - [ ] Include the mutation description in LLM prompt
  - [ ] "The function behaves the same when X is changed to Y — write a test that would catch this"
  - [ ] Validate generated test kills the specific mutant
- [ ] Implement `mutate` subcommand:
  - [ ] `mugen mutate` — run mutation testing
  - [ ] `--file` flag: focus on specific file
  - [ ] `--function` flag: focus on specific function
  - [ ] `--operators` flag: select which mutation operators to use
  - [ ] `--kill` flag: auto-generate tests for surviving mutants
  - [ ] `--jobs` flag: parallel execution concurrency
  - [ ] `--timeout` flag: per-mutant timeout
- [ ] Unit tests for each mutation operator
- [ ] Integration test: mutation test a sample function, verify surviving mutants

## Acceptance Criteria

- All mutation operators produce valid, compilable mutants
- Mutant execution correctly identifies killed vs surviving mutants
- Mutation score is calculated accurately
- Surviving mutant reports clearly explain what each survivor means
- Auto-generated killing tests actually kill the targeted mutant
- Parallel execution provides significant speedup
- Incremental mode reduces execution time for partial runs
- Mutation testing completes in reasonable time for small-medium functions

## Dependencies

- Step 002 (Code analyzer) for AST parsing and mutation
- Step 005 (LLM test generator) for killing test generation
- Step 006 (Validation loop) for validating generated killing tests
- Language test runners must be available
