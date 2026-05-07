---
stepsCompleted: ['step-1-init', 'step-2-discovery', 'step-2b-vision', 'step-2c-executive-summary', 'step-3-success', 'step-4-journeys', 'step-5-domain', 'step-7-project-type', 'step-8-scoping', 'step-9-functional', 'step-10-nonfunctional', 'step-11-polish']
releaseMode: single-release
inputDocuments:
  - _bmad-output/planning-artifacts/research/market-rust-cli-local-llm-research-2026-05-06.md
  - _bmad-output/brainstorming/brainstorming-session-2026-05-06-150705.md
  - docs/README.md
  - docs/acp-setup.md
  - docs/proxy-setup.md
documentCounts:
  researchCount: 1
  brainstormingCount: 1
  projectDocsCount: 3
  briefCount: 0
classification:
  projectType: cli_tool
  domain: scientific
  complexity: high
  projectContext: brownfield
workflowType: 'prd'
---

# Product Requirements Document - roro

**Author:** Rao Nagos
**Date:** 2026-05-07

## Executive Summary

Roro is a Rust rewrite of Mistral Vibe CLI that provides a high-performance, memory-safe coding assistant interface to local OpenAI API-compatible LLM providers. Target users are developers who need a CLI-based AI coding assistant running entirely on-premises with Ollama, LocalAI, or Llama.cpp. The problem solved is enabling offline, private, and performant local LLM interactions through a familiar CLI interface while maintaining 100% command compatibility with the original Python implementation.

### What Makes This Special

Roro distinguishes itself through its mirror architecture approach: it replicates the Python Mistral Vibe's exact CLI interface, tool safety framework (ALWAYS/ASK/NEVER permissions), and configuration schema while delivering Rust's performance, memory safety, and concurrency advantages. The core insight is leveraging the `openai_api_rust` crate for unified local provider integration via configurable base URLs, eliminating custom trait implementations while supporting Ollama, LocalAI, Llama.cpp, and any OpenAI-compatible endpoint through a single URL parameter.

## Project Classification

- **Project Type:** CLI Tool with Textual TUI
- **Domain:** Scientific (AI/ML Developer Tooling)
- **Complexity:** High
- **Project Context:** Brownfield (Rust rewrite of existing Python codebase)

---

The following success criteria define what winning looks like for roro users and the project team.

## Success Criteria

### User Success

Developers using roro experience identical CLI and TUI behavior to the Python Mistral Vibe — every command, flag, interactive TUI feature, and keyboard shortcut works without learning a new interface. Local LLM providers (Ollama, LocalAI, Llama.cpp) connect seamlessly via simple base URL configuration, with the tool safety framework (ALWAYS/ASK/NEVER permissions) preserving existing security boundaries. Users achieve faster response times and lower memory usage compared to the Python version, with zero configuration changes required for migration.

### Business Success

100% backward compatibility with existing CLI commands, TUI interface, tool permissions, and configuration files. Users migrate from Python to Rust without modifying their workflows or reconfiguring their setups. The Rust implementation achieves full feature parity including the Textual TUI experience while demonstrating measurable performance improvements that justify the rewrite investment.

### Technical Success

Effective integration of `openai_api_rust` crate for all local provider communications. Tokio async runtime enables concurrent tool execution and streaming responses. Serde-based configuration validation ensures type safety for all settings. Clap framework provides consistent CLI argument parsing. Textual-like TUI framework (ratatui/crossterm) replicates the interactive interface. Thiserror + anyhow pattern delivers precise error handling without sacrificing usability. RAII pattern guarantees resource cleanup and memory safety.

### Measurable Outcomes

- Zero breaking changes in CLI interface (100% command compatibility)
- Full TUI feature parity with Python Textual interface
- All existing tool permissions function identically
- Configuration files load without modification
- Performance benchmarks show >20% improvement in response latency
- Memory usage stable under concurrent operations

---

To achieve these success criteria, the following complete feature set must be delivered in a single release.

## Product Scope

### MVP - Minimum Viable Product

Core CLI interface mirroring, Textual TUI replication using Rust TUI framework (ratatui/crossterm), local provider integration via `openai_api_rust`, tool safety framework preservation, TOML/JSON configuration compatibility, basic error handling and logging.

