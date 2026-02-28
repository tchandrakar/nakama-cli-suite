# Step 002: Log Ingestors

## Objective

Build an extensible log ingestion layer that supports reading logs from local files (with async file watching), stdin/pipes, and remote sources including Kubernetes (kube-rs), Docker (bollard), and AWS CloudWatch (aws-sdk). Each ingestor implements a common trait producing a unified `LogEntry` stream.

## Tasks

- [ ] Define `LogEntry` struct: timestamp, source, level, message, raw, metadata (HashMap)
- [ ] Define `LogIngestor` trait with `async fn stream(&self) -> impl Stream<Item = LogEntry>`
- [ ] Define `IngestorConfig` enum for source-specific configuration
- [ ] Implement `FileIngestor`:
  - [ ] Read existing file contents from offset
  - [ ] Async file watching via `notify` crate (debounced)
  - [ ] Support glob patterns for multiple files
  - [ ] Handle log rotation (detect inode change, reopen)
- [ ] Implement `StdinIngestor`:
  - [ ] Read from stdin line-by-line
  - [ ] Detect pipe vs TTY for appropriate behavior
  - [ ] Support streaming (no buffering) mode
- [ ] Implement `KubernetesIngestor`:
  - [ ] Use `kube-rs` with `Api<Pod>::log_stream()`
  - [ ] Support namespace, pod name, container, label selectors
  - [ ] Follow mode (like `kubectl logs -f`)
  - [ ] Multi-pod support (aggregate from label selector)
- [ ] Implement `DockerIngestor`:
  - [ ] Use `bollard` crate for Docker API
  - [ ] Support container name/ID
  - [ ] Follow mode with timestamps
  - [ ] Handle both stdout and stderr streams
- [ ] Implement `CloudWatchIngestor`:
  - [ ] Use `aws-sdk-cloudwatchlogs`
  - [ ] Support log group, log stream, filter pattern
  - [ ] Pagination handling for large result sets
  - [ ] Live tail mode with polling interval
- [ ] Implement `IngestorFactory` that creates the right ingestor from config/flags
- [ ] Add `--source` flag parsing: `file:///path`, `k8s://namespace/pod`, `docker://container`, `cloudwatch://group/stream`, `-` for stdin
- [ ] Implement backpressure handling (bounded channels between ingestor and consumer)
- [ ] Add unit tests with mock file/stream sources
- [ ] Add integration tests for file ingestor with temp files

## Acceptance Criteria

- All ingestors produce a uniform `Stream<Item = LogEntry>`
- File ingestor handles rotation and new line appends
- Stdin ingestor works in pipe chains (`cat log | sharingan scan`)
- Kubernetes ingestor connects to cluster and streams pod logs
- Docker ingestor connects to Docker daemon and streams container logs
- CloudWatch ingestor pages through results and supports follow mode
- Backpressure prevents OOM on fast producers
- Source auto-detection works from CLI flag format

## Dependencies

- Step 001 (CLI scaffold) must be complete
- kube-rs, bollard, aws-sdk-cloudwatchlogs crate dependencies
- notify crate for file watching
- tokio-stream for async stream utilities
