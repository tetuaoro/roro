---
story_id: 1.1
story_key: 1-1-initialize-rust-project-structure
epic: 1
epic_title: Foundation - Local Provider Connection
epic_goal: Users can configure roro to connect to their local LLM providers (Ollama, LocalAI, Llama.cpp) and verify the connection works.
status: ready-for-dev
priority: high
difficulty: low
dependencies: []
created: 2025-01-15T00:00:00Z
updated: 2025-01-15T00:00:00Z
---

# Story 1.1: Initialize Rust Project Structure

## Story Statement

As a Rust developer,
I want to initialize a new Cargo project with all required dependencies,
So that I have the foundation for building roro.

## Epic Context

**Epic 1: Foundation - Local Provider Connection**

This epic establishes the fundamental foundation for roro, enabling users to configure and connect to local LLM providers (Ollama, LocalAI, Llama.cpp) and verify connections work properly. Without this foundation, no other functionality is possible.

**Epic Goal:** Users can configure roro to connect to their local LLM providers and verify the connection works.

**Stories in this Epic:**
- 1.1: Initialize Rust Project Structure (THIS STORY)
- 1.2: Create Configuration Module with Layered Loading
- 1.3: Implement Provider Client with openai_api_rust
- 1.4: Add Provider Connectivity Validation
- 1.5: Implement Auto-Detection of Provider Capabilities
- 1.6: Add Provider Switching Functionality
- 1.7: Implement Model Download and Version Management
- 1.8: Add Interactive Setup Command
- 1.9: Implement Provider Timeout Handling with Retry

## Acceptance Criteria

**Given** a new Rust project is being created
**When** I run `cargo init --name roro`
**Then** a valid Cargo.toml is created
**And** all required dependencies are added: tokio (full features), serde (with derive), serde_json, clap (with derive), ratatui, crossterm, openai_api_rust, config, thiserror, anyhow, reqwest (with json feature)

**Given** the project is initialized
**When** I run `cargo check`
**Then** the project compiles without errors
**And** the module structure mirrors Python Mistral Vibe (src/, with main.rs and lib.rs)

## Functional Requirements Covered

- FR76: Users can generate shell completion scripts for bash/zsh/fish (indirect - project must compile first)
- Additional: Custom Cargo project (from Architecture Decision Document)
- Additional: Starter Template Setup (from Architecture)

## Non-Functional Requirements

- **Performance:** CLI must be ready for user input within 2 seconds of invocation
- **Security:** Zero network calls to external services beyond configured provider endpoints
- **Integration:** Native integration with cargo, rustc, clippy, rustfmt with zero configuration required
- **Compliance:** All dependencies must use Rust-compatible licenses (MIT, Apache 2.0, BSD)

---

## Developer Context

### Project Overview

**roro** is a Rust rewrite of Mistral Vibe CLI that provides a high-performance, memory-safe coding assistant interface to local OpenAI API-compatible LLM providers. The project mirrors the Python Mistral Vibe's exact CLI interface, tool safety framework (ALWAYS/ASK/NEVER permissions), and configuration schema while delivering Rust's performance, memory safety, and concurrency advantages.

**Target Users:** Developers who need a CLI-based AI coding assistant running entirely on-premises with Ollama, LocalAI, or Llama.cpp.

**Core Problem Solved:** Enabling offline, private, and performant local LLM interactions through a familiar CLI interface while maintaining 100% command compatibility with the original Python implementation.

### Why This Story Matters

This is the **foundational story** for the entire roro project. Without a properly initialized Rust project with all required dependencies, no subsequent development is possible. This story establishes:

1. **Project Structure:** Mirrors Python Mistral Vibe exactly
2. **Dependency Management:** All required crates with correct features
3. **Build Verification:** Project compiles without errors
4. **Foundation for All Future Work:** Every other story depends on this

### Previous Story Intelligence

*This is the first story in Epic 1 and the first story overall. No previous story learnings to incorporate.*

---

## Technical Requirements

### Architecture Compliance

**Selected Starter:** Custom Cargo Project (from Architecture Decision Document)

**Rationale:** Roro's mirror architecture requirement (100% compatibility with Python Mistral Vibe CLI/TUI) means generic starter templates won't provide the right foundation. A custom project structure allows:
- Mirror Python module organization exactly
- Select specific crate versions matching PRD tech stack
- Configure Cargo features for conditional compilation
- Establish custom build scripts for packaging

### Technology Stack (from Architecture)

**Language & Runtime:**
- Rust 2024 edition
- Tokio 1.x async runtime
- Serde for serialization

**CLI Framework:**
- Clap for argument parsing with all required flags
- Automatic help generation
- Shell completion support

**TUI Framework:**
- ratatui for widget-based UI components
- crossterm for terminal backend
- Screen-based organization matching Textual's model

**Provider Integration:**
- openai_api_rust as primary dependency
- Reqwest for underlying HTTP
- Async trait implementations for provider agnosticism

**Development Experience:**
- Cargo workspace for potential future expansion
- Dev dependencies: tokio-test, proptest, criterion
- Proper feature flag organization

### Required Dependencies

