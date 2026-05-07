---
story_id: 1.8
story_key: 1-8-add-interactive-setup-command
epic: 1
epic_title: Foundation - Local Provider Connection
epic_goal: Users can configure roro to connect to their local LLM providers (Ollama, LocalAI, Llama.cpp) and verify the connection works.
status: backlog
priority: medium
difficulty: low
dependencies: ["1-1-initialize-rust-project-structure", "1-2-create-configuration-module-with-layered-loading"]
created: 2025-01-15T00:00:00Z
updated: 2025-01-15T00:00:00Z
---

# Story 1.8: Add Interactive Setup Command

## Story Statement

As a new user,
I want to run an interactive setup to configure my provider,
So that I can quickly get started with roro.

## Acceptance Criteria

**Given** I run `roro --setup`
**When** the setup wizard starts
**Then** I am prompted for provider type (Ollama, LocalAI, Llama.cpp, Custom)
**And** I am prompted for base URL (with sensible defaults)

**Given** I select Ollama as provider type
**When** I complete the setup
**Then** http://localhost:11434 is used as default base URL
**And** a connectivity test is performed
**And** if successful, the config is saved to ~/.vibe/config.toml

**Given** I enter an invalid base URL
**When** the connectivity test runs
**Then** I am prompted to re-enter the URL
**And** helpful error messages guide me

**Given** the setup completes successfully
**When** the process finishes
**Then** a confirmation message is displayed
**And** I can immediately start using roro

## Functional Requirements Covered

- FR9: Users can run `--setup` to configure API key interactively

## Developer Context

### Dependencies
- Story 1.1: Project structure
- Story 1.2: Config module

### Implementation
- Interactive CLI prompts
- Provider type selection
- Base URL input with defaults
- Connectivity test
- Config file generation

## Dev Agent Guardrails

### Critical Requirements
1. Interactive prompts with dialoguer or similar
2. Provider type selection menu
3. Default values for common providers
4. Connectivity validation
5. Config file creation

## Tasks/Subtasks

- [ ] Create `src/cli/setup.rs`
- [ ] Add --setup flag to CLI args
- [ ] Implement provider type selection prompt
- [ ] Implement base URL input with defaults
- [ ] Add connectivity test integration
- [ ] Create config file from user input
- [ ] Save config to ~/.vibe/config.toml
- [ ] Display confirmation message
- [ ] Add error handling for failed setup
- [ ] Add unit tests

---

## Dev Agent Record

### Implementation Plan
1. Create setup.rs
2. Add CLI flag
3. Implement interactive prompts
4. Add connectivity test
5. Save config
6. Add tests

---

## File List

- `src/cli/setup.rs` (new)
- `src/cli/mod.rs` (modified)

---

## Change Log

