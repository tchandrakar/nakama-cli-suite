# Step 007: Deep Research

## Objective

Implement the `kami deep` command that performs multi-round research by decomposing complex queries into sub-queries, executing them iteratively with Gemini grounded search, optionally fetching and reading key URLs for deeper content extraction, and synthesizing all findings into a comprehensive, cited report.

## Tasks

- [ ] Implement query decomposition:
  - Use Gemini/nakama-ai to break a complex query into 3-7 sub-queries
  - Each sub-query should explore a different facet of the original question
  - Order sub-queries by dependency (foundational queries first)
  - Generate a search plan with estimated time and API cost
  - Display the plan to the user before execution (unless `--auto` flag)
- [ ] Implement multi-round search execution:
  - Execute each sub-query via Gemini grounded search
  - Collect results from each round
  - Support adaptive querying: adjust later sub-queries based on earlier findings
  - Configurable max sub-queries (default: 5, from config `deep_research.max_sub_queries`)
  - Configurable timeout per step (default: 30s, from config)
  - Display progress with nakama-ui spinner showing current sub-query
- [ ] Implement URL fetching for deeper content:
  - Identify key URLs from search results that warrant full content extraction
  - Fetch URL content via HTTP (reqwest with configurable user-agent)
  - Convert HTML to clean markdown using `scraper` crate (readability algorithm)
  - Extract text from PDFs using `pdf-extract` or similar
  - Respect robots.txt (check before fetching)
  - Configurable max URL fetches (default: 10)
  - Cache fetched content to avoid re-fetching
- [ ] Implement cross-query synthesis:
  - Combine findings from all sub-queries into a unified knowledge base
  - Detect and resolve contradictions between sources
  - Identify consensus (multiple sources agree) vs. disputed claims
  - Highlight gaps (aspects of the query not well-covered by results)
  - Use nakama-ai for the final synthesis pass
- [ ] Implement structured report generation:
  - Generate report with sections:
    - Executive Summary
    - Detailed Findings (per sub-query or per topic)
    - Source Analysis (agreement/disagreement)
    - Gaps and Limitations
    - All Sources with annotations
  - Include inline citations throughout
  - Support output formats: terminal (rich), markdown (file), JSON (structured)
- [ ] Implement research session management:
  - Save intermediate results for resume capability
  - Support `--resume` flag to continue a previous research session
  - Store sessions in `~/.kami/research/`
- [ ] Write unit tests for query decomposition and synthesis
- [ ] Write integration tests for the full research pipeline (mocked APIs)

## Acceptance Criteria

- Complex queries are decomposed into relevant sub-queries
- Multi-round search executes all sub-queries and collects results
- URL fetching extracts clean content from HTML pages and PDFs
- Cross-query synthesis produces coherent, comprehensive answers
- Contradictions between sources are identified and highlighted
- The structured report is well-organized with inline citations
- Progress is displayed during the research process
- Research sessions can be resumed

## Dependencies

- Step 003 (Gemini Provider) must be complete for grounded search
- Step 004 (Google Search) must be complete for search results
- Step 005 (Synthesis Engine) must be complete for answer formatting
- `scraper` crate for HTML to markdown conversion
- `reqwest` for URL fetching
- `nakama-ai` for synthesis and query decomposition
