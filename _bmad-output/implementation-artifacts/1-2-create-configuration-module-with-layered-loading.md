---
story_id: 1.2
story_key: 1-2-create-configuration-module-with-layered-loading
epic: 1
epic_title: Foundation - Local Provider Connection
epic_goal: Users can configure roro to connect to their local LLM providers (Ollama, LocalAI, Llama.cpp) and verify the connection works.
status: backlog
priority: high
difficulty: medium
dependencies: ["1-1-initialize-rust-project-structure"]
created: 2025-01-15T00:00:00Z
updated: 2025-01-15T00:00:00Z
---

# Story 1.2: Create Configuration Module with Layered Loading

## Story Statement

As a developer,
I want to implement a configuration system that loads settings from multiple sources with proper precedence,
So that users can configure roro via TOML files, environment variables, and CLI flags.

## Epic Context

**Epic 1: Foundation - Local Provider Connection**

This epic establishes the fundamental foundation for roro, enabling users to configure and connect to local LLM providers (Ollama, LocalAI, Llama.cpp) and verify connections work properly.

**Previous Story:** 1.1 (Initialize Rust Project Structure) - Project foundation established
**Next Story:** 1.3 (Implement Provider Client with openai_api_rust) - Will use this config

**Stories in this Epic:**
- 1.1: Initialize Rust Project Structure ✅
- 1.2: Create Configuration Module with Layered Loading (THIS STORY)
- 1.3: Implement Provider Client with openai_api_rust
- 1.4: Add Provider Connectivity Validation
- 1.5: Implement Auto-Detection of Provider Capabilities
- 1.6: Add Provider Switching Functionality
- 1.7: Implement Model Download and Version Management
- 1.8: Add Interactive Setup Command
- 1.9: Implement Provider Timeout Handling with Retry

## Acceptance Criteria

**Given** a config file exists at ~/.vibe/config.toml
**When** roro starts
**Then** the config is loaded from file
**And** environment variables override file values (e.g., VIBE_PROVIDER_URL overrides config file)
**And** CLI flags override environment variables

**Given** environment variables contain `${VAR}` syntax
**When** config is loaded
**Then** variables are expanded (e.g., `${HOME}/.vibe` resolves to actual path)

**Given** an invalid config file exists
**When** roro starts
**Then** a clear error message is displayed
**And** the exit code is 1

## Functional Requirements Covered

- FR54: Users can configure roro via TOML configuration files
- FR55: Users can override config via environment variables
- FR56: System can expand environment variables in config values (`${VAR}`)
- FR57: Users can specify custom config file via `--config` flag
- FR58: System can validate configuration on startup
- FR59: System can provide clear errors for invalid configuration
- FR63: System can load config from multiple sources with proper precedence

## Non-Functional Requirements

- **Security:** Input sanitization for config paths to prevent path traversal attacks
- **Compliance:** Config files must follow TOML v1.0 specification with JSON compatibility layer

---

## Developer Context

### Project Overview

**roro** is a Rust rewrite of Mistral Vibe CLI that mirrors the Python version's exact CLI interface, tool safety framework, and configuration schema. Configuration backward compatibility is critical - existing Python users must be able to migrate without configuration changes.

### Why This Story Matters

This story establishes the **configuration foundation** that all other components depend on:

1. **Layered Loading:** CLI args > env vars > config file > defaults
2. **TOML Support:** Standard TOML v1.0 specification
3. **Variable Expansion:** `${VAR}` syntax support
4. **Validation:** Schema validation with clear error messages
5. **Precedence:** Proper override hierarchy

Without this, no other story can properly configure providers, tools, or any other feature.

### Previous Story Intelligence

**Story 1.1 (Initialize Rust Project Structure) Learnings:**
- Project structure mirrors Python Mistral Vibe
- All dependencies added to Cargo.toml
- Module structure created: `src/cli/`, `src/config/`, `src/provider/`, etc.
- Cargo.toml uses Rust 2024 edition
- Key crates: tokio, serde, clap, ratatui, crossterm, openai_api_rust, config, thiserror, anyhow, reqwest

**Key Files Created:**
- `Cargo.toml` with all dependencies
- `src/main.rs` (basic entry point)
- `src/lib.rs` (library entry point)
- Module directories with empty `mod.rs` files

### Git Intelligence

*No git commits yet for configuration module*

---

## Technical Requirements

### Architecture Compliance

**Configuration Management (from Architecture):** Layered config with validation using config-rs + Serde + validation traits

**Key Requirements:**
- config-rs crate for layered loading
- Serde for TOML deserialization
- Custom validation traits for business rules
- Environment variable expansion

### Technology Stack

**Configuration Libraries:**
- `config` v0.14 - Layered configuration loading
- `serde` - TOML/JSON serialization/deserialization
- `serde_json` - JSON support

### Configuration Schema

**Primary Config File:** `~/.vibe/config.toml`

