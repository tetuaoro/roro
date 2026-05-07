---
story_id: 6.3
story_key: 6-3-navigate-between-panes-using-keyboard
epic: 6
epic_title: Rich TUI Experience
status: backlog
priority: medium
difficulty: low
dependencies: ["6-1-implement-multi-pane-layout"]
---
# Story 6.3: Navigate Between Panes Using Keyboard
## Story Statement
As a user, I want to navigate between panes using keyboard, so that I can efficiently use the TUI.
## Acceptance Criteria
**Given** multiple panes visible, **When** I press Tab, **Then** focus cycles forward through panes, focused pane highlighted.
**Given** multiple panes visible, **When** I press Shift+Tab, **Then** focus cycles backward through panes.
**Given** I press Ctrl+ArrowKey, **When** pressed, **Then** focus moves to pane in that direction.
## FRs: FR23
## Tasks
- [ ] Implement Tab focus cycling
- [ ] Implement Shift+Tab reverse cycling
- [ ] Implement Ctrl+ArrowKey directional navigation
- [ ] Add focus highlighting
- [ ] Add unit tests
