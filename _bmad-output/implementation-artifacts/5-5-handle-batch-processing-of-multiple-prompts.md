---
story_id: 5.5
story_key: 5-5-handle-batch-processing-of-multiple-prompts
epic: 5
epic_title: Complete CLI Interface
status: backlog
priority: medium
difficulty: medium
dependencies: ["5-4-support-ci-cd-pipeline-usage"]
---
# Story 5.5: Handle Batch Processing of Multiple Prompts
## Story Statement
As a developer, I want to process multiple prompts in batch mode, so that I can automate bulk operations.
## Acceptance Criteria
**Given** file with prompts one per line, **When** `cat prompts.txt | roro -p --batch`, **Then** each prompt processed in sequence, responses output in order, separated by ---.
**Given** `roro -p "Prompt 1" -p "Prompt 2"`, **When** executed, **Then** both prompts processed sequentially, responses output with clear separation.
**Given** prompt in batch fails, **When** processing continues, **Then** error logged to stderr, processing continues with next prompt.
## FRs: FR69
## Tasks
- [ ] Implement multiple -p flag support
- [ ] Process prompts sequentially
- [ ] Add response separators
- [ ] Handle individual prompt failures
- [ ] Continue processing on error
- [ ] Add unit tests
