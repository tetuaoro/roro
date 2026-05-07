---
story_id: 2.1
story_key: 2-1-implement-basic-cli-entry-points
epic: 2
epic_title: Conversation Core (MVP)
status: backlog
priority: high
difficulty: low
dependencies: ["1-1-initialize-rust-project-structure"]
---
# Story 2.1: Implement Basic CLI Entry Points
## Story Statement
As a user, I want to launch roro with no arguments or with an initial prompt, so that I can start interactive conversations or pre-fill input.
## Acceptance Criteria
**Given** I run `roro` with no arguments, **When** the application starts, **Then** interactive TUI mode is launched and I can type messages.
**Given** I run `roro "Explain Rust ownership"`, **When** the application starts, **Then** the TUI launches with prompt pre-filled.
**Given** I run `roro` in a terminal without TTY, **When** the application starts, **Then** it detects non-interactive mode and waits for stdin.
## FRs: FR1, FR2
## Tasks
- [ ] Implement CLI entry point in main.rs
- [ ] Detect TTY for interactive vs programmatic mode
- [ ] Handle positional argument as initial prompt
- [ ] Add basic error handling
- [ ] Add unit tests