### Growth Features (Post-MVP)

Performance optimizations (connection pooling, request batching), advanced error recovery, health monitoring for local providers, comprehensive test suite, documentation updates, TUI enhancements (themes, keyboard shortcuts customization).

### Vision (Future)

Full feature parity with Python Mistral Vibe including all TUI features, extended provider support, community plugin system, IDE integration, cross-platform packaging, multi-pane TUI layouts, session management.

---

The following user journeys demonstrate how these capabilities come together to solve real user problems.

## User Journeys

### Journey 1: Alex - The Rust Developer (Primary User - Success Path)

**Opening Scene:** Alex, a Rust developer working on a complex async project, is frustrated with context switching between their editor and browser to look up API documentation. They've heard about AI coding assistants but worry about sending proprietary code to cloud services.

**Rising Action:** Alex discovers roro, installs it via cargo, and runs `roro` for the first time. The familiar Textual TUI interface loads instantly. They configure their local Ollama endpoint at `http://localhost:11434` and select the `mistral` model. Alex opens their project and asks roro to "explain how to properly use async/await with reqwest in Rust." The assistant responds with accurate, idiomatic Rust code examples that compile on first try.

**Climax:** Alex uses roro's tool execution to run `cargo check` on the generated code directly from the TUI. The tool safety framework asks for permission (ASK mode) before executing the command. Alex grants it and sees the code compiles successfully — all without leaving the TUI interface.

**Resolution:** Alex's workflow is transformed. They can now iterate on code 3x faster, with all documentation, code generation, and tool execution happening in a single terminal session. The local-only nature means their proprietary code never leaves their machine.

**This journey reveals requirements for:** Local provider configuration, TUI interface with code display, tool execution integration, permission system, Rust-specific code understanding.

---

### Journey 2: Alex - Provider Connection Failure (Primary User - Edge Case)

**Opening Scene:** Alex returns to work after a weekend and launches roro, but their Ollama server crashed overnight.

**Rising Action:** The TUI shows a clear error message: "Cannot connect to provider at http://localhost:11434." Instead of a cryptic error, roro displays a helpful troubleshooting guide directly in the interface: "Check if Ollama is running: `systemctl status ollama`". Alex runs this command through roro's tool system.

**Climax:** The command reveals Ollama isn't running. Alex starts it via the TUI's command palette (`/start-ollama`), which roro recognizes as a known provider command. Roro automatically retries the connection and reconnects successfully.

**Resolution:** Alex is back to coding within 30 seconds, impressed by roro's intelligent error recovery. The session state is preserved — their previous conversation context is still available.

**This journey reveals requirements for:** Provider health monitoring, intelligent error messages, command palette for provider management, session persistence, automatic reconnection.

---

### Journey 3: Jamie - The DevOps Engineer (Admin/Operations User)

**Opening Scene:** Jamie manages the development environment for a team of 10 engineers. They need to standardize AI assistant usage across the organization while maintaining data privacy. Each developer runs their own local LLM provider on different ports.

**Rising Action:** Jamie creates a shared roro configuration file that specifies provider URL templates. They use environment variable placeholders like `${OLLAMA_HOST}` and `${OLLAMA_PORT}` so each developer can customize their setup. Jamie also configures the tool permission defaults to NEVER for file-deleting commands and ALWAYS for read-only operations.

**Climax:** Jamie distributes the configuration via the team's dotfiles repository. Each developer clones it, runs `roro --config team-config.toml`, and immediately has a working setup. The configuration validation catches a typo in one engineer's environment variable before any errors occur.

**Resolution:** The entire team now has a consistent, secure roro experience. Jamie can audit tool usage through roro's logging system, and the organization maintains full data control without sacrificing developer productivity.

**This journey reveals requirements for:** Configuration templating, environment variable support, permission presets, config validation, audit logging, team configuration sharing.

---

### Journey 4: Taylor - The Power User (Secondary User - Advanced TUI)

**Opening Scene:** Taylor, an experienced developer, wants to maximize their productivity with roro. They've been using the basic chat interface but know there's more power under the hood.

