# Step 010: URL Summarization

## Objective

Implement the `kami summarize <url>` command that fetches web page content with a configurable user-agent, extracts the main content using a readability algorithm, and produces an LLM-generated summary with key points.

## Tasks

- [ ] Implement URL fetching:
  - Use reqwest with configurable user-agent (default: kami/version)
  - Support HTTP/HTTPS with TLS
  - Follow redirects (configurable max redirects)
  - Handle common errors: timeouts, 404, 403, connection refused
  - Respect rate limiting headers from target sites
  - Support configurable request timeout (default: 30s)
- [ ] Implement content extraction (readability):
  - Use `scraper` crate to parse HTML DOM
  - Implement readability-style content extraction:
    - Remove navigation, sidebars, ads, footers
    - Identify main content area (article, main, body heuristics)
    - Preserve headings, paragraphs, lists, code blocks
    - Convert to clean markdown
  - Handle special content types:
    - Plain text (pass through)
    - PDF (extract text using pdf-extract or similar)
    - JSON (pretty-print and summarize structure)
  - Extract metadata: title, author, publication date, description
- [ ] Implement LLM summarization:
  - Send extracted content to Gemini/nakama-ai for summarization
  - Generate:
    - One-paragraph executive summary
    - Key points (bulleted list, 5-10 items)
    - Key quotes (if applicable)
    - Metadata (title, author, date, word count, reading time)
  - Support configurable summary length (brief, standard, detailed)
  - Handle very long content (chunk and summarize, then synthesize)
- [ ] Implement output formatting:
  - Terminal: rich summary with sections, key points highlighted
  - Markdown: clean markdown suitable for saving/sharing
  - JSON: structured summary data
- [ ] Implement batch summarization:
  - Accept multiple URLs (space-separated or from stdin)
  - Process in parallel with configurable concurrency
  - Generate individual summaries and an optional cross-document summary
- [ ] Implement `--extract-only` flag:
  - Output the extracted content without summarization
  - Useful for piping content to other tools
- [ ] Write unit tests for content extraction (various HTML structures)
- [ ] Write integration tests with mock HTTP responses

## Acceptance Criteria

- URLs are fetched reliably with proper error handling
- Content extraction produces clean markdown from messy HTML
- LLM summaries are concise and capture the key information
- Key points are relevant and well-structured
- Batch summarization handles multiple URLs efficiently
- `--extract-only` outputs clean content without LLM processing
- PDF content is extracted and summarized correctly
- All output formats are well-structured

## Dependencies

- Step 003 (Gemini Provider) must be complete for LLM summarization
- `reqwest` crate for HTTP fetching
- `scraper` crate for HTML parsing and content extraction
- `nakama-ai` for summarization
