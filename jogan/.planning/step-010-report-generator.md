# Step 010: Report Generator

## Objective
Build a report generation system that formats diagnostic findings into a structured report with root cause identification, supporting evidence, remediation steps, severity scoring, confidence levels, and markdown export capability.

## Tasks
- [ ] Create `report.rs` module with `ReportGenerator` struct
- [ ] Define `DiagnosticReport` struct:
  - `id`: unique report identifier
  - `timestamp`: generation time
  - `target`: infrastructure target description
  - `symptoms`: original symptoms/query
  - `root_cause`: primary root cause with confidence
  - `findings`: ordered list of findings
  - `evidence_chain`: causal chain visualization
  - `remediation_plan`: ordered remediation steps
  - `severity_score`: overall severity (1-10)
  - `confidence_level`: overall confidence (High/Medium/Low)
  - `metadata`: collector versions, data freshness, analysis duration
- [ ] Implement root cause formatting:
  - Clear one-line root cause statement
  - Detailed explanation paragraph
  - Confidence percentage with reasoning
  - Alternative root causes if applicable (ranked)
- [ ] Implement evidence formatting:
  - Link findings to supporting evidence
  - Include relevant log snippets (sanitized)
  - Include metric values and thresholds
  - Include event timeline
  - Format evidence as bullet points with source attribution
- [ ] Implement remediation formatting:
  - Ordered step-by-step remediation plan
  - Each step includes: action, expected outcome, risk level
  - Include rollback instructions for risky steps
  - Mark immediate vs long-term fixes
  - Include command snippets where applicable
- [ ] Implement severity scoring:
  - Score 1-10 based on impact and scope
  - Factor in: affected resources count, user impact, data risk, recovery time
  - Color-code severity (green 1-3, yellow 4-6, red 7-9, critical 10)
- [ ] Implement confidence level calculation:
  - High: multiple corroborating evidence sources
  - Medium: single evidence source or partial correlation
  - Low: LLM inference without strong evidence
  - Display confidence reasoning
- [ ] Implement output formats:
  - Terminal: colored, formatted display with sections and tables
  - Markdown: full report as .md file
  - JSON: structured data for programmatic consumption
  - HTML: styled report (optional, via markdown conversion)
- [ ] Wire up `scan` subcommand output:
  - Display report after scan completes
  - `--format` flag: `terminal` (default), `markdown`, `json`
  - `--output` flag: file path for report export
  - `--severity-threshold` flag: only show findings above threshold
- [ ] Implement report history:
  - Save generated reports for later reference
  - `jogan report list` — show past reports
  - `jogan report show <id>` — display a past report
- [ ] Write unit tests for report formatting
- [ ] Write tests for severity scoring logic
- [ ] Write tests for markdown and JSON export

## Acceptance Criteria
- Reports clearly present root cause with confidence level
- Evidence is linked to findings with source attribution
- Remediation steps are actionable with command snippets
- Severity scoring is consistent and well-reasoned
- Markdown export produces clean, readable documents
- JSON export is parseable by downstream tools
- Tests cover formatting, scoring, and export

## Dependencies
- Step 001 (CLI scaffold)
- Step 007 (rule-based analysis produces findings)
- Step 008 (LLM analysis produces findings and causal chains)
