---
story_id: 7.10
story_key: 7-10-respect-gitignore-and-vibeignore-for-file-operations
epic: 7
epic_title: Advanced Configuration & Admin
status: backlog
priority: low
difficulty: low
dependencies: []
---
# Story 7.10: Respect .gitignore and .vibeignore for File Operations
## Story Statement
As a user, I want roro to respect .gitignore and .vibeignore, so that sensitive or irrelevant files are not accessed.
## Acceptance Criteria
**Given** directory contains .gitignore, **When** tool lists files, **Then** files matching .gitignore patterns excluded.
**Given** directory contains .vibeignore, **When** tool reads files, **Then** files matching .vibeignore patterns not accessible.
**Given** both .gitignore and .vibeignore exist, **When** operations occur, **Then** both respected, .vibeignore takes precedence.
## FRs: FR80
## Tasks
- [ ] Add ignore file parsing
- [ ] Implement .gitignore support
- [ ] Implement .vibeignore support
- [ ] Apply ignore patterns to file operations
- [ ] Handle precedence
- [ ] Add unit tests
