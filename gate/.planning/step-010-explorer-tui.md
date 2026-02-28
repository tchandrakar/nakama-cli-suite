# Step 010: Explorer TUI

## Objective

Build a ratatui-based terminal user interface for interactive API exploration. The TUI provides an endpoint browser, a request builder, a response viewer with syntax highlighting, and a history sidebar for quick access to previous requests.

## Tasks

- [ ] Add ratatui and crossterm dependencies
- [ ] Design TUI layout:
  - [ ] Left sidebar: endpoint browser (collections, history, favorites)
  - [ ] Top-center: request builder (method, URL, headers, body tabs)
  - [ ] Bottom-center: response viewer (headers, body, timing tabs)
  - [ ] Right sidebar: variables/environment panel
  - [ ] Status bar: active environment, connection status
  - [ ] Command palette (Ctrl-P)
- [ ] Implement endpoint browser:
  - [ ] Tree view of collections and folders
  - [ ] History list (most recent first)
  - [ ] Favorites/bookmarked requests
  - [ ] Search within endpoints
  - [ ] Click/Enter to load request into builder
- [ ] Implement request builder:
  - [ ] Method selector (dropdown/cycle)
  - [ ] URL input with autocomplete (from history, collections)
  - [ ] Headers tab: key-value editor with add/remove
  - [ ] Body tab: text editor with JSON syntax highlighting
  - [ ] Params tab: query parameter key-value editor
  - [ ] Auth tab: auth type selector and credential input
  - [ ] Send button (Ctrl-Enter)
- [ ] Implement response viewer:
  - [ ] Headers tab: formatted header display
  - [ ] Body tab: syntax-highlighted response body
  - [ ] Timing tab: detailed timing breakdown (DNS, TCP, TLS, TTFB)
  - [ ] Schema tab: inferred schema display
  - [ ] Scrollable body with search (/ key)
  - [ ] Collapsible JSON nodes
- [ ] Implement variables panel:
  - [ ] Show current environment variables
  - [ ] Show extracted flow variables
  - [ ] Edit variables inline
  - [ ] Switch environment
- [ ] Implement interactive controls:
  - [ ] `Ctrl-Enter` — send request
  - [ ] `Tab` — switch between panels
  - [ ] `Ctrl-S` — save request to collection
  - [ ] `Ctrl-H` — toggle history sidebar
  - [ ] `Ctrl-E` — switch environment
  - [ ] `Ctrl-D` — diff with previous response
  - [ ] `Ctrl-P` — command palette
  - [ ] `q` or `Esc` — quit (with confirmation if unsaved)
  - [ ] Arrow keys / j,k — navigate
  - [ ] `/` — search within current panel
- [ ] Implement command palette:
  - [ ] Quick access to all commands
  - [ ] Fuzzy search on command names
  - [ ] Recent commands
- [ ] Implement responsive layout:
  - [ ] Adapt to terminal size
  - [ ] Collapse sidebars on narrow terminals
  - [ ] Handle resize events
- [ ] Implement WebSocket mode:
  - [ ] Connect to WebSocket endpoint
  - [ ] Message input panel
  - [ ] Message history panel (sent/received)
  - [ ] Connection status indicator
- [ ] Unit tests for panel rendering with mock data
- [ ] Manual test checklist for TUI interactions

## Acceptance Criteria

- TUI launches and displays all panels correctly
- Request builder supports all HTTP methods with headers and body
- Response viewer shows syntax-highlighted response
- Endpoint browser loads requests from collections and history
- Send request (Ctrl-Enter) executes and shows response
- Environment switching updates variables and auth
- Command palette provides quick access to all features
- WebSocket mode supports interactive messaging
- Layout adapts to different terminal sizes
- No flickering or rendering artifacts

## Dependencies

- Step 002 (Request builder) for request construction
- Step 003 (HTTP engine) for request execution
- Step 004 (Response analyzer) for response display
- Step 006 (History) for history sidebar
- Step 007 (Environment manager) for environment panel
- Step 008 (Collections) for endpoint browser
- ratatui and crossterm crates
