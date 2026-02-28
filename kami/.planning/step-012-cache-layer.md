# Step 012: Cache Layer

## Objective

Implement the SQLite-based cache layer at `~/.kami/cache.db` for search results and URL content, with configurable TTLs, cache-aware queries that return instantly for cached data, and cache management commands.

## Tasks

- [ ] Add `rusqlite` dependency for SQLite
- [ ] Design the cache database schema:
  - `search_cache` table: query_hash, query_text, response_json, created_at, ttl_seconds, source (gemini|custom_search)
  - `url_cache` table: url_hash, url, content_markdown, metadata_json, fetched_at, ttl_seconds, content_type
  - `session_cache` table: session_id, session_json, created_at, updated_at
  - Indexes on hash columns and created_at for efficient lookups and cleanup
- [ ] Implement cache initialization:
  - Create `~/.kami/cache.db` on first use
  - Run schema migrations (create tables if not exist)
  - Handle database corruption (detect and recreate)
- [ ] Implement search result caching:
  - Before executing a search, check cache for matching query
  - Cache key: SHA-256 hash of normalized query (lowercase, trimmed)
  - If cache hit within TTL, return cached result (no API call)
  - If cache miss or expired, execute search and store result
  - Configurable TTL (default: 60 minutes from config `cache.search_ttl_minutes`)
- [ ] Implement URL content caching:
  - Before fetching a URL, check cache for matching URL
  - Cache key: SHA-256 hash of normalized URL
  - If cache hit within TTL, return cached content
  - If cache miss or expired, fetch URL and store content
  - Configurable TTL (default: 24 hours from config `cache.url_ttl_hours`)
- [ ] Implement cache-aware query pipeline:
  - Integrate caching transparently into the search and fetch pipelines
  - Display cache hit/miss status in verbose mode
  - Support `--no-cache` flag to bypass cache for any command
  - Support `--refresh` flag to force cache refresh (fetch new data and update cache)
- [ ] Implement cache size management:
  - Configurable max cache size (default: 100 MB)
  - LRU eviction when cache exceeds max size
  - Periodic cleanup of expired entries
- [ ] Implement cache management commands:
  - `kami cache stats` -- show cache size, entry count, hit rate
  - `kami cache clear` -- clear all cached data
  - `kami cache clear --search` -- clear only search cache
  - `kami cache clear --urls` -- clear only URL cache
  - `kami cache clear --older-than=7d` -- clear entries older than duration
- [ ] Write unit tests for cache operations (insert, lookup, expiry, eviction)
- [ ] Write integration tests for cache-aware search pipeline

## Acceptance Criteria

- Repeated queries within TTL return instantly from cache (no API calls)
- Cache miss correctly falls through to API calls
- `--no-cache` flag bypasses cache entirely
- `--refresh` flag forces fresh data while updating cache
- Cache size stays within configured limits via LRU eviction
- Cache management commands work correctly
- Database corruption is detected and handled gracefully
- Cache provides measurable latency improvement for repeated queries

## Dependencies

- Step 004 (Google Search) must be complete for search integration
- Step 010 (URL Summarization) must be complete for URL content caching
- `rusqlite` crate for SQLite
- `sha2` crate for hash computation
- `chrono` for timestamp management
