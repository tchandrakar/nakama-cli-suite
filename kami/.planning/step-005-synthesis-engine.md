# Step 005: Synthesis Engine

## Objective

Build the synthesis engine that takes raw search results and fetched content, and produces well-formatted answers with inline citations [1][2][3], source lists with URLs, confidence indicators, and related query suggestions. Integrate with nakama-ai for enhanced synthesis when needed.

## Tasks

- [ ] Define synthesis output types:
  - `SynthesizedAnswer` struct: formatted_text, citations Vec<Citation>, confidence, related_queries
  - `Citation` struct: index (1-based), url, title, snippet, relevance_score
  - `Confidence` enum: High, Medium, Low, Uncertain (based on source agreement and quality)
- [ ] Implement answer formatting with inline citations:
  - Parse Gemini grounded response to identify citation anchors
  - Insert `[N]` markers at the appropriate positions in the answer text
  - Build the numbered source list at the bottom
  - Handle cases where multiple sources support the same claim
  - Ensure citation indices are sequential and consistent
- [ ] Implement source list generation:
  - Collect all unique sources referenced in the answer
  - Order by first citation appearance
  - Include: index, title, URL, brief snippet
  - Deduplicate sources that point to the same content
- [ ] Implement confidence indicators:
  - Assess confidence based on:
    - Number of corroborating sources
    - Source authority (domain reputation heuristic)
    - Recency of sources
    - Agreement between sources
  - Display confidence as a visual indicator in terminal output
  - Include confidence in JSON output
- [ ] Implement related query suggestions:
  - Use Gemini to generate 3-5 follow-up queries based on the answer
  - Prioritize queries that explore different angles
  - Format as clickable suggestions in terminal output
- [ ] Implement nakama-ai integration for enhanced synthesis:
  - Use nakama-ai for synthesis when multiple search rounds need combining
  - Build synthesis prompts that include all gathered evidence
  - Handle contradictions between sources (highlight disagreements)
  - Highlight consensus vs. minority opinions
- [ ] Implement output formatting per mode:
  - Terminal: rich formatting with colored citations, confidence badge, source panel
  - Markdown: clean markdown with hyperlinked citations
  - JSON: structured data with all fields
- [ ] Write unit tests for citation formatting and confidence scoring
- [ ] Write integration tests for full synthesis pipeline

## Acceptance Criteria

- Answers include properly numbered inline citations [1][2][3]
- Source lists are complete, deduplicated, and include URLs
- Confidence indicators reflect the quality and agreement of sources
- Related query suggestions are relevant and diverse
- Synthesis correctly handles contradictions between sources
- All three output formats (terminal, markdown, JSON) are well-formatted
- Citation indices are consistent between inline markers and the source list

## Dependencies

- Step 003 (Gemini Provider) must be complete for grounded responses
- Step 004 (Google Search) must be complete for search results
- `nakama-ai` shared crate for enhanced synthesis
- `nakama-ui` shared crate for terminal formatting
