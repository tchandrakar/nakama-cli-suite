# Step 011: Focus Mode

## Objective

Build a focus mode feature with Pomodoro-style timer tracking, optional Slack status updates to indicate unavailability, notification suppression integration, and session summary generation when the focus session ends.

## Tasks

- [ ] Implement `FocusSession` struct: start_time, duration, task_description, breaks, completed_pomodoros, notes
- [ ] Implement Pomodoro timer:
  - [ ] Configurable work duration (default 25 min)
  - [ ] Configurable break duration (default 5 min)
  - [ ] Long break after N pomodoros (default 4, 15 min)
  - [ ] Timer display in terminal (countdown, progress bar)
  - [ ] Audio/visual notification at timer end (bell character, terminal flash)
  - [ ] Pause/resume support
- [ ] Implement Slack status integration (optional):
  - [ ] Set Slack status to "Focusing" with timer emoji
  - [ ] Set DND (Do Not Disturb) mode
  - [ ] Include expected end time in status
  - [ ] Restore previous status when focus ends
  - [ ] Use Slack API via reqwest, token from nakama-vault
- [ ] Implement notification suppression:
  - [ ] macOS: `defaults write` to enable DND (if supported)
  - [ ] Linux: D-Bus notification suppression
  - [ ] Configurable: opt-in, disabled by default
  - [ ] Restore notification settings on exit (including crash recovery)
- [ ] Implement session tracking:
  - [ ] Record focus sessions in local database (SQLite/JSON)
  - [ ] Track total focus time per day/week
  - [ ] Track completed pomodoros
  - [ ] Associate sessions with tasks (optional --task flag)
- [ ] Implement session summary:
  - [ ] Generate summary when focus session ends
  - [ ] Duration, pomodoros completed, breaks taken
  - [ ] LLM option: suggest next action based on task progress
  - [ ] Export summary for standup/briefing integration
- [ ] Implement `focus` subcommand:
  - [ ] `tensai focus start` — begin focus session
  - [ ] `tensai focus start --duration 90m` — custom duration
  - [ ] `tensai focus start --task "Review PR #123"` — with task context
  - [ ] `tensai focus stop` — end session early
  - [ ] `tensai focus status` — show current session status
  - [ ] `tensai focus stats` — show focus statistics
  - [ ] `--no-slack` flag to skip Slack integration
  - [ ] `--no-dnd` flag to skip notification suppression
- [ ] Handle graceful shutdown:
  - [ ] Ctrl-C: prompt to end session or cancel
  - [ ] Cleanup Slack status on any exit path
  - [ ] Save partial session data
- [ ] Unit tests for timer logic
- [ ] Unit tests for session tracking

## Acceptance Criteria

- Pomodoro timer correctly counts down with configurable durations
- Slack status is set and cleared correctly (when enabled)
- Notification suppression activates and restores on exit
- Session history tracks all focus sessions accurately
- Session summary provides useful end-of-session context
- Graceful shutdown always restores Slack/notification state
- Focus statistics show daily/weekly trends
- Timer display is clear and non-intrusive

## Dependencies

- Step 001 (CLI scaffold)
- Step 008 (Security) for Slack token via nakama-vault
- Step 007 (Briefing engine) for summary integration
- Slack API for status updates
- System APIs for notification suppression (platform-specific)
