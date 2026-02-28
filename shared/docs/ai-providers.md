# Shared AI Provider Abstraction — Nakama CLI Suite

> One interface, any AI. Claude, OpenAI, or Gemini — the user decides.

## Overview

Every Nakama tool uses AI capabilities through a shared abstraction layer. The user picks their preferred provider once, and all tools use it. They can also override per-tool if needed (e.g., use Claude for code review but Gemini for search).

---

## 1. Provider Architecture

```
┌───────────────────────────────────────────────────────────┐
│                    Tool Layer                              │
│   (zangetsu, shinigami, jogan, senku, sharingan,          │
│    tensai, mugen, gate, byakugan, kami, itachi)            │
│                                                            │
│   All tools call the same AIProvider trait                  │
├───────────────────────────────────────────────────────────┤
│                   AIProvider Trait                          │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                                                      │  │
│  │  fn complete(request: CompletionRequest)              │  │
│  │      -> Result<CompletionResponse>                    │  │
│  │                                                      │  │
│  │  fn complete_stream(request: CompletionRequest)       │  │
│  │      -> Result<Stream<CompletionChunk>>               │  │
│  │                                                      │  │
│  │  fn embed(input: Vec<String>)                         │  │
│  │      -> Result<Vec<Embedding>>                        │  │
│  │                                                      │  │
│  └──────────────────────────────────────────────────────┘  │
├───────────────────────────────────────────────────────────┤
│                  Provider Implementations                   │
│                                                            │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐  │
│  │   Anthropic    │ │    OpenAI      │ │    Google      │  │
│  │   (Claude)     │ │    (GPT)       │ │    (Gemini)    │  │
│  │                │ │                │ │                │  │
│  │ Models:        │ │ Models:        │ │ Models:        │  │
│  │ claude-opus    │ │ gpt-4.1       │ │ gemini-2.5-pro │  │
│  │ claude-sonnet  │ │ gpt-4.1-mini  │ │ gemini-2.5-    │  │
│  │ claude-haiku   │ │ gpt-4.1-nano  │ │   flash        │  │
│  │                │ │ o4-mini        │ │                │  │
│  │ API:           │ │                │ │ API:           │  │
│  │ messages API   │ │ API:           │ │ generateContent│  │
│  │ with system    │ │ chat/          │ │ with tools     │  │
│  │ prompt         │ │ completions    │ │                │  │
│  └────────────────┘ └────────────────┘ └────────────────┘  │
│                                                            │
│  ┌────────────────┐                                        │
│  │   Ollama       │  ← Local/self-hosted models            │
│  │   (Local)      │                                        │
│  │                │                                        │
│  │ Models:        │                                        │
│  │ llama, mistral │                                        │
│  │ codellama, etc │                                        │
│  │                │                                        │
│  │ API:           │                                        │
│  │ OpenAI-compat  │                                        │
│  └────────────────┘                                        │
├───────────────────────────────────────────────────────────┤
│                  Request/Response Normalization             │
│                                                            │
│  Handles differences between providers:                     │
│  - Message format normalization                             │
│  - Tool/function calling format translation                │
│  - Token counting (provider-specific tokenizers)           │
│  - Rate limit handling and retry logic                     │
│  - Streaming protocol differences                          │
│  - Error code normalization                                │
│                                                            │
└───────────────────────────────────────────────────────────┘
```

---

## 2. Unified Data Types

```rust
/// Request sent to any AI provider
pub struct CompletionRequest {
    pub system_prompt: String,
    pub messages: Vec<Message>,
    pub model: ModelSpec,
    pub max_tokens: u32,
    pub temperature: f32,
    pub tools: Option<Vec<ToolDefinition>>,
    pub response_format: Option<ResponseFormat>,
}

/// Normalized message
pub struct Message {
    pub role: Role,               // System | User | Assistant | Tool
    pub content: Vec<ContentBlock>,
}

pub enum ContentBlock {
    Text(String),
    Image { media_type: String, data: Vec<u8> },
    ToolUse { id: String, name: String, input: Value },
    ToolResult { tool_use_id: String, content: String },
}

/// Normalized response
pub struct CompletionResponse {
    pub content: Vec<ContentBlock>,
    pub model: String,
    pub usage: TokenUsage,
    pub stop_reason: StopReason,
}

pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
    pub estimated_cost_usd: f64,    // tracked for user awareness
}

/// Stream chunk
pub struct CompletionChunk {
    pub delta: ContentDelta,
    pub usage: Option<TokenUsage>,   // final chunk includes usage
}

/// Model specification
pub struct ModelSpec {
    pub provider: Provider,
    pub model_id: String,
    pub tier: ModelTier,             // fast | balanced | powerful
}

pub enum Provider {
    Anthropic,
    OpenAI,
    Google,
    Ollama,
}

/// For tools that don't care about specific model — just capability tier
pub enum ModelTier {
    Fast,        // haiku / gpt-4.1-nano / gemini-flash   — log analysis, simple tasks
    Balanced,    // sonnet / gpt-4.1-mini / gemini-flash   — most tasks
    Powerful,    // opus / gpt-4.1 / gemini-pro            — complex reasoning
}
```

---

## 3. Provider Resolution

Tools can specify what they need by tier instead of exact model:

