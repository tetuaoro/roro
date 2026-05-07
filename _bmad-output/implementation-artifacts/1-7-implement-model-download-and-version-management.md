---
story_id: 1.7
story_key: 1-7-implement-model-download-and-version-management
epic: 1
epic_title: Foundation - Local Provider Connection
epic_goal: Users can configure roro to connect to their local LLM providers (Ollama, LocalAI, Llama.cpp) and verify the connection works.
status: backlog
priority: medium
difficulty: medium
dependencies: ["1-1-initialize-rust-project-structure", "1-3-implement-provider-client-with-openai-api-rust"]
created: 2025-01-15T00:00:00Z
updated: 2025-01-15T00:00:00Z
---

# Story 1.7: Implement Model Download and Version Management

## Story Statement

As a user,
I want to download and manage model versions from provider repositories,
So that I can use the latest models without manual downloads.

## Acceptance Criteria

**Given** Ollama is configured as provider
**When** I run `roro --download llava:13b`
**Then** the model is downloaded via Ollama's API
**And** download progress is displayed
**And** a success message confirms the model is ready

**Given** multiple versions of a model exist
**When** I list available models
**Then** all versions are displayed
**And** I can select which version to use

**Given** a model update is available
**When** I run `roro --update-models`
**Then** newer versions are downloaded
**And** old versions are preserved

**Given** a model download fails
**When** the operation completes
**Then** a clear error message is displayed
**And** partial downloads are cleaned up

## Functional Requirements Covered

- FR82: Users can download models from provider repositories
- FR83: Users can update to newer model versions
- FR84: Users can manage multiple model versions simultaneously

## Developer Context

### Provider APIs
- Ollama: POST /api/pull to download models
- Ollama: GET /api/tags to list models
- LocalAI: Model management via API

### Implementation Notes
- Ollama has built-in model management
- Need to integrate with provider-specific APIs
- Progress reporting for large downloads

## Dev Agent Guardrails

### Critical Requirements
1. Provider-specific implementations
2. Progress tracking for downloads
3. Cleanup on failure
4. Version listing and selection

## Tasks/Subtasks

- [ ] Create `src/provider/models.rs`
- [ ] Implement model download for Ollama
- [ ] Implement model download for LocalAI
- [ ] Add download progress display
- [ ] Implement model listing
- [ ] Add version parsing
- [ ] Implement --download CLI flag
- [ ] Implement --update-models CLI flag
- [ ] Add error handling for failed downloads
- [ ] Add cleanup for partial downloads
- [ ] Add unit tests

---

## Dev Agent Record

### Implementation Plan
1. Create models.rs
2. Implement provider-specific download logic
3. Add progress tracking
4. Add CLI flags
5. Add error handling
6. Add tests

---

## File List

- `src/provider/models.rs` (new)
- `src/cli/args.rs` (modified)

---

## Change Log

