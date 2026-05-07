---
story_id: 6.4
story_key: 6-4-make-error-line-numbers-clickable
epic: 6
epic_title: Rich TUI Experience
status: backlog
priority: low
difficulty: medium
dependencies: ["6-1-implement-multi-pane-layout"]
---
# Story 6.4: Make Error Line Numbers Clickable
## Story Statement
As a user, I want to click on error line numbers to open them in my editor, so that I can quickly fix issues.
## Acceptance Criteria
**Given** response contains error with line numbers, **When** rendered, **Then** line numbers detected and displayed as clickable hyperlinks.
**Given** I click line number, **When** clicked, **Then** $EDITOR opens, cursor at specified line, file path extracted.
**Given** file doesn't exist, **When** clicked, **Then** error toast displayed: "File not found: {path}".
## FRs: FR24
## Tasks
- [ ] Detect line numbers in error output
- [ ] Render as clickable hyperlinks
- [ ] Implement click handler
- [ ] Open editor with correct file and line
- [ ] Handle missing files
- [ ] Add unit tests
