---
story_id: 7.2
story_key: 7-2-support-environment-variable-overrides
epic: 7
epic_title: Advanced Configuration & Admin
status: backlog
priority: high
difficulty: low
dependencies: ["1-2-create-configuration-module-with-layered-loading"]
---
# Story 7.2: Support Environment Variable Overrides
## Story Statement
As a developer, I want to override config via environment variables, so that I can customize behavior per environment.
## Acceptance Criteria
**Given** VIBE_PROVIDER_BASE_URL=http://localhost:8080, **When** roro starts, **Then** provider URL overridden, takes precedence over config.toml.
**Given** VIBE_MODEL=llama2:13b, **When** roro starts, **Then** model overridden.
**Given** snake_case env vars, **When** processed, **Then** mapped to config keys (VIBE_PROVIDER_BASE_URL -> provider.base_url).
## FRs: FR55
## Tasks
- [ ] Implement VIBE_* env var parsing
- [ ] Add snake_case to dot notation mapping
- [ ] Ensure precedence over config file
- [ ] Add validation
- [ ] Add unit tests
