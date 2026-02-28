# Mugen — Test Generator CLI

> "Relentless, covers every angle." — Inspired by Mugen from Samurai Champloo.

## Overview

Mugen is an AI-powered test generator that analyzes your code and generates comprehensive test suites. It finds edge cases you didn't think of, runs mutation testing to verify test quality, and relentlessly pursues full coverage.

## Core Commands

| Command | Description |
|---------|-------------|
| `mugen gen <file\|module>` | Generate tests for a file or module |
| `mugen cover` | Analyze coverage gaps and generate tests to fill them |
| `mugen mutate <file>` | Run mutation testing to find weak tests |
| `mugen edge <function>` | Generate edge case tests for a specific function |
| `mugen fuzz <function>` | Generate property-based / fuzz tests |
| `mugen review <test_file>` | Review existing tests and suggest improvements |
| `mugen watch` | Watch for code changes and auto-generate tests |

## Architecture

```
┌───────────────────────────────────────────────────┐
│                    CLI Layer                       │
│          (commands, watch mode, reports)           │
├───────────────────────────────────────────────────┤
│                 Code Analyzer                      │
│  ┌────────────┐ ┌────────────┐ ┌───────────────┐  │
│  │ AST Parser │ │ Type       │ │ Coverage      │  │
│  │ (tree-     │ │ Extractor  │ │ Reader        │  │
│  │  sitter)   │ │ (params,   │ │ (lcov,        │  │
│  │            │ │  returns,  │ │  cobertura,   │  │
│  │            │ │  generics) │ │  istanbul)    │  │
│  └────────────┘ └────────────┘ └───────────────┘  │
│  ┌────────────────────────────────────────────┐    │
│  │ Function Signature Analyzer                │    │
│  │  - Input types and constraints             │    │
│  │  - Return types and error types            │    │
│  │  - Side effects (I/O, mutations, globals)  │    │
│  │  - Dependencies (what to mock)             │    │
│  └────────────────────────────────────────────┘    │
├───────────────────────────────────────────────────┤
│                 Test Strategy Engine               │
│  ┌─────────────────────────────────────────────┐   │
│  │ Strategy selection per function:            │   │
│  │  - Unit test (pure functions)               │   │
│  │  - Integration test (I/O, DB, API)          │   │
│  │  - Property-based test (numeric, string)    │   │
│  │  - Snapshot test (serialization, rendering) │   │
│  │  - Edge case generation (boundaries, nulls) │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│              LLM Test Generator                    │
│  ┌─────────────────────────────────────────────┐   │
│  │ Input:                                      │   │
│  │  - Source code of target function/module    │   │
│  │  - Function signatures and types            │   │
│  │  - Existing tests (to avoid duplication)    │   │
│  │  - Testing framework conventions            │   │
│  │  - Coverage gaps (if running in cover mode) │   │
│  │                                             │   │
│  │ Output:                                     │   │
│  │  - Test code in the project's test style    │   │
│  │  - Test descriptions / names                │   │
│  │  - Mock setup code                          │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│              Mutation Testing Engine                │
│  ┌─────────────────────────────────────────────┐   │
│  │ Mutation operators:                         │   │
│  │  - Arithmetic (+ → -, * → /)               │   │
│  │  - Conditional (== → !=, < → <=)           │   │
│  │  - Boundary (off-by-one)                    │   │
│  │  - Return value (true → false, 0 → 1)      │   │
│  │  - Statement deletion                       │   │
│  │                                             │   │
│  │ Process:                                    │   │
│  │  1. Apply mutation to source                │   │
│  │  2. Run test suite                          │   │
│  │  3. If tests still pass → weak coverage     │   │
│  │  4. Generate test to kill the mutant        │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│              Test Runner & Validator               │
│  ┌─────────────────────────────────────────────┐   │
│  │ - Run generated tests to verify they pass   │   │
│  │ - Compile check before writing to disk      │   │
│  │ - Auto-fix common issues (imports, mocks)   │   │
│  └─────────────────────────────────────────────┘   │
└───────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Understand Before Generating
Mugen doesn't blindly generate tests. It first analyzes:
- **Function signatures** — input types, return types, error types
- **Code paths** — branches, loops, early returns
- **Dependencies** — what needs to be mocked or stubbed
- **Existing tests** — to avoid duplicating coverage

This analysis feeds into a strategy selection step before any test code is generated.

### 2. Test Strategy Selection
Different code needs different test approaches:

| Code Pattern | Test Strategy |
|-------------|---------------|
| Pure function with primitives | Unit test + property-based test |
| Function with complex objects | Unit test with builder pattern |
| Database interaction | Integration test with test DB |
| API call | Unit test with mock HTTP client |
| String/JSON output | Snapshot test |
| Numeric computation | Edge case (boundary values, overflow) |

### 3. Convention Detection
Mugen detects and follows the project's existing test conventions:
- Test file location (`__tests__/`, `*.test.ts`, `*_test.go`, `tests/`)
- Test framework (Jest, pytest, Go testing, Rust #[test], JUnit)
- Assertion style (expect, assert, should)
- Mock library (mockall, unittest.mock, jest.mock)
- Naming conventions

Generated tests look like they were written by the same team.

### 4. Mutation Testing
`mugen mutate` goes beyond coverage:
- Introduces small bugs (mutations) into the source code
- Runs the test suite against each mutation
- If tests still pass, that's a **surviving mutant** — the tests are weak there
- Mugen then generates new tests specifically to kill surviving mutants

This ensures tests are actually asserting correct behavior, not just executing code.

### 5. Validation Loop
Every generated test goes through:
1. **Syntax check** — parse the generated code
2. **Compilation check** — ensure it compiles/type-checks
3. **Execution** — run the test to verify it passes
4. **Auto-fix** — if it fails for fixable reasons (missing imports, wrong mock setup), fix and retry

Only validated tests are written to disk.

## Data Flow — Generate

```
Target file/function
        │
        ▼
  AST Parse ──→ function signatures, code paths
        │
        ▼
  Existing Test Scan ──→ what's already covered
        │
        ▼
  Strategy Selection ──→ unit / integration / property / snapshot
        │
        ▼
  LLM Test Generation ──→ test code
        │
        ▼
  Validation Loop ──→ compile → run → pass?
        │                              │
        │    ◄── auto-fix ◄── NO ──────┘
        │
        ▼ YES
  Write to test file
```

## Configuration

File: `~/.mugen/config.toml`

```toml
[llm]
provider = "anthropic"
model = "claude-sonnet-4-6"

[analysis]
max_function_size_lines = 200
include_private_functions = false

[generation]
style = "auto"                      # auto-detect from existing tests
max_tests_per_function = 10
include_edge_cases = true
include_property_tests = false      # opt-in, can be slow
validate_before_write = true

[mutation]
operators = ["arithmetic", "conditional", "boundary", "return_value"]
max_mutants_per_function = 20
timeout_per_mutant_seconds = 30

[watch]
debounce_ms = 2000
auto_generate = true
```

## Tech Stack

- **Language:** Rust
- **CLI framework:** clap
- **AST parsing:** tree-sitter with language grammars
- **Coverage parsing:** lcov / cobertura / istanbul format readers
- **Mutation engine:** custom, AST-based mutation operators
- **Test runners:** delegates to language-native runners (cargo test, jest, pytest, go test)
- **LLM integration:** shared nakama LLM abstraction layer
