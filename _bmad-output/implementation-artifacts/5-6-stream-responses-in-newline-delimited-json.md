---
story_id: 5.6
story_key: 5-6-stream-responses-in-newline-delimited-json
epic: 5
epic_title: Complete CLI Interface
status: backlog
priority: medium
difficulty: medium
dependencies: ["5-1-implement-all-cli-flags-and-arguments"]
---
# Story 5.6: Stream Responses in Newline-Delimited JSON
## Story Statement
As a developer, I want streaming responses in newline-delimited JSON format, so that I can process responses in real-time.
## Acceptance Criteria
**Given** `roro -p "Hello" --output streaming`, **When** streaming, **Then** each token on separate line with raw text.
**Given** `roro -p "Hello" --output streaming --json`, **When** streaming, **Then** each line valid JSON with content, model, finish_reason.
**Given** stream interrupted by Ctrl+C, **When** pressed, **Then** stream stops gracefully, partial output preserved.
## FRs: FR66
## Tasks
- [ ] Implement --output streaming flag
- [ ] Add newline-delimited output
- [ ] Implement --json with streaming
- [ ] Format each line as JSON
- [ ] Handle Ctrl+C during streaming
- [ ] Preserve partial output
- [ ] Add unit tests
