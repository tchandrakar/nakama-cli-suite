# Itachi — Atlassian Jira & Confluence Hub

> "Silent organizational intelligence master." — Inspired by Itachi Uchiha from Naruto.

## Overview

Itachi is the connective tissue of the Nakama suite. It bridges Jira (what's happening) and Confluence (what's documented) into a single, intelligent CLI. Ask questions in plain English, get answers drawn from both platforms simultaneously. Itachi never needs to ask twice — and neither should you.

The duality maps perfectly: **Jira is Itachi's Sharingan** (tracking every movement, every task, every sprint) and **Confluence is his Tsukuyomi** (the vast inner world of knowledge, documentation, and context).

## Core Commands

| Command | Description |
|---------|-------------|
| `itachi jira <query>` | Natural language Jira query (translated to JQL) |
| `itachi wiki <query>` | Natural language Confluence search (translated to CQL) |
| `itachi ask <question>` | Cross-platform intelligence — queries both Jira and Confluence |
| `itachi brief <team>` | Team briefing — sprint health, blockers, relevant docs |
| `itachi onboard <project\|service>` | Generate onboarding brief from Jira history + Confluence docs |
| `itachi standup` | Auto-generate standup from yesterday's Jira transitions |
| `itachi create <type> <summary>` | Create a Jira ticket from the CLI |
| `itachi link <issue> --doc <page>` | Link a Jira issue to a Confluence page |
| `itachi sprint [board]` | Sprint dashboard — progress, velocity, burndown |

## Architecture

```
┌────────────────────────────────────────────────────────┐
│                       CLI Layer                        │
│          (commands, interactive mode, pipe I/O)        │
├────────────────────────────────────────────────────────┤
│                    Query Planner                       │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Natural language → structured query translation  │  │
│  │                                                  │  │
│  │ Input: "what's blocking the payments team?"      │  │
│  │ Plan:                                            │  │
│  │   1. Jira: JQL → project = PAY AND status !=    │  │
│  │      Done AND flagged = impediment              │  │
│  │   2. Confluence: CQL → space = PAY AND          │  │
│  │      label = "blocker" OR label = "incident"    │  │
│  │   3. Cross-reference: link tickets to docs      │  │
│  │   4. Synthesize: combined answer                │  │
│  └──────────────────────────────────────────────────┘  │
├────────────────────────────────────────────────────────┤
│                  Platform Clients                      │
│                                                        │
│  ┌─────────────────────┐  ┌─────────────────────────┐  │
│  │     Jira Client     │  │   Confluence Client     │  │
│  │                     │  │                         │  │
│  │ ┌─────────────────┐ │  │ ┌─────────────────────┐ │  │
│  │ │ JQL Builder     │ │  │ │ CQL Builder         │ │  │
│  │ │ (NL → JQL       │ │  │ │ (NL → CQL           │ │  │
│  │ │  translation)   │ │  │ │  translation)       │ │  │
│  │ └─────────────────┘ │  │ └─────────────────────┘ │  │
│  │ ┌─────────────────┐ │  │ ┌─────────────────────┐ │  │
│  │ │ Issue Search    │ │  │ │ Page Search         │ │  │
│  │ │ Issue Create    │ │  │ │ Content Parser      │ │  │
│  │ │ Issue Update    │ │  │ │ (storage format →   │ │  │
│  │ │ Transitions     │ │  │ │  clean markdown)    │ │  │
│  │ └─────────────────┘ │  │ └─────────────────────┘ │  │
│  │ ┌─────────────────┐ │  │ ┌─────────────────────┐ │  │
│  │ │ Sprint/Board    │ │  │ │ Space Navigator     │ │  │
│  │ │ Analytics       │ │  │ │ (hierarchy, labels, │ │  │
│  │ │ (velocity,      │ │  │ │  page tree)         │ │  │
│  │ │  burndown,      │ │  │ │                     │ │  │
│  │ │  cycle time)    │ │  │ │                     │ │  │
│  │ └─────────────────┘ │  │ └─────────────────────┘ │  │
│  └─────────────────────┘  └─────────────────────────┘  │
│                                                        │
│  ┌──────────────────────────────────────────────────┐  │
│  │            Unified Query Interface               │  │
│  │                                                  │  │
│  │  fn search_jira(jql: &str) -> Vec<Issue>         │  │
│  │  fn search_confluence(cql: &str) -> Vec<Page>    │  │
│  │  fn cross_query(nl: &str) -> CrossResult         │  │
│  │  fn get_issue(key: &str) -> Issue                │  │
│  │  fn get_page(id: &str) -> Page                   │  │
│  └──────────────────────────────────────────────────┘  │
├────────────────────────────────────────────────────────┤
│               Cross-Reference Engine                   │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Links Jira tickets ↔ Confluence documents:       │  │
│  │                                                  │  │
│  │ - Explicit links (Jira remote links to Confl.)   │  │
│  │ - Implicit links (ticket keys mentioned in docs) │  │
│  │ - Semantic links (LLM matches related content)   │  │
│  │                                                  │  │
│  │ Builds a relationship graph:                     │  │
│  │   Epic PROJ-100 ──references──→ "Auth RFC" doc   │  │
│  │   Story PROJ-142 ──implements──→ "Login Flow"    │  │
│  │   Bug PROJ-189 ──relates──→ "Incident Runbook"   │  │
│  └──────────────────────────────────────────────────┘  │
├────────────────────────────────────────────────────────┤
│                Intelligence Layer                      │
│  ┌──────────────────┐  ┌────────────────────────────┐  │
│  │ Embedding Cache  │  │ Context Builder            │  │
│  │                  │  │                            │  │
│  │ Confluence pages │  │ Assembles LLM context from │  │
│  │ indexed locally  │  │ mixed sources:             │  │
│  │ for fast         │  │  - Jira issues + comments  │  │
│  │ semantic search  │  │  - Confluence page content  │  │
│  │                  │  │  - Sprint data              │  │
│  │ Incremental      │  │  - Cross-reference graph    │  │
│  │ updates on       │  │                            │  │
│  │ page changes     │  │ Respects token limits,     │  │
│  │                  │  │ prioritizes by relevance   │  │
│  └──────────────────┘  └────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────┐  │
│  │ LLM Synthesizer                                  │  │
│  │                                                  │  │
│  │ Takes assembled context + user question          │  │
│  │ Produces:                                        │  │
│  │  - Direct answer with source attribution         │  │
│  │  - Links to relevant Jira tickets                │  │
│  │  - Links to relevant Confluence pages            │  │
│  │  - Suggested actions                             │  │
│  └──────────────────────────────────────────────────┘  │
├────────────────────────────────────────────────────────┤
│                  Auth Layer                             │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Atlassian Cloud:                                 │  │
│  │  - API token (email + token pair)                │  │
│  │  - OAuth 2.0 (3LO) for Atlassian Cloud apps     │  │
│  │                                                  │  │
│  │ Atlassian Data Center / Server:                  │  │
│  │  - Personal access tokens                        │  │
│  │  - Basic auth (legacy)                           │  │
│  │                                                  │  │
│  │ Credentials stored in OS keychain                │  │
│  └──────────────────────────────────────────────────┘  │
├────────────────────────────────────────────────────────┤
│                  Output Layer                          │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐   │
│  │ Terminal     │ │ Markdown     │ │ Pipe / JSON  │   │
│  │ (tables,     │ │ (briefs,     │ │ (stdout for  │   │
│  │  panels,     │ │  onboarding  │ │  chaining    │   │
│  │  sprint      │ │  docs)       │ │  with other  │   │
│  │  charts)     │ │              │ │  nakama      │   │
│  │              │ │              │ │  tools)      │   │
│  └──────────────┘ └──────────────┘ └──────────────┘   │
└────────────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Natural Language → JQL/CQL Translation
The LLM converts plain English into proper query languages:

```
"what's blocking the payments team this sprint?"
    ↓
