# Step 007: Build nakama-ai (Multi-Provider AI Abstraction)

## Objective
Implement the AI provider abstraction — one interface for Claude, OpenAI, Gemini, and Ollama.

## Tasks
- AIProvider trait: complete(), complete_stream(), embed()
- Unified types: CompletionRequest, CompletionResponse, Message, ContentBlock, TokenUsage, ModelSpec
- ModelTier enum: Fast, Balanced, Powerful → resolved to concrete models per provider
- Anthropic provider: Messages API, streaming, tool use, system prompts
- OpenAI provider: Chat Completions API, streaming, function calling
- Google Gemini provider: generateContent API, streaming, tool config
- Ollama provider: OpenAI-compatible API, local models
- Provider resolution order: CLI flag → tool config → global config → default (anthropic)
- Rate limiter: token bucket (governor crate), shared across tools via lockfile
- Retry logic: exponential backoff with jitter, max 3 retries, circuit breaker after 5 fails
- Token counting: per-provider (tiktoken-rs for OpenAI, estimated for others)
- Cost estimation: per-provider pricing tables, tracked per request
- Streaming: async stream normalization across all providers
- PromptBuilder utility: system(), context(), user(), build()
- Credential retrieval: API keys fetched from nakama-vault at runtime
- Audit integration: every LLM call logged via nakama-audit
- Unit tests: mock provider, rate limiting, retry logic, cost calculation, provider resolution

## Acceptance Criteria
- Same code works with any provider by changing config
- `zangetsu ask "find files" --ai-provider=openai` overrides default
- Rate limiter prevents hitting provider limits across concurrent tools
- Cost tracking accurate to within 10% of actual billing
- Streaming works smoothly with all providers
- API keys never appear in logs or error messages

## Dependencies
- Step 002 (nakama-core), Step 003 (nakama-vault), Step 004 (nakama-log), Step 006 (nakama-audit)
