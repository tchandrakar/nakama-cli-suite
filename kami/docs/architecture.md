# Kami — Gemini-Powered Google Search Integration

> "Divine omniscient awareness." — Inspired by Kami, Guardian of Earth from Dragon Ball.

## Overview

Kami is a CLI tool that brings Google's search and Gemini AI directly into your terminal. It performs grounded searches (Gemini answers backed by real-time Google Search results with citations), multi-step research, URL summarization, and conversational Q&A — all pipe-friendly and scriptable. Your divine oracle for developer knowledge.

## Core Commands

| Command | Description |
|---------|-------------|
| `kami search <query>` | Quick grounded search with cited results |
| `kami deep <query>` | Multi-step research — explores, follows links, synthesizes |
| `kami summarize <url>` | Fetch a URL and summarize with Gemini |
| `kami ask <question>` | Conversational mode with session memory |
| `kami grounded <claim>` | Fact-check a claim with grounded citations |
| `kami compare <a> vs <b>` | Side-by-side comparison with sourced evidence |
| `kami pipe` | Read from stdin, enrich with search context, output to stdout |

## Architecture

```
┌───────────────────────────────────────────────────────┐
│                      CLI Layer                        │
│     (commands, conversational REPL, pipe mode)        │
├───────────────────────────────────────────────────────┤
│                  Query Planner                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │ Classifies query intent:                        │   │
│  │                                                 │   │
│  │ - quick_search: single query, direct answer     │   │
│  │ - deep_research: multi-step, needs exploration  │   │
│  │ - summarize: URL content extraction             │   │
│  │ - factcheck: claim verification                 │   │
│  │ - compare: multi-entity comparison              │   │
│  │                                                 │   │
│  │ For deep_research, generates a search plan:     │   │
│  │   1. Break question into sub-queries            │   │
│  │   2. Determine search order and dependencies    │   │
│  │   3. Define synthesis strategy                  │   │
│  └─────────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────────┤
│                  Auth Layer                             │
│  ┌──────────────────┐ ┌───────────────────────────┐    │
│  │ OAuth2 Flow      │ │ Token Manager             │    │
│  │                  │ │                           │    │
│  │ - Browser-based  │ │ - Secure local storage    │    │
│  │   consent flow   │ │   (OS keychain)           │    │
│  │ - Localhost       │ │ - Auto-refresh on expiry  │    │
│  │   redirect       │ │ - Scope management        │    │
│  │ - Scopes:        │ │                           │    │
│  │   generative AI, │ │                           │    │
│  │   custom search  │ │                           │    │
│  └──────────────────┘ └───────────────────────────┘    │
├───────────────────────────────────────────────────────┤
│                  Provider Layer                         │
│  ┌──────────────────────┐ ┌────────────────────────┐   │
│  │ Gemini API           │ │ Google Search          │   │
│  │                      │ │                        │   │
│  │ - generateContent    │ │ - Custom Search JSON   │   │
│  │ - Grounded search    │ │   API                  │   │
│  │   (search_tool in    │ │ - Programmable Search  │   │
│  │    tool_config)      │ │   Engine (backup)      │   │
│  │ - Streaming support  │ │                        │   │
│  │ - Chat sessions      │ │                        │   │
│  │   (multi-turn)       │ │                        │   │
│  └──────────────────────┘ └────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │ URL Fetcher                                      │   │
│  │                                                  │   │
│  │ - HTTP fetch with configurable user-agent        │   │
│  │ - HTML → Markdown extraction (readability)       │   │
│  │ - PDF text extraction                            │   │
│  │ - Respects robots.txt                            │   │
│  └──────────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────────┤
│                  Synthesis Engine                       │
│  ┌─────────────────────────────────────────────────┐   │
│  │ Takes raw search results + fetched content and  │   │
│  │ produces structured output:                     │   │
│  │                                                 │   │
│  │ - Answer with inline citations [1][2][3]        │   │
│  │ - Confidence indicators                         │   │
│  │ - Source list with URLs                         │   │
│  │ - "Related queries" suggestions                 │   │
│  │                                                 │   │
│  │ For deep research:                              │   │
│  │ - Synthesizes across multiple search rounds     │   │
│  │ - Resolves contradictions between sources       │   │
│  │ - Highlights consensus vs. disagreement         │   │
│  └─────────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────────┤
│                  Cache Layer                            │
│  ┌─────────────────────────────────────────────────┐   │
│  │ Local SQLite cache:                             │   │
│  │  - Search results (TTL: configurable, def. 1h)  │   │
│  │  - Fetched URL content (TTL: 24h)               │   │
│  │  - Conversation sessions                        │   │
│  │                                                 │   │
│  │ Cache-aware: identical queries within TTL       │   │
│  │ return instantly without API calls              │   │
│  └─────────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────────┤
│                  Output Layer                          │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐   │
│  │ Terminal     │ │ Markdown     │ │ Pipe / JSON  │   │
│  │ (rich,       │ │ (export)     │ │ (stdout for  │   │
│  │  colored,    │ │              │ │  chaining)   │   │
│  │  citations   │ │              │ │              │   │
│  │  highlighted)│ │              │ │              │   │
│  └──────────────┘ └──────────────┘ └──────────────┘   │
└───────────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Gemini Grounded Search as the Core
The primary feature is Gemini's **grounded generation** — where Gemini generates answers backed by real-time Google Search results. This means:
- Answers are based on current, live data (not just training data)
- Every claim comes with a citation to the source
- The user can verify any statement by following the source link

This is done via Gemini's `tools: [{ google_search: {} }]` configuration, which lets Gemini autonomously decide when to search and what to search for.

### 2. Multi-Step Deep Research
`kami deep` goes beyond a single search:
```
kami deep "compare React Server Components vs Astro Islands for performance"
```
1. **Plan:** Break into sub-queries ("RSC performance benchmarks", "Astro Islands hydration strategy", "RSC vs Islands architecture comparison")
2. **Search:** Execute each sub-query via grounded Gemini
3. **Fetch:** Optionally fetch and read key URLs for deeper content
4. **Synthesize:** Combine all findings into a comprehensive, cited answer
5. **Present:** Structured output with sections, citations, and a summary

### 3. Pipe-Friendly Output
Kami is designed to fit into Unix pipelines:
```bash
# Enrich a log error with context
echo "ECONNREFUSED 127.0.0.1:5432" | kami pipe "what causes this error?"

