---
story_id: 6.5
story_key: 6-5-display-session-metadata-in-tui
epic: 6
epic_title: Rich TUI Experience
status: backlog
priority: low
difficulty: low
dependencies: ["6-1-implement-multi-pane-layout"]
---
# Story 6.5: Display Session Metadata in TUI
## Story Statement
As a user, I want to see session metadata in the TUI, so that I can track conversation details.
## Acceptance Criteria
**Given** session active, **When** TUI displayed, **Then** metadata shown in status bar: session ID (shortened), model, token count, tool count.
**Given** I hover over session ID, **When** hovered, **Then** tooltip shows full ID and creation timestamp.
## FRs: FR26
## Tasks
- [ ] Add status bar to TUI
- [ ] Display session metadata in status bar
- [ ] Add hover tooltip for session ID
- [ ] Update metadata on session changes
- [ ] Add unit tests
