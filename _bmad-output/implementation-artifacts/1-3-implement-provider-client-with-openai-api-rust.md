---
story_id: 1.3
story_key: 1-3-implement-provider-client-with-openai-api-rust
epic: 1
epic_title: Foundation - Local Provider Connection
epic_goal: Users can configure roro to connect to their local LLM providers (Ollama, LocalAI, Llama.cpp) and verify the connection works.
status: backlog
priority: high
difficulty: high
dependencies: ["1-1-initialize-rust-project-structure", "1-2-create-configuration-module-with-layered-loading"]
created: 2025-01-15T00:00:00Z
updated: 2025-01-15T00:00:00Z
---

# Story 1.3: Implement Provider Client with openai_api_rust

## Story Statement

As a developer,
I want to create a provider client that uses the openai_api_rust crate directly,
So that roro can communicate with OpenAI API-compatible local providers.

## Epic Context

**Epic 1: Foundation - Local Provider Connection**

**Previous Stories:** 1.1 (Project Structure), 1.2 (Configuration)
**Next Story:** 1.4 (Connectivity Validation)

This story creates the core provider integration layer. All LLM communication flows through this client.

## Acceptance Criteria

**Given** a provider base URL is configured (e.g., http://localhost:11434 for Ollama)
**When** the provider client is initialized
**Then** it creates an OpenAIClient with the correct base URL
**And** it sets appropriate timeout values (default 120 seconds)

**Given** a chat completion request is made
**When** the client sends the request
**Then** it uses the OpenAI Chat API v1 format
**And** it includes the model parameter from config
**And** it streams responses when requested

**Given** the request fails
**When** an error occurs
**Then** a ProviderError is returned with context
**And** the original error is preserved via error chaining

## Functional Requirements Covered

- FR29: Users can configure provider base URL (Ollama, LocalAI, Llama.cpp, or custom)
- FR34: Users can override model selection per request
- FR77: System can inherit environment variables for tool execution
- FR78: System can access files within trusted directories
- FR79: Users can temporarily trust directories for file access
- FR80: System can respect `.gitignore` and `.vibeignore` for file operations

## Developer Context

### Architecture Decision

**Provider Client Architecture:** Direct `openai_api_rust` crate usage (no wrapper trait pattern)

This was selected to:
- Reduce abstraction overhead
- Leverage existing OpenAI API compatibility
- Simplify maintenance (no custom trait to maintain)
- Enable direct access to crate features

### Technical Stack

- **openai_api_rust**: Primary provider client
- **reqwest**: Underlying HTTP (via openai_api_rust)
- **thiserror**: Type-safe error definitions
- **anyhow**: Ergonomic error handling in application code

### Provider Client Structure

```
src/provider/
├── mod.rs              # Main exports
├── client.rs           # OpenAIClient wrapper
├── error.rs            # ProviderError enum
├── models.rs           # Model management
└── types.rs            # Custom types for requests/responses
```

### Key Implementation Details

**Client Initialization:**
```rust
use openai_api_rust::OpenAIClient;
use std::time::Duration;

pub struct ProviderClient {
    client: OpenAIClient,
    base_url: String,
    default_model: String,
    timeout: Duration,
}

impl ProviderClient {
    pub fn new(base_url: String, default_model: String, timeout_secs: u64) -> Result<Self, ProviderError> {
        let client = OpenAIClient::new(base_url.clone(), timeout_secs)?;
        Ok(Self {
            client,
            base_url,
            default_model,
            timeout: Duration::from_secs(timeout_secs),
        })
    }
}
```

**Request Format:**
- OpenAI Chat API v1 compatible
- Streaming support for real-time responses
- Proper error propagation with context

## Dev Agent Guardrails

### Critical Requirements

1. **Direct Usage:** Must use openai_api_rust directly, no wrapper traits
2. **Error Chaining:** Must preserve original errors via `from` or `context`
3. **Timeout Configuration:** Must be configurable via config
4. **Base URL:** Must support all OpenAI-compatible providers

### Error Handling Pattern

```rust
#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Connection failed to {url}: {source}")]
    ConnectionFailed { url: String, source: reqwest::Error },
    #[error("API error: {message}")]
    ApiError { message: String },
    #[error("Timeout after {duration} seconds")]
    Timeout { duration: u64 },
}
```

### Testing Requirements

- Unit tests for client initialization
- Integration tests for API communication (mocked)
- Error handling tests for various failure scenarios

## Tasks/Subtasks

- [ ] Create `src/provider/mod.rs` with main exports
- [ ] Create `src/provider/client.rs` with ProviderClient struct
- [ ] Implement new() constructor with base_url, model, timeout
- [ ] Create `src/provider/error.rs` with ProviderError enum
- [ ] Implement chat completion method with streaming
- [ ] Add model override support per request
- [ ] Create `src/provider/types.rs` for request/response types
- [ ] Implement proper error conversion from openai_api_rust errors
- [ ] Add timeout configuration support
- [ ] Add unit tests for client initialization
- [ ] Add integration tests with mock server
- [ ] Verify client works with Ollama endpoint
- [ ] Verify client works with LocalAI endpoint
- [ ] Verify client works with Llama.cpp endpoint

---

## Dev Agent Record

### Implementation Plan

1. Create provider module structure
2. Implement ProviderClient with OpenAIClient wrapper
3. Define ProviderError enum with thiserror
4. Implement chat completion with streaming
5. Add error handling and conversion
6. Add tests

### Debug Log

*No debugging yet*

### Completion Notes

*Story not yet implemented*

---

## File List

- `src/provider/mod.rs`
- `src/provider/client.rs`
- `src/provider/error.rs`
- `src/provider/types.rs`

---

## Change Log

*Will be populated during implementation*
