---
story_id: 6.10
story_key: 6-10-auto-recover-from-provider-connection-loss
epic: 6
epic_title: Rich TUI Experience
status: backlog
priority: medium
difficulty: medium
dependencies: ["1-4-add-provider-connectivity-validation"]
---
# Story 6.10: Auto-Recover from Provider Connection Loss
## Story Statement
As a user, I want auto-recovery from provider connection loss, so that my session isn't interrupted by temporary issues.
## Acceptance Criteria
**Given** connection lost during request, **When** detected, **Then** automatic reconnection attempted, request retried up to 3 times with exponential backoff (1s, 2s, 4s).
**Given** reconnection succeeds, **When** request retries, **Then** conversation continues seamlessly, notification toast shown.
**Given** reconnection fails after all attempts, **When** final fails, **Then** clear error displayed, session state preserved for manual retry.
## FRs: FR72
## Tasks
- [ ] Implement auto-reconnection logic
- [ ] Add exponential backoff (1s, 2s, 4s)
- [ ] Retry request up to 3 times
- [ ] Show reconnection notification
- [ ] Preserve session state
- [ ] Add manual retry option
- [ ] Add unit tests
