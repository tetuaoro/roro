---
story_id: 1.9
story_key: 1-9-implement-provider-timeout-handling-with-retry
epic: 1
epic_title: Foundation - Local Provider Connection
epic_goal: Users can configure roro to connect to their local LLM providers (Ollama, LocalAI, Llama.cpp) and verify the connection works.
status: backlog
priority: high
difficulty: medium
dependencies: ["1-3-implement-provider-client-with-openai-api-rust"]
created: 2025-01-15T00:00:00Z
updated: 2025-01-15T00:00:00Z
---

# Story 1.9: Implement Provider Timeout Handling with Retry

## Story Statement

As a user,
I want automatic retry with exponential backoff for failed provider requests,
So that transient network issues don't interrupt my workflow.

## Acceptance Criteria

**Given** a provider request times out
**When** the first attempt fails
**Then** a retry is attempted after 1 second
**And** if that fails, another retry after 2 seconds
**And** if that fails, a final retry after 4 seconds

**Given** all retry attempts fail
**When** the operation completes
**Then** a clear error message is displayed
**And** exit code 1 is returned

**Given** a request succeeds on retry
**When** the operation completes
**Then** the response is returned normally
**And** a warning is logged about the retry

**Given** the timeout is configurable
**When** I set timeout in config
**Then** the custom timeout is used for all requests

## Functional Requirements Covered

- FR35: System can handle provider timeouts with automatic retry

## Developer Context

### Retry Strategy
- Exponential backoff: 1s, 2s, 4s
- Maximum 3 attempts
- Configurable timeout per request
- Log warnings on retry

## Dev Agent Guardrails

### Critical Requirements
1. Exponential backoff with specific delays
2. Maximum 3 retry attempts
3. Configurable timeout
4. Preserve error context on final failure
5. Log retry attempts

## Tasks/Subtasks

- [ ] Create `src/provider/retry.rs`
- [ ] Implement retry logic with exponential backoff
- [ ] Add RetryConfig struct
- [ ] Integrate retry into ProviderClient
- [ ] Add configurable timeout
- [ ] Add retry attempt logging
- [ ] Preserve error context
- [ ] Add unit tests for retry logic

---

## Dev Agent Record

### Implementation Plan
1. Create retry.rs
2. Implement RetryConfig
3. Implement retry wrapper
4. Integrate with client
5. Add logging
6. Add tests

---

## File List

- `src/provider/retry.rs` (new)
- `src/provider/client.rs` (modified)

---

## Change Log