JQL: project = PAY AND sprint in openSprints()
     AND (flagged = impediment OR priority in (Blocker, Critical))
     AND status != Done
```

```
"find the architecture decision record for auth service"
    ↓
CQL: space = ENG AND type = page
     AND (label = "adr" OR label = "architecture-decision")
     AND text ~ "auth service"
```

The LLM learns your project keys, space names, and custom fields from configuration — queries get more accurate over time.

### 2. Cross-Platform Intelligence (The Killer Feature)
When you run `itachi ask`, it doesn't just search one platform:

```
itachi ask "what's the status of the migration project?"
```

1. **Jira query:** Find the migration epic, its child stories, current sprint status, blockers
2. **Confluence query:** Find related design docs, ADRs, runbooks, meeting notes
3. **Cross-reference:** Link stories to their design docs, find gaps (stories without docs, docs without tickets)
4. **Synthesize:** One coherent answer with links to both platforms

This is the feature that makes Itachi the "connective tissue" of the suite.

### 3. Local Embedding Cache for Confluence
Confluence pages are embedded locally for fast semantic search:
- On first run, indexes all pages in configured spaces
- Incremental updates detect page changes via Confluence's `lastModified`
- Semantic search finds relevant docs even when keyword search fails
- Embeddings stored in a local vector database (LanceDB)

This means `itachi wiki "how do we handle PCI compliance?"` returns results in milliseconds, not seconds.

### 4. Standup Generation
`itachi standup` pulls your actual Jira activity from the last 24 hours:

```
 Standup for Feb 28, 2026

Yesterday:
  - PAY-142: Moved "Add retry logic to payment processor" → In Review
  - PAY-145: Resolved "Timeout on webhook delivery" (Bug fix)
  - Reviewed PAY-148: "Idempotency key implementation"

Today:
  - PAY-150: "Payment reconciliation batch job" (In Progress)
  - PAY-142: Address review comments and merge

Blockers:
  - PAY-151: Waiting on API spec from partner team
    (blocked since Feb 26, assigned to @external-team)