**Rising Action:** Taylor discovers the `/` command palette and explores available commands. They learn to split the TUI into multiple panes: one for chat, one for tool output, and one for code preview. Taylor sets up keyboard shortcuts for their most common actions: Ctrl+Enter to execute the last tool command, Ctrl+S to save the conversation.

**Climax:** Taylor uses roro's multi-tool execution feature to run `cargo test`, `cargo clippy`, and `cargo fmt` simultaneously across their project. The results appear in separate panes, and Taylor can navigate between them with keyboard shortcuts. When a test fails, Taylor can click on the error line number to jump directly to that location in their editor.

**Resolution:** Taylor's development workflow is supercharged. They can now execute complex multi-step operations in a single command, navigate results efficiently, and maintain full context across all their work. Taylor shares their custom keybindings configuration with the team.

**This journey reveals requirements for:** Multi-pane TUI layout, customizable keyboard shortcuts, multi-tool concurrent execution, result navigation, editor integration, configuration export/import.

---

### Journey Requirements Summary

| Journey | Key Capabilities Revealed |
|---------|---------------------------|
| Alex - Success Path | Local provider config, TUI code display, tool execution, permission system, Rust code understanding |
| Alex - Edge Case | Health monitoring, error recovery, command palette, session persistence, auto-reconnect |
| Jamie - Admin | Config templating, env vars, permission presets, validation, audit logging, team sharing |
| Taylor - Power User | Multi-pane layout, keybindings, concurrent execution, result navigation, editor integration |

---

These journeys require a CLI tool with specific technical characteristics. The following sections detail the project-type requirements for roro as a CLI tool.

## CLI Tool Specific Requirements

### Project-Type Overview

Roro mirrors the Python Mistral Vibe CLI exactly, providing both interactive Textual TUI mode and programmatic headless mode. It operates as a local-first AI coding assistant that users interact with through the same terminal interface as the Python version.

### Technical Architecture Considerations

**Interactive vs Scriptable:** Roro supports both modes identical to Python version. Interactive mode (default) launches the Textual TUI for rich chat interface. Programmatic mode (`--prompt` / `-p`) enables headless operation that sends prompt, auto-approves all tools, outputs response, and exits — mirroring the Python behavior exactly.

**Output Formats:** Text (default, human-readable), JSON (all messages at end), and streaming (newline-delimited JSON per message) for programmatic mode, matching Python output exactly.

**Config Method:** TOML configuration files with environment variable expansion, identical schema to Python version. Configuration precedence: CLI args > environment variables > config file > defaults.

**Shell Completion:** Generate completion scripts for bash, zsh, and fish — same as Python version.

### Command Structure

```
roro [OPTIONS] [INITIAL_PROMPT]

Arguments:
  INITIAL_PROMPT    Initial prompt to start interactive session with

Options:
  -p, --prompt TEXT     Run in programmatic mode: send prompt, auto-approve all tools, output response, exit
  --max-turns N         Maximum number of assistant turns (programmatic mode only)
  --max-price DOLLARS   Maximum cost in dollars (programmatic mode only)
  --enabled-tools TOOL  Enable specific tools (can be repeated, glob/regex patterns supported)
  --output FORMAT       Output format for programmatic mode: text, json, streaming [default: text]
  --agent NAME          Agent to use (builtin: default, plan, accept-edits, auto-approve, or custom)
  --setup               Setup API key and exit
  --workdir DIR         Change to this directory before running
  --trust               Trust working directory for this invocation only
  -c, --continue        Continue from the most recent saved session
  --resume [SESSION_ID] Resume a session; without ID shows interactive picker
  -v, --version         Show version information
  -h, --help            Show help message

Hidden Options:
  --teleport            Enable teleport feature (experimental)
```

### Scripting Support

Programmatic mode (`-p/--prompt`) enables:
- Piping input via stdin: `echo "explain this" | roro -p`
- Exit codes: 0 = success, 1 = error, 2 = teleport error, 3 = conversation limit
- JSON output format with full message history
- Streaming output format for real-time processing
- Tool auto-approval for non-interactive workflows
- `--max-turns` and `--max-price` for cost control

