# Step 011: Comparison Mode

## Objective

Implement the `kami compare <a> vs <b>` command that performs side-by-side research on two (or more) topics, generates a structured comparison table with sourced evidence for each point, and synthesizes a recommendation or summary.

## Tasks

- [ ] Implement comparison query parsing:
  - Parse `<a> vs <b>` format from command arguments
  - Support multiple items: `<a> vs <b> vs <c>`
  - Extract comparison dimensions from the query context
  - Use Gemini to identify the most relevant comparison criteria
- [ ] Implement parallel research per item:
  - For each item, perform grounded search to gather information
  - Focus searches on the identified comparison criteria
  - Collect structured data points per criterion per item
  - Configurable search depth (quick: 1 search per item, thorough: 3-5 per item)
- [ ] Implement comparison table generation:
  - Build a structured table with:
    - Rows: comparison criteria (features, performance, cost, ecosystem, etc.)
    - Columns: items being compared (a, b, c...)
    - Cells: evidence-backed assessments with citations
  - Identify clear winners per criterion (if applicable)
  - Highlight key differentiators
- [ ] Implement sourced evidence per point:
  - Each cell in the comparison table links to evidence sources
  - Include inline citations for specific claims
  - Distinguish between official sources, benchmarks, and community opinions
- [ ] Implement synthesis and recommendation:
  - Generate a summary paragraph highlighting key differences
  - Provide a "best for" recommendation per use case (if applicable)
  - Note areas where items are equivalent
  - Flag areas where evidence is insufficient for comparison
- [ ] Implement output formatting:
  - Terminal: formatted table with aligned columns, color-coded winners
  - Markdown: clean comparison table with hyperlinked citations
  - JSON: structured comparison data with all evidence
- [ ] Write unit tests for comparison parsing and table generation
- [ ] Write integration tests for the full comparison pipeline

## Acceptance Criteria

- `kami compare X vs Y` produces a well-structured comparison
- Comparison criteria are relevant to the items being compared
- Each comparison point is backed by cited evidence
- The comparison table is readable in terminal, markdown, and JSON formats
- Multiple items (3+) are supported
- Synthesis provides actionable insights
- Research is thorough enough to produce meaningful comparisons

## Dependencies

- Step 003 (Gemini Provider) must be complete for grounded search
- Step 004 (Google Search) must be complete for evidence gathering
- Step 005 (Synthesis Engine) must be complete for citation formatting
- `nakama-ui` for terminal table rendering
