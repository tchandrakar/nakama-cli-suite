# Step 003: Convention Detector

## Objective

Build a convention detection engine that discovers test file locations, identifies the testing framework in use, determines assertion styles, detects mock libraries, and learns naming conventions from existing tests. Generated tests must match the project's existing style.

## Tasks

- [ ] Define `TestConventions` struct: framework, file_pattern, assertion_style, mock_library, naming_pattern, setup_pattern, organization
- [ ] Implement test file location detection:
  - [ ] Rust: `#[cfg(test)] mod tests`, `tests/` directory, `*_test.rs`
  - [ ] JavaScript/TypeScript: `__tests__/`, `*.test.ts`, `*.spec.ts`, same dir vs `test/` dir
  - [ ] Python: `tests/`, `test_*.py`, `*_test.py`
  - [ ] Go: `*_test.go` (same package)
  - [ ] Java: `src/test/java/`, `*Test.java`, `*Tests.java`
  - [ ] Detect project-specific custom patterns
- [ ] Implement framework detection:
  - [ ] Rust: built-in `#[test]`, tokio::test, rstest, proptest
  - [ ] JavaScript: Jest, Mocha, Vitest, Playwright, Cypress
  - [ ] TypeScript: Jest, Vitest, ts-jest
  - [ ] Python: pytest, unittest, nose2
  - [ ] Go: testing package, testify, gomock
  - [ ] Java: JUnit 4, JUnit 5, TestNG, Mockito
  - [ ] Detect from: imports, config files (jest.config, pytest.ini), package.json
- [ ] Implement assertion style detection:
  - [ ] expect/assert/should patterns
  - [ ] Matcher style (toBe, toEqual, assertEqual)
  - [ ] Assertion library (chai, assertj, hamcrest)
  - [ ] Learn from existing test files
- [ ] Implement mock library detection:
  - [ ] Rust: mockall, mockito
  - [ ] JavaScript/TypeScript: jest.mock, sinon, nock, msw
  - [ ] Python: unittest.mock, pytest-mock, responses
  - [ ] Go: gomock, testify/mock
  - [ ] Java: Mockito, PowerMock, WireMock
- [ ] Implement naming convention detection:
  - [ ] Function naming: test_*, it('should...'), describe/it, test('...'), @Test
  - [ ] Descriptive vs technical naming
  - [ ] BDD style (given/when/then) vs assertion style
  - [ ] Learn from existing test names in project
- [ ] Implement setup/teardown pattern detection:
  - [ ] beforeAll/beforeEach, setUp/tearDown, @Before/@After
  - [ ] Fixture usage patterns
  - [ ] Test data builders
  - [ ] Database setup/cleanup patterns
- [ ] Implement convention scoring:
  - [ ] Analyze multiple test files to build confidence
  - [ ] Handle mixed conventions (use majority pattern)
  - [ ] Fall back to language defaults when no tests exist
- [ ] Add `--framework` CLI flag to override detection
- [ ] Unit tests for each language's convention detection
- [ ] Integration test: detect conventions from real project structures

## Acceptance Criteria

- Test file locations are correctly identified for all supported languages
- Framework is detected from imports, config files, and code patterns
- Assertion style matches the project's existing tests
- Mock library is correctly identified
- Naming conventions are learned from existing tests
- Generated tests use detected conventions consistently
- Framework override flag allows manual specification
- Handles projects with no existing tests (falls back to language defaults)

## Dependencies

- Step 001 (CLI scaffold)
- Step 002 (Code analyzer) for AST parsing of existing test files
- Access to project configuration files (package.json, Cargo.toml, etc.)