### Implementation Considerations

- Clap framework for argument parsing with automatic help generation matching Python argparse style
- Shell completion generation using clap_complete crate
- Config file watching for hot-reload during development
- Argument validation matching Python version exactly
- Subcommand-specific help with examples
- Session continuation logic mirroring Python implementation

---

As a scientific domain product dealing with AI/ML developer tooling, roro has specific domain constraints and requirements.

## Domain-Specific Requirements

### Compliance & Standards

- **OpenAI API Compatibility:** Strict adherence to OpenAI Chat API and Embeddings API specifications to ensure compatibility with all local providers (Ollama, LocalAI, Llama.cpp) without provider-specific code paths
- **Local-Only Guarantee:** Zero telemetry, zero external network calls beyond configured provider endpoints, zero data exfiltration to third parties

### Technical Constraints

- **Performance:** Response latency <500ms for typical coding queries, streaming tokens at minimum 20 tokens/second, concurrent request handling for multi-tool execution
- **Memory:** Stable memory usage under sustained load, automatic memory cleanup after conversation sessions, graceful degradation when approaching system limits
- **Reliability:** Automatic retry with exponential backoff for provider timeouts, circuit breaker pattern to prevent cascading failures, health checks on provider endpoints before request dispatch
- **Accuracy:** Context window preservation across multi-turn conversations, code generation that compiles on first attempt >80% of the time, hallucination detection for tool command suggestions

### Integration Requirements

- **Provider Agnosticism:** Single configuration interface for any OpenAI-compatible endpoint, automatic capability detection (supported models, max context length, available tools), provider version compatibility matrix
- **Tool Ecosystem:** Native integration with Rust toolchain (cargo, rustc, clippy, rustfmt), shell command execution with environment inheritance, file system access with permission boundaries

### Risk Mitigations

- **Prompt Injection:** Input sanitization for tool commands, permission confirmation for destructive operations (file deletion, system modifications), read-only mode for untrusted model outputs
- **Resource Exhaustion:** Request timeout caps (default 30s, configurable), concurrent connection limits per provider, memory usage monitoring with user warnings at 80% threshold
- **Provider Failures:** Graceful degradation when provider is unavailable, clear error messages with actionable troubleshooting steps, session state preservation for reconnection scenarios

---

The following scoping decisions define how all these requirements will be delivered.

## Project Scoping

### Strategy & Philosophy

**Approach:** Single-release mirror implementation — roro must achieve 100% functional parity with Python Mistral Vibe in one delivery. The philosophy is "mirror first, optimize later" to ensure users can seamlessly migrate without workflow changes.

**Resource Requirements:** Senior Rust developer with CLI/TUI experience, familiarity with OpenAI API, Tokio async patterns, and Textual-like TUI frameworks (ratatui/crossterm). Estimated 3-6 months depending on team size.

### Complete Feature Set

**Core User Journeys Supported:**
- All 4 journeys: Developer success path, error recovery, admin/configuration, power user features

**Must-Have Capabilities:**
- Full CLI command mirroring (all flags: `-p`, `--continue`, `--resume`, `--agent`, `--setup`, `--workdir`, `--trust`, `--output`, `--max-turns`, `--max-price`, `--enabled-tools`)
- Complete Textual TUI replication with multi-pane layouts, command palette, keyboard shortcuts
- Local provider integration via `openai_api_rust` for Ollama, LocalAI, Llama.cpp
- Tool safety framework (ALWAYS/ASK/NEVER permissions) with identical behavior
- Session management (save, resume, continue) with persistence
- Configuration backward compatibility (TOML/JSON, environment variables, precedence)
- Programmatic mode with text/JSON/streaming output
- Shell completion generation for bash/zsh/fish

**Nice-to-Have Capabilities:**
- Performance benchmarking suite
- Provider health monitoring dashboard in TUI
- Advanced error recovery (auto-retry, circuit breaker)
- Community plugin system
- IDE integration protocols

### Risk Mitigation Strategy

