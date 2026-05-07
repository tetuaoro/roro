---
story_id: 6.1
story_key: 6-1-implement-multi-pane-layout
epic: 6
epic_title: Rich TUI Experience
status: backlog
priority: high
difficulty: high
dependencies: ["2-2-implement-interactive-chat-interface"]
---
# Story 6.1: Implement Multi-Pane Layout
## Story Statement
As a user, I want a multi-pane layout for the TUI, so that I can see chat, tool output, and code preview simultaneously.
## Acceptance Criteria
**Given** TUI running, **When** rendered, **Then** 3 panes visible: chat history (top), input (bottom), tool output (right).
**Given** terminal resized, **When** event occurs, **Then** all panes resize proportionally, layout remains usable.
**Given** code block detected, **When** message renders, **Then** code preview pane opens automatically, can be dismissed with Esc.
## FRs: FR21
## Tasks
- [ ] Create multi-pane layout structure
- [ ] Implement chat history pane
- [ ] Implement input pane
- [ ] Implement tool output pane
- [ ] Add code preview pane
- [ ] Handle terminal resize events
- [ ] Add pane dismissal
- [ ] Add unit tests
