---
story_id: 1.6
story_key: 1-6-add-provider-switching-functionality
epic: 1
epic_title: Foundation - Local Provider Connection
epic_goal: Users can configure roro to connect to their local LLM providers (Ollama, LocalAI, Llama.cpp) and verify the connection works.
status: backlog
priority: high
difficulty: low
dependencies: ["1-1-initialize-rust-project-structure", "1-2-create-configuration-module-with-layered-loading", "1-3-implement-provider-client-with-openai-api-rust"]
created: 2025-01-15T00:00:00Z
updated: 2025-01-15T00:00:00Z
---

# Story 1.6: Add Provider Switching Functionality

## Story Statement

As a user,
I want to switch between configured providers,
So that I can use different LLMs for different tasks.

## Acceptance Criteria

**Given** multiple providers are configured in config.toml
**When** I specify a provider via CLI flag `--provider ollama`
**Then** roro uses the Ollama provider for the session
**And** the active provider is displayed in the UI

**Given** no provider is explicitly specified
**When** roro starts
**Then** the default provider from config is used

**Given** I want to switch providers during a session
**When** I use the TUI provider switch command
**Then** the current conversation context is preserved
**And** subsequent requests use the new provider

**Given** an invalid provider name is specified
**When** roro starts
**Then** an error is displayed listing valid providers
**And** exit code 1 is returned

## Functional Requirements Covered

- FR31: Users can switch between configured providers

## Developer Context

### Dependencies
- Story 1.1: Project structure
- Story 1.2: Config module (provider profiles)
- Story 1.3: ProviderClient

### Implementation
- Add --provider CLI flag
- Support provider profiles in config
- Default provider selection
- Runtime switching

## Dev Agent Guardrails

### Critical Requirements
1. CLI flag: --provider <name>
2. Config: Multiple [provider.<name>] sections
3. Default: default_provider setting
4. Validation: Check provider exists before use

## Tasks/Subtasks

- [ ] Add --provider flag to CLI args
- [ ] Update config schema to support multiple providers
- [ ] Add provider profile selection logic
- [ ] Set default provider from config
- [ ] Add runtime provider switching
- [ ] Validate provider exists
- [ ] Display active provider in UI
- [ ] List available providers on invalid selection
- [ ] Add unit tests for switching logic

---

## Dev Agent Record

### Implementation Plan
1. Add CLI flag
2. Update config schema
3. Implement provider selection
4. Add switching mechanism
5. Add validation
6. Add tests

---

## File List

- `src/cli/args.rs` (modified)
- `src/config/schema.rs` (modified)
- `src/provider/client.rs` (modified)

---

## Change Log

