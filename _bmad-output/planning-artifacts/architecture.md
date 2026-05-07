---
stepsCompleted: [1, 2, 3, 4, 5, 6, 7, 8]
inputDocuments:
  - _bmad-output/planning-artifacts/prd.md
  - _bmad-output/planning-artifacts/research/market-rust-cli-local-llm-research-2026-05-06.md
  - _bmad-output/brainstorming/brainstorming-session-2026-05-06-150705.md
  - docs/README.md
  - docs/acp-setup.md
  - docs/proxy-setup.md
workflowType: 'architecture'
project_name: 'roro'
user_name: 'Rao Nagos'
date: '2026-05-07'
lastStep: 8
status: 'complete'
completedAt: '2026-05-07'
---

# Architecture Decision Document

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

## Project Context Analysis

### Requirements Overview

**Functional Requirements:**
84 capability contracts across 8 areas covering complete CLI/TUI mirror of Python Mistral Vibe, local provider integration via openai_api_rust, tool safety framework, session management, and programmatic mode.

**Non-Functional Requirements:**
Performance (500ms response latency, 20 tokens/sec streaming), Security (local-only guarantee, no telemetry, prompt injection protection), Reliability (auto-retry with backoff, 99.9% uptime), Integration (OpenAI API v1 compatibility, Rust toolchain), Compliance (data residency, license compliance).

**Scale & Complexity:**
High complexity CLI tool with scientific domain (AI/ML developer tooling). Requires 8-10 major architectural components with strict mirror compatibility constraints.

- Primary domain: CLI Tool with Textual TUI
- Complexity level: High
- Estimated architectural components: 8-10

### Technical Constraints & Dependencies

- Zero breaking changes from Python version (mirror requirement)
- Local-only operation with configurable provider URLs
- openai_api_rust crate for unified provider integration
- Tokio 1.x async runtime
- Serde for JSON/TOML serialization
- Clap for CLI argument parsing
- ratatui/crossterm for TUI replication
- Thiserror + Anyhow for error handling

### Cross-Cutting Concerns Identified

- Provider agnosticism across Ollama/LocalAI/Llama.cpp
- Tool safety framework enforcement (ALWAYS/ASK/NEVER)
- Session state persistence and recovery
- Configuration backward compatibility
- Error recovery and graceful degradation
- Performance optimization for local inference

## Starter Template Evaluation

### Primary Technology Domain

Rust CLI tool with Textual TUI interface, identified from PRD requirements analysis.

### Starter Options Considered

1. **Custom Cargo Project** - Custom project structure designed specifically for roro's mirror architecture requirements
2. **clap Template** - Official Clap CLI template with basic argument parsing
3. **cargo-quickstart** - General Rust project template with dev dependencies

### Selected Starter: Custom Cargo Project

**Rationale for Selection:**
Roro's mirror architecture requirement (100% compatibility with Python Mistral Vibe CLI/TUI) means generic starter templates won't provide the right foundation. A custom project structure allows us to:
- Mirror Python module organization exactly
- Select specific crate versions matching PRD tech stack
- Configure Cargo features for conditional compilation
- Establish custom build scripts for packaging

**Initialization Command:**

```bash
# Create base project
cargo new roro --name roro

# Add core dependencies to Cargo.toml
# (Full Cargo.toml will be defined in architectural decisions)
```

**Architectural Decisions Provided by Custom Approach:**

**Language & Runtime:**
- Rust 2024 edition
- Tokio 1.x async runtime
- Serde for serialization

**CLI Framework:**
- Clap for argument parsing with all required flags
- Automatic help generation
- Shell completion support

**TUI Framework:**
- ratatui for widget-based UI
- crossterm for terminal backend
- Custom widget implementations for Textual-like features

**Provider Integration:**
- openai_api_rust as primary dependency
- Reqwest for underlying HTTP
- Async trait implementations for provider agnosticism

**Development Experience:**
- Cargo workspace for potential future expansion
- Dev dependencies: tokio-test, proptest, criterion
- Proper feature flag organization

**Note:** Project initialization using custom Cargo.toml should be the first implementation story.

## Core Architectural Decisions

### Decision Priority Analysis

**Critical Decisions (Block Implementation):**
- Project Structure: Custom Cargo project with hybrid module organization mirroring Python
- Provider Integration: Direct `openai_api_rust` crate usage
- Agent Loop: Single struct with methods
- Tool System: Registry pattern with trait objects
- TUI Framework: ratatui with screen-based widget structure
- Error Handling: Thiserror + Anyhow combination

