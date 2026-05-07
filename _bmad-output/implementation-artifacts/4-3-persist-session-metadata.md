---
story_id: 4.3
story_key: 4-3-persist-session-metadata
epic: 4
epic_title: Session Persistence
status: backlog
priority: low
difficulty: low
dependencies: ["4-1-implement-session-save-and-resume"]
---
# Story 4.3: Persist Session Metadata
## Story Statement
As a user, I want session metadata to be persisted, so that I can track details about my conversations.
## Acceptance Criteria
**Given** session saved, **When** I inspect file, **Then** it contains: session ID, creation timestamp, last update, model, tool count, token count.
**Given** I resume session, **When** loaded, **Then** metadata displayed in TUI status bar with creation and update times.
## FRs: FR53
## Tasks
- [ ] Add metadata fields to session struct
- [ ] Update session saving to include metadata
- [ ] Display metadata in TUI
- [ ] Add timestamp tracking
- [ ] Add unit tests