```

No manual tracking — generated from real Jira transitions.

### 5. Onboarding Brief Generation
`itachi onboard payments-service` generates a comprehensive onboarding document:

```
 Onboarding Brief: Payments Service

 Overview
  [Pulled from Confluence "Payments Service" root page]

 Architecture
  [From Confluence ADR + architecture docs]

 Key People
  - Tech Lead: @alice (most Jira activity in PAY project)
  - Domain Expert: @bob (most Confluence edits in PAY space)

 Recent Activity (last 30 days)
  - 12 stories completed, 3 bugs fixed
  - Major: Payment retry logic shipped (PAY-142)
  - In Progress: Reconciliation batch job (PAY-150)

 Key Documents
  - [Payment Architecture RFC](confluence-link)
  - [PCI Compliance Checklist](confluence-link)
  - [Incident Response Runbook](confluence-link)

 Open Questions / Active Discussions
  - Partner API integration approach (PAY-151)
  - Reconciliation frequency (see RFC comments)
```

### 6. Sprint Analytics
`itachi sprint` provides at-a-glance sprint health:
- Completion percentage and burndown trajectory
- Velocity trend (last N sprints)
- Scope creep detection (stories added mid-sprint)
- Cycle time distribution
- Team workload balance

## Nakama Suite Synergies

Itachi becomes exponentially more powerful when paired with other Nakama tools:

| Combination | Synergy |
|-------------|---------|
| **Itachi + Tensai** | Morning briefing auto-includes Jira sprint data + Confluence updates |
| **Itachi + Shinigami** | Commit messages auto-link to Jira tickets, PR descriptions pull Confluence context |
| **Itachi + Byakugan** | PR reviews reference Jira requirements and Confluence design docs for completeness |
| **Itachi + Senku** | Codebase queries enriched with org context ("why was this built?" pulls code + Confluence RFC) |

### Inter-Tool Communication
Itachi exposes a pipe-friendly JSON API for other tools:
```bash
# Tensai pulls Jira data for morning brief
itachi jira "my open tickets" --format=json | tensai ingest --source=jira

# Byakugan enriches PR review with Jira context
itachi jira "PROJ-142" --format=json | byakugan review --context=stdin

# Shinigami auto-links commits to Jira
shinigami commit | itachi link --from-commit
```

## Data Flow — Cross-Platform Query

```
User: "what's the status of the migration project?"
        │
        ▼
  Query Planner
        │
        ├──→ Jira Query Plan:
        │      JQL for migration epic + stories + sprint
        │
        ├──→ Confluence Query Plan:
        │      CQL for migration design docs + ADRs
        │
        ▼
  Parallel Execution
        │
        ├──→ Jira Client ──→ issues, sprint data, blockers
        │
        ├──→ Confluence Client ──→ pages, content
        │
        ├──→ Embedding Search ──→ semantically related docs
        │
        ▼
  Cross-Reference Engine
        │
        ├── Match tickets ↔ documents
        ├── Detect gaps (undocumented work, untracked docs)
        │
        ▼
  Context Builder ──→ assemble LLM context (within token limits)
        │
        ▼
  LLM Synthesizer ──→ answer with source attribution
        │
        ▼
  Output (terminal with links to Jira/Confluence)
```

## Configuration

File: `~/.itachi/config.toml`

```toml
[llm]
provider = "anthropic"
model = "claude-sonnet-4-6"

[atlassian]
instance = "https://your-org.atlassian.net"
auth_method = "api_token"            # api_token | oauth2 | pat
email = ""                           # for API token auth
# token stored in OS keychain

[jira]
default_project = "PAY"
boards = ["PAY Board", "Backend Board"]
custom_fields = { story_points = "customfield_10016" }

[confluence]
default_spaces = ["ENG", "PAY", "PLATFORM"]
index_on_start = false               # true to auto-index on first query

[embedding]
provider = "anthropic"
model = "voyage-code-3"
store = "~/.itachi/embeddings/"
auto_update = true
update_interval_hours = 6

[cross_reference]
scan_explicit_links = true
scan_ticket_mentions = true           # find PROJ-XXX in Confluence pages
semantic_matching = true              # LLM-based relationship discovery
semantic_threshold = 0.75

[standup]
lookback_hours = 24
include_reviews = true
include_comments = false
format = "markdown"                   # markdown | slack | plain

[sprint]
velocity_sprints = 5                  # how many past sprints for velocity calc
burndown_resolution = "daily"

[output]
color = true
links = true                          # clickable Jira/Confluence links
max_results = 20
```

## Tech Stack

- **Language:** Rust
- **CLI framework:** clap
- **Jira API:** reqwest + Jira REST API v3
- **Confluence API:** reqwest + Confluence REST API v2
- **OAuth2:** oauth2-rs (for Atlassian Cloud 3LO)
- **Credential storage:** keyring-rs (OS keychain)
- **Embeddings:** voyage-code-3 via API
- **Vector store:** LanceDB (embedded, local)
- **HTML parsing:** scraper (for Confluence storage format → markdown)
- **TUI:** ratatui (for sprint dashboard)
- **LLM integration:** shared nakama LLM abstraction layer
