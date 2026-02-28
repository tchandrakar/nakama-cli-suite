# Step 005: Calendar Aggregator

## Objective

Implement a Google Calendar aggregator using the Google Calendar API (via reqwest) that fetches today's meetings, identifies available focus time blocks, and handles OAuth authentication via nakama-vault. This provides schedule context for intelligent briefings and planning.

## Tasks

- [ ] Implement `CalendarAggregator` struct implementing `Aggregator` trait
- [ ] Implement Google OAuth2 flow:
  - [ ] OAuth credentials stored in nakama-vault
  - [ ] Token refresh handling
  - [ ] Interactive auth flow for initial setup (`tensai auth calendar`)
  - [ ] Secure token storage via nakama-vault
- [ ] Fetch today's calendar events:
  - [ ] Event title, start/end time, duration
  - [ ] Meeting type detection (1:1, team, external, standup)
  - [ ] Attendee list and count
  - [ ] Location (physical/virtual) and meeting link
  - [ ] All-day events vs timed events
  - [ ] Recurring vs one-time events
- [ ] Fetch upcoming events (configurable lookahead, default 24h):
  - [ ] Next meeting with countdown
  - [ ] Remaining meetings today
  - [ ] Tomorrow's first meeting (for EOD planning)
- [ ] Calculate available focus blocks:
  - [ ] Identify gaps between meetings
  - [ ] Minimum focus block duration (configurable, default 30min)
  - [ ] Account for meeting buffer time (configurable, default 5min)
  - [ ] Rank focus blocks by duration
  - [ ] Label as "deep work" (>2h), "task work" (30min-2h), "quick task" (<30min)
- [ ] Implement meeting prep context:
  - [ ] Extract agenda from event description
  - [ ] Identify related PRs/issues mentioned in event
  - [ ] Flag meetings needing preparation
- [ ] Support multiple calendars:
  - [ ] Primary calendar
  - [ ] Shared/team calendars (configurable)
  - [ ] Calendar selection in config
- [ ] Add configuration:
  - [ ] Working hours (default 9am-5pm)
  - [ ] Buffer time between meetings
  - [ ] Minimum focus block duration
  - [ ] Calendars to include
  - [ ] Event categories to exclude
- [ ] Handle timezone correctly (user's local timezone)
- [ ] Unit tests with mocked API responses
- [ ] Integration test with Google Calendar API (optional, requires OAuth)

## Acceptance Criteria

- Today's meetings are listed with all relevant details
- Focus blocks are correctly calculated between meetings
- OAuth flow works for initial setup and token refresh
- Multiple calendars can be aggregated
- Timezone handling is correct for user's locale
- Meeting type classification works for common patterns
- Graceful handling when calendar API is unreachable

## Dependencies

- Step 001 (CLI scaffold)
- Step 002 (Aggregator trait) must be complete
- Step 008 (Security) for OAuth token storage in nakama-vault
- reqwest for HTTP requests to Google Calendar API
- chrono for time calculations
