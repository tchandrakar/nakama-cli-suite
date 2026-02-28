# Step 004: Google Search

## Objective

Implement the Google Custom Search JSON API client for direct search queries, integrate Gemini's built-in grounded search tool as the primary search mechanism, and build result parsing and normalization to produce a unified search result format regardless of the search method used.

## Tasks

- [ ] Implement Google Custom Search JSON API client:
  - Create `GoogleSearchClient` struct with reqwest client, API key, and search engine ID (cx)
  - Implement `search()` method: `GET https://www.googleapis.com/customsearch/v1?key=KEY&cx=CX&q=QUERY`
  - Parse search results into `SearchResult` struct (title, snippet, url, displayLink)
  - Support pagination (`start` parameter for offset)
  - Support search options: num results, language, region, date range, site restriction
  - Handle API errors (quota exceeded, invalid key)
- [ ] Implement Gemini grounded search integration:
  - Use Gemini's built-in `google_search` tool (from step 003)
  - Extract search grounding metadata from Gemini responses
  - Parse grounding chunks into `GroundedResult` struct (segment text, sources with URLs)
  - Map Gemini's citation indices to source URLs
  - This is the primary search path (richer answers than raw Custom Search)
- [ ] Define unified search result types:
  - `SearchResult` struct: title, url, snippet, source (gemini_grounded | custom_search)
  - `GroundedAnswer` struct: answer text, inline citations, sources list, search queries used
  - `SearchResponse` struct: answer (if grounded), results Vec<SearchResult>, metadata
- [ ] Implement result normalization:
  - Normalize both Gemini grounded results and Custom Search results to unified types
  - Deduplicate sources across both methods if used together
  - Rank results by relevance
- [ ] Implement search configuration:
  - Default to Gemini grounded search (requires Gemini API access)
  - Fall back to Custom Search JSON API if Gemini grounded search fails
  - Support `--engine=gemini|custom` flag to force a specific search method
  - Configurable result count, region, language in `~/.kami/config.toml`
- [ ] Implement related query suggestions:
  - Extract "People also ask" or related queries from search results
  - Use Gemini to generate related query suggestions based on the original query
- [ ] Write unit tests with mocked Google Search API responses
- [ ] Write integration tests for grounded search result parsing

## Acceptance Criteria

- Gemini grounded search returns answers with inline citations and source URLs
- Custom Search JSON API returns structured search results
- Both search methods produce results in the unified format
- Fallback from grounded search to custom search works transparently
- Related query suggestions are generated
- API quota errors are handled gracefully with clear messages
- Search configuration (region, language, result count) is respected

## Dependencies

- Step 002 (Auth Layer) must be complete for Google API authentication
- Step 003 (Gemini Provider) must be complete for grounded search
- `reqwest` crate for HTTP requests
- Google Custom Search API key and search engine ID (cx)