**Technical Risks:**
- **TUI Parity:** Textual (Python) to ratatui (Rust) feature mapping gaps. Mitigation: Prioritize core TUI features first, use Python TUI as reference implementation, implement progressive enhancement.
- **Provider Variations:** Different local providers may have subtle API differences. Mitigation: Comprehensive provider capability detection, extensive integration testing matrix.

**Market Risks:**
- **Adoption:** Users may resist switching from working Python version. Mitigation: Zero-breaking-change guarantee, migration guide, side-by-side usage support.

**Resource Risks:**
- **Complexity Underestimation:** TUI replication may take longer than expected. Mitigation: Modular architecture allowing TUI to be developed in parallel with CLI core, regular progress checkpoints.

---

The following functional requirements form the capability contract for roro. Every feature must trace back to these requirements.

## Functional Requirements

### CLI Interface & Commands

- FR1: Users can launch roro with no arguments to start interactive TUI mode
- FR2: Users can provide initial prompt as positional argument to start session with pre-filled input
- FR3: Users can use `--prompt` / `-p` flag to run in programmatic mode
- FR4: Users can specify `--max-turns` to limit conversation turns in programmatic mode
- FR5: Users can specify `--max-price` to cap cost in programmatic mode
- FR6: Users can enable specific tools via `--enabled-tools` flag with glob/regex pattern support
- FR7: Users can select output format via `--output` (text, json, streaming) for programmatic mode
- FR8: Users can choose agent via `--agent` flag (builtin or custom)
- FR9: Users can run `--setup` to configure API key interactively
- FR10: Users can change directory via `--workdir` before execution
- FR11: Users can trust current directory via `--trust` flag
- FR12: Users can continue from latest session via `--continue` / `-c` flag
- FR13: Users can resume specific session via `--resume` with optional SESSION_ID
- FR14: Users can view version via `--version` / `-v` flag
- FR15: Users can view help via `--help` / `-h` flag

### Textual TUI Interface

- FR16: Users can interact with chat interface in terminal
- FR17: Users can view conversation history in scrollable pane
- FR18: Users can see code blocks with syntax highlighting
- FR19: Users can execute tools directly from TUI with permission prompts
- FR20: Users can access command palette via `/` prefix
- FR21: Users can split interface into multiple panes (chat, tool output, code preview)
- FR22: Users can customize keyboard shortcuts
- FR23: Users can navigate between panes using keyboard
- FR24: Users can click on error line numbers to open in editor
- FR25: Users can save conversation history
- FR26: Users can view session metadata (session ID, timestamps)
- FR27: Users can see provider status indicators
- FR28: Users can view tool execution results in dedicated output pane

### Local Provider Integration

- FR29: Users can configure provider base URL (Ollama, LocalAI, Llama.cpp, or custom)
- FR30: System can auto-detect provider capabilities (models, context length, features)
- FR31: Users can switch between configured providers
- FR32: System can validate provider connectivity on startup
- FR33: System can display provider-specific model lists
- FR34: Users can override model selection per request
- FR35: System can handle provider timeouts with automatic retry
- FR36: System can display clear error messages for provider failures

### Tool Execution & Safety Framework

- FR37: Users can execute shell commands through tool system
- FR38: System can request user permission before tool execution (ASK mode)
- FR39: System can auto-approve specific tools (ALWAYS mode)
- FR40: System can block dangerous tools entirely (NEVER mode)
- FR41: System can validate tool arguments before execution
- FR42: System can capture and display tool stdout/stderr
- FR43: System can return tool exit codes
- FR44: System can handle tool timeouts gracefully
- FR45: Users can configure default permission levels per tool
- FR46: System can log all tool executions for audit

### Session Management

- FR47: System can save conversation history automatically
- FR48: Users can resume previous sessions by ID
- FR49: System can restore full context (messages, state) from saved sessions
- FR50: System can handle session continuation from latest session
- FR51: Users can list available saved sessions
- FR52: Users can delete saved sessions
- FR53: System can persist session metadata (timestamps, model used, tool count)

### Configuration Management

