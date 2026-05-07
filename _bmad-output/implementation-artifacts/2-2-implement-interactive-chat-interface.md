---
story_id: 2.2
story_key: 2-2-implement-interactive-chat-interface
epic: 2
epic_title: Conversation Core (MVP)
status: backlog
priority: high
difficulty: high
dependencies: ["2-1-implement-basic-cli-entry-points"]
---
# Story 2.2: Implement Interactive Chat Interface
## Story Statement
As a user, I want to interact with a chat interface in my terminal, so that I can have conversations with the LLM.
## Acceptance Criteria
**Given** roro is running in TUI mode, **When** I type a message and press Enter, **Then** the message is sent and response displayed.
**Given** the LLM is streaming, **When** tokens arrive, **Then** they appear in real-time.
**Given** I press Ctrl+C, **When** detected, **Then** response is cancelled gracefully.
**Given** I navigate with arrow keys, **When** I press Up, **Then** previous prompt is displayed.
## FRs: FR16
## Tasks
- [ ] Create TUI chat interface
- [ ] Implement message input and display
- [ ] Add streaming response handling
- [ ] Add Ctrl+C interruption handling
- [ ] Add command history with arrow keys
- [ ] Add unit tests
