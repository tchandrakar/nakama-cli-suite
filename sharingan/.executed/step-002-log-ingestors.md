# Step 002: Log Ingestors

## Status: NOT STARTED

| Task | Status | Date | Notes |
|------|--------|------|-------|
| Define LogEntry struct | - | - | - |
| Define LogIngestor trait | - | - | - |
| Define IngestorConfig enum | - | - | - |
| Implement FileIngestor (async watching, glob, rotation) | - | - | - |
| Implement StdinIngestor (pipe/TTY detection) | - | - | - |
| Implement KubernetesIngestor (kube-rs, follow mode) | - | - | - |
| Implement DockerIngestor (bollard, stdout/stderr) | - | - | - |
| Implement CloudWatchIngestor (aws-sdk, pagination, tail) | - | - | - |
| Implement IngestorFactory | - | - | - |
| Add --source flag parsing | - | - | - |
| Implement backpressure handling | - | - | - |
| Unit tests with mock sources | - | - | - |
| Integration tests for file ingestor | - | - | - |

Status legend: `-` Not started | `WIP` In progress | `DONE` Complete | `BLOCKED` Blocked
