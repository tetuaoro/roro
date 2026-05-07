---
story_id: 1.4
story_key: 1-4-add-provider-connectivity-validation
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

# Story 1.4: Add Provider Connectivity Validation

## Story Statement

As a user,
I want roro to validate provider connectivity on startup,
So that I know immediately if my local provider is unavailable.

## Acceptance Criteria

**Given** a provider is configured
**When** roro starts
**Then** a connectivity check is performed against the base URL
**And** if successful, a "Connected to {provider}" message is displayed
**And** if failed, a clear error message shows the issue (connection refused, timeout, etc.)

**Given** multiple providers are configured
**When** roro starts
**Then** all providers are validated
**And** only the first healthy provider is used as default

**Given** provider validation fails
**When** roro starts
**Then** exit code 1 is returned
**And** troubleshooting suggestions are displayed (e.g., "Check if Ollama is running")

## Functional Requirements Covered

- FR32: System can validate provider connectivity on startup
- FR36: System can display clear error messages for provider failures

## Developer Context

### Dependencies
- Story 1.1: Project structure exists
- Story 1.2: Config module available
- Story 1.3: ProviderClient available

### Implementation Location
- New file: `src/provider/health.rs`
- Integration: Modify `src/main.rs` to call health check on startup

### Connectivity Check Approach
1. Attempt HTTP GET to provider base_url/v1/models or /api/tags
2. Check for successful response (200 OK)
3. Handle various error cases: connection refused, timeout, DNS resolution failure
4. Provide actionable error messages

## Dev Agent Guardrails

### Critical Requirements
1. **Non-blocking:** Health check should be async
2. **Timeout:** Must have reasonable timeout (5-10 seconds)
3. **Multiple Providers:** Check all configured providers
4. **Error Messages:** Must be clear and actionable
5. **Exit Code:** Return 1 on failure

### Error Types to Handle
- Connection refused (provider not running)
- Timeout (provider slow to respond)
- DNS resolution failure (invalid URL)
- HTTP errors (404, 500, etc.)
- SSL/TLS errors

### Suggested Error Messages
```
"Connection refused: Is {provider} running at {url}?"
"Connection timed out: {provider} at {url} took too long to respond"
"Invalid URL: Could not resolve {url}"
"HTTP error {status}: {provider} returned an error"
```

## Tasks/Subtasks

- [ ] Create `src/provider/health.rs` with connectivity checking
- [ ] Implement async health check function
- [ ] Add ProviderHealth struct with status and error details
- [ ] Implement health check for single provider
- [ ] Implement health check for all configured providers
- [ ] Add timeout configuration (default 5 seconds)
- [ ] Integrate health check into startup in `src/main.rs`
- [ ] Display connection status messages
- [ ] Return exit code 1 on all provider failures
- [ ] Add troubleshooting suggestions to error messages
- [ ] Add unit tests for health check logic
- [ ] Test with Ollama provider
- [ ] Test with LocalAI provider
- [ ] Test with invalid provider URL

---

## Dev Agent Record

### Implementation Plan
1. Create health.rs with async health checking
2. Implement ProviderHealth status
3. Add health check function
4. Integrate into main.rs startup
5. Add error handling and messages
6. Add tests

### Debug Log

*No debugging yet*

### Completion Notes

*Story not yet implemented*

---

## File List

- `src/provider/health.rs` (new)
- `src/main.rs` (modified)

---

## Change Log

*Will be populated during implementation*
