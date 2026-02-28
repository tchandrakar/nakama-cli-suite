# Step 011: Property-Based and Fuzz Testing

## Objective

Build a property-based and fuzz testing generator that defines testable properties from function signatures, sets up appropriate value generators, implements shrinking strategies for minimal failing examples, and generates fuzz test harnesses.

## Tasks

- [ ] Define `Property` struct: description, input_generators, invariant_assertion, shrink_strategy
- [ ] Define `FuzzHarness` struct: target_function, input_generator, crash_detector, corpus_path
- [ ] Implement property inference from signatures:
  - [ ] Idempotency: f(f(x)) == f(x) (for normalizers, formatters)
  - [ ] Inverse: decode(encode(x)) == x (for serializers)
  - [ ] Commutativity: f(a, b) == f(b, a) (for symmetric operations)
  - [ ] Monotonicity: a <= b implies f(a) <= f(b)
  - [ ] Length preservation: len(f(x)) == len(x) (for maps, transforms)
  - [ ] No-crash: f(x) does not panic for any valid x
  - [ ] Type invariants: output satisfies type constraints
  - [ ] Domain-specific: infer from function name and docs
- [ ] Implement value generators:
  - [ ] Primitive generators: integers (range-bounded), floats, bools, chars
  - [ ] String generators: ascii, unicode, alphanumeric, with length bounds
  - [ ] Collection generators: Vec, HashMap, with size bounds
  - [ ] Custom type generators: construct from field generators
  - [ ] Composite generators: tuples, enums (all variants)
  - [ ] Constrained generators: values satisfying preconditions
- [ ] Implement shrinking strategies:
  - [ ] Numeric shrinking: toward zero
  - [ ] String shrinking: remove characters, simplify to ascii
  - [ ] Collection shrinking: remove elements, shrink elements
  - [ ] Custom type shrinking: shrink fields independently
  - [ ] Binary search shrinking for numeric bounds
- [ ] Implement property test generation:
  - [ ] Rust: proptest / quickcheck macros
  - [ ] Python: hypothesis decorators
  - [ ] JavaScript/TypeScript: fast-check
  - [ ] Go: testing/quick or gopter
  - [ ] Java: jqwik
  - [ ] Use LLM to generate appropriate property definitions
- [ ] Implement fuzz test generation:
  - [ ] Rust: cargo-fuzz / libfuzzer harness
  - [ ] Python: atheris harness
  - [ ] Go: go-fuzz harness
  - [ ] Generate corpus seed files
  - [ ] Detect crash-inducing inputs
- [ ] Implement `fuzz` subcommand:
  - [ ] `mugen fuzz <file>` — generate fuzz/property tests
  - [ ] `--function` flag: specific function
  - [ ] `--iterations` flag: number of test iterations
  - [ ] `--property` flag: specify property to test
  - [ ] `--shrink` flag: enable/disable shrinking
  - [ ] `--corpus` flag: corpus directory for fuzz testing
- [ ] Unit tests for generators and shrinking
- [ ] Integration test: property test finds bug in intentionally buggy function

## Acceptance Criteria

- Properties are correctly inferred from function signatures and names
- Generators produce valid values for all supported types
- Shrinking produces minimal failing examples
- Generated property tests use the correct framework per language
- Fuzz harnesses are valid and can be run by language-specific fuzz tools
- Property tests actually find bugs in sample buggy code
- Generated tests include clear property descriptions
- Iteration count is configurable and defaults are reasonable

## Dependencies

- Step 002 (Code analyzer) for function signatures and type information
- Step 003 (Convention detector) for framework-appropriate test generation
- Step 005 (LLM test generator) for property definition assistance
- Step 006 (Validation loop) for test validation
- Property testing libraries must be available per language
