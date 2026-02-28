# Step 003: Gemini Provider

## Objective

Build the Gemini API client using reqwest and the REST API, supporting `generateContent` with the grounded search tool, streaming responses, multi-turn chat sessions, and configurable safety settings.

## Tasks

- [ ] Create `GeminiClient` struct with fields: reqwest client, api_key or OAuth token, model name, config
- [ ] Implement the base API request builder:
  - Construct request URL: `https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent`
  - Attach authentication (API key as query param or OAuth bearer token)
  - Set appropriate headers (Content-Type, User-Agent)
- [ ] Implement `generateContent` (non-streaming):
  - Build request body with `contents` (user/model message history)
  - Support `systemInstruction` for system prompts
  - Support `tools` configuration (for grounded search)
  - Parse response into structured `GeminiResponse` type
  - Extract text content, citations, and grounding metadata
- [ ] Implement grounded search tool configuration:
  - Add `google_search` tool to `tools` array: `{ "google_search": {} }`
  - Parse grounding metadata from response (search queries used, sources found)
  - Extract inline citations and source URLs from grounding chunks
  - Map grounding support to citation objects `[1][2][3]`
- [ ] Implement streaming response handling:
  - Use `streamGenerateContent` endpoint with `alt=sse` parameter
  - Parse Server-Sent Events (SSE) stream
  - Yield partial content chunks as they arrive
  - Handle stream errors and reconnection
  - Support progress display during streaming (via nakama-ui spinner)
- [ ] Implement multi-turn chat sessions:
  - Maintain conversation history as `Vec<Content>` (user/model alternating)
  - Append user message, send full history, append model response
  - Support system instruction that persists across turns
  - Manage token budget (truncate old messages when approaching limit)
- [ ] Implement safety settings:
  - Support configurable safety thresholds per category:
    - HARM_CATEGORY_HARASSMENT
    - HARM_CATEGORY_HATE_SPEECH
    - HARM_CATEGORY_SEXUALLY_EXPLICIT
    - HARM_CATEGORY_DANGEROUS_CONTENT
  - Default to BLOCK_MEDIUM_AND_ABOVE
  - Allow user override in config
- [ ] Implement model selection and configuration:
  - Default model: `gemini-2.5-pro`
  - Configurable: temperature, topP, topK, maxOutputTokens
  - Support model listing (`kami models list`)
- [ ] Implement error handling:
  - Parse API error responses (quota exceeded, invalid request, safety block)
  - Provide user-friendly error messages
  - Retry on transient errors (5xx, rate limit) with exponential backoff
- [ ] Write unit tests with mocked Gemini API responses
- [ ] Write integration tests for streaming and multi-turn sessions

## Acceptance Criteria

- Non-streaming `generateContent` works and returns structured responses
- Grounded search tool is correctly configured and citations are extracted
- Streaming responses display content progressively in the terminal
- Multi-turn chat maintains context across messages
- Safety settings are configurable and respected
- API errors are handled gracefully with clear messages
- Token budget management prevents context overflow in long sessions

## Dependencies

- Step 002 (Auth Layer) must be complete for authentication
- `reqwest` crate with streaming support
- `serde` / `serde_json` for request/response serialization
- `tokio` for async runtime
- `futures` crate for stream processing
