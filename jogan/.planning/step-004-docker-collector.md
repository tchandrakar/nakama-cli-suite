# Step 004: Docker Collector

## Objective
Implement a Docker collector using `bollard` that gathers container status, logs, resource stats, network inspection details, and volume status from the local Docker daemon.

## Tasks
- [ ] Add `bollard` crate dependency for Docker API communication
- [ ] Create `docker.rs` module implementing the `Collector` trait
- [ ] Implement Docker connection:
  - Auto-detect Docker socket (`/var/run/docker.sock` or `$DOCKER_HOST`)
  - Support TCP connections for remote Docker daemons
  - Support TLS-authenticated connections
  - Health check: verify Docker daemon is reachable and responsive
- [ ] Implement container status collection:
  - List containers (running, stopped, all)
  - Container state: running, paused, restarting, exited (with exit code), dead
  - Container health check status (if configured)
  - Restart count and last restart reason
  - Detect unhealthy containers and abnormal exit codes
- [ ] Implement container log collection:
  - Retrieve logs (last N lines, since timestamp, follow)
  - Separate stdout and stderr streams
  - Filter logs by pattern (basic grep)
  - Detect common error patterns in logs
- [ ] Implement container stats collection:
  - CPU usage percentage
  - Memory usage and limit
  - Network I/O (bytes sent/received)
  - Block I/O (bytes read/written)
  - PIDs count
- [ ] Implement network inspection:
  - List Docker networks
  - Show container network attachments
  - Detect network isolation issues
  - Identify port binding conflicts
  - DNS resolution status within networks
- [ ] Implement volume status:
  - List volumes with usage
  - Identify orphaned volumes (not attached to any container)
  - Volume mount points and drivers
  - Detect permission issues
- [ ] Implement Docker Compose awareness:
  - Detect Compose projects from labels
  - Group containers by Compose project
  - Show Compose service status
- [ ] Implement image information:
  - Image size and layers
  - Detect outdated images (by tag comparison)
  - Identify unused images
- [ ] Handle Docker daemon unavailability gracefully
- [ ] Write unit tests with mock Docker API responses
- [ ] Write integration tests with Docker (if available)

## Acceptance Criteria
- Collector gathers comprehensive container, network, and volume data
- Container health and status issues are detected
- Logs are retrievable with filtering options
- Resource stats provide CPU, memory, network, and I/O metrics
- Compose projects are recognized and grouped
- Health check verifies Docker daemon connectivity
- Tests cover all resource types with mock responses

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (collector trait to implement)