```toml
[package]
name = "roro"
version = "0.1.0"
edition = "2024"
authors = ["Rao Nagos"]
description = "Rust rewrite of Mistral Vibe CLI with local LLM provider support"
license = "MIT"
readme = "README.md"

[dependencies]
# Async runtime
tokio = { version = "1", features = ["full"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# CLI argument parsing
clap = { version = "4", features = ["derive", "env"] }

# TUI framework
ratatui = "0.27"
crossterm = "0.28"

# OpenAI API compatible client
openai_api_rust = "0.1"

# Configuration
config = "0.14"

# HTTP client (used by openai_api_rust)
reqwest = { version = "0.12", features = ["json"] }

# Error handling
thiserror = "2"
anyhow = "1"

[dev-dependencies]
tokio-test = "0.4"

[profile.release]
opt-level = 2
lto = true
codegen-units = 1
```

### Module Structure

Mirror Python Mistral Vibe structure:
```
src/
├── main.rs          # CLI entry point
├── lib.rs           # Library entry point
├── cli/             # CLI argument parsing
│   ├── mod.rs
│   ├── args.rs
│   └── ...
├── config/          # Configuration management
│   ├── mod.rs
│   ├── mod.rs
│   └── ...
├── provider/        # Provider client and management
│   ├── mod.rs
│   ├── client.rs
│   └── ...
├── tui/             # Textual TUI implementation
│   ├── mod.rs
│   ├── screens/
│   └── widgets/
├── session/         # Session persistence
│   ├── mod.rs
│   └── ...
├── tools/           # Tool execution framework
│   ├── mod.rs
│   ├── registry.rs
│   └── ...
└── system/          # System utilities
    ├── mod.rs
    └── ...
```

---

## Dev Agent Guardrails

### Critical Architecture Requirements

1. **Project Initialization:** Must use `cargo init --name roro` as base
2. **Cargo.toml:** Must include all dependencies listed above with exact versions
3. **Edition:** Must use Rust 2024 edition
4. **Module Structure:** Must mirror Python Mistral Vibe exactly
5. **No Wrapper Traits:** Direct `openai_api_rust` usage (no wrapper trait pattern)

### Library/Framework Requirements

| Library | Version | Features | Purpose |
|---------|---------|----------|---------|
| tokio | 1.x | full | Async runtime |
| serde | 1.x | derive | Serialization |
| clap | 4.x | derive, env | CLI parsing |
| ratatui | 0.27 | - | TUI widgets |
| crossterm | 0.28 | - | Terminal backend |
| openai_api_rust | 0.1 | - | Provider integration |
| config | 0.14 | - | Configuration loading |
| reqwest | 0.12 | json | HTTP client |
| thiserror | 2.x | - | Type-safe errors |
| anyhow | 1.x | - | Ergonomic errors |

### File Structure Requirements

- All source code in `src/` directory
- `main.rs` as CLI entry point
- `lib.rs` as library entry point
- Modules organized by feature (cli, config, provider, tui, session, tools, system)
- Each module has its own `mod.rs` file

### Coding Standards

**From AGENTS.md:**
- No relative imports - always use absolute paths from crate root
- Use `pathlib.Path` equivalent (std::path::Path) for file operations
- Modern type hints: built-in generics, `|` unions
- Enums: `StrEnum` / `IntEnum` equivalent with `auto()` and UPPERCASE members
- Write declarative, minimalist code
- Never call a private method from outside its class/module

**Rust-Specific:**
- Prefer `match` / `case` (Rust's `match`) over long `if` / `else` chains
- Use `clap` derive for CLI argument parsing
- Use `thiserror` for library errors, `anyhow` for application errors
- Async functions use Tokio runtime
- Error handling: chain with `?` and use `anyhow::Context` for context

### Testing Requirements

- All code must compile with `cargo check`
- No runtime tests required for this story (it's just project initialization)
- Subsequent stories will add tests

---

## Tasks/Subtasks

- [ ] Create new Rust project with `cargo init --name roro`
- [ ] Configure Cargo.toml with Rust 2024 edition
- [ ] Add all required dependencies with correct versions and features
- [ ] Create `src/lib.rs` with empty library entry point
- [ ] Verify project compiles with `cargo check`
- [ ] Create directory structure: `src/cli/`, `src/config/`, `src/provider/`, `src/tui/`, `src/session/`, `src/tools/`, `src/system/`
- [ ] Create `src/main.rs` with basic entry point
- [ ] Create empty `mod.rs` files in each module directory
- [ ] Add `.gitignore` for Rust/Cargo targets
- [ ] Verify complete project structure matches Python Mistral Vibe organization

---

## Dev Agent Record

### Implementation Plan

1. **Initialize Project:** Run `cargo init --name roro`
2. **Configure Cargo.toml:** 
   - Set edition to "2024"
   - Add all dependencies from the table above
   - Configure features for tokio and clap
3. **Create Module Structure:**
   - Create all module directories
   - Create empty `mod.rs` files in each
4. **Create Entry Points:**
   - `src/main.rs` with basic fn main()
   - `src/lib.rs` with empty pub mod declarations
5. **Verify:** Run `cargo check` to ensure compilation

### Debug Log

*No debugging yet - story not started*

### Completion Notes

*Story not yet implemented*

---

## File List

*No files created yet - will be updated during implementation*

---

## Change Log

*No changes yet - will be updated during implementation*
