# Step 005: Cloud Collectors

## Objective
Implement cloud infrastructure collectors, starting with a full AWS collector using `aws-sdk-rust` (covering EC2, ECS/EKS, CloudWatch, IAM), and providing placeholder implementations for GCP and Azure collectors.

## Tasks
- [ ] Add AWS SDK crate dependencies: `aws-sdk-ec2`, `aws-sdk-ecs`, `aws-sdk-eks`, `aws-sdk-cloudwatch`, `aws-sdk-iam`, `aws-config`
- [ ] Create `cloud/mod.rs` with cloud collector sub-modules
- [ ] Create `cloud/aws.rs` implementing the `Collector` trait for AWS
- [ ] Implement AWS credential handling:
  - Auto-detect credentials from standard chain (env vars, config files, instance profile)
  - Support profile selection via `--aws-profile` flag
  - Support region specification via `--aws-region` flag
  - Validate credentials on health_check
- [ ] Implement EC2 collector:
  - List instances with status (running, stopped, terminated)
  - Instance health checks (system status, instance status)
  - Security group rules analysis
  - EBS volume status and IOPS
  - Network interface status
- [ ] Implement ECS/EKS collector:
  - ECS cluster status, service health, task status
  - EKS cluster status and node group health
  - Task definition analysis
  - Service deployment status
- [ ] Implement CloudWatch collector:
  - Recent alarms (state: OK, ALARM, INSUFFICIENT_DATA)
  - Metric queries for key infrastructure metrics
  - Log group queries for recent errors
  - Custom metric retrieval
- [ ] Implement IAM collector:
  - Current user/role identification
  - Permission boundary analysis (what can jogan access?)
  - Detect overly permissive policies (informational)
  - Access key age and rotation status
- [ ] Create `cloud/gcp.rs` placeholder:
  - Stub implementation of Collector trait
  - Document planned GCP services (GKE, Compute Engine, Cloud Logging)
  - Return `HealthStatus::Unreachable("GCP collector not yet implemented")`
- [ ] Create `cloud/azure.rs` placeholder:
  - Stub implementation of Collector trait
  - Document planned Azure services (AKS, VMs, Monitor)
  - Return `HealthStatus::Unreachable("Azure collector not yet implemented")`
- [ ] Implement cross-cloud resource mapping (normalize resource types)
- [ ] Handle API rate limiting with exponential backoff
- [ ] Handle pagination for large resource lists
- [ ] Write unit tests for AWS collector with mock API responses
- [ ] Write tests for credential detection logic

## Acceptance Criteria
- AWS collector gathers EC2, ECS/EKS, CloudWatch, and IAM data
- Credential auto-detection works for standard AWS configurations
- CloudWatch alarms and metrics are retrievable
- GCP and Azure placeholders exist with clear documentation
- API rate limiting is handled gracefully
- Health check verifies AWS connectivity and permissions
- Tests cover AWS operations with mock responses

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (collector trait to implement)
