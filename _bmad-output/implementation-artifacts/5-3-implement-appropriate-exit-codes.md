---
story_id: 5.3
story_key: 5-3-implement-appropriate-exit-codes
epic: 5
epic_title: Complete CLI Interface
status: backlog
priority: medium
difficulty: low
dependencies: ["5-1-implement-all-cli-flags-and-arguments"]
---
# Story 5.3: Implement Appropriate Exit Codes
## Story Statement
As a developer, I want appropriate exit codes for different scenarios, so that I can handle errors in scripts.
## Acceptance Criteria
**Given** successful conversation, **When** roro completes, **Then** exit code 0 returned.
**Given** general error, **When** roro fails, **Then** exit code 1 returned.
**Given** tool execution error, **When** roro fails, **Then** exit code 2 returned.
**Given** conversation limit reached, **When** roro stops, **Then** exit code 3 returned.
## FRs: FR67
## Tasks
- [ ] Define exit code constants
- [ ] Return exit code 0 on success
- [ ] Return exit code 1 on general errors
- [ ] Return exit code 2 on tool errors
- [ ] Return exit code 3 on conversation limits
- [ ] Add unit tests for exit codes
