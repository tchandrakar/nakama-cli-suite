# Step 004: Response Analyzer

## Objective

Build a response analysis engine that detects content types, applies syntax highlighting for JSON/XML/HTML, pretty-prints structured data, infers response schemas, and optionally provides LLM-powered analysis of API responses.

## Tasks

- [ ] Define `AnalyzedResponse` struct: status, headers, body, content_type, timing, size, schema, analysis
- [ ] Implement content type detection:
  - [ ] From Content-Type header
  - [ ] Fallback: content sniffing (JSON, XML, HTML, binary)
  - [ ] Handle charset encoding (UTF-8, ISO-8859-1, etc.)
- [ ] Implement syntax highlighting:
  - [ ] JSON: keys, strings, numbers, booleans, null (color-coded)
  - [ ] XML: tags, attributes, text content, CDATA
  - [ ] HTML: tags, attributes, text, scripts, styles
  - [ ] YAML: keys, values, lists
  - [ ] Use syntect or custom ANSI coloring
- [ ] Implement pretty-printing:
  - [ ] JSON: indented with configurable indent (default 2 spaces)
  - [ ] XML: indented tag structure
  - [ ] HTML: formatted structure
  - [ ] Headers: aligned key-value display
  - [ ] `--compact` flag to disable pretty-printing
- [ ] Implement schema inference:
  - [ ] Infer JSON Schema from response body
  - [ ] Detect field types (string, number, boolean, array, object, null)
  - [ ] Detect nullable fields
  - [ ] Detect array element types
  - [ ] Detect enum values (from small cardinality sets)
  - [ ] Generate TypeScript interface from inferred schema
  - [ ] `--schema` flag to show inferred schema
- [ ] Implement LLM analysis (optional):
  - [ ] Send response to nakama-ai for analysis
  - [ ] Prompt: "Analyze this API response and explain the data structure"
  - [ ] Identify potential issues (unexpected null, missing fields, error patterns)
  - [ ] Suggest improvements to API design
  - [ ] `--analyze` flag to trigger LLM analysis
- [ ] Implement response filtering:
  - [ ] JSONPath queries: `--jq '.data[0].name'`
  - [ ] XPath queries for XML: `--xpath '//item/title'`
  - [ ] Header extraction: `--header-only`, `--header X-Request-Id`
  - [ ] Status-only: `--status-only`
- [ ] Implement response output modes:
  - [ ] Full (headers + body)
  - [ ] Body only (default)
  - [ ] Headers only
  - [ ] Status only
  - [ ] JSON output (machine-readable)
  - [ ] Save to file
- [ ] Implement response size and timing display:
  - [ ] Status code with color (2xx=green, 3xx=yellow, 4xx/5xx=red)
  - [ ] Response size (human-readable)
  - [ ] Response time
  - [ ] Header count
- [ ] Unit tests for each content type handler
- [ ] Unit tests for schema inference
- [ ] Unit tests for JSONPath/XPath filtering

## Acceptance Criteria

- Content type is correctly detected from headers and content sniffing
- JSON, XML, and HTML are syntax-highlighted in terminal
- Pretty-printing produces readable, indented output
- Schema inference correctly identifies types, nullability, and arrays
- LLM analysis provides useful insights about response structure
- JSONPath queries filter JSON responses correctly
- Status codes are color-coded for quick visual identification
- Response timing is accurate and clearly displayed

## Dependencies

- Step 001 (CLI scaffold)
- Step 003 (HTTP engine) for response data
- nakama-ai shared crate for LLM analysis (optional)
- syntect or similar for syntax highlighting
- serde_json for JSON handling, quick-xml for XML
