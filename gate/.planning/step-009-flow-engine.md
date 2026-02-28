# Step 009: Flow Engine

## Objective

Build a multi-step request flow engine that executes YAML-defined request chains with variable extraction from responses (via JSONPath), conditional branching, loops, assertions at each step, and step-by-step output for debugging API workflows.

## Tasks

- [ ] Define `Flow` struct: name, description, steps, variables, on_error (abort, continue, retry)
- [ ] Define `FlowStep` struct: name, request, extract, assert, condition, loop_config, on_success, on_failure
- [ ] Define `FlowExtract` struct: variable_name, source (body, header, status), path (JSONPath/XPath)
- [ ] Define `FlowAssert` struct: check_type (status, body, header, timing), expression, message
- [ ] Implement flow file format:
  - [ ] YAML definition files in `.gate/flows/`
  - [ ] Example:
    ```yaml
    name: User Registration Flow
    steps:
      - name: Create User
        request:
          method: POST
          url: /api/users
          body: { name: "Test User", email: "test@example.com" }
        extract:
          - var: user_id
            from: body
            path: $.data.id
          - var: auth_token
            from: header
            path: Authorization
        assert:
          - status: 201
          - body: $.data.id != null
      - name: Get User
        request:
          method: GET
          url: /api/users/{{user_id}}
          headers:
            Authorization: "{{auth_token}}"
        assert:
          - status: 200
          - body: $.data.name == "Test User"
    ```
- [ ] Implement variable extraction:
  - [ ] JSONPath extraction from response body
  - [ ] XPath extraction from XML response body
  - [ ] Header value extraction (by header name)
  - [ ] Status code extraction
  - [ ] Response time extraction
  - [ ] Regex extraction from body text
  - [ ] Store extracted values for subsequent steps
- [ ] Implement conditional branching:
  - [ ] `if` condition on step (skip step if condition false)
  - [ ] `if/else` branching (execute different steps based on condition)
  - [ ] Condition expressions: variable comparisons, status checks
  - [ ] `switch` on response status or body values
- [ ] Implement loops:
  - [ ] `repeat: N` — repeat step N times
  - [ ] `for_each: {{variable}}` — iterate over array variable
  - [ ] `while: condition` — repeat while condition is true
  - [ ] Loop variable access (`{{$index}}`, `{{$item}}`)
  - [ ] Break condition
- [ ] Implement assertions:
  - [ ] Status code assertions (exact, range: 2xx)
  - [ ] Body assertions (JSONPath value check)
  - [ ] Header assertions (header exists, value matches)
  - [ ] Timing assertions (response time < threshold)
  - [ ] Custom assertion expressions
  - [ ] Assertion failure modes: fail-fast, continue, warn
- [ ] Implement step-by-step output:
  - [ ] Show each step name and status (pass/fail)
  - [ ] Show extracted variables after each step
  - [ ] Show assertion results (pass/fail with details)
  - [ ] Show request/response summary per step
  - [ ] Progress indicator (step X of Y)
  - [ ] `--verbose` shows full request/response per step
- [ ] Implement error handling:
  - [ ] On error: abort flow, continue, retry step
  - [ ] Retry with configurable count and backoff
  - [ ] Cleanup steps (always-run steps for teardown)
  - [ ] Error summary at flow end
- [ ] Implement `flow` subcommand:
  - [ ] `gate flow run <name>` — execute a flow
  - [ ] `gate flow list` — list available flows
  - [ ] `gate flow validate <name>` — validate flow definition
  - [ ] `--step` flag: start from specific step
  - [ ] `--env` flag: use specific environment
  - [ ] `--var` flag: override flow variables
- [ ] Unit tests for variable extraction (JSONPath, header)
- [ ] Unit tests for conditional logic and loops
- [ ] Unit tests for assertion evaluation
- [ ] Integration test: multi-step flow with mock server

## Acceptance Criteria

- Flows execute multi-step request chains sequentially
- Variables are extracted from responses and used in subsequent steps
- JSONPath extraction works for nested JSON structures
- Conditional branching correctly skips/selects steps
- Loops iterate correctly (repeat, for_each, while)
- Assertions catch mismatches and report clear failures
- Step-by-step output shows progress and results
- Error handling respects abort/continue/retry configuration
- Cleanup steps always execute (even on failure)

## Dependencies

- Step 002 (Request builder) for request definitions
- Step 003 (HTTP engine) for request execution
- Step 007 (Environment manager) for variable substitution
- jsonpath-rust or similar for JSONPath queries
- serde_yaml for flow definition parsing
