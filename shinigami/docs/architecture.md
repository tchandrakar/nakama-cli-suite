# Shinigami — Git Workflow Automator

> "Writing fate, documenting every action." — Inspired by the Shinigami from Death Note.

## Overview

Shinigami is an AI-powered git workflow tool that automates commit messages, changelogs, branch management, and release notes. It observes your changes and writes the story of your code's evolution.

## Core Commands

| Command | Description |
|---------|-------------|
| `shinigami commit` | Stage and commit with an AI-generated message |
| `shinigami reap` | Generate changelog from commit history |
| `shinigami branch <description>` | Create a well-named branch from a description |
| `shinigami squash` | Interactively squash commits with a summarized message |
| `shinigami release <version>` | Generate release notes for a version |
| `shinigami review` | Pre-commit review — summarize what you're about to commit |
| `shinigami hook install` | Install as a git hook (prepare-commit-msg) |

## Architecture

```
┌──────────────────────────────────────────────────┐
│                    CLI Layer                      │
│          (commands, flags, interactive UI)        │
├──────────────────────────────────────────────────┤
│                 Git Interface                     │
│  ┌────────────┐ ┌───────────┐ ┌───────────────┐  │
│  │ Diff       │ │ Log       │ │ Branch /      │  │
│  │ Parser     │ │ Reader    │ │ Tag Manager   │  │
│  └────────────┘ └───────────┘ └───────────────┘  │
├──────────────────────────────────────────────────┤
│              Semantic Analyzer                    │
│  ┌────────────────────────────────────────────┐   │
│  │ Diff classification:                       │   │
│  │  - feat / fix / refactor / docs / test     │   │
│  │ Scope detection:                           │   │
│  │  - module, file, function affected         │   │
│  │ Breaking change detection                  │   │
│  └────────────────────────────────────────────┘   │
├──────────────────────────────────────────────────┤
│              LLM Message Generator               │
│  ┌────────────────────────────────────────────┐   │
│  │ Input: diff + classification + repo context│   │
│  │ Output: conventional commit message        │   │
│  │ Style: configurable (conventional, gitmoji,│   │
│  │         freeform, team-custom)             │   │
│  └────────────────────────────────────────────┘   │
├──────────────────────────────────────────────────┤
│              Changelog Engine                     │
│  ┌───────────────┐ ┌──────────────────────────┐   │
│  │ Commit        │ │ Template Renderer        │   │
│  │ Aggregator    │ │ (keep-a-changelog, etc.) │   │
│  └───────────────┘ └──────────────────────────┘   │
├──────────────────────────────────────────────────┤
│              Git Hook Integration                 │
│       (prepare-commit-msg, commit-msg)           │
└──────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Diff-Aware Commit Messages
Shinigami doesn't just look at file names — it reads the actual diff to understand:
- **What changed:** Added a function? Fixed a null check? Renamed a variable?
- **Why it matters:** Is this a bug fix, a feature, a refactor?
- **Scope:** Which module/component is affected?

### 2. Conventional Commits by Default
Generated messages follow the Conventional Commits spec:
```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```
Users can switch to other styles (gitmoji, freeform, or a custom template).

### 3. Changelog Generation (`shinigami reap`)
- Reads commits between two refs (tags, SHAs, or date ranges)
- Groups by type (Features, Bug Fixes, Breaking Changes, etc.)
- Renders to Markdown using configurable templates
- Supports Keep a Changelog format out of the box

### 4. Branch Naming
`shinigami branch "add user authentication with OAuth"` generates:
```
feat/add-user-auth-oauth
```
Naming conventions are configurable (prefix/kebab, prefix/snake, etc.).

### 5. Git Hook Mode
When installed as a `prepare-commit-msg` hook:
- Runs automatically on every `git commit`
- Pre-fills the commit message editor with an AI suggestion
- User can edit before saving — never commits without human review

## Data Flow — Commit

```
git diff (staged) ──→ Diff Parser
                          │
                          ▼
                   Semantic Analyzer
                   (type, scope, breaking?)
                          │
                          ▼
                   LLM Message Generator
                   (diff + context → message)
                          │
                          ▼
                   User Confirmation (edit / accept / reject)
                          │
                          ▼
                   git commit -m "<message>"
```

## Configuration

File: `~/.shinigami/config.toml`

```toml
[llm]
provider = "anthropic"
model = "claude-haiku-4-5-20251001"

[commit]
style = "conventional"           # conventional | gitmoji | freeform | custom
include_body = true
max_subject_length = 72
sign_commits = false

[changelog]
template = "keep-a-changelog"
output = "CHANGELOG.md"
group_by = "type"

[branch]
pattern = "{type}/{kebab-description}"
max_length = 50
```

## Tech Stack

- **Language:** Rust
- **CLI framework:** clap
- **Git operations:** git2-rs (libgit2 bindings)
- **LLM integration:** shared nakama LLM abstraction layer
- **Templating:** tera (for changelog/release note templates)
