# Step 003: Kubernetes Collector

## Objective
Implement a Kubernetes collector using `kube-rs` that gathers pod status, service endpoints, node conditions, deployment state, events, logs, and resource usage from a Kubernetes cluster.

## Tasks
- [ ] Add `kube`, `kube-runtime`, `k8s-openapi` crate dependencies
- [ ] Create `kubernetes.rs` module implementing the `Collector` trait
- [ ] Implement kubeconfig loading:
  - Auto-detect kubeconfig from `$KUBECONFIG` or `~/.kube/config`
  - Support in-cluster config detection
  - Support context switching via `--context` flag
  - Validate cluster connectivity on health_check
- [ ] Implement pod data collection:
  - List pods by namespace, label selector, or name
  - Collect pod status: phase, conditions, container statuses, restart counts
  - Detect common issues: CrashLoopBackOff, ImagePullBackOff, OOMKilled, Pending stuck
  - Collect pod resource requests/limits vs actual usage
- [ ] Implement service data collection:
  - List services with endpoints
  - Detect missing endpoints (no backing pods)
  - Check service selector matches
  - Identify external vs internal services
- [ ] Implement node data collection:
  - Node conditions (Ready, MemoryPressure, DiskPressure, PIDPressure)
  - Node capacity and allocatable resources
  - Node taints and labels
  - Detect unhealthy or unschedulable nodes
- [ ] Implement deployment data collection:
  - Deployment status (available, updated, unavailable replicas)
  - Rollout status and history
  - Detect stalled rollouts
  - ReplicaSet generation tracking
- [ ] Implement event collection:
  - Recent events for a resource (warnings, errors)
  - Cluster-wide warning events
  - Event deduplication and counting
- [ ] Implement log collection:
  - Pod log retrieval (last N lines or since timestamp)
  - Container selection for multi-container pods
  - Previous container logs (for crashed containers)
  - Log streaming support
- [ ] Implement resource usage collection:
  - CPU and memory usage via metrics API (if available)
  - Resource quota status per namespace
  - PVC status and storage usage
- [ ] Add namespace filtering: `--namespace`, `--all-namespaces`
- [ ] Handle RBAC limitations gracefully (report what cannot be accessed)
- [ ] Write unit tests with mock Kubernetes API responses
- [ ] Write integration tests (optional: use k3d or kind for local cluster)

## Acceptance Criteria
- Collector gathers comprehensive pod, service, node, and deployment data
- Common Kubernetes issues (CrashLoopBackOff, OOMKilled, etc.) are detected
- Events and logs are collected and associated with resources
- Health check verifies cluster connectivity
- RBAC limitations produce warnings, not errors
- Tests cover all resource types with mock API responses

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (collector trait to implement)
