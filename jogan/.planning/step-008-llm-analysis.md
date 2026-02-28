# Step 008: LLM Analysis

## Objective
Build an LLM-powered analysis engine for novel and complex issues that cannot be resolved by rule-based checks alone. Implement cross-layer correlation, temporal correlation, and causal chain building to reason about multi-faceted infrastructure problems.

## Tasks
- [ ] Create `analysis.rs` module with `LlmAnalyzer` struct
- [ ] Design system prompt for infrastructure diagnosis:
  - Include collected data summary (not raw data - summarized for token efficiency)
  - Include rule-based findings as context
  - Instruct structured output: root cause, evidence chain, confidence, remediation
  - Include infrastructure topology context
- [ ] Implement novel issue reasoning:
  - Send collected data + rule findings to `nakama-ai`
  - Ask LLM to identify issues not caught by rules
  - Parse structured response into findings
  - Score confidence based on evidence quality
- [ ] Implement cross-layer correlation engine:
  - Correlate Kubernetes events with pod status changes
  - Correlate container restarts with node resource pressure
  - Correlate service errors with upstream dependency failures
  - Correlate application logs with infrastructure events
  - Build correlation graph: event A -> caused event B -> caused symptom C
- [ ] Implement temporal correlation:
  - Align events across collectors by timestamp
  - Detect cascading failures (failure sequence across components)
  - Identify the "first domino" - earliest event in a failure chain
  - Timeline visualization of correlated events
- [ ] Implement causal chain building:
  - Build directed graph of cause -> effect relationships
  - Identify root cause (node with no incoming edges)
  - Calculate chain confidence (product of individual confidences)
  - Support multiple possible root causes with ranked probability
- [ ] Implement analysis result merging:
  - Combine rule-based findings with LLM findings
  - Deduplicate overlapping findings
  - Merge evidence from both sources
  - Present unified diagnosis with clear sourcing (rule vs LLM)
- [ ] Add `--no-llm` flag to use only rule-based analysis
- [ ] Add `--deep` flag for more thorough LLM analysis (more data, longer context)
- [ ] Implement data summarization for token-efficient LLM prompts
- [ ] Write unit tests for correlation logic
- [ ] Write tests for causal chain building with mock data
- [ ] Write tests for analysis merging

## Acceptance Criteria
- LLM analysis identifies issues missed by rule-based checks
- Cross-layer correlation connects related events across infrastructure layers
- Temporal correlation identifies cascading failure sequences
- Causal chains trace back to root cause with confidence scores
- Rule-based and LLM findings merge into a unified diagnosis
- Data summarization keeps prompts within token limits
- Tests cover correlation, causation, and merging logic

## Dependencies
- Step 001 (CLI scaffold)
- Step 002-005 (collectors provide data)
- Step 006 (symptom interpreter provides diagnosis plan)
- Step 007 (rule-based analysis provides initial findings)
