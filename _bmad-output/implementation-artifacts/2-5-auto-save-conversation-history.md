---
story_id: 2.5
story_key: 2-5-auto-save-conversation-history
epic: 2
epic_title: Conversation Core (MVP)
status: backlog
priority: high
difficulty: medium
dependencies: ["2-2-implement-interactive-chat-interface"]
---
# Story 2.5: Auto-Save Conversation History
## Story Statement
As a user, I want my conversations to be saved automatically, so that I can resume later without losing context.
## Acceptance Criteria
**Given** I send a message, **When** sent, **Then** conversation is saved to JSON file in ~/.vibe/sessions/.
**Given** I send multiple messages, **When** each sent, **Then** session file is updated atomically.
**Given** a session file exists, **When** I resume, **Then** all previous messages are loaded and new appended.
## FRs: FR47
## Tasks
- [ ] Create session persistence module
- [ ] Implement auto-save on message send
- [ ] Create JSON serialization for sessions
- [ ] Implement atomic file writes
- [ ] Add session loading logic
- [ ] Add unit tests
