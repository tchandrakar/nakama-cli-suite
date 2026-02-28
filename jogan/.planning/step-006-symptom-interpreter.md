# Step 006: Symptom Interpreter

## Objective
Build a symptom interpretation engine that converts natural language problem descriptions into structured diagnosis plans, using `nakama-ai` for understanding and mapping symptoms to known patterns and appropriate collectors.

## Tasks
- [ ] Create `symptom.rs` module with `SymptomInterpreter` struct
- [ ] Implement NL symptom parsing via `nakama-ai`:
  - Convert user description to structured symptom list
  - Example: "my pods keep restarting" -> Symptom { resource: Pod, issue: CrashLoop, severity: High }
  - Example: "website is slow" -> Symptom { resource: Service, issue: HighLatency, severity: Medium }
  - Handle vague descriptions with clarifying questions
- [ ] Define `Symptom` struct:
  - `description`: original text
  - `resource_type`: affected resource type (Pod, Container, Instance, etc.)
  - `issue_category`: enum (CrashLoop, OOM, NetworkIssue, DiskFull, HighLatency, AuthFailure, etc.)
  - `severity`: Low, Medium, High, Critical
  - `affected_components`: list of specific resources if mentioned
- [ ] Build `DiagnosisPlan` struct:
  - `symptoms`: parsed symptom list
  - `collectors_needed`: which collectors to invoke
  - `data_queries`: specific queries for each collector
  - `check_order`: priority-ordered list of checks to perform
  - `related_checks`: additional checks that might reveal root cause
- [ ] Implement known symptom mapping (rule-based, pre-LLM):
  - Map common error strings to symptom categories
  - "CrashLoopBackOff" -> Pod crash loop diagnosis plan
  - "OOMKilled" -> Memory exhaustion diagnosis plan
  - "connection refused" -> Network/service connectivity plan
  - "disk full" -> Storage exhaustion plan
  - "permission denied" -> Auth/RBAC plan
- [ ] Implement collector selection logic:
  - Based on symptom, select relevant collectors
  - Order collectors by relevance and speed
  - Include cross-cutting collectors (events, logs) for all diagnoses
- [ ] Wire up `diagnose` subcommand:
  - `jogan diagnose "my pods keep restarting in production"`
  - Display interpreted symptoms and diagnosis plan
  - Confirm plan with user before executing collectors
  - `--auto` flag to skip confirmation
- [ ] Wire up `explain` subcommand:
  - `jogan explain "What is CrashLoopBackOff?"`
  - LLM-powered explanation of infrastructure concepts and errors
- [ ] Add `--quick` flag for fast diagnosis (skip deep checks)
- [ ] Write unit tests for symptom parsing with various descriptions
- [ ] Write tests for diagnosis plan generation

## Acceptance Criteria
- Natural language descriptions are correctly parsed into structured symptoms
- Known error strings are mapped without LLM invocation (fast path)
- Diagnosis plans select appropriate collectors and queries
- Vague descriptions produce reasonable plans with broader checks
- The explain command provides clear, accurate explanations
- Tests cover common symptom patterns and plan generation

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (collector trait for plan target selection)
- Step 003-005 (collectors must be defined for plan references)
