# Step 010: LLM Answer Generation

## Objective
Build a RAG (Retrieval-Augmented Generation) pipeline that combines retrieved code context with an LLM via `nakama-ai` to generate grounded, source-attributed answers to questions about the codebase.

## Tasks
- [ ] Create `answer.rs` module with `AnswerGenerator` struct
- [ ] Design RAG system prompt:
  - Instruct the LLM to answer questions based only on provided code context
  - Require source attribution (file:line) for every claim
  - Instruct to say "I don't have enough context" rather than hallucinate
  - Include project metadata (languages, structure) for background
- [ ] Implement RAG pipeline:
  - Receive user question
  - Run semantic retrieval (Step 008) to get relevant chunks
  - Run graph queries (Step 009) if structural question detected
  - Assemble context window: question + retrieved chunks + graph data
  - Send to `nakama-ai` for answer generation
  - Parse and format response
- [ ] Implement source attribution:
  - Extract file:line references from LLM response
  - Validate that cited files and lines exist
  - Format citations as clickable links (terminal hyperlinks if supported)
  - Add citation footnotes at end of response
- [ ] Implement grounded answer enforcement:
  - Cross-reference LLM claims with retrieved context
  - Flag unsupported claims (no matching context)
  - Score answer groundedness (percentage of claims with evidence)
  - Display groundedness indicator
- [ ] Wire up `ask` subcommand:
  - `senku ask "how does authentication work in this project?"`
  - Display answer with syntax-highlighted code references
  - Show source files referenced
  - Show confidence/groundedness score
  - `--verbose` flag to show retrieved context alongside answer
- [ ] Wire up `explain` subcommand:
  - `senku explain src/auth/mod.rs` — explain a file
  - `senku explain "parse_config"` — explain a function
  - `senku explain src/auth/ --module` — explain a module
  - Generate structured explanation: purpose, inputs/outputs, key logic, usage
- [ ] Implement conversation context (multi-turn):
  - Maintain conversation history within a session
  - Allow follow-up questions that reference previous answers
  - Clear context with `--new` flag
- [ ] Add `--format` flag: `terminal` (default), `markdown`, `json`
- [ ] Add `--sources-only` flag: just show relevant source files without LLM answer
- [ ] Write unit tests for prompt assembly
- [ ] Write tests for source attribution extraction and validation
- [ ] Write integration tests with mock LLM

## Acceptance Criteria
- Questions about the codebase receive accurate, context-grounded answers
- Every answer includes source attributions (file:line references)
- Citations are validated against actual source files
- Unsupported claims are flagged or omitted
- The explain command produces structured explanations for files, functions, and modules
- Follow-up questions work within a conversation session
- Tests cover prompt assembly, attribution, and answer quality

## Dependencies
- Step 001 (CLI scaffold)
- Step 008 (semantic retrieval provides context)
- Step 009 (graph queries provide structural context)