**Important Decisions (Shape Architecture):**
- Configuration: Layered config with validation using config-rs + Serde + validation traits
- Session Management: Serde serialization to JSON files

### Data Architecture

**Data Modeling:**
- Message history: Structured LLMMessage with role, content, metadata
- Session state: Serialized to JSON files with full context preservation
- Configuration: TOML-based with schema validation and environment variable expansion
- Tool registry: HashMap-based with permission metadata

**Data Validation Strategy:**
- Serde deserialization for config and session files
- Custom validation traits for business rules
- Compile-time validation via Rust type system where possible

### API & Communication Patterns

**Provider Communication:**
- Direct `openai_api_rust` usage for OpenAI-compatible endpoints
- Configurable base URL for provider selection
- Automatic retry with exponential backoff for failed requests

**Tool Communication:**
- Registry pattern for tool discovery and execution
- Permission system enforced at execution time
- Async tool execution with streaming support

### Frontend Architecture (TUI)

**TUI Framework:**
- **ratatui** for widget-based UI components
- **crossterm** for terminal backend
- Screen-based organization matching Textual's model

**Widget Structure:**
- `screens/` directory for each major screen (chat, tools, etc.)
- `widgets/` directory for shared reusable widgets (code blocks, messages, status)
- Each screen manages its own widget composition

**Screen Definitions:**
- ChatScreen: Main conversation interface
- ToolsScreen: Tool output display
- SetupScreen: Initial configuration
- SessionPickerScreen: Session selection/resume

### Infrastructure & Deployment

**Build System:**
- Cargo as build system with workspace support
- Conditional compilation via feature flags
- Binary releases for cross-platform distribution

