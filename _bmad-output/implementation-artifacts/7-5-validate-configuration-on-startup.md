---
story_id: 7.5
story_key: 7-5-validate-configuration-on-startup
epic: 7
epic_title: Advanced Configuration & Admin
status: backlog
priority: high
difficulty: low
dependencies: ["1-2-create-configuration-module-with-layered-loading"]
---
# Story 7.5: Validate Configuration on Startup
## Story Statement
As a user, I want configuration to be validated on startup, so that I catch errors early.
## Acceptance Criteria
**Given** config with invalid value, **When** roro starts, **Then** clear error message displayed, invalid field/value shown, exit code 1.
**Given** required field missing, **When** roro starts, **Then** error displayed with missing field and expected format.
**Given** valid config, **When** roro starts, **Then** no validation errors, roro starts normally.
## FRs: FR58, FR59
## Tasks
- [ ] Implement config validation on startup
- [ ] Add clear error messages
- [ ] Show invalid field and value
- [ ] Show expected format
- [ ] Return exit code 1 on error
- [ ] Add unit tests
