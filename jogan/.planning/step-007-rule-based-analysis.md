# Step 007: Rule-Based Analysis

## Objective
Build a rule-based analysis engine that checks collected infrastructure data against known failure patterns (OOMKilled, CrashLoopBackOff, ImagePullBackOff, disk full, port conflicts, DNS failures) and produces diagnostic findings without requiring LLM invocation.

## Tasks
- [ ] Create `rules.rs` module with `RuleEngine` struct
- [ ] Define `Rule` trait:
  - `fn name(&self) -> &str` — rule identifier
  - `fn description(&self) -> &str` — what the rule checks
  - `fn applicable_to(&self) -> Vec<ResourceType>` — which resources
  - `fn check(&self, data: &CollectedData) -> Option<Finding>` — evaluate rule
- [ ] Define `Finding` struct:
  - `rule_name`: which rule triggered
  - `severity`: Low, Medium, High, Critical
  - `summary`: one-line description
  - `detail`: full explanation
  - `evidence`: collected data that triggered the finding
  - `remediation`: suggested fix steps
  - `confidence`: High, Medium, Low
- [ ] Implement Kubernetes-specific rules:
  - **OOMKilled**: detect containers killed by OOM, compare usage to limits
  - **CrashLoopBackOff**: detect crash loops, analyze container exit codes and logs
  - **ImagePullBackOff**: detect image pull failures, check image name/tag/registry
  - **Pending pods**: detect pods stuck in Pending, check node resources and scheduling constraints
  - **Evicted pods**: detect evicted pods, identify eviction reason
  - **Failed probes**: detect liveness/readiness probe failures
  - **Resource exhaustion**: detect nodes at capacity (CPU, memory, disk)
- [ ] Implement Docker-specific rules:
  - **Unhealthy containers**: detect containers failing health checks
  - **Restart loops**: detect containers with high restart counts
  - **Port conflicts**: detect port binding conflicts
  - **Orphaned volumes**: detect volumes not attached to containers
  - **High resource usage**: detect containers exceeding resource thresholds
- [ ] Implement general infrastructure rules:
  - **Disk full**: detect filesystems at >90% capacity
  - **DNS resolution failure**: detect DNS-related errors in logs
  - **Certificate expiry**: detect soon-to-expire TLS certificates
  - **Connection refused**: detect connection refused patterns in logs
  - **Timeout patterns**: detect repeated timeout errors
- [ ] Implement `RuleEngine`:
  - `register(rule: Box<dyn Rule>)` — add a rule
  - `evaluate(data: &[CollectedData]) -> Vec<Finding>` — run all applicable rules
  - `evaluate_parallel(data)` — parallel rule evaluation for speed
- [ ] Sort findings by severity (Critical first)
- [ ] Deduplicate findings (same issue across multiple resources)
- [ ] Add rule configuration: enable/disable specific rules, adjust thresholds
- [ ] Write unit tests for each rule with sample data
- [ ] Write tests for rule engine evaluation and deduplication

## Acceptance Criteria
- All listed known patterns are detected by corresponding rules
- Findings include severity, evidence, and remediation suggestions
- Rules produce zero false positives on healthy infrastructure data
- Rule evaluation completes in under 1 second for typical data sets
- Findings are sorted by severity and deduplicated
- Each rule has dedicated unit tests

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (collector trait for data types)
- Step 003-005 (collectors provide the data to analyze)
