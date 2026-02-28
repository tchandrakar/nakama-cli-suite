# Gate — API Explorer / HTTP Client

> "Sending messages across boundaries." — Inspired by Steins;Gate.

## Overview

Gate is an interactive API explorer and HTTP client for the terminal. It lets you discover, test, and replay API calls with AI-assisted request building, response analysis, and automatic documentation. Your gateway to external services. *El Psy Kongroo.*

## Core Commands

| Command | Description |
|---------|-------------|
| `gate send <method> <url>` | Send an HTTP request |
| `gate explore <base_url>` | Interactive API exploration mode |
| `gate import <spec>` | Import from OpenAPI/Swagger, Postman, cURL |
| `gate replay <request_id>` | Replay a saved request |
| `gate flow <name>` | Run a multi-step request flow (chained requests) |
| `gate mock <spec>` | Start a local mock server from an API spec |
| `gate diff <req1> <req2>` | Compare two API responses |
| `gate doc <collection>` | Generate API documentation from saved requests |

## Architecture

```
┌───────────────────────────────────────────────────┐
│                    CLI Layer                       │
│     (commands, interactive explorer TUI)          │
├───────────────────────────────────────────────────┤
│                Request Builder                     │
│  ┌────────────┐ ┌────────────┐ ┌───────────────┐  │
│  │ Manual     │ │ Natural    │ │ Import        │  │
│  │ (flags,    │ │ Language   │ │ (OpenAPI,     │  │
│  │  args)     │ │ ("GET all  │ │  Postman,     │  │
│  │            │ │  users     │ │  cURL, HAR)   │  │
│  │            │ │  sorted    │ │               │  │
│  │            │ │  by name") │ │               │  │
│  └────────────┘ └────────────┘ └───────────────┘  │
│  ┌─────────────────────────────────────────────┐   │
│  │ Request Object:                             │   │
│  │  { method, url, headers, query_params,      │   │
│  │    body, auth, timeout }                    │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│                HTTP Engine                          │
│  ┌─────────────────────────────────────────────┐   │
│  │ - Connection pooling and reuse              │   │
│  │ - TLS with certificate pinning support      │   │
│  │ - Redirect following (configurable)         │   │
│  │ - Retry with backoff                        │   │
│  │ - Streaming response support                │   │
│  │ - WebSocket upgrade support                 │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│              Response Analyzer                      │
│  ┌────────────────┐ ┌──────────────────────────┐   │
│  │ Format &       │ │ LLM Analysis             │   │
│  │ Highlight      │ │ (explain response,       │   │
│  │ (JSON, XML,    │ │  suggest next requests,  │   │
│  │  HTML, binary) │ │  detect errors/issues)   │   │
│  └────────────────┘ └──────────────────────────┘   │
│  ┌────────────────┐ ┌──────────────────────────┐   │
│  │ Schema         │ │ Diff Engine              │   │
│  │ Inference      │ │ (compare responses,      │   │
│  │ (auto-detect   │ │  track API changes)      │   │
│  │  response      │ │                          │   │
│  │  structure)    │ │                          │   │
│  └────────────────┘ └──────────────────────────┘   │
├───────────────────────────────────────────────────┤
│              Flow Engine (Chained Requests)         │
│  ┌─────────────────────────────────────────────┐   │
│  │ - Sequential request chains                 │   │
│  │ - Variable extraction from responses        │   │
│  │   (e.g., token from login → auth header)    │   │
│  │ - Conditional branching                     │   │
│  │ - Loop over collections                     │   │
│  │ - Assertions on responses                   │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│              Storage & Collections                  │
│  ┌─────────────────┐ ┌────────────────────────┐   │
│  │ Request         │ │ Environment            │   │
│  │ History         │ │ Manager                │   │
│  │ (all requests   │ │ (dev, staging, prod    │   │
│  │  + responses)   │ │  variable sets)        │   │
│  └─────────────────┘ └────────────────────────┘   │
│  ┌─────────────────────────────────────────────┐   │
│  │ Collections                                 │   │
│  │ (organized groups of saved requests,        │   │
│  │  shareable as YAML/JSON files)              │   │
│  └─────────────────────────────────────────────┘   │
├───────────────────────────────────────────────────┤
│              Mock Server                            │
│  ┌─────────────────────────────────────────────┐   │
│  │ - Generate mock responses from OpenAPI spec │   │
│  │ - Record real responses → replay as mocks   │   │
│  │ - Configurable latency, error simulation    │   │
│  └─────────────────────────────────────────────┘   │
└───────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Natural Language Request Building
Gate understands natural language descriptions of API calls:
```
gate send "create a new user with name John and email john@example.com" \
  --base https://api.example.com