# Feed search results into another tool
kami search "kubernetes pod eviction" --format=json | jq '.sources[].url'

# Chain with other nakama tools
jogan diagnose "pod crash" | kami pipe "search for known issues matching this"
```

`--format=json` outputs structured JSON for machine consumption. Default is rich terminal output for humans.

### 4. OAuth2 with Local Token Storage
Authentication flow:
1. First run: `kami auth` opens browser for Google OAuth2 consent
2. User approves scopes (Gemini API, Custom Search)
3. Token is stored in the OS keychain (macOS Keychain, Linux secret-service, Windows Credential Manager)
4. Subsequent runs use the stored token, auto-refreshing as needed
5. Alternative: `GEMINI_API_KEY` env var for API-key auth (simpler, no OAuth)

### 5. Conversation Sessions
`kami ask` maintains a conversation with session memory:
```
$ kami ask
kami> what are the best rust async runtimes?
[Answer with citations...]

kami> how does tokio compare to smol?
[Follow-up answer that remembers the context...]

kami> /save rust-async-research
Session saved.
```
Sessions are stored locally and can be resumed later.

### 6. Fact-Checking Mode
`kami grounded` is specifically designed for verifying claims:
```
kami grounded "Log4Shell CVE-2021-44228 affects all versions of Log4j 2.x"
```
Output:
```
Claim: "Log4Shell CVE-2021-44228 affects all versions of Log4j 2.x"

Verdict: PARTIALLY TRUE

Details: CVE-2021-44228 affects Apache Log4j 2.0-beta9 through 2.14.1,
not all 2.x versions. Versions 2.15.0+ include fixes, though 2.15.0
itself had an incomplete fix (CVE-2021-45046). [1][2]

Sources:
[1] https://nvd.nist.gov/vuln/detail/CVE-2021-44228
[2] https://logging.apache.org/log4j/2.x/security.html
```

## Data Flow — Search

```
User query
        │
        ▼
  Query Planner ──→ classify intent, generate search plan
        │
        ▼
  Cache Check ──→ HIT → return cached result
        │ MISS
        ▼
  Gemini Grounded Search ──→ answer + search results + citations
        │
        ▼
  Synthesis Engine ──→ format answer, attach citations, suggest follow-ups
        │
        ▼
  Cache Store ──→ save result with TTL
        │
        ▼
  Output (terminal / markdown / JSON)
```

## Data Flow — Deep Research

```
User query
        │
        ▼
  Query Planner ──→ decompose into sub-queries
        │
        ▼
  ┌─── Loop over sub-queries ───┐
  │                              │
  │  Gemini Grounded Search      │
  │         │                    │
  │         ▼                    │
  │  Optional: Fetch key URLs    │
  │         │                    │
  │         ▼                    │
  │  Accumulate findings         │
  │                              │
  └──────────────────────────────┘
        │
        ▼
  Cross-Query Synthesis ──→ resolve contradictions, find consensus
        │
        ▼
  Structured Report with sections + all citations
```

## Configuration

File: `~/.kami/config.toml`

```toml
[auth]
method = "oauth2"                    # oauth2 | api_key
# api_key = ""                      # or use GEMINI_API_KEY env var

[gemini]
model = "gemini-2.5-pro"
temperature = 0.3
max_output_tokens = 4096
safety_settings = "default"

[search]
grounding = true
max_results = 10
region = "us"                        # search region
language = "en"                      # search language

[cache]
enabled = true
store = "~/.kami/cache.db"
search_ttl_minutes = 60
url_ttl_hours = 24
max_cache_size_mb = 100

[deep_research]
max_sub_queries = 5
max_url_fetches = 10
timeout_per_step_seconds = 30

[output]
default_format = "terminal"          # terminal | markdown | json
show_citations = true
show_confidence = true
show_related_queries = true

[sessions]
store = "~/.kami/sessions/"
max_history_per_session = 50
```

## Tech Stack

- **Language:** Rust
- **CLI framework:** clap
- **Gemini API:** reqwest + google-generativeai REST API
- **Google Search:** Custom Search JSON API via reqwest
- **OAuth2:** oauth2-rs with browser-based flow
- **Token storage:** keyring-rs (OS keychain abstraction)
- **URL fetching:** reqwest + scraper (HTML → text extraction)
- **Cache:** SQLite via rusqlite
- **Output:** crossterm for rich terminal, serde_json for JSON output
- **REPL:** rustyline (for conversational mode)
