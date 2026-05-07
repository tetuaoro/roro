---
story_id: 3.6
story_key: 3-6-handle-keyboard-interrupts-gracefully
epic: 3
epic_title: Tool Execution Framework
status: backlog
priority: medium
difficulty: low
dependencies: ["3-2-implement-shell-command-execution-tool"]
---
# Story 3.6: Handle Keyboard Interrupts Gracefully
## Story Statement
As a user, I want to handle Ctrl+C gracefully during tool execution, so that I can cancel long-running operations.
## Acceptance Criteria
**Given** tool executing, **When** I press Ctrl+C, **Then** tool process terminated, "Cancelled" message displayed, I can enter new prompt.
**Given** multiple tools executing, **When** I press Ctrl+C, **Then** all pending executions cancelled, conversation state preserved.
## FRs: FR81
## Tasks
- [ ] Implement signal handling for Ctrl+C
- [ ] Terminate tool processes on interrupt
- [ ] Display cancellation message
- [ ] Preserve conversation state
- [ ] Add unit tests
