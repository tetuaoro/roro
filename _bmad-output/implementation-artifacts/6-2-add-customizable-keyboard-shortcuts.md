---
story_id: 6.2
story_key: 6-2-add-customizable-keyboard-shortcuts
epic: 6
epic_title: Rich TUI Experience
status: backlog
priority: medium
difficulty: medium
dependencies: ["6-1-implement-multi-pane-layout"]
---
# Story 6.2: Add Customizable Keyboard Shortcuts
## Story Statement
As a user, I want to customize keyboard shortcuts, so that I can optimize my workflow.
## Acceptance Criteria
**Given** shortcuts configured in config.toml, **When** key pressed, **Then** configured action triggered.
**Given** I press `?`, **When** pressed, **Then** help overlay shows all shortcuts and bindings.
**Given** shortcut conflicts, **When** config loaded, **Then** warning displayed, last configured takes precedence.
## FRs: FR22
## Tasks
- [ ] Add keybinding configuration
- [ ] Implement custom shortcut handling
- [ ] Create help overlay with `?`
- [ ] Detect and warn about conflicts
- [ ] Add unit tests
