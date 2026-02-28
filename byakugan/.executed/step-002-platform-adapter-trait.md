# Step 002: Platform Adapter Trait

## Status: NOT STARTED

| Task | Status | Date | Notes |
|------|--------|------|-------|
| Define PullRequest, UnifiedDiff, DiffFile, Hunk, DiffLine models | - | - | - |
| Define Comment, ReviewComment, Review, ReviewVerdict models | - | - | - |
| Define async PlatformAdapter trait with all 6 methods | - | - | - |
| Implement PlatformDetector (parse git remote URLs) | - | - | - |
| Support SSH and HTTPS URL formats | - | - | - |
| Implement detect_platform() returning Box<dyn PlatformAdapter> | - | - | - |
| Implement detect_current_pr() for current branch | - | - | - |
| Unit tests for URL parsing and platform detection | - | - | - |
| Unit tests for model serialization/deserialization | - | - | - |

Status legend: `-` Not started | `WIP` In progress | `DONE` Complete | `BLOCKED` Blocked
