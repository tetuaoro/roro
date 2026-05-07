---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/planning-artifacts/prd.md
  - _bmad-output/planning-artifacts/architecture.md
---

# roro - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for roro, decomposing the requirements from the PRD and Architecture requirements into implementable stories.

## Requirements Inventory

### Functional Requirements

FR1: Users can launch roro with no arguments to start interactive TUI mode
FR2: Users can provide initial prompt as positional argument to start session with pre-filled input
FR3: Users can use `--prompt` / `-p` flag to run in programmatic mode
FR4: Users can specify `--max-turns` to limit conversation turns in programmatic mode
FR5: Users can specify `--max-price` to cap cost in programmatic mode
FR6: Users can enable specific tools via `--enabled-tools` flag with glob/regex pattern support
FR7: Users can select output format via `--output` (text, json, streaming) for programmatic mode
FR8: Users can choose agent via `--agent` flag (builtin or custom)
FR9: Users can run `--setup` to configure API key interactively
FR10: Users can change directory via `--workdir` before execution
FR11: Users can trust current directory via `--trust` flag
FR12: Users can continue from latest session via `--continue` / `-c` flag
FR13: Users can resume specific session via `--resume` with optional SESSION_ID
FR14: Users can view version via `--version` / `-v` flag
FR15: Users can view help via `--help` / `-h` flag
FR16: Users can interact with chat interface in terminal
FR17: Users can view conversation history in scrollable pane
FR18: Users can see code blocks with syntax highlighting
FR19: Users can execute tools directly from TUI with permission prompts
FR20: Users can access command palette via `/` prefix
FR21: Users can split interface into multiple panes (chat, tool output, code preview)
FR22: Users can customize keyboard shortcuts
FR23: Users can navigate between panes using keyboard
FR24: Users can click on error line numbers to open in editor
FR25: Users can save conversation history
FR26: Users can view session metadata (session ID, timestamps)
FR27: Users can see provider status indicators
FR28: Users can view tool execution results in dedicated output pane
FR29: Users can configure provider base URL (Ollama, LocalAI, Llama.cpp, or custom)
FR30: System can auto-detect provider capabilities (models, context length, features)
FR31: Users can switch between configured providers
FR32: System can validate provider connectivity on startup
FR33: System can display provider-specific model lists
FR34: Users can override model selection per request
FR35: System can handle provider timeouts with automatic retry
FR36: System can display clear error messages for provider failures
FR37: Users can execute shell commands through tool system
FR38: System can request user permission before tool execution (ASK mode)
FR39: System can auto-approve specific tools (ALWAYS mode)
FR40: System can block dangerous tools entirely (NEVER mode)
FR41: System can validate tool arguments before execution
FR42: System can capture and display tool stdout/stderr
FR43: System can return tool exit codes
FR44: System can handle tool timeouts gracefully
FR45: Users can configure default permission levels per tool
FR46: System can log all tool executions for audit
FR47: System can save conversation history automatically
FR48: Users can resume previous sessions by ID
FR49: System can restore full context (messages, state) from saved sessions
FR50: System can handle session continuation from latest session
FR51: Users can list available saved sessions
FR52: Users can delete saved sessions
FR53: System can persist session metadata (timestamps, model used, tool count)
FR54: Users can configure roro via TOML configuration files
FR55: Users can override config via environment variables
FR56: System can expand environment variables in config values (`${VAR}`)
FR57: Users can specify custom config file via `--config` flag
FR58: System can validate configuration on startup
FR59: System can provide clear errors for invalid configuration
FR60: Users can configure multiple provider profiles
FR61: Users can set default provider and model
FR62: Users can configure tool permission presets
FR63: System can load config from multiple sources with proper precedence
FR64: Users can pipe input via stdin in programmatic mode
FR65: System can output responses in JSON format
FR66: System can stream responses in newline-delimited JSON
FR67: System can return appropriate exit codes (0=success, 1=error, 2=teleport error, 3=conversation limit)
FR68: Users can use programmatic mode in CI/CD pipelines
FR69: System can handle batch processing of multiple prompts
FR70: System can display clear, actionable error messages
FR71: System can provide troubleshooting suggestions for common errors
FR72: System can auto-recover from provider connection loss
FR73: System can preserve session state during error recovery
FR74: Users can trigger manual reconnection to provider
FR75: System can detect and warn about resource exhaustion
FR76: Users can generate shell completion scripts for bash/zsh/fish
FR77: System can inherit environment variables for tool execution
FR78: System can access files within trusted directories
FR79: Users can temporarily trust directories for file access
FR80: System can respect `.gitignore` and `.vibeignore` for file operations
FR81: System can handle keyboard interrupts gracefully
FR82: Users can download models from provider repositories
FR83: Users can update to newer model versions
FR84: Users can manage multiple model versions simultaneously

### NonFunctional Requirements

**Performance:**
- Response latency: First token in streaming response must appear within 500ms of request submission for 95% of queries
- Streaming throughput: Minimum 20 tokens/second sustained streaming rate for typical coding queries
- TUI rendering: Interface must maintain 60+ FPS during active conversation and tool execution
- Concurrent sessions: System must support 10+ simultaneous active sessions without performance degradation
- Startup time: CLI must be ready for user input within 2 seconds of invocation
- Tool execution: Tool commands must execute with <100ms overhead beyond native execution time

**Security:**
- Local-only guarantee: Zero network calls to external services beyond configured provider endpoints; all code and data remain on user's machine
- No telemetry: Zero analytics, tracking, or usage data collection
- Input sanitization: All tool commands must be validated and sanitized before execution to prevent prompt injection attacks
- Permission enforcement: ALWAYS/ASK/NEVER tool permissions must be strictly enforced with no bypass possible
- Environment isolation: Tool execution must inherit user's environment variables but cannot modify them
- File access: File operations limited to explicitly trusted directories; read-only by default for untrusted paths

**Reliability:**
- Provider timeouts: Automatic retry with exponential backoff (1s, 2s, 4s) for failed provider requests, maximum 3 attempts
- Session persistence: Saved sessions must be recoverable after unexpected termination (crash, kill signal)
- Error recovery: System must maintain state and allow retry after provider connection loss without data loss
- Uptime target: 99.9% availability during active development sessions (excludes provider downtime)
- Crash handling: All errors must be catchable and display user-friendly messages; no silent failures

**Integration:**
- OpenAI API compatibility: 100% compatibility with OpenAI Chat API v1 and Embeddings API v1 specifications
- Provider detection: Automatic detection and adaptation to provider-specific capabilities and limits
- Rust toolchain: Native integration with cargo, rustc, clippy, rustfmt with zero configuration required
- Shell integration: Preserve exit codes, stdout/stderr, and environment from shell commands executed through tool system
- Editor integration: Error line numbers must be clickable to open in user's default editor

**Compliance:**
- Data residency: All data (conversations, config, session history) stored locally; no cloud synchronization without explicit user action
- License compliance: All dependencies must use Rust-compatible licenses (MIT, Apache 2.0, BSD); no GPL dependencies
- Configuration standards: Config files must follow TOML v1.0 specification with JSON compatibility layer

### Additional Requirements