```toml
# Provider Configuration
[provider]
base_url = "http://localhost:11434"
model = "mistral:7b"
timeout = 120

# Tool Configuration
[tools]
default_permission = "ASK"

# Session Configuration
[session]
session_dir = "${HOME}/.vibe/sessions"
auto_save = true

# Logging Configuration
[logging]
level = "INFO"
file = "${HOME}/.vibe/logs/vibe.log"
```

**Environment Variable Mapping:**
- `VIBE_PROVIDER_BASE_URL` → `provider.base_url`
- `VIBE_PROVIDER_MODEL` → `provider.model`
- `VIBE_SESSION_DIR` → `session.session_dir`
- All env vars use snake_case and map to config keys via dot notation

**CLI Flag Overrides:**
- `--provider` → Overrides `provider.base_url`
- `--model` → Overrides `provider.model`
- `--config` → Specifies custom config file path

### Precedence Order

1. **CLI Flags** (highest priority)
2. **Environment Variables** (`VIBE_*` prefix)
3. **Config File** (TOML)
4. **Defaults** (lowest priority)

### Variable Expansion

Support `${VAR}` syntax in config values:
- `${HOME}` → User's home directory
- `${OLLAMA_HOST}` → Environment variable
- `${PROJECT_ROOT}` → Project root directory
- Nested: `${VAR}/subdir/${ANOTHER_VAR}`

---

## Dev Agent Guardrails

### Critical Architecture Requirements

1. **Layered Loading:** Must use config-rs crate for proper precedence
2. **Validation:** Must validate all config values on startup
3. **Error Messages:** Must provide clear, actionable error messages for invalid config
4. **Expansion:** Must support `${VAR}` syntax in all string values
5. **Schema:** Must use Serde for schema definition and validation

### Library/Framework Requirements

| Library | Version | Purpose |
|---------|---------|---------|
| config | 0.14 | Layered configuration loading |
| serde | 1.x | TOML/JSON serialization |
| serde_json | 1.x | JSON support |

### File Structure Requirements

```
src/config/
├── mod.rs              # Main config module exports
├── loading.rs          # Layered loading logic
├── schema.rs           # Config schema definitions
├── validation.rs       # Validation logic
├── expansion.rs        # Variable expansion
└── error.rs            # Configuration error types
```

### Coding Standards

**Config Schema Definition:**
```rust
use config::{Config, File, Environment, Value};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderConfig {
    pub base_url: String,
    pub model: String,
    pub timeout: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub provider: ProviderConfig,
    pub session: SessionConfig,
    // ... other config sections
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        // Layered loading implementation
    }
}
```

**Validation Approach:**
- Use `config::Config` builder pattern
- Load from file, then environment, then merge
- Validate after full loading
- Return clear errors for missing/invalid values

### Security Requirements

- **Path Traversal:** Sanitize all config file paths
- **Variable Expansion:** Limit expansion depth to prevent infinite recursion
- **File Access:** Restrict config file access to known locations

### Testing Requirements

- Unit tests for config loading with different precedence scenarios
- Tests for variable expansion edge cases
- Tests for invalid config handling
- Integration tests for full config loading flow

---

## Tasks/Subtasks

- [ ] Create `src/config/mod.rs` with main exports
- [ ] Create `src/config/schema.rs` with AppConfig, ProviderConfig, ToolConfig structs
- [ ] Implement config schema with Serde Deserialize
- [ ] Create `src/config/loading.rs` with layered loading function
- [ ] Implement config loading from file (TOML)
- [ ] Implement environment variable loading with VIBE_* prefix
- [ ] Implement CLI flag override support
- [ ] Create `src/config/expansion.rs` with variable expansion
- [ ] Implement `${VAR}` expansion in all string fields
- [ ] Create `src/config/validation.rs` with validation logic
- [ ] Implement validation for all config fields
- [ ] Create `src/config/error.rs` with ConfigError enum
- [ ] Define clear error types for all failure modes
- [ ] Add unit tests for config loading and expansion
- [ ] Verify config loads correctly from default locations
- [ ] Test precedence: CLI > env > file > defaults

---

## Dev Agent Record

### Implementation Plan

1. **Schema Definition:** Define all config structs in `schema.rs`
2. **Loading Implementation:** Create layered loading in `loading.rs`
3. **Expansion:** Implement variable expansion in `expansion.rs`
4. **Validation:** Add validation logic in `validation.rs`
5. **Error Handling:** Define ConfigError in `error.rs`
6. **Integration:** Connect config to main.rs entry point
7. **Testing:** Add comprehensive unit tests

**Dependency:** Story 1.1 must be complete (project structure exists)

### Debug Log

*No debugging yet - story not started*

### Completion Notes

*Story not yet implemented*

---

## File List

*Will be populated during implementation*

- `src/config/mod.rs`
- `src/config/schema.rs`
- `src/config/loading.rs`
- `src/config/expansion.rs`
- `src/config/validation.rs`
- `src/config/error.rs`

---

## Change Log

*Will be populated during implementation*
