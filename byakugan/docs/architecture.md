# Byakugan — Platform Agnostic PR Reviewer

> "360-degree vision, no blind spots." — Inspired by the Hyuga clan's dojutsu from Naruto.

## Overview

Byakugan is an AI-powered pull request reviewer that works identically across GitHub, GitLab, and Bitbucket. It analyzes diffs, detects bugs, security issues, and style violations, posts inline review comments, and generates unified review summaries — all from one CLI, regardless of where your code lives.

## Core Commands

| Command | Description |
|---------|-------------|
| `byakugan review` | Auto-detect platform from git remote and review the current PR/MR |
| `byakugan scan --platform=github --pr=142` | Review a specific PR on GitHub |
| `byakugan scan --platform=gitlab --mr=33` | Review a specific MR on GitLab |
| `byakugan scan --platform=bitbucket --pr=87` | Review a specific PR on Bitbucket |
| `byakugan report` | Generate a unified review summary (terminal + markdown) |
| `byakugan watch` | Daemon mode — auto-review new PRs across all configured repos |
| `byakugan rules` | List, add, or edit custom review rules |
| `byakugan comment <pr> <message>` | Post a review comment to a PR/MR |

## Architecture

```
┌───────────────────────────────────────────────────────┐
│                      CLI Layer                        │
│         (commands, flags, daemon mode)                │
├───────────────────────────────────────────────────────┤
│                 Platform Adapters                      │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐   │
│  │   GitHub     │ │   GitLab     │ │  Bitbucket   │   │
│  │   Adapter    │ │   Adapter    │ │  Adapter     │   │
│  │              │ │              │ │              │   │
│  │ - REST API   │ │ - REST API   │ │ - REST API   │   │
│  │ - GraphQL    │ │ - GraphQL    │ │ - Pipes      │   │
│  │ - Webhooks   │ │ - Webhooks   │ │ - Webhooks   │   │
│  └──────┬───────┘ └──────┬───────┘ └──────┬───────┘   │
│         └────────────────┼────────────────┘            │
│                          ▼                             │
│  ┌─────────────────────────────────────────────────┐   │
│  │          Unified Platform Interface             │   │
│  │                                                 │   │
│  │  trait PlatformAdapter:                         │   │
│  │    fn fetch_pr(id) -> PullRequest               │   │
│  │    fn get_diff(pr) -> UnifiedDiff               │   │
│  │    fn get_comments(pr) -> Vec<Comment>          │   │
│  │    fn post_comment(pr, comment) -> Result       │   │
│  │    fn post_review(pr, review) -> Result         │   │
│  │    fn list_open_prs() -> Vec<PullRequest>       │   │
│  └─────────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────────┤
│                  Diff Analysis Engine                  │
│  ┌────────────────┐ ┌─────────────────────────────┐   │
│  │ Unified Diff   │ │ Semantic Diff Analyzer      │   │
│  │ Parser         │ │                             │   │
│  │                │ │ - Function-level changes    │   │
│  │ - Hunk         │ │ - Import/dependency changes │   │
│  │   extraction   │ │ - Type signature changes    │   │
│  │ - File type    │ │ - Control flow changes      │   │
│  │   detection    │ │ - API surface changes       │   │
│  │ - Rename       │ │                             │   │
│  │   detection    │ │                             │   │
│  └────────────────┘ └─────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ Context Enrichment                              │   │
│  │                                                 │   │
│  │ - Surrounding code (not just the diff)          │   │
│  │ - PR description and linked issues              │   │
│  │ - Previous review comments                      │   │
│  │ - File history (who usually owns this code)     │   │
│  │ - CI/CD status                                  │   │
│  └─────────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────────┤
│                  Review Engine (LLM-Powered)          │
│  ┌─────────────────────────────────────────────────┐   │
│  │ Review Passes:                                  │   │
│  │                                                 │   │
│  │ 1. Correctness                                  │   │
│  │    - Logic errors, off-by-ones, null handling   │   │
│  │    - Race conditions, deadlocks                 │   │
│  │    - Edge cases not covered                     │   │
│  │                                                 │   │
│  │ 2. Security                                     │   │
│  │    - Injection vulnerabilities (SQL, XSS, cmd)  │   │
│  │    - Auth/authz issues                          │   │
│  │    - Secret/credential exposure                 │   │
│  │    - Insecure dependencies                      │   │
│  │                                                 │   │
│  │ 3. Performance                                  │   │
│  │    - N+1 queries, unbounded loops               │   │
│  │    - Memory leaks, unnecessary allocations      │   │
│  │    - Missing indexes, slow paths                │   │
│  │                                                 │   │
│  │ 4. Style & Conventions                          │   │
│  │    - Project coding standards                   │   │
│  │    - Naming consistency                         │   │
│  │    - Documentation completeness                 │   │
│  │                                                 │   │
│  │ 5. Architecture                                 │   │
│  │    - Separation of concerns                     │   │
│  │    - API contract changes                       │   │
│  │    - Dependency direction violations            │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ Custom Rules Engine                             │   │
│  │                                                 │   │
│  │ User-defined rules in .byakugan.yml:            │   │
│  │  - "Never use console.log in production code"   │   │
│  │  - "All API endpoints must have auth middleware" │   │
│  │  - "Database migrations must be reversible"     │   │
│  │                                                 │   │
│  │ Rules are injected into the LLM review prompt   │   │
│  └─────────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────────┤
│                  Output Layer                         │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐   │
│  │ Terminal     │ │ Inline       │ │ Markdown     │   │
│  │ Summary      │ │ Comments     │ │ Report       │   │
│  │ (rich)       │ │ (post back   │ │ (export)     │   │
│  │              │ │  to platform)│ │              │   │
│  └──────────────┘ └──────────────┘ └──────────────┘   │
├───────────────────────────────────────────────────────┤
│                  Watch Daemon                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │ - Polls configured repos for new PRs/MRs        │   │
│  │ - Webhook listener mode (optional)              │   │
│  │ - Auto-triggers review on new PRs               │   │
│  │ - Respects rate limits per platform             │   │
│  │ - Sends notifications (terminal, Slack, email)  │   │
│  └─────────────────────────────────────────────────┘   │
└───────────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Adapter Pattern — One Vision, Multiple Targets
The core design principle mirrors the Byakugan itself: one unified vision system, multiple targets. Each platform adapter implements the same `PlatformAdapter` trait:

```
trait PlatformAdapter {
    fn fetch_pr(id: &str) -> Result<PullRequest>
    fn get_diff(pr: &PullRequest) -> Result<UnifiedDiff>
    fn get_comments(pr: &PullRequest) -> Result<Vec<Comment>>
    fn post_comment(pr: &PullRequest, comment: ReviewComment) -> Result<()>
    fn post_review(pr: &PullRequest, review: Review) -> Result<()>
    fn list_open_prs(repo: &str) -> Result<Vec<PullRequest>>
}
```

The review engine never knows or cares which platform it's talking to. Adding a new platform (e.g., Gitea, Azure DevOps) means implementing one trait.

### 2. Auto-Detection from Git Remote
`byakugan review` (no flags) inspects the current repo's git remotes:
- `github.com` in remote URL → GitHub adapter
- `gitlab.com` or self-hosted GitLab patterns → GitLab adapter
- `bitbucket.org` → Bitbucket adapter

It then detects the current branch, finds the associated PR/MR, and reviews it. Zero-config for the common case.

### 3. Multi-Pass Review
Rather than a single monolithic review, Byakugan runs multiple focused passes:
1. **Correctness** — logic bugs, edge cases
2. **Security** — OWASP top 10, secrets, auth issues
3. **Performance** — N+1, memory, slow paths
4. **Style** — project conventions, naming
5. **Architecture** — design patterns, coupling

Each pass uses a specialized system prompt. Users can enable/disable passes per project.

### 4. Context-Rich Reviews
Byakugan doesn't just look at the diff in isolation. It enriches context with:
- Surrounding code (full file context, not just changed lines)
- The PR description and linked issues
- Previous review comments (to avoid repeating feedback)
- CI status (if tests are already failing, flag it)

This produces reviews that feel like they come from a senior engineer who understands the project.

### 5. Custom Rules
Teams can define project-specific review rules in `.byakugan.yml`:
```yaml
rules:
  - name: no-console-log
    description: "console.log statements should not be in production code"
    severity: warning
    pattern: "console\\.log"
    exclude: ["**/*.test.*", "**/debug/**"]

  - name: require-auth-middleware
    description: "All API route handlers must include auth middleware"
    severity: error
    context: "When reviewing route definitions"