```rust
// Tool says "I need a fast model for log parsing"
let model = ai.resolve_model(ModelTier::Fast)?;
// → resolves to claude-haiku / gpt-4.1-nano / gemini-flash based on user config

// Tool says "I need the most capable model for code review"
let model = ai.resolve_model(ModelTier::Powerful)?;
// → resolves to claude-opus / gpt-4.1 / gemini-pro based on user config
```

### Default Model Mapping

| Tier | Anthropic | OpenAI | Google | Ollama |
|------|-----------|--------|--------|--------|
| Fast | claude-haiku-4-5 | gpt-4.1-nano | gemini-2.5-flash | llama3:8b |
| Balanced | claude-sonnet-4-6 | gpt-4.1-mini | gemini-2.5-flash | llama3:70b |
| Powerful | claude-opus-4-6 | gpt-4.1 | gemini-2.5-pro | — |

Users can override any mapping in config.

---

## 4. Rate Limiting & Retry

```
┌─────────────────────────────────────────┐
│          Rate Limit Manager             │
│                                         │
│  Shared across all tools per provider:  │
│                                         │
│  - Token bucket per provider            │
│  - Respects HTTP 429 + Retry-After      │
│  - Exponential backoff with jitter      │
│  - Max retries: 3 (configurable)        │
│  - Circuit breaker: open after 5 fails  │
│  - Queue: tools wait for capacity       │
│                                         │
│  Cross-tool coordination via lockfile:  │
│  ~/.nakama/rate_limits.lock             │
└─────────────────────────────────────────┘
```

When multiple Nakama tools run simultaneously (e.g., `itachi standup | tensai ingest`), the rate limiter ensures they don't collectively exceed provider limits.

---

## 5. Cost Tracking

Every API call tracks token usage and estimated cost:

```
$ nakama usage

  AI Usage (last 7 days)
  ┌──────────────┬────────────┬─────────────┬──────────┐
  │ Tool         │ Provider   │ Tokens      │ Cost     │
  ├──────────────┼────────────┼─────────────┼──────────┤
  │ byakugan     │ Anthropic  │ 125,400     │ $0.47    │
  │ shinigami    │ Anthropic  │  42,300     │ $0.08    │
  │ senku        │ OpenAI     │ 230,100     │ $0.92    │
  │ kami         │ Google     │  85,600     │ $0.21    │
  ├──────────────┼────────────┼─────────────┼──────────┤
  │ Total        │            │ 483,400     │ $1.68    │
  └──────────────┴────────────┴─────────────┴──────────┘

  Budget: $10.00/week — 16.8% used
```

Optional spending limits:
```toml
[ai.budget]
weekly_limit_usd = 10.00
alert_threshold_percent = 80
hard_limit = true               # true = block requests at limit
```

---

## 6. Configuration

### Global (all tools)
File: `~/.nakama/config.toml`

```toml
[ai]
default_provider = "anthropic"         # anthropic | openai | google | ollama

[ai.anthropic]
# API key stored in vault, not here
model_fast = "claude-haiku-4-5-20251001"
model_balanced = "claude-sonnet-4-6"
model_powerful = "claude-opus-4-6"
base_url = "https://api.anthropic.com"    # override for proxy

[ai.openai]
model_fast = "gpt-4.1-nano"
model_balanced = "gpt-4.1-mini"
model_powerful = "gpt-4.1"
base_url = "https://api.openai.com"

[ai.google]
model_fast = "gemini-2.5-flash"
model_balanced = "gemini-2.5-flash"
model_powerful = "gemini-2.5-pro"

[ai.ollama]
base_url = "http://localhost:11434"
model_fast = "llama3:8b"
model_balanced = "llama3:70b"

[ai.retry]
max_retries = 3
initial_backoff_ms = 1000
max_backoff_ms = 30000
backoff_multiplier = 2.0
```

### Per-Tool Override
In any tool's config (e.g., `~/.byakugan/config.toml`):
```toml
[ai]
provider = "openai"               # override just for this tool
model = "gpt-4.1"                 # exact model override
```

### CLI Override (highest priority)
```bash
# One-off provider switch
zangetsu ask "find large files" --ai-provider=openai --ai-model=gpt-4.1
```

### Resolution Order
```
CLI flag  →  Tool config  →  Global config  →  Default (anthropic)
```

---

## 7. Prompt Management

Each tool has its own system prompts, but the framework provides utilities:

```rust
/// Load a prompt template with variable substitution
pub fn load_prompt(tool: &str, name: &str, vars: &HashMap<&str, &str>) -> String;

/// Structured prompt builder
pub struct PromptBuilder {
    pub fn system(text: &str) -> Self;
    pub fn context(label: &str, content: &str) -> Self;
    pub fn user(text: &str) -> Self;
    pub fn build() -> Vec<Message>;
}
```

Prompts are stored as embedded resources (compiled into the binary), not external files that could be tampered with.

## Tech Stack

- **HTTP:** reqwest with connection pooling
- **Streaming:** tokio + async streams
- **Serialization:** serde for request/response normalization
- **Token counting:** tiktoken-rs (OpenAI), estimated for others
- **Rate limiting:** governor (token bucket)
- **Cost tracking:** SQLite (shared with audit)
