# Step 003: LLM Translation Engine

## Objective
Build the core translation pipeline that takes a natural language query, combines it with shell context, sends it to an LLM via `nakama-ai`, and parses the response into a structured command with explanation and risk assessment.

## Tasks
- [ ] Design the system prompt for NL-to-shell translation:
  - Instruct the LLM to output structured JSON: `{ "command": "...", "explanation": "...", "risk": "low|medium|high", "alternatives": [...] }`
  - Include rules: prefer portable commands, avoid destructive defaults, explain flags
  - Inject shell context into system prompt for platform-specific accuracy
- [ ] Create `TranslationRequest` struct: query text, shell context, optional constraints
- [ ] Create `TranslationResponse` struct: command, explanation, risk level, alternatives list
- [ ] Implement `translate(query: &str, context: &ShellContext) -> Result<TranslationResponse>` using `nakama-ai`
- [ ] Build prompt assembly: system prompt + context + user query
- [ ] Implement response parsing: extract JSON from LLM response, handle markdown code blocks
- [ ] Add fallback parsing: if JSON extraction fails, attempt to parse freeform text response
- [ ] Implement the `ask` subcommand handler: collect context -> translate -> display result
- [ ] Implement the `explain` subcommand handler: take a command string, ask LLM to explain it
- [ ] Display translated command with syntax highlighting (using `nakama-ui`)
- [ ] Display risk level with color coding (green/yellow/red)
- [ ] Show alternatives if available
- [ ] Add retry logic: retry once on parse failure with a clarifying prompt
- [ ] Add `--dry-run` flag to `ask` command (show command without executing)
- [ ] Add unit tests with mock LLM responses

## Acceptance Criteria
- `zangetsu ask "find large files over 100MB"` returns a valid command, explanation, and risk level
- `zangetsu explain "tar -czf archive.tar.gz dir/"` returns a human-readable explanation
- Response parsing handles both clean JSON and markdown-wrapped JSON
- Risk level is always one of: low, medium, high
- Display is styled with colors and formatting via `nakama-ui`
- Mock-based unit tests validate prompt assembly and response parsing

## Dependencies
- Step 001 (CLI scaffold)
- Step 002 (context collector for prompt enrichment)
