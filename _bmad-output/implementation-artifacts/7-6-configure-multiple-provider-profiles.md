---
story_id: 7.6
story_key: 7-6-configure-multiple-provider-profiles
epic: 7
epic_title: Advanced Configuration & Admin
status: backlog
priority: medium
difficulty: low
dependencies: ["1-2-create-configuration-module-with-layered-loading"]
---
# Story 7.6: Configure Multiple Provider Profiles
## Story Statement
As a user, I want to configure multiple provider profiles, so that I can switch between different providers easily.
## Acceptance Criteria
**Given** config with [provider.ollama] and [provider.localai], **When** `roro --provider ollama`, **Then** Ollama provider used.
**Given** `roro --provider localai`, **When** executed, **Then** LocalAI provider used.
**Given** default_provider = "ollama" in config, **When** roro starts without --provider, **Then** Ollama provider used automatically.
## FRs: FR60, FR61
## Tasks
- [ ] Update config schema for multiple providers
- [ ] Add provider profile selection
- [ ] Implement default_provider setting
- [ ] Add provider switching via --provider
- [ ] Validate provider exists
- [ ] Add unit tests
