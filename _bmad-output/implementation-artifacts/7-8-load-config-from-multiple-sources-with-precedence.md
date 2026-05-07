---
story_id: 7.8
story_key: 7-8-load-config-from-multiple-sources-with-precedence
epic: 7
epic_title: Advanced Configuration & Admin
status: backlog
priority: medium
difficulty: medium
dependencies: ["1-2-create-configuration-module-with-layered-loading", "7-2-support-environment-variable-overrides", "7-4-support-custom-config-file-via-config-flag"]
---
# Story 7.8: Load Config from Multiple Sources with Precedence
## Story Statement
As a user, I want config loaded from multiple sources with proper precedence, so that I can override settings at different levels.
## Acceptance Criteria
**Given** system config, user config, project config, env vars, CLI flags, **When** roro starts, **Then** settings loaded: defaults < system < user < project < env < CLI, later overrides earlier.
**Given** setting in both user config and CLI flag, **When** roro starts, **Then** CLI flag value takes precedence.
## FRs: FR63
## Tasks
- [ ] Implement layered loading order
- [ ] Merge configs with proper precedence
- [ ] Test precedence with multiple sources
- [ ] Add unit tests
