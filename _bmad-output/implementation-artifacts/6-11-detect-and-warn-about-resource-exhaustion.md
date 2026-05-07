---
story_id: 6.11
story_key: 6-11-detect-and-warn-about-resource-exhaustion
epic: 6
epic_title: Rich TUI Experience
status: backlog
priority: low
difficulty: medium
dependencies: []
---
# Story 6.11: Detect and Warn About Resource Exhaustion
## Story Statement
As a user, I want to be warned about resource exhaustion, so that I can prevent crashes and data loss.
## Acceptance Criteria
**Given** system memory > 90%, **When** new request made, **Then** warning displayed, request not sent.
**Given** context window would be exceeded, **When** message about to be sent, **Then** warning displayed with suggestions to clear history or start fresh.
**Given** disk space < 100MB, **When** session save attempted, **Then** warning displayed with suggestion to delete old sessions.
## FRs: FR75
## Tasks
- [ ] Implement resource monitoring
- [ ] Check memory usage
- [ ] Check context window usage
- [ ] Check disk space
- [ ] Display appropriate warnings
- [ ] Add suggestions for each warning
- [ ] Add unit tests
