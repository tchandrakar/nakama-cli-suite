# Step 004: Confluence Client

## Objective

Build the Confluence REST API v2 client using reqwest, supporting page search via CQL, content fetching, storage format to markdown conversion using scraper, space navigation, and label management.

## Tasks

- [ ] Create `ConfluenceClient` struct with fields: reqwest client, base_url, auth (from step 002)
- [ ] Implement the base request builder:
  - Construct request URLs: `{base_url}/wiki/api/v2/{endpoint}` (v2 API)
  - Fallback to v1: `{base_url}/wiki/rest/api/{endpoint}` where v2 doesn't support the operation
  - Attach authentication headers
  - Handle pagination (cursor-based for v2, startAt/limit for v1)
- [ ] Implement page operations:
  - `search_pages(cql: &str) -> Result<Vec<Page>>` -- execute CQL query
  - `get_page(id: &str) -> Result<Page>` -- fetch single page by ID
  - `get_page_by_title(space_key, title) -> Result<Option<Page>>` -- find page by title
  - `get_page_content(id, format) -> Result<String>` -- fetch page body (storage, view, atlas_doc_format)
  - `get_page_children(id) -> Result<Vec<Page>>` -- list child pages
  - `get_page_ancestors(id) -> Result<Vec<Page>>` -- list parent pages (breadcrumb)
- [ ] Define Confluence data models:
  - `Page` struct: id, title, space_key, status, body (storage format), version, created_at, updated_at, author, labels, url
  - `Space` struct: key, name, description, type (global/personal), homepage_id
  - `Label` struct: id, name, prefix
  - `SearchResult` struct: page, excerpt, last_modified, matching_text
- [ ] Implement storage format to markdown conversion:
  - Use `scraper` crate to parse Confluence storage format (XHTML-based)
  - Convert common elements:
    - Headings (h1-h6)
    - Paragraphs, lists (ordered/unordered)
    - Tables
    - Code blocks (ac:structured-macro with language)
    - Links (ac:link, ri:page references)
    - Images (ac:image)
    - Panels, info/warning/note macros
    - Expand macros
  - Handle nested macros and complex layouts
  - Preserve meaningful formatting while removing Confluence-specific markup
- [ ] Implement space operations:
  - `get_spaces() -> Result<Vec<Space>>` -- list all spaces
  - `get_space(key) -> Result<Space>` -- fetch space details
  - `get_space_pages(key) -> Result<Vec<Page>>` -- list all pages in a space
  - `get_space_hierarchy(key) -> Result<PageTree>` -- page tree structure
- [ ] Implement label operations:
  - `get_page_labels(page_id) -> Result<Vec<Label>>` -- list labels on a page
  - `search_by_label(space_key, label) -> Result<Vec<Page>>` -- find pages by label
- [ ] Implement CQL helper utilities:
  - Validate CQL syntax
  - Support common CQL patterns: space, type, label, text, lastModified
- [ ] Write unit tests for storage format to markdown conversion
- [ ] Write integration tests with mocked Confluence API responses

## Acceptance Criteria

- CQL queries execute and return paginated results
- Page content is correctly fetched in storage format
- Storage format to markdown conversion handles all common elements
- Space navigation (list, hierarchy, page tree) works correctly
- Label-based search works
- Pagination is handled for all list endpoints
- Both v1 and v2 API endpoints are supported where needed

## Dependencies

- Step 002 (Auth Layer) must be complete for authentication
- `reqwest` crate with JSON support
- `scraper` crate for HTML/XHTML parsing
- `serde` / `serde_json` for response deserialization
