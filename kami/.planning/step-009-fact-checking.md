# Step 009: Fact-Checking Mode

## Objective

Implement the `kami grounded` command that extracts claims from input text, verifies each claim against grounded search results, generates verdicts (TRUE, FALSE, PARTIAL, UNVERIFIABLE), and assembles evidence with citations for each verdict.

## Tasks

- [ ] Implement claim extraction:
  - Accept a claim string as input
  - For longer text, use Gemini/nakama-ai to extract individual verifiable claims
  - Identify the key factual assertions in each claim
  - Separate opinions from verifiable facts
- [ ] Implement claim verification:
  - For each extracted claim, perform a grounded search
  - Craft targeted search queries to verify the specific claim
  - Gather evidence from multiple sources (at least 3 sources per claim)
  - Assess whether evidence supports, contradicts, or is ambiguous
- [ ] Implement verdict generation:
  - `TRUE` -- claim is fully supported by multiple reliable sources
  - `FALSE` -- claim is contradicted by reliable sources
  - `PARTIAL` -- claim is partially correct (some elements true, some false or imprecise)
  - `UNVERIFIABLE` -- insufficient evidence to verify or deny the claim
  - Include confidence score (0.0-1.0) for each verdict
- [ ] Implement evidence assembly:
  - For each claim, collect supporting and contradicting evidence
  - Include inline citations to sources
  - Highlight the specific parts of the claim that are true/false
  - Provide corrected version for FALSE/PARTIAL claims
- [ ] Implement output formatting:
  - Terminal: colored verdict badge, claim text, evidence, citations
  - Markdown: structured fact-check report
  - JSON: machine-readable verdicts with evidence
  - Example format:
    ```
    Claim: "X affects all versions of Y"
    Verdict: PARTIALLY TRUE (confidence: 0.85)
    Details: X affects versions A through B, not all versions...
    Sources: [1] url1  [2] url2
    ```
- [ ] Implement batch fact-checking:
  - Accept multiple claims (newline-separated or from stdin)
  - Process claims in parallel where possible
  - Generate a summary report with overall statistics
- [ ] Write unit tests for claim extraction and verdict generation
- [ ] Write integration tests with mocked search results

## Acceptance Criteria

- Single claims are correctly verified with appropriate verdicts
- Evidence assembly includes citations from reliable sources
- PARTIAL verdicts clearly explain which parts are true and which are not
- Corrected versions are provided for FALSE/PARTIAL claims
- Batch fact-checking processes multiple claims efficiently
- All output formats (terminal, markdown, JSON) are well-structured
- Confidence scores reflect the strength of evidence

## Dependencies

- Step 003 (Gemini Provider) must be complete for grounded search
- Step 004 (Google Search) must be complete for evidence gathering
- Step 005 (Synthesis Engine) must be complete for citation formatting
- `nakama-ai` for claim extraction and evidence analysis
