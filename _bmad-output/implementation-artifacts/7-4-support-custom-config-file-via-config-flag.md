---
story_id: 7.4
story_key: 7-4-support-custom-config-file-via-config-flag
epic: 7
epic_title: Advanced Configuration & Admin
status: backlog
priority: medium
difficulty: low
dependencies: ["1-2-create-configuration-module-with-layered-loading"]
---
# Story 7.4: Support Custom Config File via --config Flag
## Story Statement
As a user, I want to specify a custom config file, so that I can use different configurations for different projects.
## Acceptance Criteria
**Given** `roro --config custom.toml`, **When** executed, **Then** custom.toml loaded, all settings applied.
**Given** `roro -c /path/to/config.toml`, **When** executed, **Then** specified config loaded.
**Given** config file doesn't exist, **When** executed, **Then** error displayed, exit code 1.
## FRs: FR57
## Tasks
- [ ] Add --config flag
- [ ] Load specified config file
- [ ] Handle missing files
- [ ] Validate custom config
- [ ] Add unit tests
