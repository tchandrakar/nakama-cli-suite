# Step 003: Kubernetes Collector

## Status: NOT STARTED

| Task | Status | Date | Notes |
|------|--------|------|-------|
| Add kube-rs and k8s-openapi dependencies | - | - | - |
| Create kubernetes.rs implementing Collector trait | - | - | - |
| Implement kubeconfig loading (auto-detect, in-cluster, context switching) | - | - | - |
| Implement pod data collection (status, conditions, restart counts) | - | - | - |
| Detect common pod issues (CrashLoopBackOff, ImagePullBackOff, OOMKilled) | - | - | - |
| Implement service data collection (endpoints, selectors) | - | - | - |
| Implement node data collection (conditions, capacity, taints) | - | - | - |
| Implement deployment data collection (replicas, rollout status) | - | - | - |
| Implement event collection (warnings, errors, deduplication) | - | - | - |
| Implement log collection (last N lines, previous container) | - | - | - |
| Implement resource usage collection (metrics API, quotas, PVCs) | - | - | - |
| Add namespace filtering (--namespace, --all-namespaces) | - | - | - |
| Handle RBAC limitations gracefully | - | - | - |
| Unit tests with mock Kubernetes API | - | - | - |

Status legend: `-` Not started | `WIP` In progress | `DONE` Complete | `BLOCKED` Blocked