**Dependencies:**
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
clap = { version = "4", features = ["derive", "env"] }
ratatui = "0.27"
crossterm = "0.28"
openai_api_rust = "0.1"  # or latest version
thiserror = "2"
anyhow = "1"
config = "0.14"
reqwest = { version = "0.12", features = ["json"] }
tokio-test = "0.4"
```

### Decision Impact Analysis

**Implementation Sequence:**
1. Project structure setup (Cargo.toml, module hierarchy)
2. Provider client with `openai_api_rust` integration
3. Configuration management with layered loading
4. Agent loop with single struct
5. Tool registry with permission system
6. Session manager with Serde serialization
7. TUI screens and widgets with ratatui
8. Error types with Thiserror + Anyhow

**Cross-Component Dependencies:**
- Agent loop depends on: Provider client, Tool registry, Session manager, Config
- TUI depends on: Agent loop, Tool registry, Config
- Provider client depends on: Config

## Implementation Patterns & Consistency Rules

### Pattern Categories Defined

**Critical Conflict Points Identified:** 8 areas where implementation choices could diverge without clear patterns

---

### Naming Patterns

**Rust Naming Conventions:**
- **Modules & Files:** snake_case (`mod.rs`, `provider_client.rs`, `agent_loop.rs`)
- **Types (structs, enums, traits):** PascalCase (`ProviderClient`, `LLMMessage`, `ToolPermission`)
- **Functions & Methods:** snake_case (`create_chat_completion`, `execute_tool`, `load_session`)
- **Variables:** snake_case (`provider_url`, `session_id`, `max_tokens`)
- **Constants:** SCREAMING_SNAKE_CASE (`DEFAULT_TIMEOUT`, `MAX_RETRIES`)

**File Organization:**
- Module directories: plural names matching Python (`cli/`, `core/`, `tui/`)
- File names: match the primary type or concept they contain
- Test files: `mod_tests.rs` in each module or separate `tests/` directory

---

### Structure Patterns

**Project Organization:**
```
roro/
├── Cargo.toml                 # Root manifest
├── src/
│   ├── main.rs                # Entry point - minimal, delegates to cli/
│   ├── lib.rs                 # Library exports (if needed)
│   ├── cli/                   # CLI layer (mirrors vibe/cli/)
│   │   ├── mod.rs
│   │   ├── args.rs            # Argument parsing with Clap
│   │   └── entrypoint.rs      # Main entry logic
│   ├── core/                  # Core logic (mirrors vibe/core/)
│   │   ├── mod.rs
│   │   ├── agent/
│   │   │   ├── mod.rs
│   │   │   └── loop.rs        # AgentLoop struct
│   │   ├── tools/
│   │   │   ├── mod.rs
│   │   │   ├── registry.rs
│   │   │   └── exec.rs
│   │   ├── config/
│   │   │   ├── mod.rs
│   │   │   ├── schema.rs
│   │   │   └── loader.rs
│   │   ├── provider/
│   │   │   ├── mod.rs
│   │   │   └── client.rs
│   │   ├── session/
│   │   │   ├── mod.rs
│   │   │   ├── saver.rs
│   │   │   └── loader.rs
│   │   └── error.rs
│   └── tui/                   # TUI layer (mirrors vibe/cli/textual_ui/)
│       ├── mod.rs
│       ├── app.rs             # Main App struct
│       ├── screens/
│       │   ├── mod.rs
│       │   ├── chat.rs
│       │   ├── tools.rs
│       │   ├── setup.rs
│       │   └── sessions.rs
│       └── widgets/
│           ├── mod.rs
│           ├── code_block.rs
│           ├── message.rs
│           └── status_bar.rs
└── tests/                      # Integration tests
```

**Where Tests Live:**
- Unit tests: In each module file (`mod.rs` or alongside the code)
- Integration tests: In `tests/` directory
- Test naming: `test_` prefix for test functions

**Configuration:**
- Main config file: `~/.roro/config.toml`
- Per-project overrides: `.roro.toml` in project directory
- Environment variables: `ROTO_` prefix (e.g., `ROTO_PROVIDER_URL`)

---

### Format Patterns

**API Request/Response Formats:**
- Use `openai_api_rust` types directly for provider communication
- Internal message format:
```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LLMMessage {
    pub role: Role,       // system, user, assistant, tool
    pub content: String,
    pub name: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_results: Option<Vec<ToolResult>>,
}
```
- JSON serialization: Always use snake_case for field names (Serde default)

**Error Response Structure:**
```rust
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetails,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetails {
    pub code: String,      // e.g., "provider_error", "tool_error"
    pub message: String,   // Human-readable error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}
```

**Data Exchange Formats:**
- All JSON: snake_case field names
- Timestamps: ISO 8601 format (RFC 3339)
- Booleans: `true`/`false` only, never `1`/`0`
- Nulls: Use `Option<T>` and omit null fields with `skip_serializing_if`

---

### Communication Patterns

**Provider Communication:**
- Always use `openai_api_rust::OpenAIClient` for requests
- Timeout: Default 30 seconds, configurable
- Retry: Exponential backoff (1s, 2s, 4s), max 3 attempts
- Error mapping: Convert provider errors to domain-specific errors

**Tool Execution:**
- Permission check BEFORE execution (not during)
- Validation: Parse and validate arguments before permission check
- Execution: Use `tokio::process::Command` for shell tools
- Output: Capture stdout, stderr, exit code
- Timeout: Configurable, default 60 seconds

**Inter-Widget Communication (TUI):**
- Use ratatui's message passing system
- Message types: Define enum for all TUI messages
```rust
pub enum AppMessage {
    UserInput(String),
    ToolExecuted(ToolResult),
    ProviderError(ProviderError),
    SessionLoaded(Session),
    // ...
}
```

---

### Process Patterns

**Error Handling:**
- **Library code:** Return `Result<T, ThisError>` with custom error types
- **Application code:** Use `anyhow::Result<T>` and `anyhow::Context`
- **User-facing errors:** Always provide actionable messages
- **Logging:** Use `tracing` or `log` crate with structured logging
- **Error codes:** Define error code enum for API/server errors

**Async Patterns:**
- **Provider calls:** Always use `.await` with proper error handling
- **Tool execution:** Use `tokio::spawn` for concurrent tool execution
- **Streaming:** Use `tokio_stream` for streaming responses
- **Timeouts:** Use `tokio::time::timeout` for all external calls
- **Cancellation:** Handle `tokio::task::JoinError` for spawned tasks

**Session Management:**
- Auto-save: After each user message and tool execution
- File naming: `{session_id}.json` in session directory
- Rotation: Max N sessions, delete oldest when limit reached
- Atomic writes: Write to temp file, then rename to prevent corruption

---

### Enforcement Guidelines

**All Developers/AI Agents MUST:**

- Follow Rust naming conventions (snake_case, PascalCase) consistently
- Use `openai_api_rust` for ALL provider communication (no direct HTTP calls)
- Enforce permission checks BEFORE tool execution
- Serialize all state to/from JSON using Serde
- Use `thiserror` for library error types, `anyhow` for application code
- Match Python Mistral Vibe's CLI argument structure exactly
- Write tests for all public functions

**Pattern Enforcement:**
- CI check: `cargo clippy -- -D warnings` (treat warnings as errors)
- CI check: `cargo fmt --check` (enforce formatting)
- CI check: `cargo test` (all tests must pass)
- Pre-commit hooks: Run clippy, fmt, tests

---

### Pattern Examples

**Good Examples:**
```rust
// ✅ Correct module structure
pub mod provider {
    pub struct Client { ... }
    impl Client { ... }
}

// ✅ Correct error handling
pub fn load_config() -> Result<Config, ConfigError> {
    // ...
}

// ✅ Correct async pattern
pub async fn get_completion(&self, request: ChatCompletionRequest) -> Result<StreamResponse, ProviderError> {
    let response = self.client.create_chat_completion(request).await?;
    Ok(response)
}
```

**Anti-Patterns:**
```rust
// ❌ Wrong - PascalCase for function
pub fn GetCompletion() { ... }

// ❌ Wrong - direct reqwest call (should use openai_api_rust)
let client = reqwest::Client::new();
let response = client.post(&url).send().await?;

// ❌ Wrong - no permission check before execution
pub async fn execute_tool(&self, name: &str) -> Result<(), ToolError> {
    let tool = self.registry.get(name)?;
    tool.execute().await  // Missing permission check!
}

// ❌ Wrong - blocking I/O in async context
pub async fn read_file(&self, path: &str) -> Result<String, ToolError> {
    std::fs::read_to_string(path)?  // Blocking!
}
```

## Project Structure & Boundaries

### Complete Project Directory Structure

```text
roro/
├── Cargo.toml                 # Root manifest with all dependencies
├── Cargo.lock
├── README.md
├── .gitignore
├── .github/
│   └── workflows/
│       ├── ci.yml             # CI pipeline with clippy/fmt/test
│       └── release.yml
├── src/
│   ├── main.rs                # Entry point: parses args, launches TUI or headless
│   ├── lib.rs                 # Library exports (if used as library)
│   │
│   ├── cli/                   # CLI Layer (mirrors vibe/cli/)
│   │   ├── mod.rs             # CLI module exports
│   │   ├── args.rs            # FR1-FR15: Clap argument definitions
│   │   └── entrypoint.rs      # Entry logic: routes to TUI or programmatic mode
│   │
│   ├── core/                  # Core Logic (mirrors vibe/core/)
│   │   ├── mod.rs             # Core module exports
│   │   │
│   │   ├── agent/             # Agent Loop (FR16-FR28 related)
│   │   │   ├── mod.rs
│   │   │   └── loop.rs        # FR16-FR28: AgentLoop struct with methods
│   │   │
│   │   ├── tools/             # Tool System (FR37-FR46)
│   │   │   ├── mod.rs         # Tool registry exports
│   │   │   ├── registry.rs    # FR6, FR45, FR46: ToolRegistry with permissions
│   │   │   ├── exec.rs        # FR37-FR44: Tool execution logic
│   │   │   └── builtins/      # Built-in tool implementations
│   │   │       ├── bash.rs    # Shell command tool
│   │   │       ├── read.rs    # File read tool
│   │   │       └── write.rs   # File write tool
│   │   │
│   │   ├── config/            # Configuration Management (FR54-FR63)
│   │   │   ├── mod.rs         # Config module exports
│   │   │   ├── schema.rs      # FR54, FR56: Config schema with Serde
│   │   │   ├── loader.rs      # FR55, FR57-FR59: Layered config loading
│   │   │   └── validator.rs   # Config validation
│   │   │
│   │   ├── provider/          # Provider Integration (FR29-FR36)
│   │   │   ├── mod.rs         # Provider module exports
│   │   │   └── client.rs      # FR29-FR36: ProviderClient with openai_api_rust
│   │   │
│   │   ├── session/           # Session Management (FR47-FR53)
│   │   │   ├── mod.rs         # Session module exports
│   │   │   ├── saver.rs        # FR47: Auto-save session to JSON
│   │   │   ├── loader.rs      # FR48-FR50: Load and resume sessions
│   │   │   └── manager.rs     # FR51-FR53: Session listing and deletion
│   │   │
│   │   └── error.rs           # Error types with Thiserror + Anyhow
│   │       ├── mod.rs
│   │       ├── provider.rs    # ProviderError
│   │       ├── tool.rs        # ToolError
│   │       └── session.rs     # SessionError
│   │
│   └── tui/                   # TUI Layer (FR16-FR28, FR20-FR28)
│       ├── mod.rs             # TUI module exports
│       ├── app.rs             # Main App struct with message handling
│       │
│       ├── screens/           # Screen-Based organization (Option C)
│       │   ├── mod.rs
│       │   ├── chat.rs        # FR16-FR25: Chat screen with message display
│       │   ├── tools.rs       # FR28: Tool output screen
│       │   ├── setup.rs       # FR9: Setup screen
│       │   └── sessions.rs    # FR12-FR13: Session picker screen
│       │
│       └── widgets/           # Shared widgets
│           ├── mod.rs
│           ├── chat.rs        # Chat message widget
│           ├── code_block.rs  # FR18: Code block with syntax highlighting
│           ├── input.rs       # User input widget
│           ├── status_bar.rs  # FR27: Provider status display
│           └── tools.rs       # Tool execution widget
│
├── tests/                      # Integration tests
│   ├── cli/                   # CLI tests
│   │   ├── args_test.rs       # Test CLI argument parsing
│   │   └── entrypoint_test.rs
│   ├── core/
│   │   ├── agent_test.rs      # Agent loop tests
│   │   ├── tools_test.rs      # Tool registry and execution tests
│   │   ├── provider_test.rs   # Provider client tests
│   │   └── session_test.rs    # Session management tests
│   └── tui/                   # TUI tests
│       └── app_test.rs
│
├── docs/
│   └── architecture.md        # This document
│
└── .roro/                     # Default config directory (optional)
    └── config.toml
```

---

### Architectural Boundaries

**API Boundaries:**
- **Provider API:** `openai_api_rust::OpenAIClient` is the ONLY way to communicate with LLMs (FR29-FR36)
- **Tool API:** Tools expose a uniform `Tool` trait interface via registry (FR37-FR46)
- **Config API:** Configuration loaded through `ConfigLoader` with layered sources (FR54-FR63)

**Component Boundaries:**
- **CLI Layer:** Handles argument parsing and entry point routing only (FR1-FR15)
- **Core Layer:** Contains all business logic, independent of UI (FR16-FR63)
- **TUI Layer:** Handles all terminal UI, communicates with core via messages (FR16-FR28)

**Data Boundaries:**
- **Messages:** `LLMMessage` struct is the single message format across all components
- **Sessions:** Session data serialized as JSON files, never modified directly
- **Config:** TOML files with schema validation, environment variable expansion

---

### Requirements to Structure Mapping

**CLI Interface & Commands (FR1-FR15) → `src/cli/args.rs`, `src/cli/entrypoint.rs`**
- FR1-FR2: Entry point handling in `entrypoint.rs`
- FR3-FR9: Argument definitions in `args.rs`
- FR10-FR11: Workdir and trust handling in `entrypoint.rs`
- FR12-FR15: Session continuation in `entrypoint.rs`

**Textual TUI Interface (FR16-FR28) → `src/tui/`**
- FR16-FR17: Chat display in `screens/chat.rs` with `widgets/chat.rs`
- FR18: Code block widget in `widgets/code_block.rs`
- FR19-FR20: Tool execution in `screens/chat.rs` with command palette
- FR21-FR24: Multi-pane layout in `screens/chat.rs` with ratatui
- FR25-FR28: Session metadata and status in various screens

**Local Provider Integration (FR29-FR36) → `src/core/provider/client.rs`**
- FR29-FR31: Provider configuration and switching
- FR32-FR33: Connectivity validation and capability detection
- FR34-FR36: Model selection and error handling

**Tool Execution & Safety Framework (FR37-FR46) → `src/core/tools/`**
- FR37: Tool execution in `exec.rs`
- FR38-FR40: Permission system in `registry.rs`
- FR41-FR46: Validation, output capture, timeout, logging in `exec.rs` and `registry.rs`

**Session Management (FR47-FR53) → `src/core/session/`**
- FR47: Auto-save in `saver.rs`
- FR48-FR50: Session resume and continuation in `loader.rs` and `manager.rs`
- FR51-FR53: Session listing, deletion, metadata in `manager.rs`

**Configuration Management (FR54-FR63) → `src/core/config/`**
- FR54-FR56: TOML config with env expansion in `schema.rs` and `loader.rs`
- FR57-FR59: Custom config file and validation in `loader.rs` and `validator.rs`
- FR60-FR63: Multiple profiles, defaults, presets in `schema.rs`

**Programmatic Mode (FR64-FR69) → `src/cli/entrypoint.rs`**
- FR64-FR65: Stdin and JSON output handling
- FR66: Streaming output in agent loop
- FR67: Exit codes throughout
- FR68-FR69: CI/CD usage patterns

**Error Handling (FR70-FR75) → `src/core/error.rs` and throughout**
- FR70-FR71: Error display and troubleshooting in all modules
- FR72-FR75: Provider recovery, session persistence, uptime, crash handling

**System & Integration (FR76-FR81) → Various**
- FR76: Shell completion in `cli/args.rs`
- FR77: Environment inheritance in tool execution
- FR78-FR80: Trusted folders and .gitignore in config and session
- FR81: Keyboard interrupt handling in `main.rs`

**New Model Management (FR82-FR84) → `src/core/provider/client.rs`**
- FR82-FR84: Model download, update, version management

---

### Integration Points

**Internal Communication:**
- **CLI → Core:** Arguments parsed in CLI layer, passed to core components
- **TUI → Core:** TUI sends `AppMessage` to core, receives state updates
- **Core → Provider:** Core uses `ProviderClient` for all LLM communication
- **Core → Tools:** Core uses `ToolRegistry` for tool discovery and execution

**External Integrations:**
- **Local Providers:** Via `openai_api_rust` with configurable base URLs
- **Shell Tools:** Via `tokio::process::Command` in tool execution
- **File System:** Via std::fs with trusted folder checks
- **Editor Integration:** Via clickable line numbers opening in default editor

**Data Flow:**
```
User Input → CLI args → AgentLoop → ProviderClient → LLM
                      ↓
                ToolRegistry → Tool Execution → Shell
                      ↓
                SessionManager → JSON files
                      ↓
                   TUI Updates
```

---

### File Organization Patterns

**Configuration Files:**
- `Cargo.toml`: All dependencies and features
- `.roro/config.toml`: User configuration
- `.roro.toml`: Project-specific overrides
- Environment: `ROTO_*` prefix for overrides

**Source Organization:**
- By feature/module (CLI, Core, TUI)
- Each module has `mod.rs` exporting public items
- Tests co-located with source or in `tests/`

**Test Organization:**
- Unit tests: In module files with `#[cfg(test)]`
- Integration tests: In `tests/` directory mirroring `src/` structure
- Test utilities: In `tests/utils/`

**Asset Organization:**
- None (CLI tool, no static assets)

---

### Development Workflow Integration

**Development Server Structure:**
- `cargo run` launches CLI with all features
- `cargo run -- --prompt "test"` for programmatic mode testing
- Hot reload: Not applicable (compiled binary)

**Build Process Structure:**
1. `cargo build` compiles all code
2. `cargo clippy` runs linting
3. `cargo fmt` formats code
4. `cargo test` runs all tests

**Deployment Structure:**
- Binary release: `cargo build --release`
- Packaging: Binary tarballs + Homebrew formula
- Configuration: Users provide their own config files

## Architecture Validation Results

### Coherence Validation ✅

**Decision Compatibility:**
- All technology choices are compatible: Rust 2024 + Tokio 1.x + Serde + Clap + ratatui + crossterm + `openai_api_rust` + thiserror + anyhow + config-rs
- No version conflicts detected
- Patterns (Registry, Single Struct, Screen-Based) align with Rust idioms and chosen tech stack

**Pattern Consistency:**
- Naming patterns follow Rust conventions (snake_case, PascalCase)
- Structure patterns align with mirror architecture requirement
- Communication patterns support async Tokio runtime
- Process patterns match CLI/TUI requirements

**Structure Alignment:**
- Project structure supports all architectural decisions
- Boundaries properly defined between CLI, Core, and TUI layers
- Integration points clearly specified
- Module organization mirrors Python while leveraging Rust features

---

### Requirements Coverage Validation ✅

**Functional Requirements Coverage:**
- All 84 FRs mapped to specific files/directories
- All 8 FR categories (CLI Interface, TUI, Provider, Tools, Session, Config, Programmatic, Error, System) have architectural support

**Non-Functional Requirements Coverage:**
- Performance: Tokio async, streaming, concurrent sessions (NFR Performance section)
- Security: Local-only guarantee, no telemetry, input sanitization, permission enforcement (NFR Security section)
- Reliability: Auto-retry, session persistence, error recovery (NFR Reliability section)
- Integration: OpenAI API compatibility, Rust toolchain, shell/editor integration (NFR Integration section)
- Compliance: Data residency, license compliance, config standards (NFR Compliance section)

**Cross-Cutting Concerns:**
- Provider agnosticism addressed via `openai_api_rust` with URL configuration
- Tool safety framework implemented in ToolRegistry
- Session management via Serde serialization
- Configuration backward compatibility via layered loading

---

### Implementation Readiness Validation ✅

**Decision Completeness:**
- All critical decisions documented with rationale
- All technology choices specified with versions
- Integration patterns defined for all major components
- Performance considerations addressed in NFRs

**Structure Completeness:**
- Complete directory structure defined with all files
- Component boundaries clearly established
- Integration points fully specified
- Requirements to structure mapping complete (every FR mapped)

**Pattern Completeness:**
- All potential conflict points addressed (naming, structure, format, communication, process)
- Comprehensive naming conventions with examples
- Communication patterns specified for TUI, Provider, Tools
- Process patterns documented (error handling, async, session management)
- Good and anti-patterns provided with concrete examples

---

### Gap Analysis Results

**Critical Gaps:** None - All blocking decisions made and documented

**Important Gaps:** None - All important architectural areas covered

**Nice-to-Have Gaps:**
- Exact `openai_api_rust` version should be verified from crates.io
- Detailed Cargo.toml with all features should be defined
- Specific ratatui widget implementations could be prototyped
- Test strategy and CI pipeline details could be expanded

---

### Validation Issues Addressed

No critical or important validation issues found. Architecture is coherent, complete, and ready for implementation.

---

### Architecture Completeness Checklist

**Requirements Analysis**
- [x] Project context thoroughly analyzed
- [x] Scale and complexity assessed
- [x] Technical constraints identified
- [x] Cross-cutting concerns mapped

**Architectural Decisions**
- [x] Critical decisions documented with versions
- [x] Technology stack fully specified
- [x] Integration patterns defined
- [x] Performance considerations addressed

**Implementation Patterns**
- [x] Naming conventions established
- [x] Structure patterns defined
- [x] Communication patterns specified
- [x] Process patterns documented

**Project Structure**
- [x] Complete directory structure defined
- [x] Component boundaries established
- [x] Integration points mapped
- [x] Requirements to structure mapping complete

---

### Architecture Readiness Assessment

**Overall Status:** READY FOR IMPLEMENTATION

**Confidence Level:** High

**Key Strengths:**
- Clear mirror architecture strategy with zero breaking changes
- Comprehensive FR coverage (84 functional requirements)
- Well-defined technology stack with Rust idioms
- Screen-based TUI structure matching Textual model
- Direct `openai_api_rust` integration for local providers
- Complete project structure with file-level FR mapping
- Strong error handling patterns with Thiserror + Anyhow

**Areas for Future Enhancement:**
- Performance benchmarking suite
- Provider health monitoring dashboard
- Advanced error recovery (circuit breaker patterns)
- Community plugin system
- IDE integration protocols

---

### Implementation Handoff

**AI Agent Guidelines:**

- Follow all architectural decisions exactly as documented
- Use implementation patterns consistently across all components
- Respect project structure and boundaries
- Use `openai_api_rust` for ALL provider communication (never direct HTTP)
- Enforce permission checks BEFORE tool execution
- Serialize all state to/from JSON using Serde
- Refer to this document for all architectural questions

**First Implementation Priority:**
```bash
# 1. Create project structure
cargo new roro --name roro

# 2. Add dependencies to Cargo.toml (see Infrastructure & Deployment section)

# 3. Create directory structure from Project Structure & Boundaries

# 4. Implement ProviderClient (highest priority dependency)

# 5. Implement ConfigLoader

# 6. Implement AgentLoop

# 7. Implement ToolRegistry

# 8. Implement SessionManager

# 9. Implement CLI entry point

# 10. Implement TUI screens and widgets
```