- FR54: Users can configure roro via TOML configuration files
- FR55: Users can override config via environment variables
- FR56: System can expand environment variables in config values (`${VAR}`)
- FR57: Users can specify custom config file via `--config` flag
- FR58: System can validate configuration on startup
- FR59: System can provide clear errors for invalid configuration
- FR60: Users can configure multiple provider profiles
- FR61: Users can set default provider and model
- FR62: Users can configure tool permission presets
- FR63: System can load config from multiple sources with proper precedence

### Programmatic Mode & Scripting

- FR64: Users can pipe input via stdin in programmatic mode
- FR65: System can output responses in JSON format
- FR66: System can stream responses in newline-delimited JSON
- FR67: System can return appropriate exit codes (0=success, 1=error, 2=teleport error, 3=conversation limit)
- FR68: Users can use programmatic mode in CI/CD pipelines
- FR69: System can handle batch processing of multiple prompts

### Error Handling & Recovery

- FR70: System can display clear, actionable error messages
- FR71: System can provide troubleshooting suggestions for common errors
- FR72: System can auto-recover from provider connection loss
- FR73: System can preserve session state during error recovery
- FR74: Users can trigger manual reconnection to provider
- FR75: System can detect and warn about resource exhaustion

### System & Integration

- FR76: Users can generate shell completion scripts for bash/zsh/fish
- FR77: System can inherit environment variables for tool execution
- FR78: System can access files within trusted directories
- FR79: Users can temporarily trust directories for file access
- FR80: System can respect `.gitignore` and `.vibeignore` for file operations
- FR81: System can handle keyboard interrupts gracefully
- FR82: Users can download models from provider repositories
- FR83: Users can update to newer model versions
- FR84: Users can manage multiple model versions simultaneously

---

The following non-functional requirements define the quality attributes and performance standards for roro.

## Non-Functional Requirements

### Performance

- **Response Latency:** First token in streaming response must appear within 500ms of request submission for 95% of queries
- **Streaming Throughput:** Minimum 20 tokens/second sustained streaming rate for typical coding queries
- **TUI Rendering:** Interface must maintain 60+ FPS during active conversation and tool execution
- **Concurrent Sessions:** System must support 10+ simultaneous active sessions without performance degradation
- **Startup Time:** CLI must be ready for user input within 2 seconds of invocation
- **Tool Execution:** Tool commands must execute with <100ms overhead beyond native execution time

### Security

- **Local-Only Guarantee:** Zero network calls to external services beyond configured provider endpoints; all code and data remain on user's machine
- **No Telemetry:** Zero analytics, tracking, or usage data collection
- **Input Sanitization:** All tool commands must be validated and sanitized before execution to prevent prompt injection attacks
- **Permission Enforcement:** ALWAYS/ASK/NEVER tool permissions must be strictly enforced with no bypass possible
- **Environment Isolation:** Tool execution must inherit user's environment variables but cannot modify them
- **File Access:** File operations limited to explicitly trusted directories; read-only by default for untrusted paths

### Reliability

- **Provider Timeouts:** Automatic retry with exponential backoff (1s, 2s, 4s) for failed provider requests, maximum 3 attempts
- **Session Persistence:** Saved sessions must be recoverable after unexpected termination (crash, kill signal)
- **Error Recovery:** System must maintain state and allow retry after provider connection loss without data loss
- **Uptime Target:** 99.9% availability during active development sessions (excludes provider downtime)
- **Crash Handling:** All errors must be catchable and display user-friendly messages; no silent failures

### Integration

- **OpenAI API Compatibility:** 100% compatibility with OpenAI Chat API v1 and Embeddings API v1 specifications
- **Provider Detection:** Automatic detection and adaptation to provider-specific capabilities and limits
- **Rust Toolchain:** Native integration with cargo, rustc, clippy, rustfmt with zero configuration required
- **Shell Integration:** Preserve exit codes, stdout/stderr, and environment from shell commands executed through tool system
- **Editor Integration:** Error line numbers must be clickable to open in user's default editor

### Compliance

- **Data Residency:** All data (conversations, config, session history) stored locally; no cloud synchronization without explicit user action
- **License Compliance:** All dependencies must use Rust-compatible licenses (MIT, Apache 2.0, BSD); no GPL dependencies
- **Configuration Standards:** Config files must follow TOML v1.0 specification with JSON compatibility layer
