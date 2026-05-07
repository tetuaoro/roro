---
story_id: 6.8
story_key: 6-8-display-clear-actionable-error-messages
epic: 6
epic_title: Rich TUI Experience
status: backlog
priority: medium
difficulty: low
dependencies: []
---
# Story 6.8: Display Clear, Actionable Error Messages
## Story Statement
As a user, I want clear, actionable error messages, so that I can troubleshoot issues easily.
## Acceptance Criteria
**Given** error occurs, **When** displayed, **Then** message clearly states what went wrong, suggests how to fix it, displayed in visible error box.
**Given** provider connection error, **When** displayed, **Then** message includes error type and provider URL, troubleshooting steps shown.
**Given** config error, **When** displayed, **Then** message shows invalid key, expected format, and example.
## FRs: FR70
## Tasks
- [ ] Create error display module
- [ ] Format error messages with context
- [ ] Add troubleshooting suggestions
- [ ] Display in visible error box
- [ ] Add error type detection
- [ ] Add unit tests
