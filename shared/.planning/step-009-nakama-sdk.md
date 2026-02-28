# Step 009: Build nakama-sdk (Tool-to-Tool SDK)

## Objective
Create the SDK that allows tools to invoke each other as library calls (not just pipes).

## Tasks
- Re-export all shared crate interfaces from one import
- Prelude module (common imports for tool developers)
- Per-tool public API modules (zangetsu.rs, shinigami.rs, ... itachi.rs)
  - Define the public interface each tool exposes for SDK consumption
  - Initially stubs — filled in as each tool is built
- TraceContext propagation for direct invocation
- Shared configuration passing between tools
- Documentation and examples

## Acceptance Criteria
- `use nakama_sdk::prelude::*` gives tool developers everything they need
- Tools can call each other without spawning processes
- Trace context maintained across direct invocations
- Clean, well-documented API

## Dependencies
- Steps 002-008 (all shared crates)
