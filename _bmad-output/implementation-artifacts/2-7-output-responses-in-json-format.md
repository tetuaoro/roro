---
story_id: 2.7
story_key: 2-7-output-responses-in-json-format
epic: 2
epic_title: Conversation Core (MVP)
status: backlog
priority: medium
difficulty: low
dependencies: ["2-1-implement-basic-cli-entry-points"]
---
# Story 2.7: Output Responses in JSON Format
## Story Statement
As a developer, I want to output responses in JSON format, so that I can parse roro output programmatically.
## Acceptance Criteria
**Given** I run `roro -p "Hello" --output json`, **When** response received, **Then** full response output as valid JSON with content, model, usage, finish_reason.
**Given** I run `roro -p "Hello" --output json --stream`, **When** streaming, **Then** each token as separate JSON line (NDJSON).
**Given** output is JSON and error occurs, **When** error occurs, **Then** error output as JSON with error and message fields.
## FRs: FR65
## Tasks
- [ ] Implement --output flag parsing
- [ ] Add JSON output formatter
- [ ] Implement NDJSON streaming output
- [ ] Add error JSON formatting
- [ ] Add unit tests
