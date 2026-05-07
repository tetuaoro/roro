---
story_id: 2.6
story_key: 2-6-support-stdin-piping-for-programmatic-mode
epic: 2
epic_title: Conversation Core (MVP)
status: backlog
priority: medium
difficulty: low
dependencies: ["2-1-implement-basic-cli-entry-points"]
---
# Story 2.6: Support stdin Piping for Programmatic Mode
## Story Statement
As a developer, I want to pipe input via stdin in programmatic mode, so that I can use roro in shell scripts and pipelines.
## Acceptance Criteria
**Given** I run `echo "Hello" | roro -p`, **When** executed, **Then** "Hello" is used as prompt and response printed to stdout.
**Given** I run `cat prompts.txt | roro -p --batch`, **When** executed, **Then** each line processed as separate prompt.
**Given** stdin is empty, **When** roro starts in programmatic mode, **Then** it waits for input.
## FRs: FR64
## Tasks
- [ ] Implement stdin reading in programmatic mode
- [ ] Handle piped input
- [ ] Add --batch flag support
- [ ] Process multiple prompts sequentially
- [ ] Add unit tests
