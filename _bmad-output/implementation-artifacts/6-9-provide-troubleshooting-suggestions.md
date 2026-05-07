---
story_id: 6.9
story_key: 6-9-provide-troubleshooting-suggestions
epic: 6
epic_title: Rich TUI Experience
status: backlog
priority: low
difficulty: low
dependencies: ["6-8-display-clear-actionable-error-messages"]
---
# Story 6.9: Provide Troubleshooting Suggestions
## Story Statement
As a user, I want troubleshooting suggestions for common errors, so that I can resolve issues without external help.
## Acceptance Criteria
**Given** common error (provider not running, timeout), **When** displayed, **Then** collapsible "Troubleshooting" section shown with step-by-step solutions.
**Given** provider timeout, **When** displayed, **Then** suggestions include network check, timeout increase, URL verification.
**Given** tool permission denied, **When** displayed, **Then** instructions shown to enable tool in config.toml.
## FRs: FR71
## Tasks
- [ ] Add troubleshooting section to errors
- [ ] Add suggestions for provider errors
- [ ] Add suggestions for timeout errors
- [ ] Add suggestions for permission errors
- [ ] Make section collapsible
- [ ] Add unit tests
