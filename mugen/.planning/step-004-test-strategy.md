# Step 004: Test Strategy Selection

## Objective

Build a strategy selection engine that analyzes function characteristics to choose the optimal testing approach for each function. Pure functions get unit + property tests, complex logic gets builder patterns, database operations get integration tests, APIs get mock tests, output-dependent code gets snapshot tests, and numeric code gets edge case tests.

## Tasks

- [ ] Define `TestStrategy` enum: Unit, Property, Builder, Integration, Mock, Snapshot, EdgeCase, Fuzz, Combined
- [ ] Define `StrategyReason` struct: strategy, confidence, reasoning, function_characteristics
- [ ] Implement strategy classifier:
  - [ ] **Pure functions** (no side effects, deterministic) -> Unit + Property tests
  - [ ] **Complex logic** (many branches, state machines) -> Builder pattern tests
  - [ ] **Database operations** (SQL, ORM calls) -> Integration tests with test DB
  - [ ] **API consumers** (HTTP calls, external services) -> Mock/stub tests
  - [ ] **API endpoints** (handlers, controllers) -> Request/response mock tests
  - [ ] **Output-dependent** (formatters, serializers, renderers) -> Snapshot tests
  - [ ] **Numeric/math** (calculations, algorithms) -> Edge case + boundary tests
  - [ ] **Parsers** (input processing, validation) -> Fuzz + edge case tests
  - [ ] **State machines** (FSM, workflows) -> State transition tests
- [ ] Implement characteristic analysis:
  - [ ] Count branches (if/else, match, switch)
  - [ ] Detect recursion
  - [ ] Detect iteration (loops, iterators)
  - [ ] Detect error handling patterns (Result, try/catch, throw)
  - [ ] Detect async patterns
  - [ ] Calculate cyclomatic complexity
- [ ] Implement strategy combination:
  - [ ] Some functions benefit from multiple strategies
  - [ ] Primary strategy + supplementary strategies
  - [ ] Example: parser = Unit + EdgeCase + Fuzz
  - [ ] Example: DB function = Integration + Mock (for unit testing)
- [ ] Implement confidence scoring:
  - [ ] High confidence: clear indicators (e.g., pure function with typed params)
  - [ ] Medium confidence: ambiguous (e.g., function with some side effects)
  - [ ] Low confidence: unknown patterns (fall back to unit test)
- [ ] Add `--strategy` CLI flag to override automatic selection
- [ ] Add `--prefer` flag to bias toward certain strategies (e.g., --prefer unit)
- [ ] Unit tests with sample functions of each type
- [ ] Validation: strategy selection matches expert human judgment on test corpus

## Acceptance Criteria

- Pure functions are correctly identified and get unit + property strategies
- Side-effectful functions get appropriate mock/integration strategies
- Complex functions (high cyclomatic complexity) get builder pattern strategies
- Numeric functions get edge case strategies with boundary values
- Strategy confidence is calibrated (high confidence = correct >90% of the time)
- Multiple strategies can be combined per function
- Override flags work correctly
- Strategy selection completes in <100ms per function

## Dependencies

- Step 002 (Code analyzer) for function characteristics and side effect detection
- Step 003 (Convention detector) for framework-appropriate strategy mapping
- No external service dependencies (local analysis only)
