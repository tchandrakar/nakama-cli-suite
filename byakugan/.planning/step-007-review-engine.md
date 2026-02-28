# Step 007: Review Engine (LLM-Powered)

## Objective

Build the core LLM-powered review engine that performs multi-pass code review using the `nakama-ai` shared abstraction layer. The engine runs 5 specialized review passes (correctness, security, performance, style, architecture), deduplicates findings across passes, and assigns severity scores to produce a prioritized list of review findings.

## Tasks

- [ ] Add `nakama-ai` shared crate dependency
- [ ] Design the review pass architecture:
  - Define `ReviewPass` enum: Correctness, Security, Performance, Style, Architecture
  - Define `ReviewFinding` struct: pass, severity, file, line_range, title, description, suggestion, confidence
  - Define `Severity` enum: Critical, Error, Warning, Info, Suggestion
  - Define `ReviewResult` struct: findings Vec<ReviewFinding>, summary String, verdict ReviewVerdict
- [ ] Implement system prompts for each review pass:
  - **Correctness pass**: Logic errors, off-by-one errors, null/nil handling, race conditions, deadlocks, unhandled edge cases
  - **Security pass**: Injection vulnerabilities (SQL, XSS, command), auth/authz issues, secret/credential exposure, insecure dependencies, OWASP top 10
  - **Performance pass**: N+1 queries, unbounded loops/recursion, memory leaks, unnecessary allocations, missing indexes, slow algorithmic paths
  - **Style pass**: Project coding standards compliance, naming consistency, documentation completeness, dead code, code clarity
  - **Architecture pass**: Separation of concerns, API contract changes, dependency direction violations, abstraction leaks, coupling issues
- [ ] Implement the review orchestrator:
  - Accept `DiffAnalysis` (from step 006) as input
  - Run all enabled passes sequentially or in parallel (configurable)
  - For each pass: construct LLM prompt with diff + semantic analysis + context + pass-specific system prompt
  - Parse LLM responses into structured `ReviewFinding` objects
  - Support configurable pass selection (users can disable passes)
- [ ] Implement finding deduplication:
  - Detect duplicate findings across passes (same file, same line range, similar description)
  - Merge duplicates, preserving the most severe rating and combining descriptions
  - Use text similarity (Levenshtein or cosine on embeddings) for fuzzy dedup
- [ ] Implement severity scoring:
  - Base severity from the LLM's assessment
  - Boost severity for security findings in sensitive files (auth, crypto, payments)
  - Boost severity for changes to public API surfaces
  - Lower severity for style issues in test files
  - Apply configurable severity threshold (filter findings below user's threshold)
- [ ] Implement the review summary generator:
  - Produce a natural language summary of all findings
  - Group findings by severity and pass
  - Determine overall verdict (Approve if no errors/criticals, Request Changes otherwise)
  - Include statistics (N findings, breakdown by severity)
- [ ] Implement token budget management:
  - Estimate token count for diff + context
  - If over budget, prioritize: changed lines > surrounding context > PR description > previous comments
  - Split large diffs into chunks and review each chunk separately, then merge findings
- [ ] Write unit tests with mock LLM responses
- [ ] Write integration tests for the full review pipeline

## Acceptance Criteria

- All 5 review passes produce structured findings from LLM responses
- Finding deduplication correctly merges similar findings across passes
- Severity scoring adjusts appropriately based on file context
- The review summary is coherent and accurately reflects all findings
- Token budget management prevents LLM context overflow
- Users can configure which passes to enable/disable
- The engine works with any LLM provider supported by nakama-ai
- Unit tests pass with mocked LLM responses

## Dependencies

- Step 006 (Diff Analysis Engine) must be complete
- `nakama-ai` shared crate must be available for LLM interaction
- At least one LLM provider must be configured (via nakama-ai)
