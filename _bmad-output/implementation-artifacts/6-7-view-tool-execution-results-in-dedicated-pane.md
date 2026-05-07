---
story_id: 6.7
story_key: 6-7-view-tool-execution-results-in-dedicated-pane
epic: 6
epic_title: Rich TUI Experience
status: backlog
priority: medium
difficulty: low
dependencies: ["3-2-implement-shell-command-execution-tool"]
---
# Story 6.7: View Tool Execution Results in Dedicated Pane
## Story Statement
As a user, I want to view tool execution results in a dedicated pane, so that I can see output separately from chat.
## Acceptance Criteria
**Given** tool executed, **When** output generated, **Then** appears in tool output pane with stdout in green, stderr in red.
**Given** multiple tools executed, **When** output generated, **Then** each separated by horizontal rule with tool name and time headers.
**Given** tool execution in progress, **When** streaming, **Then** output appears in real-time in tool pane.
## FRs: FR28
## Tasks
- [ ] Render tool output in dedicated pane
- [ ] Color stdout green, stderr red
- [ ] Separate multiple tool outputs
- [ ] Show tool name and execution time
- [ ] Add streaming output support
- [ ] Add unit tests
