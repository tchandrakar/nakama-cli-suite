# Step 008: Custom Rules Engine

## Objective

Implement the custom rules system that loads project-specific review rules from `.byakugan.yml`, injects them into LLM review prompts, and supports both pattern-based rules (regex matching) and semantic rules (LLM-interpreted natural language constraints).

## Tasks

- [ ] Define the `.byakugan.yml` configuration schema:
  ```yaml
  rules:
    - name: string (unique identifier)
      description: string (human-readable, injected into LLM prompt)
      severity: critical | error | warning | info
      type: pattern | semantic
      pattern: string (regex, only for pattern type)
      context: string (when to apply, for semantic type)
      exclude: [glob patterns] (files to skip)
      include: [glob patterns] (files to target, default: all)
      enabled: bool (default: true)
  ```
- [ ] Implement `.byakugan.yml` loader:
  - Search for config file in current directory, then parent directories up to repo root
  - Parse YAML using `serde_yaml`
  - Validate rule definitions (check required fields, compile regex patterns)
  - Report clear errors for malformed rules
- [ ] Implement pattern-based rules:
  - Compile regex patterns from rule definitions
  - Scan diff lines for pattern matches
  - Generate `ReviewFinding` for each match with the rule's severity and description
  - Respect include/exclude glob patterns for file filtering
  - Handle multi-line patterns across hunk boundaries
- [ ] Implement semantic rules:
  - Inject rule descriptions into the LLM review prompt as additional constraints
  - Format rules as a "project conventions" section in the system prompt
  - Include the rule's context field to guide the LLM on when to apply the rule
  - Parse LLM responses to attribute findings to specific custom rules
- [ ] Implement the `rules` CLI subcommand:
  - `byakugan rules list` -- display all loaded rules with status
  - `byakugan rules add` -- interactive rule creation wizard
  - `byakugan rules edit` -- open `.byakugan.yml` in $EDITOR
  - `byakugan rules test <file>` -- test pattern rules against a file
  - `byakugan rules validate` -- validate the configuration file
- [ ] Integrate custom rules into the review engine (step 007):
  - Run pattern rules as a separate pass before LLM passes
  - Inject semantic rules into each LLM pass's system prompt
  - Include custom rule findings in the final deduplication and scoring
- [ ] Provide built-in rule templates:
  - `no-console-log`: warn on console.log in non-test files
  - `require-error-handling`: warn on unhandled async errors
  - `no-hardcoded-secrets`: error on hardcoded API keys, passwords
  - `require-tests`: warn on new functions without corresponding tests
- [ ] Write unit tests for YAML parsing, regex matching, and rule integration

## Acceptance Criteria

- `.byakugan.yml` is correctly loaded from the project root
- Pattern rules detect regex matches in diffs and produce findings
- Semantic rules are correctly injected into LLM prompts
- The `rules` subcommand works for listing, adding, editing, testing, and validating
- Include/exclude glob patterns correctly filter target files
- Custom rule findings are deduplicated with LLM findings
- Invalid configurations produce clear, actionable error messages
- Built-in templates provide useful starting points

## Dependencies

- Step 007 (Review Engine) must be complete
- `serde_yaml` crate for YAML parsing
- `regex` crate for pattern compilation
- `glob` or `globset` crate for file pattern matching
