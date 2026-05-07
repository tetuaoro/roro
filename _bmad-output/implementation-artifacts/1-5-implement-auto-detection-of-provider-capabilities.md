---
story_id: 1.5
story_key: 1-5-implement-auto-detection-of-provider-capabilities
epic: 1
epic_title: Foundation - Local Provider Connection
epic_goal: Users can configure roro to connect to their local LLM providers (Ollama, LocalAI, Llama.cpp) and verify the connection works.
status: backlog
priority: high
difficulty: medium
dependencies: ["1-1-initialize-rust-project-structure", "1-2-create-configuration-module-with-layered-loading", "1-3-implement-provider-client-with-openai-api-rust"]
created: 2025-01-15T00:00:00Z
updated: 2025-01-15T00:00:00Z
---

# Story 1.5: Implement Auto-Detection of Provider Capabilities

## Story Statement

As a user,
I want roro to auto-detect provider capabilities,
So that I can see available models and their specifications without manual configuration.

## Acceptance Criteria

**Given** a connected provider
**When** capability detection runs
**Then** the provider's /v1/models endpoint is queried
**And** the list of available models is returned
**And** each model's context length and features are parsed

**Given** a provider is Ollama
**When** capability detection runs
**Then** Ollama-specific models are identified
**And** model tags (e.g., llava:13b, mistral:7b) are extracted

**Given** a provider is LocalAI
**When** capability detection runs
**Then** LocalAI model list is retrieved
**And** custom model metadata is preserved

**Given** capability detection fails
**When** the operation completes
**Then** a warning is logged
**And** the application continues with default values

## Functional Requirements Covered

- FR30: System can auto-detect provider capabilities (models, context length, features)
- FR33: System can display provider-specific model lists

## Developer Context

### Provider Model Endpoints
- Ollama: GET /api/tags or GET /v1/models
- LocalAI: GET /v1/models
- Llama.cpp: GET /v1/models (if compatible endpoint available)

### Model Metadata to Extract
- Model ID/name
- Context length (max tokens)
- Provider-specific tags/versions
- Creation/modification dates (if available)

## Dev Agent Guardrails

### Critical Requirements
1. **Fallback:** Continue with defaults if detection fails
2. **Caching:** Cache results to avoid repeated detection
3. **Provider-Specific:** Handle different provider API responses
4. **Error Handling:** Don't block startup on detection failure

## Tasks/Subtasks

- [ ] Create `src/provider/capabilities.rs`
- [ ] Implement Model struct with metadata
- [ ] Implement CapabilityDetector trait
- [ ] Add Ollama-specific detector
- [ ] Add LocalAI-specific detector
- [ ] Add Llama.cpp-specific detector
- [ ] Implement /v1/models endpoint querying
- [ ] Parse model list and metadata
- [ ] Add caching mechanism
- [ ] Integrate with ProviderClient
- [ ] Add fallback to defaults on failure
- [ ] Add unit tests with mock responses

---

## Dev Agent Record

### Implementation Plan
1. Create capabilities.rs module
2. Implement model detection for each provider type
3. Add caching layer
4. Integrate with client
5. Add error handling
6. Add tests

### Debug Log

*No debugging yet*

### Completion Notes

*Story not yet implemented*

---

## File List

- `src/provider/capabilities.rs` (new)
- `src/provider/client.rs` (modified)

---

## Change Log

*Will be populated during implementation*
