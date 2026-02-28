# Step 008: Conversational Mode

## Objective

Implement the `kami ask` conversational mode with a REPL interface using rustyline, multi-turn session memory that maintains context across questions, and session save/resume functionality with storage in `~/.kami/sessions/`.

## Tasks

- [ ] Add `rustyline` dependency for REPL functionality
- [ ] Implement the REPL loop:
  - Display `kami>` prompt using rustyline
  - Accept user input with line editing, history navigation (up/down arrows)
  - Handle special commands:
    - `/save [name]` -- save current session
    - `/load <name>` -- load a previous session
    - `/history` -- show conversation history
    - `/clear` -- clear conversation context
    - `/search <query>` -- perform a one-off search without adding to conversation
    - `/deep <query>` -- perform deep research from within the conversation
    - `/help` -- show available commands
    - `/quit` or `/exit` -- exit the REPL
  - Handle Ctrl+C (cancel current input) and Ctrl+D (exit)
- [ ] Implement session memory (multi-turn):
  - Maintain conversation history as a list of user/assistant message pairs
  - Send full conversation history to Gemini for each new message
  - Manage token budget: when history exceeds limit, summarize older messages
  - Support system instruction that persists across turns
  - Track search queries and results within the session for context
- [ ] Implement session persistence:
  - Save sessions to `~/.kami/sessions/<name>.json`
  - Session format: metadata (name, created_at, updated_at), messages list, search history
  - Auto-save on exit (configurable)
  - `/save` creates a named checkpoint
  - `/load` restores a previous session with full context
  - List available sessions with timestamps
- [ ] Implement `kami ask <question>` (single-shot mode):
  - If a question is provided as an argument, answer it and exit (no REPL)
  - Support `--session=<name>` to continue a previous session non-interactively
  - Useful for scripting: `kami ask "what is X?" --session=research`
- [ ] Implement conversation-aware search:
  - When the user asks a follow-up, include prior context in the search
  - Example: "what are the best rust async runtimes?" -> "how does tokio compare?" (include "rust async runtimes" context)
  - Use Gemini's multi-turn chat to maintain context naturally
- [ ] Implement session configuration:
  - `max_history_per_session` (default: 50 message pairs)
  - `auto_save` (default: true)
  - `session_store` path (default: `~/.kami/sessions/`)
  - Configurable in `~/.kami/config.toml`
- [ ] Write unit tests for session management and persistence
- [ ] Write integration tests for the REPL loop (simulated input)

## Acceptance Criteria

- `kami ask` launches an interactive REPL with line editing and history
- Multi-turn conversations maintain context (follow-up questions work)
- Sessions can be saved, listed, and loaded
- Single-shot mode (`kami ask "question"`) works without entering REPL
- Token budget management prevents context overflow in long sessions
- Special commands (`/save`, `/load`, `/history`, etc.) work correctly
- Sessions persist across kami restarts

## Dependencies

- Step 003 (Gemini Provider) must be complete for multi-turn chat
- Step 005 (Synthesis Engine) must be complete for answer formatting
- `rustyline` crate for REPL interface
- `serde_json` for session serialization
- `chrono` for timestamps