- Starter Template: Custom Cargo project (not a generic template) - project initialization using custom Cargo.toml should be the first implementation story
- Project Structure: Hybrid module organization mirroring Python Mistral Vibe structure exactly
- Provider Client Architecture: Direct `openai_api_rust` crate usage (no wrapper trait pattern)
- Agent Loop Architecture: Single struct with methods (simple, refactorable approach)
- Tool System Architecture: Registry pattern with trait objects for tool discovery and execution
- Configuration Management: Layered config with validation using config-rs + Serde + validation traits
- Session Management: Serde serialization to JSON files matching Python approach
- TUI Framework: ratatui + crossterm with Screen-Based widget structure (mirrors Textual's model)
- Error Handling: Thiserror + Anyhow combination for type-safe library errors and ergonomic application code
- Technology Stack: Tokio 1.x, Serde, Clap, ratatui, crossterm, openai_api_rust, thiserror, anyhow, config-rs, reqwest

### UX Design Requirements

None - This is a CLI/TUI application, UX design requirements are embedded in the functional and technical requirements.

### FR Coverage Map

FR1: Epic 2 - Users can launch roro with no arguments to start interactive TUI mode
FR2: Epic 2 - Users can provide initial prompt as positional argument to start session with pre-filled input
FR3: Epic 5 - Users can use `--prompt` / `-p` flag to run in programmatic mode
FR4: Epic 5 - Users can specify `--max-turns` to limit conversation turns in programmatic mode
FR5: Epic 5 - Users can specify `--max-price` to cap cost in programmatic mode
FR6: Epic 5 - Users can enable specific tools via `--enabled-tools` flag with glob/regex pattern support
FR7: Epic 5 - Users can select output format via `--output` (text, json, streaming) for programmatic mode
FR8: Epic 5 - Users can choose agent via `--agent` flag (builtin or custom)
FR9: Epic 1 - Users can run `--setup` to configure API key interactively
FR10: Epic 5 - Users can change directory via `--workdir` before execution
FR11: Epic 5 - Users can trust current directory via `--trust` flag
FR12: Epic 4 - Users can continue from latest session via `--continue` / `-c` flag
FR13: Epic 4 - Users can resume specific session via `--resume` with optional SESSION_ID
FR14: Epic 5 - Users can view version via `--version` / `-v` flag
FR15: Epic 5 - Users can view help via `--help` / `-h` flag
FR16: Epic 2, Epic 6 - Users can interact with chat interface in terminal
FR17: Epic 2, Epic 6 - Users can view conversation history in scrollable pane
FR18: Epic 2, Epic 6 - Users can see code blocks with syntax highlighting
FR19: Epic 3 - Users can execute tools directly from TUI with permission prompts
FR20: Epic 3 - Users can access command palette via `/` prefix
FR21: Epic 6 - Users can split interface into multiple panes (chat, tool output, code preview)
FR22: Epic 6 - Users can customize keyboard shortcuts
FR23: Epic 6 - Users can navigate between panes using keyboard
FR24: Epic 6 - Users can click on error line numbers to open in editor
FR25: Epic 6 - Users can save conversation history
FR26: Epic 6 - Users can view session metadata (session ID, timestamps)
FR27: Epic 6 - Users can see provider status indicators
FR28: Epic 6 - Users can view tool execution results in dedicated output pane
FR29: Epic 1 - Users can configure provider base URL (Ollama, LocalAI, Llama.cpp, or custom)
FR30: Epic 1 - System can auto-detect provider capabilities (models, context length, features)
FR31: Epic 1 - Users can switch between configured providers
FR32: Epic 1 - System can validate provider connectivity on startup
FR33: Epic 1 - System can display provider-specific model lists
FR34: Epic 1 - Users can override model selection per request
FR35: Epic 1, Epic 7 - System can handle provider timeouts with automatic retry
FR36: Epic 1, Epic 7 - System can display clear error messages for provider failures
FR37: Epic 3 - Users can execute shell commands through tool system
FR38: Epic 3 - System can request user permission before tool execution (ASK mode)
FR39: Epic 3 - System can auto-approve specific tools (ALWAYS mode)
FR40: Epic 3 - System can block dangerous tools entirely (NEVER mode)
FR41: Epic 3 - System can validate tool arguments before execution
FR42: Epic 3 - System can capture and display tool stdout/stderr
FR43: Epic 3 - System can return tool exit codes
FR44: Epic 3, Epic 7 - System can handle tool timeouts gracefully
FR45: Epic 3 - Users can configure default permission levels per tool
FR46: Epic 3 - System can log all tool executions for audit
FR47: Epic 2, Epic 4 - System can save conversation history automatically
FR48: Epic 4 - Users can resume previous sessions by ID
FR49: Epic 4 - System can restore full context (messages, state) from saved sessions
FR50: Epic 4 - System can handle session continuation from latest session
FR51: Epic 4 - Users can list available saved sessions
FR52: Epic 4 - Users can delete saved sessions
FR53: Epic 4 - System can persist session metadata (timestamps, model used, tool count)
FR54: Epic 1, Epic 7 - Users can configure roro via TOML configuration files
FR55: Epic 1, Epic 7 - Users can override config via environment variables
FR56: Epic 1, Epic 7 - System can expand environment variables in config values (`${VAR}`)
FR57: Epic 1, Epic 7 - Users can specify custom config file via `--config` flag
FR58: Epic 1, Epic 7 - System can validate configuration on startup
FR59: Epic 1, Epic 7 - System can provide clear errors for invalid configuration
FR60: Epic 7 - Users can configure multiple provider profiles
FR61: Epic 7 - Users can set default provider and model
FR62: Epic 7 - Users can configure tool permission presets
FR63: Epic 1, Epic 7 - System can load config from multiple sources with proper precedence
FR64: Epic 2, Epic 5 - Users can pipe input via stdin in programmatic mode
FR65: Epic 2, Epic 5 - System can output responses in JSON format
FR66: Epic 5 - System can stream responses in newline-delimited JSON
FR67: Epic 3, Epic 5, Epic 7 - System can return appropriate exit codes (0=success, 1=error, 2=teleport error, 3=conversation limit)
FR68: Epic 5 - Users can use programmatic mode in CI/CD pipelines
FR69: Epic 5 - System can handle batch processing of multiple prompts
FR70: Epic 6, Epic 7 - System can display clear, actionable error messages
FR71: Epic 6, Epic 7 - System can provide troubleshooting suggestions for common errors
FR72: Epic 1, Epic 7 - System can auto-recover from provider connection loss
FR73: Epic 4, Epic 7 - System can preserve session state during error recovery
FR74: Epic 1, Epic 7 - Users can trigger manual reconnection to provider
FR75: Epic 1, Epic 7 - System can detect and warn about resource exhaustion
FR76: Epic 5 - Users can generate shell completion scripts for bash/zsh/fish
FR77: Epic 3, Epic 7 - System can inherit environment variables for tool execution
FR78: Epic 7 - System can access files within trusted directories
FR79: Epic 7 - Users can temporarily trust directories for file access
FR80: Epic 7 - System can respect `.gitignore` and `.vibeignore` for file operations
FR81: Epic 3, Epic 7 - System can handle keyboard interrupts gracefully
FR82: Epic 1 - Users can download models from provider repositories
FR83: Epic 1 - Users can update to newer model versions
FR84: Epic 1 - Users can manage multiple model versions simultaneously

## Epic List

### Epic 1: Foundation - Local Provider Connection
Users can configure roro to connect to their local LLM providers (Ollama, LocalAI, Llama.cpp) and verify the connection works.
**FRs covered:** FR29-FR36, FR54-FR59, FR77-FR80, FR76, FR82-FR84

### Epic 2: Conversation Core (MVP)
Users can have basic chat conversations with local LLMs through a functional interface.
**FRs covered:** FR1-FR2, FR16-FR18, FR47, FR64-FR65

### Epic 3: Tool Execution Framework
Users can execute shell commands and other tools directly from the conversation with proper permission checks.
**FRs covered:** FR19-FR20, FR37-FR46, FR77, FR81

### Epic 4: Session Persistence
Users can save, resume, and continue conversations across sessions without losing context.
**FRs covered:** FR12-FR13, FR48-FR53

### Epic 5: Complete CLI Interface
Users can use all CLI commands identical to Python Mistral Vibe, including programmatic mode for scripting.
**FRs covered:** FR3-FR11, FR14-FR15, FR66-FR69

### Epic 6: Rich TUI Experience
Users have a full interactive Textual-like TUI with multi-pane layouts, command palette, and all visual features.
**FRs covered:** FR16-FR28, FR70-FR75

### Epic 7: Advanced Configuration & Admin
DevOps/team leads can configure roro for teams with shared configs, permission presets, and audit logging.
**FRs covered:** FR54-FR63, FR78-FR80

---

## Epic Details

---

### Epic 1: Foundation - Local Provider Connection

**Goal:** Users can configure roro to connect to their local LLM providers (Ollama, LocalAI, Llama.cpp) and verify the connection works.

#### Story 1.1: Initialize Rust Project Structure

As a Rust developer,
I want to initialize a new Cargo project with all required dependencies,
So that I have the foundation for building roro.

**Acceptance Criteria:**

**Given** a new Rust project is being created
**When** I run `cargo init --name roro`
**Then** a valid Cargo.toml is created
**And** all required dependencies are added: tokio (full features), serde (with derive), serde_json, clap (with derive), ratatui, crossterm, openai_api_rust, config, thiserror, anyhow, reqwest (with json feature)

**Given** the project is initialized
**When** I run `cargo check`
**Then** the project compiles without errors
**And** the module structure mirrors Python Mistral Vibe (src/, with main.rs and lib.rs)

**Git Commit:**
```bash
git add Cargo.toml src/
git commit -m "feat(roro): Initialize Rust project with dependencies"
```

#### Story 1.2: Create Configuration Module with Layered Loading

As a developer,
I want to implement a configuration system that loads settings from multiple sources with proper precedence,
So that users can configure roro via TOML files, environment variables, and CLI flags.

**Acceptance Criteria:**

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

**FRs Covered:** FR54-FR59, FR76

**Git Commit:**
```bash
git add src/config/
git commit -m "feat(config): Add layered configuration with TOML, env vars, and CLI precedence"
```

#### Story 1.3: Implement Provider Client with openai_api_rust

As a developer,
I want to create a provider client that uses the openai_api_rust crate directly,
So that roro can communicate with OpenAI API-compatible local providers.

**Acceptance Criteria:**

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

**FRs Covered:** FR29, FR34, FR77-FR80

**Git Commit:**
```bash
git add src/provider/
git commit -m "feat(provider): Add OpenAI API-compatible client using openai_api_rust"
```

#### Story 1.4: Add Provider Connectivity Validation

As a user,
I want roro to validate provider connectivity on startup,
So that I know immediately if my local provider is unavailable.

**Acceptance Criteria:**

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

**FRs Covered:** FR32, FR36

**Git Commit:**
```bash
git add src/provider/health.rs
git commit -m "feat(provider): Add connectivity validation on startup"
```

#### Story 1.5: Implement Auto-Detection of Provider Capabilities

As a user,
I want roro to auto-detect provider capabilities,
So that I can see available models and their specifications without manual configuration.

**Acceptance Criteria:**

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

**FRs Covered:** FR30, FR33

**Git Commit:**
```bash
git add src/provider/capabilities.rs
git commit -m "feat(provider): Add auto-detection of models and capabilities"
```

#### Story 1.6: Add Provider Switching Functionality

As a user,
I want to switch between configured providers,
So that I can use different LLMs for different tasks.

**Acceptance Criteria:**

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

**FRs Covered:** FR31

**Git Commit:**
```bash
git add src/provider/switch.rs src/cli/provider_args.rs
git commit -m "feat(provider): Add provider switching via CLI and config"
```

#### Story 1.7: Implement Model Download and Version Management

As a user,
I want to download and manage model versions from provider repositories,
So that I can use the latest models without manual downloads.

**Acceptance Criteria:**

**Given** Ollama is configured as provider
**When** I run `roro --download llava:13b`
**Then** the model is downloaded via Ollama's API
**And** download progress is displayed
**And** a success message confirms the model is ready

**Given** multiple versions of a model exist
**When** I list available models
**Then** all versions are displayed (e.g., mistral:7b, mistral:7b-instruct)
**And** I can select which version to use

**Given** a model update is available
**When** I run `roro --update-models`
**Then** newer versions are downloaded
**And** old versions are preserved unless explicitly removed

**Given** a model download fails
**When** the operation completes
**Then** a clear error message is displayed
**And** partial downloads are cleaned up

**FRs Covered:** FR82-FR84

**Git Commit:**
```bash
git add src/provider/models.rs
git commit -m "feat(provider): Add model download and version management"
```

#### Story 1.8: Add Interactive Setup Command

As a new user,
I want to run an interactive setup to configure my provider,
So that I can quickly get started with roro.

**Acceptance Criteria:**

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
**And** helpful error messages guide me (e.g., "Is Ollama running?")

**Given** the setup completes successfully
**When** the process finishes
**Then** a confirmation message is displayed
**And** I can immediately start using roro

**FRs Covered:** FR9

**Git Commit:**
```bash
git add src/cli/setup.rs
git commit -m "feat(cli): Add interactive --setup command for provider configuration"
```

#### Story 1.9: Implement Provider Timeout Handling with Retry

As a user,
I want automatic retry with exponential backoff for failed provider requests,
So that transient network issues don't interrupt my workflow.

**Acceptance Criteria:**

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

**FRs Covered:** FR35

**Git Commit:**
```bash
git add src/provider/retry.rs
git commit -m "feat(provider): Add exponential backoff retry for provider timeouts"
```

---

### Epic 2: Conversation Core (MVP)

**Goal:** Users can have basic chat conversations with local LLMs through a functional interface.

#### Story 2.1: Implement Basic CLI Entry Points

As a user,
I want to launch roro with no arguments or with an initial prompt,
So that I can start interactive conversations or pre-fill input.

**Acceptance Criteria:**

**Given** I run `roro` with no arguments
**When** the application starts
**Then** interactive TUI mode is launched
**And** I can type messages to the LLM

**Given** I run `roro "Explain Rust ownership"`
**When** the application starts
**Then** the TUI launches with "Explain Rust ownership" pre-filled in the input
**And** the prompt is visible but not yet sent

**Given** I run `roro` in a terminal without TTY
**When** the application starts
**Then** it detects non-interactive mode
**And** waits for stdin input

**FRs Covered:** FR1-FR2

**Git Commit:**
```bash
git add src/cli/mod.rs src/main.rs
git commit -m "feat(cli): Add basic entry points for interactive and pre-filled prompt modes"
```

#### Story 2.2: Implement Interactive Chat Interface

As a user,
I want to interact with a chat interface in my terminal,
So that I can have conversations with the LLM.

**Acceptance Criteria:**

**Given** roro is running in TUI mode
**When** I type a message and press Enter
**Then** the message is sent to the provider
**And** the response is displayed in the chat pane
**And** the input is cleared for the next message

**Given** the LLM is streaming a response
**When** tokens are received
**Then** they appear in real-time in the chat pane
**And** the cursor remains at the input line

**Given** I press Ctrl+C during LLM response
**When** the interrupt is detected
**Then** the response is cancelled gracefully
**And** I can enter a new prompt immediately

**Given** I navigate with arrow keys
**When** I press Up arrow
**Then** my previous prompt is displayed in the input
**And** I can edit and resend it

**FRs Covered:** FR16

**Git Commit:**
```bash
git add src/tui/chat.rs
git commit -m "feat(tui): Add interactive chat interface with streaming responses"
```

#### Story 2.3: Add Scrollable Conversation History Pane

As a user,
I want to view conversation history in a scrollable pane,
So that I can review previous messages without losing context.

**Acceptance Criteria:**

**Given** I have an active conversation with multiple messages
**When** the chat pane is displayed
**Then** all messages are visible in chronological order
**And** the most recent message is at the bottom

**Given** the conversation exceeds the terminal height
**When** I scroll up
**Then** older messages come into view
**And** I can scroll down to return to the bottom

**Given** I use the mouse wheel
**When** I scroll
**Then** the chat pane scrolls smoothly
**And** the scroll position is preserved

**Given** new messages arrive
**When** they are added to the chat
**Then** the pane auto-scrolls to the bottom if I was already at the bottom
**And** the scroll position is preserved if I was scrolled up

**FRs Covered:** FR17

**Git Commit:**
```bash
git add src/tui/history.rs
git commit -m "feat(tui): Add scrollable conversation history pane"
```

#### Story 2.4: Add Syntax Highlighting for Code Blocks

As a user,
I want to see code blocks with syntax highlighting,
So that I can easily read and understand code in LLM responses.

**Acceptance Criteria:**

**Given** an LLM response contains a code block with language specifier (e.g., ```rust)
**When** the message is rendered
**Then** the code is displayed with syntax highlighting
**And** the language is shown in the code block header

**Given** an LLM response contains a code block without language specifier
**When** the message is rendered
**Then** the code is displayed with monospace font
**And** default text color is used

**Given** the terminal supports 24-bit color
**When** code is displayed
**Then** full color syntax highlighting is used
**And** colors match the terminal theme

**Given** the terminal only supports 256 colors
**When** code is displayed
**Then** appropriate color mapping is used
**And** code remains readable

**Given** unsupported language is specified
**When** the message is rendered
**Then** the code is displayed without errors
**And** monospace formatting is still applied

**FRs Covered:** FR18

**Git Commit:**
```bash
git add src/tui/syntax.rs
git commit -m "feat(tui): Add syntax highlighting for code blocks using syntect"
```

#### Story 2.5: Auto-Save Conversation History

As a user,
I want my conversations to be saved automatically,
So that I can resume later without losing context.

**Acceptance Criteria:**

**Given** I have an active conversation
**When** I send a message
**Then** the conversation (messages, timestamps, metadata) is saved to a JSON file
**And** the file is stored in ~/.vibe/sessions/

**Given** I send multiple messages in a session
**When** each message is sent
**Then** the session file is updated atomically
**And** no data is lost if roro crashes

**Given** a session file already exists
**When** I resume the conversation
**Then** all previous messages are loaded
**And** new messages are appended to the existing file

**FRs Covered:** FR47

**Git Commit:**
```bash
git add src/session/persistence.rs
git commit -m "feat(session): Add automatic conversation history saving"
```

#### Story 2.6: Support stdin Piping for Programmatic Mode

As a developer,
I want to pipe input via stdin in programmatic mode,
So that I can use roro in shell scripts and pipelines.

**Acceptance Criteria:**

**Given** I run `echo "Hello" | roro -p`
**When** the command executes
**Then** "Hello" is used as the prompt
**And** the response is printed to stdout

**Given** I run `cat prompts.txt | roro -p --batch`
**When** the command executes
**Then** each line in prompts.txt is processed as a separate prompt
**And** responses are printed sequentially

**Given** stdin is empty
**When** roro starts in programmatic mode
**Then** it waits for input
**And** processes it when received

**FRs Covered:** FR64

**Git Commit:**
```bash
git add src/cli/stdin.rs
git commit -m "feat(cli): Add stdin piping support for programmatic mode"
```

#### Story 2.7: Output Responses in JSON Format

As a developer,
I want to output responses in JSON format,
So that I can parse roro output programmatically.

**Acceptance Criteria:**

**Given** I run `roro -p "Hello" --output json`
**When** the response is received
**Then** the full response is output as a valid JSON object
**And** the JSON includes: content, model, usage, finish_reason

**Given** I run `roro -p "Hello" --output json --stream`
**When** the response streams
**Then** each token/chunk is output as a separate JSON line (NDJSON)
**And** each line is a valid JSON object

**Given** the output is JSON
**When** an error occurs
**Then** the error is output as a JSON object with error and message fields
**And** exit code is non-zero

**FRs Covered:** FR65

**Git Commit:**
```bash
git add src/cli/output.rs
git commit -m "feat(cli): Add JSON and streaming JSON output formats"
```

---

### Epic 3: Tool Execution Framework

**Goal:** Users can execute shell commands and other tools directly from the conversation with proper permission checks.

#### Story 3.1: Create Tool Registry with Permission System

As a developer,
I want a tool registry that enforces permissions,
So that tools can be safely executed based on configuration.

**Acceptance Criteria:**

**Given** a tool is registered with ALWAYS permission
**When** the tool is invoked
**Then** it executes immediately without user prompt

**Given** a tool is registered with ASK permission
**When** the tool is invoked
**Then** a permission prompt is displayed in the TUI
**And** the tool executes only after user approval

**Given** a tool is registered with NEVER permission
**When** the tool is invoked
**Then** execution is blocked
**And** a "Tool blocked by permission" message is displayed

**Given** a tool is not registered
**When** it is invoked
**Then** execution is blocked
**And** a "Tool not found" error is displayed

**FRs Covered:** FR38-FR40, FR45

**Git Commit:**
```bash
git add src/tools/registry.rs src/tools/permissions.rs
git commit -m "feat(tools): Add tool registry with ALWAYS/ASK/NEVER permission system"
```

#### Story 3.2: Implement Shell Command Execution Tool

As a user,
I want to execute shell commands directly from the conversation,
So that I can use roro to perform system tasks.

**Acceptance Criteria:**

**Given** I type a request to run a shell command
**When** the LLM identifies it as a tool call
**Then** the command is extracted and validated
**And** if permission is ASK, a prompt is displayed

**Given** I approve a shell command
**When** it executes
**Then** stdout and stderr are captured in real-time
**And** the output is displayed in the tool output pane
**And** the exit code is returned

**Given** a shell command times out
**When** the timeout is reached
**Then** the command is killed
**And** a timeout error is displayed
**And** partial output is still shown

**FRs Covered:** FR37, FR39, FR41-FR44

**Git Commit:**
```bash
git add src/tools/shell.rs
git commit -m "feat(tools): Add shell command execution with output capture"
```

#### Story 3.3: Add Command Palette Access via `/` Prefix

As a user,
I want to access a command palette via / prefix,
So that I can use special commands without natural language.

**Acceptance Criteria:**

**Given** I type `/` in the input
**When** I continue typing
**Then** a command palette overlay appears
**And** available commands are filtered as I type

**Given** I type `/help`
**When** I press Enter
**Then** a help message lists all available commands
**And** their descriptions

**Given** I type `/clear`
**When** I press Enter
**Then** the conversation history is cleared
**And** a confirmation prompt is shown

**Given** I type an unknown command
**When** I press Enter
**Then** an "Unknown command" error is displayed
**And** suggestions for similar commands are shown

**FRs Covered:** FR20

**Git Commit:**
```bash
git add src/tui/command_palette.rs
git commit -m "feat(tui): Add command palette with / prefix"
```

#### Story 3.4: Validate Tool Arguments Before Execution

As a security-conscious user,
I want tool arguments to be validated before execution,
So that I am protected from prompt injection attacks.

**Acceptance Criteria:**

**Given** a tool is invoked with arguments
**When** validation runs
**Then** arguments are checked against a blocklist
**And** dangerous patterns are rejected (e.g., `rm -rf`, `;`, `|`, `&&`, `>`, backticks)

**Given** a tool argument contains `; rm -rf /`
**When** validation runs
**Then** execution is blocked
**And** a "Dangerous command detected" error is displayed

**Given** a tool argument is valid
**When** validation runs
**Then** execution proceeds normally

**FRs Covered:** FR41

**Git Commit:**
```bash
git add src/tools/validation.rs
git commit -m "feat(tools): Add argument validation to prevent injection attacks"
```

#### Story 3.5: Log All Tool Executions for Audit

As an administrator,
I want all tool executions to be logged,
So that I can audit what actions were performed.

**Acceptance Criteria:**

**Given** a tool is executed
**When** the execution completes
**Then** an audit log entry is created in ~/.vibe/audit.log
**And** the entry includes: timestamp, tool name, arguments, exit code, duration

**Given** I want to view audit logs
**When** I check the log file
**Then** all tool executions are listed in chronological order

**Given** a tool execution fails
**When** the execution completes
**Then** the error is logged
**And** the failure reason is included

**FRs Covered:** FR46

**Git Commit:**
```bash
git add src/tools/audit.rs
git commit -m "feat(tools): Add audit logging for all tool executions"
```

#### Story 3.6: Handle Keyboard Interrupts Gracefully

As a user,
I want to handle Ctrl+C gracefully during tool execution,
So that I can cancel long-running operations.

**Acceptance Criteria:**

**Given** a tool is executing
**When** I press Ctrl+C
**Then** the tool process is terminated
**And** a "Cancelled" message is displayed
**And** I can enter a new prompt

**Given** multiple tools are executing
**When** I press Ctrl+C
**Then** all pending tool executions are cancelled
**And** the conversation state is preserved

**FRs Covered:** FR81

**Git Commit:**
```bash
git add src/tools/interrupt.rs
git commit -m "feat(tools): Add graceful handling of keyboard interrupts"
```

---

### Epic 4: Session Persistence

**Goal:** Users can save, resume, and continue conversations across sessions without losing context.

#### Story 4.1: Implement Session Save and Resume

As a user,
I want to save and resume conversations,
So that I can continue my work across sessions.

**Acceptance Criteria:**

**Given** I have an active conversation
**When** I use `/save` command or the session ends
**Then** the session is saved to ~/.vibe/sessions/{uuid}.json
**And** the session ID is returned

**Given** I run `roro --resume {uuid}`
**When** the command executes
**Then** the specified session is loaded
**And** all messages, metadata, and state are restored

**Given** I run `roro --continue`
**When** the command executes
**Then** the most recent session is loaded
**And** I can continue the conversation

**FRs Covered:** FR12-FR13, FR48-FR50

**Git Commit:**
```bash
git add src/session/management.rs
git commit -m "feat(session): Add session save, resume, and continue functionality"
```

#### Story 4.2: List and Delete Saved Sessions

As a user,
I want to list and delete saved sessions,
So that I can manage my conversation history.

**Acceptance Criteria:**

**Given** I run `roro --list-sessions`
**When** the command executes
**Then** all saved sessions are displayed in a table
**And** each entry shows: session ID, timestamp, model used, message count

**Given** I run `roro --delete-session {uuid}`
**When** the command executes
**Then** the specified session is deleted
**And** a confirmation message is displayed

**Given** I run `roro --delete-session {uuid}` with invalid UUID
**When** the command executes
**Then** an error is displayed
**And** the list of valid session IDs is shown

**FRs Covered:** FR51-FR52

**Git Commit:**
```bash
git add src/session/listing.rs
git commit -m "feat(session): Add session listing and deletion commands"
```

#### Story 4.3: Persist Session Metadata

As a user,
I want session metadata to be persisted,
So that I can track details about my conversations.

**Acceptance Criteria:**

**Given** a session is saved
**When** I inspect the session file
**Then** it contains: session ID, creation timestamp, last update timestamp, model used, tool execution count, token count

**Given** I resume a session
**When** the session loads
**Then** the metadata is displayed in the TUI status bar
**And** I can see when the session was created and last updated

**FRs Covered:** FR53

**Git Commit:**
```bash
git add src/session/metadata.rs
git commit -m "feat(session): Add session metadata persistence"
```

#### Story 4.4: Preserve Session State During Error Recovery

As a user,
I want session state to be preserved during error recovery,
So that I don't lose my conversation if something goes wrong.

**Acceptance Criteria:**

**Given** a session is active
**When** a provider connection error occurs
**Then** the current conversation state is saved to a temporary file
**And** I can reconnect and continue

**Given** roro crashes unexpectedly
**When** I restart roro with `--continue`
**Then** I can resume the session from before the crash
**And** all messages are intact

**FRs Covered:** FR73

**Git Commit:**
```bash
git add src/session/recovery.rs
git commit -m "feat(session): Add session state preservation for error recovery"
```

---

### Epic 5: Complete CLI Interface

**Goal:** Users can use all CLI commands identical to Python Mistral Vibe, including programmatic mode for scripting.

#### Story 5.1: Implement All CLI Flags and Arguments

As a developer,
I want all CLI flags from Python Mistral Vibe to be available,
So that scripts using roro work without modification.

**Acceptance Criteria:**

**Given** I run `roro -p "Hello" --max-turns 5 --max-price 0.10 --enabled-tools "bash:read" --output text --agent default --workdir /tmp --trust`
**When** the command executes
**Then** all flags are parsed correctly
**And** the conversation respects all limits and settings

**Given** I run `roro --version`
**When** the command executes
**Then** the version is displayed (e.g., "roro 0.1.0")
**And** exit code is 0

**Given** I run `roro --help`
**When** the command executes
**Then** full help text is displayed with all flags, descriptions, and examples

**FRs Covered:** FR3-FR11, FR14-FR15

**Git Commit:**
```bash
git add src/cli/args.rs
git commit -m "feat(cli): Add all CLI flags matching Python Mistral Vibe"
```

#### Story 5.2: Add Shell Completion Scripts

As a developer,
I want shell completion scripts for bash, zsh, and fish,
So that I can use tab completion for roro commands.

**Acceptance Criteria:**

**Given** I run `roro --generate-completion bash`
**When** the command executes
**Then** a bash completion script is output to stdout
**And** instructions for installation are displayed

**Given** I run `roro --generate-completion zsh`
**When** the command executes
**Then** a zsh completion script is generated

**Given** I run `roro --generate-completion fish`
**When** the command executes
**Then** a fish completion script is generated

**FRs Covered:** FR76

**Git Commit:**
```bash
git add src/cli/completion.rs
git commit -m "feat(cli): Add shell completion scripts for bash, zsh, fish"
```

#### Story 5.3: Implement Appropriate Exit Codes

As a developer,
I want appropriate exit codes for different scenarios,
So that I can handle errors in scripts.

**Acceptance Criteria:**

**Given** a successful conversation
**When** roro completes
**Then** exit code 0 is returned

**Given** a general error occurs (config, file not found)
**When** roro fails
**Then** exit code 1 is returned

**Given** a tool execution error occurs
**When** roro fails
**Then** exit code 2 is returned

**Given** a conversation limit is reached (max-turns, max-price)
**When** roro stops
**Then** exit code 3 is returned

**FRs Covered:** FR67

**Git Commit:**
```bash
git add src/cli/exit_codes.rs
git commit -m "feat(cli): Add appropriate exit codes (0, 1, 2, 3)"
```

#### Story 5.4: Support CI/CD Pipeline Usage

As a developer,
I want to use roro in CI/CD pipelines,
So that I can automate tasks with LLM assistance.

**Acceptance Criteria:**

**Given** I run roro in a CI environment (no TTY)
**When** the command executes
**Then** it automatically uses non-interactive mode
**And** reads from stdin if provided
**And** outputs to stdout

**Given** I run roro in CI with `--batch`
**When** multiple prompts are provided via stdin
**Then** all prompts are processed sequentially
**And** results are output to stdout

**Given** roro is used in a GitHub Actions workflow
**When** the workflow runs
**Then** roro executes successfully
**And** respects resource limits
**And** returns appropriate exit codes

**FRs Covered:** FR68

**Git Commit:**
```bash
git add src/cli/cicd.rs
git commit -m "feat(cli): Add CI/CD pipeline support for headless execution"
```

#### Story 5.5: Handle Batch Processing of Multiple Prompts

As a developer,
I want to process multiple prompts in batch mode,
So that I can automate bulk operations.

**Acceptance Criteria:**

**Given** I have a file with prompts (one per line)
**When** I run `cat prompts.txt | roro -p --batch`
**Then** each prompt is processed in sequence
**And** responses are output in order
**And** each response is separated by `---`

**Given** I run `roro -p "Prompt 1" -p "Prompt 2"`
**When** the command executes
**Then** both prompts are processed sequentially
**And** responses are output with clear separation

**Given** a prompt in batch mode fails
**When** processing continues
**Then** the error is logged to stderr
**And** processing continues with the next prompt

**FRs Covered:** FR69

**Git Commit:**
```bash
git add src/cli/batch.rs
git commit -m "feat(cli): Add batch processing for multiple prompts"
```

#### Story 5.6: Stream Responses in Newline-Delimited JSON

As a developer,
I want streaming responses in newline-delimited JSON format,
So that I can process responses in real-time.

**Acceptance Criteria:**

**Given** I run `roro -p "Hello" --output streaming`
**When** the response streams
**Then** each token/chunk is output as a separate line
**And** each line contains the raw token text

**Given** I run `roro -p "Hello" --output streaming --json`
**When** the response streams
**Then** each line is a valid JSON object
**And** each JSON object contains: content, model, finish_reason fields

**Given** the stream is interrupted
**When** Ctrl+C is pressed
**Then** the stream stops gracefully
**And** partial output is preserved

**FRs Covered:** FR66

**Git Commit:**
```bash
git add src/cli/streaming.rs
git commit -m "feat(cli): Add newline-delimited JSON streaming output"
```

---

### Epic 6: Rich TUI Experience

**Goal:** Users have a full interactive Textual-like TUI with multi-pane layouts, command palette, and all visual features.

#### Story 6.1: Implement Multi-Pane Layout

As a user,
I want a multi-pane layout for the TUI,
So that I can see chat, tool output, and code preview simultaneously.

**Acceptance Criteria:**

**Given** roro TUI is running
**When** the interface renders
**Then** at least 3 panes are visible: chat history (top), input (bottom), tool output (right sidebar)

**Given** I resize the terminal
**When** the resize event occurs
**Then** all panes resize proportionally
**And** the layout remains usable

**Given** a code block is detected in the response
**When** the message renders
**Then** a code preview pane opens automatically
**And** it can be dismissed with Esc

**FRs Covered:** FR21

**Git Commit:**
```bash
git add src/tui/layout.rs
git commit -m "feat(tui): Add multi-pane layout (chat, input, tool output, code preview)"
```

#### Story 6.2: Add Customizable Keyboard Shortcuts

As a user,
I want to customize keyboard shortcuts,
So that I can optimize my workflow.

**Acceptance Criteria:**

**Given** I configure shortcuts in config.toml
**When** I press the configured key combination
**Then** the configured action is triggered

**Given** I press `?` in the TUI
**When** the key is pressed
**Then** a help overlay shows all available shortcuts
**And** their current bindings

**Given** a shortcut conflicts with another
**When** config is loaded
**Then** a warning is displayed
**And** the last configured shortcut takes precedence

**FRs Covered:** FR22

**Git Commit:**
```bash
git add src/tui/keybindings.rs
git commit -m "feat(tui): Add customizable keyboard shortcuts"
```

#### Story 6.3: Navigate Between Panes Using Keyboard

As a user,
I want to navigate between panes using keyboard,
So that I can efficiently use the TUI.

**Acceptance Criteria:**

**Given** multiple panes are visible
**When** I press Tab
**Then** focus cycles forward through all panes
**And** the focused pane is highlighted with a border

**Given** multiple panes are visible
**When** I press Shift+Tab
**Then** focus cycles backward through all panes

**Given** I press Ctrl+ArrowKey
**When** the key is pressed
**Then** focus moves to the pane in that direction

**FRs Covered:** FR23

**Git Commit:**
```bash
git add src/tui/navigation.rs
git commit -m "feat(tui): Add keyboard navigation between panes"
```

#### Story 6.4: Make Error Line Numbers Clickable

As a user,
I want to click on error line numbers to open them in my editor,
So that I can quickly fix issues.

**Acceptance Criteria:**

**Given** an LLM response contains error output with line numbers
**When** the response is rendered
**Then** line numbers are detected (e.g., `error at line 42`)
**And** they are displayed as clickable hyperlinks

**Given** I click on a line number link
**When** the click is detected
**Then** the editor specified in $EDITOR opens
**And** the cursor is placed at the specified line
**And** the file path is extracted from context

**Given** the file doesn't exist
**When** I click the line number
**Then** an error toast is displayed: "File not found: {path}"

**FRs Covered:** FR24

**Git Commit:**
```bash
git add src/tui/clickable.rs
git commit -m "feat(tui): Add clickable error line numbers for editor integration"
```

#### Story 6.5: Display Session Metadata in TUI

As a user,
I want to see session metadata in the TUI,
So that I can track conversation details.

**Acceptance Criteria:**

**Given** a session is active
**When** the TUI is displayed
**Then** session metadata is shown in a status bar at the bottom
**And** it includes: session ID (shortened), model name, token count, tool count

**Given** I hover over the session ID
**When** the hover event occurs
**Then** a tooltip shows the full session ID and creation timestamp

**FRs Covered:** FR26

**Git Commit:**
```bash
git add src/tui/metadata_display.rs
git commit -m "feat(tui): Add session metadata display in status bar"
```

#### Story 6.6: Show Provider Status Indicators

As a user,
I want to see provider status indicators,
So that I know the health of my connection.

**Acceptance Criteria:**

**Given** roro is connected to a provider
**When** the TUI is displayed
**Then** a status indicator in the top-right shows: "Connected: {provider} ({model})"

**Given** the provider connection is lost
**When** the disconnection is detected
**Then** the status indicator changes to: "Disconnected"
**And** it uses red color

**Given** a connection is being re-established
**When** reconnection is in progress
**Then** the status shows: "Reconnecting..." with a spinning animation

**FRs Covered:** FR27

**Git Commit:**
```bash
git add src/tui/provider_status.rs
git commit -m "feat(tui): Add provider status indicators with connection state"
```

#### Story 6.7: View Tool Execution Results in Dedicated Pane

As a user,
I want to view tool execution results in a dedicated pane,
So that I can see output separately from chat.

**Acceptance Criteria:**

**Given** a tool is executed
**When** the output is generated
**Then** it appears in the tool output pane (right sidebar)
**And** stdout is displayed in green
**And** stderr is displayed in red

**Given** multiple tools are executed
**When** output is generated
**Then** each tool's output is separated by a horizontal rule
**And** the tool name and execution time are shown as headers

**Given** a tool execution is in progress
**When** output is streaming
**Then** it appears in real-time in the tool pane
**And** I can watch the progress

**FRs Covered:** FR28

**Git Commit:**
```bash
git add src/tui/tool_output.rs
git commit -m "feat(tui): Add dedicated tool execution results pane with colored output"
```

#### Story 6.8: Display Clear, Actionable Error Messages

As a user,
I want clear, actionable error messages,
So that I can troubleshoot issues easily.

**Acceptance Criteria:**

**Given** an error occurs
**When** it is displayed
**Then** the message clearly states what went wrong
**And** it suggests how to fix it
**And** it is displayed in a visible error box

**Given** a provider connection error occurs
**When** it is displayed
**Then** the message includes: error type, provider URL
**And** troubleshooting steps are shown: "Check if {provider} is running at {url}"

**Given** a configuration error occurs
**When** it is displayed
**Then** the message shows the invalid config key
**And** it shows the expected format and an example

**FRs Covered:** FR70

**Git Commit:**
```bash
git add src/tui/error_display.rs
git commit -m "feat(tui): Add clear, actionable error messages with troubleshooting"
```

#### Story 6.9: Provide Troubleshooting Suggestions

As a user,
I want troubleshooting suggestions for common errors,
So that I can resolve issues without external help.

**Acceptance Criteria:**

**Given** a common error occurs (provider not running, invalid API key, timeout)
**When** the error is displayed
**Then** a "Troubleshooting" collapsible section is shown
**And** it includes step-by-step solutions

**Given** a provider timeout occurs
**When** the error is displayed
**Then** suggestions include: "Check your network connection", "Increase timeout in config", "Verify provider URL is correct"

**Given** a tool permission is denied
**When** the error is displayed
**Then** instructions are shown: "To enable this tool, add to config.toml: [tools.{name}] permission = 'ALWAYS'"

**FRs Covered:** FR71

**Git Commit:**
```bash
git add src/tui/troubleshooting.rs
git commit -m "feat(tui): Add troubleshooting suggestions for common errors"
```

#### Story 6.10: Auto-Recover from Provider Connection Loss

As a user,
I want auto-recovery from provider connection loss,
So that my session isn't interrupted by temporary issues.

**Acceptance Criteria:**

**Given** a provider connection is lost during a request
**When** the error is detected
**Then** an automatic reconnection is attempted
**And** the request is retried up to 3 times with exponential backoff (1s, 2s, 4s)

**Given** reconnection succeeds
**When** the request retries
**Then** the conversation continues seamlessly
**And** a notification toast is shown: "Reconnected to {provider}"

**Given** reconnection fails after all attempts
**When** the final attempt fails
**Then** a clear error is displayed
**And** the session state is preserved for manual retry with `/retry`

**FRs Covered:** FR72

**Git Commit:**
```bash
git add src/provider/recovery.rs
git commit -m "feat(provider): Add auto-recovery from connection loss with exponential backoff"
```

#### Story 6.11: Detect and Warn About Resource Exhaustion

As a user,
I want to be warned about resource exhaustion,
So that I can prevent crashes and data loss.

**Acceptance Criteria:**

**Given** system memory usage exceeds 90%
**When** a new request is made
**Then** a warning is displayed: "System memory low ({%} used). Save your work."
**And** the request is not sent

**Given** the provider's context window would be exceeded by the next message
**When** a new message is about to be sent
**Then** a warning is displayed: "Context window would be exceeded. Consider starting a new session."
**And** suggestions are shown: "/clear to clear history, /new to start fresh"

**Given** disk space is below 100MB free
**When** a session save is attempted
**Then** a warning is displayed: "Low disk space. Consider deleting old sessions."
**And** the oldest sessions are suggested: "Run `roro --list-sessions` to see old sessions"

**FRs Covered:** FR75

**Git Commit:**
```bash
git add src/system/resource_monitor.rs
git commit -m "feat(system): Add resource exhaustion detection and warnings"
```

---

### Epic 7: Advanced Configuration & Admin

**Goal:** DevOps/team leads can configure roro for teams with shared configs, permission presets, and audit logging.

#### Story 7.1: Create TOML Configuration File Structure

As a developer,
I want to configure roro via TOML configuration files,
So that I can customize behavior without code changes.

**Acceptance Criteria:**

**Given** a config file at ~/.vibe/config.toml
**When** roro starts
**Then** the file is loaded and parsed
**And** settings are applied

**Given** the config file contains:
```toml
[provider]
base_url = "http://localhost:11434"
model = "mistral:7b"

[tools.bash]
permission = "ASK"
timeout = 30
```
**When** roro starts
**Then** the provider is configured to use Ollama at localhost:11434
**And** bash tool requires user confirmation
**And** bash tool has a 30 second timeout

**FRs Covered:** FR54, FR61

**Git Commit:**
```bash
git add src/config/mod.rs src/config/schema.rs
git commit -m "feat(config): Add TOML configuration file structure and parsing"
```

#### Story 7.2: Support Environment Variable Overrides

As a developer,
I want to override config via environment variables,
So that I can customize behavior per environment.

**Acceptance Criteria:**

**Given** I set `VIBE_PROVIDER_BASE_URL=http://localhost:8080`
**When** roro starts
**Then** the provider base URL is overridden to localhost:8080
**And** it takes precedence over config.toml

**Given** I set `VIBE_MODEL=llama2:13b`
**When** roro starts
**Then** the model is overridden to llama2:13b

**Given** environment variables use snake_case
**When** they are processed
**Then** they map to config keys (e.g., VIBE_PROVIDER_BASE_URL -> provider.base_url)

**FRs Covered:** FR55

**Git Commit:**
```bash
git add src/config/env_vars.rs
git commit -m "feat(config): Add environment variable overrides with snake_case mapping"
```

#### Story 7.3: Expand Environment Variables in Config Values

As a user,
I want to use `${VAR}` syntax in config values,
So that I can reference environment variables and home directory.

**Acceptance Criteria:**

**Given** config.toml contains: `base_url = "${OLLAMA_HOST}:11434"`
**And** OLLAMA_HOST is set to "localhost"
**When** config is loaded
**Then** base_url resolves to "localhost:11434"

**Given** config.toml contains: `session_dir = "${HOME}/.vibe/sessions"`
**When** config is loaded
**Then** session_dir resolves to the actual home directory path

**Given** a variable is not set
**When** config is loaded
**Then** the original ${VAR} string is preserved
**And** a warning is logged

**FRs Covered:** FR56

**Git Commit:**
```bash
git add src/config/expansion.rs
git commit -m "feat(config): Add environment variable expansion in config values"
```

#### Story 7.4: Support Custom Config File via --config Flag

As a user,
I want to specify a custom config file,
So that I can use different configurations for different projects.

**Acceptance Criteria:**

**Given** I run `roro --config custom.toml`
**When** the command executes
**Then** custom.toml is loaded instead of the default config
**And** all settings from custom.toml are applied

**Given** I run `roro -c /path/to/config.toml`
**When** the command executes
**Then** the specified config file is loaded

**Given** the custom config file doesn't exist
**When** the command executes
**Then** an error is displayed
**And** exit code 1 is returned

**FRs Covered:** FR57

**Git Commit:**
```bash
git add src/cli/config_flag.rs
git commit -m "feat(cli): Add --config flag for custom config file path"
```

#### Story 7.5: Validate Configuration on Startup

As a user,
I want configuration to be validated on startup,
So that I catch errors early.

**Acceptance Criteria:**

**Given** config.toml contains an invalid value
**When** roro starts
**Then** a clear error message is displayed
**And** the invalid field and value are shown
**And** exit code 1 is returned

**Given** a required field is missing
**When** roro starts
**Then** an error is displayed showing the missing field
**And** the expected format is shown

**Given** config is valid
**When** roro starts
**Then** no validation errors are shown
**And** roro starts normally

**FRs Covered:** FR58-FR59

**Git Commit:**
```bash
git add src/config/validation.rs
git commit -m "feat(config): Add configuration validation on startup"
```

#### Story 7.6: Configure Multiple Provider Profiles

As a user,
I want to configure multiple provider profiles,
So that I can switch between different providers easily.

**Acceptance Criteria:**

**Given** config.toml contains:
```toml
[provider.ollama]
base_url = "http://localhost:11434"
model = "mistral:7b"

[provider.localai]
base_url = "http://localhost:8080"
model = "gpt-3.5-turbo"
```
**When** I run `roro --provider ollama`
**Then** the Ollama provider is used

**Given** I run `roro --provider localai`
**When** the command executes
**Then** the LocalAI provider is used

**Given** I set `default_provider = "ollama"` in config
**When** I run roro without --provider flag
**Then** the Ollama provider is used automatically

**FRs Covered:** FR60-FR61

**Git Commit:**
```bash
git add src/config/providers.rs
git commit -m "feat(config): Add multiple provider profile support"
```

#### Story 7.7: Configure Tool Permission Presets

As an administrator,
I want to configure tool permission presets,
So that I can set default permissions for different tool categories.

**Acceptance Criteria:**

**Given** config.toml contains:
```toml
[tool_permissions]
default = "ASK"
read_commands = "ALWAYS"
write_commands = "ASK"
dangerous_commands = "NEVER"
```
**When** a read command (cat, ls) is invoked
**Then** it executes without prompt

**When** a write command (echo >, mv) is invoked
**Then** a permission prompt is shown

**When** a dangerous command (rm, dd) is invoked
**Then** execution is blocked

**FRs Covered:** FR62

**Git Commit:**
```bash
git add src/config/tool_permissions.rs
git commit -m "feat(config): Add tool permission preset configuration"
```

#### Story 7.8: Load Config from Multiple Sources with Precedence

As a user,
I want config loaded from multiple sources with proper precedence,
So that I can override settings at different levels.

**Acceptance Criteria:**

**Given** I have:
- System config at /etc/vibe/config.toml
- User config at ~/.vibe/config.toml
- Project config at ./.vibe.toml
- Environment variables
- CLI flags
**When** roro starts
**Then** settings are loaded in order: defaults < system < user < project < env < CLI
**And** later sources override earlier ones

**Given** a setting is defined in both user config and CLI flag
**When** roro starts
**Then** the CLI flag value takes precedence

**FRs Covered:** FR63

**Git Commit:**
```bash
git add src/config/loading.rs
git commit -m "feat(config): Add layered config loading with proper precedence"
```

#### Story 7.9: Access Files Within Trusted Directories

As a user,
I want to access files within trusted directories,
So that I can use tools that read/write files safely.

**Acceptance Criteria:**

**Given** I add a directory to trusted directories in config
**When** a tool tries to access a file in that directory
**Then** the operation is allowed
**And** the file is read/written normally

**Given** a tool tries to access a file outside trusted directories
**When** the operation is attempted
**Then** a permission denied error is displayed
**And** the operation is blocked

**Given** I use --trust flag
**When** roro starts
**Then** the current working directory is added to trusted directories

**FRs Covered:** FR78

**Git Commit:**
```bash
git add src/system/trusted_dirs.rs
git commit -m "feat(system): Add trusted directory file access control"
```

#### Story 7.10: Respect .gitignore and .vibeignore for File Operations

As a user,
I want roro to respect .gitignore and .vibeignore,
So that sensitive or irrelevant files are not accessed.

**Acceptance Criteria:**

**Given** a directory contains a .gitignore file
**When** a tool tries to list files in that directory
**Then** files matching .gitignore patterns are excluded from the listing

**Given** a directory contains a .vibeignore file
**When** a tool tries to read files
**Then** files matching .vibeignore patterns are not accessible

**Given** both .gitignore and .vibeignore exist
**When** file operations occur
**Then** both files are respected
**And** .vibeignore takes precedence over .gitignore

**FRs Covered:** FR80

**Git Commit:**
```bash
git add src/system/ignore_files.rs
git commit -m "feat(system): Add .gitignore and .vibeignore support for file operations"
```

---

## Validation Summary

| Epic | Title | Stories | FRs Covered |
|------|-------|---------|-------------|
| 1 | Foundation - Local Provider Connection | 9 | FR9, FR29-FR36, FR54-FR59, FR76-FR80, FR82-FR84 |
| 2 | Conversation Core (MVP) | 7 | FR1-FR2, FR16-FR18, FR47, FR64-FR65 |
| 3 | Tool Execution Framework | 6 | FR19-FR20, FR37-FR46, FR77, FR81 |
| 4 | Session Persistence | 4 | FR12-FR13, FR48-FR53 |
| 5 | Complete CLI Interface | 6 | FR3-FR11, FR14-FR15, FR66-FR69 |
| 6 | Rich TUI Experience | 11 | FR16-FR18, FR21-FR28, FR70-FR75 |
| 7 | Advanced Configuration & Admin | 10 | FR54-FR63, FR78-FR80 |

**Total: 53 Stories covering all 84 Functional Requirements**
