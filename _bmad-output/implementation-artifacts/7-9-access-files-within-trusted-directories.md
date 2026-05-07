---
story_id: 7.9
story_key: 7-9-access-files-within-trusted-directories
epic: 7
epic_title: Advanced Configuration & Admin
status: backlog
priority: medium
difficulty: low
dependencies: []
---
# Story 7.9: Access Files Within Trusted Directories
## Story Statement
As a user, I want to access files within trusted directories, so that I can use tools that read/write files safely.
## Acceptance Criteria
**Given** directory in trusted directories in config, **When** tool accesses file in that dir, **Then** operation allowed, file read/written normally.
**Given** tool tries to access file outside trusted dirs, **When** attempted, **Then** permission denied error displayed, operation blocked.
**Given** --trust flag, **When** roro starts, **Then** current working directory added to trusted directories.
## FRs: FR78
## Tasks
- [ ] Add trusted_directories config
- [ ] Implement file access control
- [ ] Check file paths against trusted dirs
- [ ] Add --trust flag support
- [ ] Add unit tests
