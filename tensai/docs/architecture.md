# Tensai — Dev Briefing Dashboard

> "Seeing the whole field, orchestrating plays." — Inspired by the prodigies of Kuroko no Basket.

## Overview

Tensai is your personal developer briefing tool. It aggregates data from GitHub, CI/CD, calendar, and project management tools to give you a single, prioritized view of your dev day. Like a genius coach who sees the entire court.

## Core Commands

| Command | Description |
|---------|-------------|
| `tensai brief` | Morning briefing — everything you need to know |
| `tensai standup` | Generate a standup update from your recent activity |
| `tensai plan` | AI-prioritized task list for today |
| `tensai status` | Live dashboard of PRs, CI, issues |
| `tensai review` | Show PRs that need your review |
| `tensai focus` | Enter focus mode — suppress notifications, track deep work |

## Architecture

```
┌───────────────────────────────────────────────────┐
│                    CLI Layer                       │
│        (commands, TUI dashboard mode)             │
├───────────────────────────────────────────────────┤
│                Data Aggregators                    │
│  ┌──────────┐ ┌──────────┐ ┌───────────────────┐  │
│  │ GitHub   │ │ CI/CD    │ │ Calendar          │  │
│  │          │ │          │ │                   │  │
│  │ - PRs    │ │ - Builds │ │ - Meetings today  │  │
│  │ - Issues │ │ - Deploy │ │ - Focus blocks    │  │
│  │ - Reviews│ │ - Status │ │ - Deadlines       │  │
│  │ - Notifs │ │          │ │                   │  │
│  └──────────┘ └──────────┘ └───────────────────┘  │
│  ┌──────────┐ ┌──────────┐ ┌───────────────────┐  │
│  │ Jira /   │ │ Slack    │ │ Git (local)       │  │
│  │ Linear   │ │          │ │                   │  │
│  │          │ │ - DMs    │ │ - Branches        │  │
│  │ - Sprint │ │ - Channel│ │ - Uncommitted     │  │
│  │ - Tickets│ │   mentions│ │ - Stashes        │  │
│  └──────────┘ └──────────┘ └───────────────────┘  │
├───────────────────────────────────────────────────┤
│              Prioritization Engine                  │
│  ┌─────────────────────────────────────────────┐   │
│  │ Scoring factors:                            │   │
│  │  - Urgency (deadlines, blocking others)     │   │
│  │  - Importance (sprint goals, user impact)   │   │
│  │  - Context cost (how many context switches) │   │
│  │  - Available time (meetings, focus blocks)  │   │
│  │                                             │   │
│  │ LLM-assisted reasoning for complex tradeoffs│   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│              Standup Generator                      │
│  ┌─────────────────────────────────────────────┐   │
│  │ Input: commits, PRs merged, issues closed,  │   │
│  │   reviews done (last 24h)                   │   │
│  │ Output: "Yesterday I..., Today I'll...,     │   │
│  │   Blockers: ..."                            │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│              Output Renderers                      │
│  ┌──────────┐ ┌────────────┐ ┌────────────────┐   │
│  │ Terminal │ │ TUI        │ │ Markdown       │   │
│  │ (brief)  │ │ (dashboard)│ │ (export)       │   │
│  └──────────┘ └────────────┘ └────────────────┘   │
└───────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Plugin-Based Data Aggregators
Each data source is an independent plugin:
- **GitHub** — uses `gh` CLI / GitHub API for PRs, issues, notifications
- **CI/CD** — GitHub Actions, GitLab CI, CircleCI, Jenkins
- **Calendar** — Google Calendar, Outlook via API
- **Project management** — Jira, Linear, Notion via respective APIs
- **Slack** — unread DMs and mentions
- **Local git** — branches, uncommitted work, stashes

Users enable only what they use. Each plugin implements a common `DataSource` trait.

### 2. AI-Powered Prioritization
`tensai plan` doesn't just list tasks — it prioritizes them:
- Considers deadlines, who's blocked, sprint goals
- Accounts for your meeting schedule (no big tasks before a meeting in 30 min)
- Minimizes context switching (groups related tasks)
- The LLM synthesizes all signals into a recommended order with reasoning

### 3. Standup Generation
`tensai standup` generates a ready-to-paste standup message:
```
Yesterday:
- Merged PR #142: Add rate limiting to auth service
- Reviewed PR #145: Database migration for user preferences
- Fixed issue #89: Timeout on large file uploads

Today:
- Continue PR #148: OAuth2 integration
- Review PR #150: Frontend refactor

Blockers:
- Waiting on API spec from backend team (issue #91)
```
Generated from actual git/GitHub activity — no manual tracking needed.

### 4. Focus Mode
`tensai focus` starts a focus session:
- Tracks time spent on the current task
- Optionally sets Slack status to "In Focus Mode"
- Suppresses non-critical notifications
- On exit, logs what was accomplished

### 5. Morning Briefing Format
`tensai brief` outputs a concise, scannable briefing:
```
 Good morning! Here's your dev briefing for Feb 27, 2026

 PRs Needing Review (3)
  #150 Frontend refactor — @alice, 2 days old
  #152 API versioning — @bob, waiting on you
  #153 Bug fix: login redirect — @charlie, small

 Your Open PRs (2)
  #148 OAuth2 integration — CI passing, 1 approval
  #149 Rate limit config — needs rebase

 CI/CD
  main: green | staging: deploying | prod: stable

 Calendar
  10:00 Sprint planning (1h)
  14:00 1:1 with manager (30m)

 Suggested Plan
  1. Review #152 (small, blocking Bob)
  2. Rebase #149 before sprint planning
  3. Continue #148 after sprint planning
  4. Review #150 and #153 after 1:1
```

## Configuration

File: `~/.tensai/config.toml`

```toml
[llm]
provider = "anthropic"
model = "claude-haiku-4-5-20251001"

[sources]
github = true
ci = "github_actions"
calendar = "google"
project = "linear"
slack = false
local_git = true

[github]
username = "your-username"
orgs = ["your-org"]

[calendar]
calendar_id = "primary"

[brief]
time = "09:00"                       # suggested briefing time
include_weather = false

[standup]
lookback_hours = 24
format = "markdown"                  # markdown | slack | plain

[focus]
default_duration_minutes = 90
update_slack_status = true
```

## Tech Stack

- **Language:** Rust
- **CLI framework:** clap
- **TUI:** ratatui (for live dashboard)
- **GitHub:** octocrab (GitHub API client)
- **Calendar:** Google Calendar API via reqwest
- **Project management:** REST API clients (Jira, Linear)
- **LLM integration:** shared nakama LLM abstraction layer
