# Step 010: Edge Case Generation

## Objective

Build an edge case test generator that systematically produces boundary values for numeric types, empty/null/undefined values, string edge cases (empty, unicode, extremely long), and collection edge cases. These tests catch the bugs that typical happy-path testing misses.

## Tasks

- [ ] Define `EdgeCase` struct: value, category, description, risk_level
- [ ] Define `EdgeCaseCategory` enum: Boundary, Empty, Null, Overflow, Unicode, Length, TypeCoercion, Precision
- [ ] Implement numeric edge cases:
  - [ ] Zero (0, 0.0, -0.0)
  - [ ] One (1, -1)
  - [ ] Min/max for type (i32::MIN, i32::MAX, f64::INFINITY, f64::NAN)
  - [ ] Boundary: N-1, N, N+1 for any constant N in the function
  - [ ] Powers of 2 (for bit manipulation code)
  - [ ] Large numbers (overflow potential)
  - [ ] Negative numbers (when positive expected)
  - [ ] Floating point precision issues (0.1 + 0.2)
- [ ] Implement empty/null/undefined edge cases:
  - [ ] Null / None / nil
  - [ ] Empty string ""
  - [ ] Empty collection ([], {}, empty set)
  - [ ] Optional::None / undefined
  - [ ] Default/zero values for custom types
- [ ] Implement string edge cases:
  - [ ] Empty string
  - [ ] Single character
  - [ ] Very long strings (10K+ characters)
  - [ ] Unicode: emojis, RTL text, combining characters, zero-width chars
  - [ ] Special characters: newlines, tabs, null bytes
  - [ ] SQL injection patterns (for security awareness)
  - [ ] HTML/script injection patterns
  - [ ] Whitespace-only strings
  - [ ] Strings with leading/trailing whitespace
- [ ] Implement collection edge cases:
  - [ ] Empty collection
  - [ ] Single element
  - [ ] Duplicate elements
  - [ ] Very large collections (performance testing)
  - [ ] Nested collections (deeply nested)
  - [ ] Collections with null/None elements
  - [ ] Sorted, reverse-sorted, already-sorted inputs (for sort algorithms)
- [ ] Implement type-specific edge cases:
  - [ ] Dates: epoch, far future, DST transitions, leap years, Feb 29
  - [ ] Files: non-existent, empty, permissions denied, very large
  - [ ] URLs: malformed, missing scheme, IPv6, special characters
  - [ ] Regex: catastrophic backtracking patterns
- [ ] Implement edge case generation from function analysis:
  - [ ] Analyze parameter types to select relevant edge cases
  - [ ] Analyze conditional branches to find boundary conditions
  - [ ] Analyze error handling to find unchecked edge cases
  - [ ] Generate test values specific to the function's domain
- [ ] Implement `edge` subcommand:
  - [ ] `mugen edge <file>` — generate edge case tests
  - [ ] `--function` flag: specific function
  - [ ] `--category` flag: specific edge case category
  - [ ] `--exhaustive` flag: generate all applicable edge cases
  - [ ] `--quick` flag: only most critical edge cases
- [ ] Unit tests for edge case value generation per type
- [ ] Integration test: generate edge case tests for sample functions

## Acceptance Criteria

- Numeric edge cases include all boundary values for the parameter type
- String edge cases cover empty, unicode, and injection patterns
- Collection edge cases cover empty, single, and oversized
- Edge cases are relevant to the function's parameter types (no string tests for int params)
- Generated tests compile and have meaningful assertions
- Edge cases detect real bugs in sample code (validated against known buggy functions)
- `--exhaustive` mode generates comprehensive coverage
- `--quick` mode generates only highest-risk edge cases

## Dependencies

- Step 002 (Code analyzer) for parameter type information
- Step 005 (LLM test generator) for test code generation with edge case values
- Step 006 (Validation loop) for test validation
