---
story_id: 2.4
story_key: 2-4-add-syntax-highlighting-for-code-blocks
epic: 2
epic_title: Conversation Core (MVP)
status: backlog
priority: medium
difficulty: medium
dependencies: ["2-3-add-scrollable-conversation-history-pane"]
---
# Story 2.4: Add Syntax Highlighting for Code Blocks
## Story Statement
As a user, I want to see code blocks with syntax highlighting, so that I can easily read and understand code in LLM responses.
## Acceptance Criteria
**Given** response contains code block with language specifier, **When** rendered, **Then** code is displayed with syntax highlighting.
**Given** response contains code block without language, **When** rendered, **Then** code is displayed with monospace font.
**Given** terminal supports 24-bit color, **When** code displayed, **Then** full color highlighting is used.
## FRs: FR18
## Tasks
- [ ] Add syntect dependency for syntax highlighting
- [ ] Detect code blocks in LLM responses
- [ ] Extract language from code block markers
- [ ] Apply syntax highlighting
- [ ] Handle unsupported languages
- [ ] Add color support detection
- [ ] Add unit tests
