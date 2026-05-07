---
story_id: 3.2
story_key: 3-2-implement-shell-command-execution-tool
epic: 3
epic_title: Tool Execution Framework
status: backlog
priority: high
difficulty: high
dependencies: ["3-1-create-tool-registry-with-permission-system"]
---
# Story 3.2: Implement Shell Command Execution Tool
## Story Statement
As a user, I want to execute shell commands directly from the conversation, so that I can use roro to perform system tasks.
## Acceptance Criteria
**Given** shell command request, **When** identified, **Then** command extracted, validated, and ASK prompt shown if needed.
**Given** approved shell command, **When** executes, **Then** stdout/stderr captured in real-time, displayed in tool pane, exit code returned.
**Given** shell command times out, **When** timeout reached, **Then** command killed, timeout error displayed, partial output shown.
## FRs: FR37, FR39, FR41, FR42, FR43, FR44
## Tasks
- [ ] Create shell tool implementation
- [ ] Add command extraction and validation
- [ ] Implement command execution with streaming
- [ ] Capture stdout/stderr separately
- [ ] Return exit codes
- [ ] Add timeout handling
- [ ] Add permission integration
- [ ] Add unit tests
