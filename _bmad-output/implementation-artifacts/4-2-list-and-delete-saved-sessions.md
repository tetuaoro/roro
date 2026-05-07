---
story_id: 4.2
story_key: 4-2-list-and-delete-saved-sessions
epic: 4
epic_title: Session Persistence
status: backlog
priority: medium
difficulty: low
dependencies: ["4-1-implement-session-save-and-resume"]
---
# Story 4.2: List and Delete Saved Sessions
## Story Statement
As a user, I want to list and delete saved sessions, so that I can manage my conversation history.
## Acceptance Criteria
**Given** I run `roro --list-sessions`, **When** executed, **Then** all sessions displayed in table with ID, timestamp, model, message count.
**Given** I run `roro --delete-session {uuid}`, **When** executed, **Then** specified session deleted with confirmation.
**Given** invalid UUID, **When** executed, **Then** error displayed with list of valid IDs.
## FRs: FR51, FR52
## Tasks
- [ ] Implement --list-sessions flag
- [ ] Implement session listing with table format
- [ ] Implement --delete-session flag
- [ ] Add session deletion logic
- [ ] Add validation for session IDs
- [ ] Add unit tests
