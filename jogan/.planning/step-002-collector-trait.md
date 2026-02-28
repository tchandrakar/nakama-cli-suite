# Step 002: Collector Trait

## Objective
Define the `Collector` trait that all infrastructure data sources must implement, along with a plugin registration system that allows dynamic discovery and invocation of collectors based on the target environment.

## Tasks
- [ ] Create `collector.rs` module with the core trait definition
- [ ] Define `Collector` trait with required methods:
  - `async fn collect(&self, resource: &ResourceQuery) -> Result<CollectedData>` — gather data for a specific resource
  - `async fn health_check(&self) -> Result<HealthStatus>` — verify collector can reach its target
  - `fn supported_resources(&self) -> Vec<ResourceType>` — list resource types this collector handles
  - `fn name(&self) -> &str` — human-readable collector name
  - `fn priority(&self) -> u8` — collection priority (lower = higher priority)
- [ ] Define supporting types:
  - `ResourceType` enum: Pod, Service, Node, Deployment, Container, Network, Volume, Instance, Cluster, LoadBalancer, Database, Queue
  - `ResourceQuery` struct: resource type, name/selector, namespace (optional), time range
  - `CollectedData` struct: resource info, status, metrics, events, logs (optional)
  - `HealthStatus` enum: Healthy, Degraded(String), Unreachable(String)
- [ ] Implement `CollectorRegistry`:
  - `register(collector: Box<dyn Collector>)` — add a collector
  - `get_collectors_for(resource: ResourceType) -> Vec<&dyn Collector>` — find capable collectors
  - `health_check_all() -> Vec<(String, HealthStatus)>` — check all collectors
  - `auto_discover()` — detect available infrastructure and register appropriate collectors
- [ ] Implement auto-discovery logic:
  - Check for kubeconfig -> register Kubernetes collector
  - Check for Docker socket -> register Docker collector
  - Check for AWS credentials -> register AWS collector
  - Check for GCP credentials -> register GCP collector (placeholder)
- [ ] Define error types specific to collection failures
- [ ] Add trait object safety considerations (Send + Sync bounds)
- [ ] Write unit tests for registry operations
- [ ] Write tests for auto-discovery with mock filesystem

## Acceptance Criteria
- `Collector` trait is fully defined with all required methods
- `CollectorRegistry` can register, discover, and query collectors
- Auto-discovery correctly detects available infrastructure
- Type system is flexible enough for all planned collectors (k8s, docker, cloud)
- Trait is object-safe and works with async dispatch
- Tests cover registration, discovery, and querying

## Dependencies
- Step 001 (CLI scaffold must exist to house this module)
