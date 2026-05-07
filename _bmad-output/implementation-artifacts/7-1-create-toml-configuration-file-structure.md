---
story_id: 7.1
story_key: 7-1-create-toml-configuration-file-structure
epic: 7
epic_title: Advanced Configuration & Admin
status: backlog
priority: high
difficulty: low
dependencies: ["1-2-create-configuration-module-with-layered-loading"]
---
# Story 7.1: Create TOML Configuration File Structure
## Story Statement
As a developer, I want to configure roro via TOML configuration files, so that I can customize behavior without code changes.
## Acceptance Criteria
**Given** config file at ~/.vibe/config.toml, **When** roro starts, **Then** file loaded and parsed, settings applied.
**Given** config contains provider and tool settings, **When** roro starts, **Then** provider configured, bash tool requires confirmation, timeout set.
## FRs: FR54, FR61
## Tasks
- [ ] Finalize TOML schema
- [ ] Document example config.toml
- [ ] Create default config generation
- [ ] Validate TOML parsing
- [ ] Add unit tests