```
The LLM infers the method (POST), path (/users), and body format from the description and any imported API spec.

### 2. Interactive Exploration Mode
`gate explore https://api.example.com` launches a TUI where you can:
- Browse available endpoints (from imported spec or auto-discovered)
- Build requests visually with parameter editing
- See response with syntax highlighting and folding
- Chain requests by extracting values from responses
- View request history in a sidebar

### 3. Flow Engine for Multi-Step Scenarios
Flows are YAML-defined sequences of requests:
```yaml
name: user-login-flow
steps:
  - name: login
    method: POST
    url: "{{base_url}}/auth/login"
    body:
      email: "{{email}}"
      password: "{{password}}"
    extract:
      token: "response.body.access_token"

  - name: get-profile
    method: GET
    url: "{{base_url}}/users/me"
    headers:
      Authorization: "Bearer {{token}}"
    assert:
      status: 200
      body.email: "{{email}}"
```
Variables chain between steps. Assertions validate the flow.

### 4. Environment Management
Like Postman environments but in config files:
```toml
[environments.dev]
base_url = "http://localhost:3000"
email = "test@dev.local"

[environments.staging]
base_url = "https://staging-api.example.com"
email = "test@staging.example.com"
```
Switch with `gate send --env staging ...`

### 5. Response Diffing
`gate diff` compares two responses to detect API changes:
- Structural differences (new/removed fields)
- Type changes (string → number)
- Value changes with context
- Useful for testing API version upgrades or comparing environments

### 6. Mock Server
`gate mock openapi.yaml --port 8080` starts a local mock server that:
- Serves realistic fake responses based on the spec
- Supports record mode (proxy real API, save responses, replay later)
- Configurable error injection and latency simulation

## Data Flow — Send

```
User input (flags or natural language)
        │
        ▼
  Request Builder ──→ structured request object
        │
        ▼
  Environment Resolution ──→ fill in variables
        │
        ▼
  HTTP Engine ──→ send request, receive response
        │
        ▼
  Response Analyzer
        ├── Format & highlight
        ├── Schema inference
        └── LLM analysis (optional)
        │
        ▼
  Display + Store in History
```

## Configuration

File: `~/.gate/config.toml`

```toml
[llm]
provider = "anthropic"
model = "claude-haiku-4-5-20251001"

[http]
timeout_seconds = 30
follow_redirects = true
max_redirects = 10
verify_ssl = true
user_agent = "gate-cli/0.1"

[display]
color = true
pretty_print = true
max_body_display_kb = 100

[history]
store = "~/.gate/history.db"
max_entries = 10000

[environments]
default = "dev"

[mock]
default_port = 8080
latency_ms = 50
```

## Tech Stack

- **Language:** Rust
- **CLI framework:** clap
- **HTTP client:** reqwest (with connection pooling)
- **WebSocket:** tokio-tungstenite
- **TUI:** ratatui (for explore mode)
- **Mock server:** axum (lightweight async HTTP server)
- **Storage:** SQLite for history, YAML/TOML for collections
- **OpenAPI parsing:** openapiv3
- **LLM integration:** shared nakama LLM abstraction layer
