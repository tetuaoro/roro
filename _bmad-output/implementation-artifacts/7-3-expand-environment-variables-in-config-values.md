---
story_id: 7.3
story_key: 7-3-expand-environment-variables-in-config-values
epic: 7
epic_title: Advanced Configuration & Admin
status: backlog
priority: high
difficulty: low
dependencies: ["7-2-support-environment-variable-overrides"]
---
# Story 7.3: Expand Environment Variables in Config Values
## Story Statement
As a user, I want to use ${VAR} syntax in config values, so that I can reference environment variables and home directory.
## Acceptance Criteria
**Given** config: base_url = "${OLLAMA_HOST}:11434", OLLAMA_HOST=localhost, **When** loaded, **Then** base_url resolves to "localhost:11434".
**Given** config: session_dir = "${HOME}/.vibe/sessions", **When** loaded, **Then** session_dir resolves to actual home path.
**Given** variable not set, **When** loaded, **Then** original ${VAR} preserved, warning logged.
## FRs: FR56
## Tasks
- [ ] Implement ${VAR} expansion
- [ ] Expand HOME, PWD, etc.
- [ ] Handle nested expansions
- [ ] Warn on unset variables
- [ ] Add unit tests