```
Rules are injected into the LLM's review prompt alongside the diff.

### 6. Watch Daemon
`byakugan watch` runs as a background daemon that:
- Polls all configured repos on a schedule
- Detects new PRs/MRs
- Auto-runs review and posts comments
- Respects platform rate limits
- Optionally listens for webhooks for instant triggering

## Data Flow — Review

```
Input: PR identifier (or auto-detect from current branch)
        │
        ▼
  Platform Detection ──→ select adapter (GitHub / GitLab / Bitbucket)
        │
        ▼
  Fetch PR Metadata ──→ title, description, author, linked issues
        │
        ▼
  Fetch Diff ──→ unified diff + full file context
        │
        ▼
  Context Enrichment ──→ previous comments, CI status, file history
        │
        ▼
  Review Engine (multi-pass)
        ├── Pass 1: Correctness ──→ findings
        ├── Pass 2: Security ──→ findings
        ├── Pass 3: Performance ──→ findings
        ├── Pass 4: Style ──→ findings
        └── Pass 5: Architecture ──→ findings
        │
        ▼
  Custom Rules Check ──→ additional findings
        │
        ▼
  Deduplicate & Prioritize ──→ ranked findings with severity
        │
        ▼
  Output
        ├── Terminal summary (rich formatted)
        ├── Post inline comments to platform (optional)
        └── Markdown report (optional)
```

## Configuration

File: `~/.byakugan/config.toml`

```toml
[llm]
provider = "anthropic"
model = "claude-sonnet-4-6"

[platforms.github]
token = ""                          # or use GITHUB_TOKEN env var
api_url = "https://api.github.com"  # override for GitHub Enterprise

[platforms.gitlab]
token = ""                          # or use GITLAB_TOKEN env var
api_url = "https://gitlab.com"      # override for self-hosted

[platforms.bitbucket]
username = ""
app_password = ""                   # or use BITBUCKET_TOKEN env var
api_url = "https://api.bitbucket.org/2.0"

[review]
passes = ["correctness", "security", "performance", "style", "architecture"]
max_comments = 20                   # cap inline comments to avoid noise
severity_threshold = "info"         # info | warning | error
auto_post_comments = false          # require confirmation before posting

[watch]
enabled_repos = []                  # ["org/repo1", "org/repo2"]
poll_interval_seconds = 300
auto_review = true
notify = "terminal"                 # terminal | slack | email
```

## Tech Stack

- **Language:** Rust
- **CLI framework:** clap
- **GitHub API:** octocrab
- **GitLab API:** reqwest + gitlab REST/GraphQL
- **Bitbucket API:** reqwest + Bitbucket REST
- **Diff parsing:** custom unified diff parser
- **LLM integration:** shared nakama LLM abstraction layer
- **Daemon:** tokio background tasks with signal handling
