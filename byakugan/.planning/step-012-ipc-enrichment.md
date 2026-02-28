# Step 012: IPC Enrichment

## Objective

Integrate byakugan with other nakama suite tools via inter-process communication (IPC). Consume Jira context from itachi, codebase knowledge from senku, test coverage from mugen, and Confluence documentation from itachi to produce richer, more context-aware reviews.

## Tasks

- [ ] Define IPC data contracts (JSON schemas) for byakugan input:
  - `JiraContext` schema: issue key, summary, description, acceptance criteria, linked issues, status
  - `CodebaseKnowledge` schema: related files, module structure, dependency graph, patterns
  - `TestCoverage` schema: file-level coverage percentages, uncovered lines, test file mappings
  - `DocumentationContext` schema: relevant Confluence pages, ADRs, design docs
- [ ] Implement itachi integration (Jira context):
  - Accept Jira context via `--context=stdin` (pipe from itachi)
  - Parse linked Jira issue keys from PR description (PROJ-NNN pattern)
  - If itachi is available, fetch issue details for linked tickets
  - Inject Jira acceptance criteria into review prompts ("verify these requirements are met")
  - Check if PR changes align with the Jira issue description
- [ ] Implement senku integration (codebase knowledge):
  - Accept codebase context via pipe or IPC
  - Use senku's knowledge of the codebase to understand:
    - What module/component is being changed
    - What other files might be affected
    - Coding patterns used in the project
  - Inject codebase context into review prompts for more relevant reviews
- [ ] Implement mugen integration (test coverage):
  - Accept test coverage data via pipe or IPC
  - Identify changed lines that lack test coverage
  - Generate findings for untested code changes
  - Suggest which test files should be updated
  - Flag regressions (previously covered code now uncovered)
- [ ] Implement itachi integration (Confluence docs):
  - Accept documentation context via pipe or IPC
  - Fetch relevant design docs, ADRs, and runbooks for the changed component
  - Check if changes comply with documented architecture decisions
  - Flag changes that may require documentation updates
- [ ] Implement the `--enrich` flag:
  - `byakugan review --enrich=jira,tests,docs` -- specify which enrichments to use
  - `byakugan review --enrich=all` -- use all available enrichments
  - Auto-detect available tools and use them if `--enrich=auto`
- [ ] Implement JSON output for downstream consumption:
  - Export review results in a format consumable by tensai (daily brief)
  - Export findings in a format consumable by mugen (test generation suggestions)
  - Include enrichment metadata in output (which context sources were used)
- [ ] Write integration tests with mock IPC data

## Acceptance Criteria

- Jira context from itachi enriches reviews with requirement checking
- Codebase knowledge from senku makes reviews more contextually aware
- Test coverage from mugen identifies untested code changes
- Documentation from itachi enables architecture compliance checking
- The `--enrich` flag correctly controls which enrichments are active
- JSON output includes enrichment metadata for downstream tools
- Graceful degradation: reviews work fine when enrichment tools are unavailable
- IPC data contracts are documented and versioned

## Dependencies

- Step 007 (Review Engine) must be complete
- Step 010 (Output Layer) must be complete
- `itachi` tool must expose JSON output for Jira/Confluence data
- `senku` tool must expose JSON output for codebase knowledge
- `mugen` tool must expose JSON output for test coverage data
- IPC contracts should be defined in the `shared` crate for consistency
