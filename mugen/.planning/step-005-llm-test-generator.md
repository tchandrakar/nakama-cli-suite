# Step 005: LLM Test Generator

## Objective

Integrate with nakama-ai to generate test code using LLM. Build context-rich prompts that include source code, function signatures, existing tests, and detected conventions. Generate well-structured test code with proper mock setups that align with the project's testing style.

## Tasks

- [ ] Add nakama-ai dependency and initialize AI client
- [ ] Define `TestGenerationRequest` struct: function_info, strategy, conventions, context, existing_tests
- [ ] Define `GeneratedTest` struct: test_code, test_name, file_path, imports, description, confidence
- [ ] Implement context assembly:
  - [ ] Source function code (full implementation)
  - [ ] Function signature with type information
  - [ ] Existing tests for the same function (to avoid duplicates)
  - [ ] Existing tests in the same file (for style reference)
  - [ ] Detected conventions (framework, assertion style, naming)
  - [ ] Related type definitions (structs, enums used in params/returns)
  - [ ] Import context (what modules are available)
  - [ ] Strategy instruction ("generate property-based tests" etc.)
  - [ ] Truncate to token budget (configurable, default 4096)
- [ ] Implement prompt templates per strategy:
  - [ ] Unit test prompt: test individual behavior, all branches
  - [ ] Property test prompt: define properties, generators, shrinking
  - [ ] Integration test prompt: setup, execute, verify, cleanup
  - [ ] Mock test prompt: mock dependencies, verify interactions
  - [ ] Snapshot test prompt: capture output, assert stability
  - [ ] Edge case prompt: boundaries, nulls, empty, overflow
  - [ ] Fuzz test prompt: random inputs, invariant assertions
- [ ] Implement prompt templates per language:
  - [ ] Rust-specific (ownership, lifetimes, Result handling)
  - [ ] TypeScript-specific (type assertions, async/await)
  - [ ] Python-specific (fixtures, parametrize, type hints)
  - [ ] Go-specific (table-driven, subtests, testify)
  - [ ] Java-specific (JUnit lifecycle, Mockito, Spring)
- [ ] Implement test code generation:
  - [ ] Send assembled prompt to nakama-ai
  - [ ] Parse LLM response (extract code blocks)
  - [ ] Clean up generated code (fix indentation, remove markdown)
  - [ ] Validate generated code structure (AST parse)
- [ ] Implement mock setup generation:
  - [ ] Detect dependencies needing mocks from side effect analysis
  - [ ] Generate mock definitions/declarations
  - [ ] Generate mock expectations/verifications
  - [ ] Handle mock library-specific syntax
- [ ] Implement batch generation:
  - [ ] Generate tests for multiple functions in one pass
  - [ ] Group by file/module for efficient context sharing
  - [ ] Respect rate limits on LLM API
- [ ] Add `--model` flag to select LLM model
- [ ] Add `--temperature` flag for generation creativity
- [ ] Unit tests with mocked LLM responses
- [ ] Quality test: generated tests for sample functions compile and are meaningful

## Acceptance Criteria

- Generated tests match project's framework and assertion style
- Generated tests compile/pass syntax check without manual editing
- Mock setups are correct for the detected mock library
- Context assembly stays within token budget
- Batch generation handles multiple functions efficiently
- Tests cover multiple branches of the target function
- Generated test names follow project naming conventions
- Each generated test has a clear description of what it tests

## Dependencies

- Step 002 (Code analyzer) for function info and side effects
- Step 003 (Convention detector) for framework and style info
- Step 004 (Test strategy) for strategy selection
- nakama-ai shared crate for LLM completion
- nakama-vault for API key retrieval (via Step 007)
