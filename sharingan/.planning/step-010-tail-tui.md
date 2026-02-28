# Step 010: Tail TUI (Live Dashboard)

## Objective

Build a ratatui-based terminal user interface for the `tail` command that provides a live log streaming dashboard with real-time annotations, anomaly highlighting, a statistics sidebar, and interactive controls for filtering, pausing, and drilling into specific entries.

## Tasks

- [ ] Add ratatui and crossterm dependencies
- [ ] Design TUI layout:
  - [ ] Main log stream panel (scrollable, auto-scroll with pause)
  - [ ] Stats sidebar (error rate, anomaly count, source breakdown)
  - [ ] Analysis panel (bottom, shows deep analysis results)
  - [ ] Status bar (source info, entry count, filter active, connection status)
  - [ ] Command bar (bottom, for interactive filter input)
- [ ] Implement log stream display:
  - [ ] Color-coded log levels (ERROR=red, WARN=yellow, INFO=green, DEBUG=gray)
  - [ ] Timestamp formatting (configurable: relative, absolute, ISO)
  - [ ] Source labels with distinct colors per source
  - [ ] Line wrapping and horizontal scroll
  - [ ] Anomaly highlighting (background color flash on detected anomalies)
  - [ ] Inline annotations from fast analysis (icons/markers)
- [ ] Implement stats sidebar:
  - [ ] Real-time entry count per level (sparkline graphs)
  - [ ] Error rate (entries/second, rolling average)
  - [ ] Active anomaly count
  - [ ] Source status (connected/disconnected per source)
  - [ ] Memory usage indicator
- [ ] Implement analysis panel:
  - [ ] Show latest deep analysis results
  - [ ] Expandable/collapsible
  - [ ] Scrollable for long explanations
  - [ ] Loading spinner during LLM calls
- [ ] Implement interactive controls:
  - [ ] `Space` — pause/resume auto-scroll
  - [ ] `f` — open filter input (NL or structured)
  - [ ] `Enter` on entry — drill into detail view (full entry + analysis)
  - [ ] `e` — trigger deep analysis on selected entry
  - [ ] `s` — toggle stats sidebar
  - [ ] `a` — toggle analysis panel
  - [ ] `q` — quit
  - [ ] `j/k` or arrow keys — scroll
  - [ ] `/` — search within visible logs
  - [ ] `Tab` — switch between sources (if multiple)
- [ ] Implement detail view:
  - [ ] Full log entry with all fields
  - [ ] Fast analysis results
  - [ ] Deep analysis results (fetch on demand)
  - [ ] Related entries (same trace ID, same time window)
  - [ ] Back navigation
- [ ] Implement responsive layout (adapt to terminal size)
- [ ] Handle terminal resize events
- [ ] Implement graceful degradation (no TUI if not a TTY, fall back to plain output)
- [ ] Unit tests for layout rendering with mock data
- [ ] Manual test checklist for TUI interactions

## Acceptance Criteria

- TUI launches and displays live log stream in real-time
- Log entries are color-coded by level with clear source attribution
- Anomalies are visually highlighted as they appear
- Stats sidebar shows accurate real-time metrics with sparkline graphs
- All keyboard shortcuts work as documented
- Filter input accepts natural language queries
- Detail view shows comprehensive entry information
- TUI handles terminal resize without crashing
- Falls back to plain text output when stdout is not a TTY
- No flickering or rendering artifacts during normal operation

## Dependencies

- Step 002 (Log ingestors) for live log streaming
- Step 003 (Log parser) for structured entry display
- Step 004 (Fast analysis) for real-time annotations
- Step 005 (Deep analysis) for on-demand LLM analysis
- Step 009 (NL filtering) for interactive filter input
- ratatui and crossterm crates
