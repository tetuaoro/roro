---
story_id: 6.6
story_key: 6-6-show-provider-status-indicators
epic: 6
epic_title: Rich TUI Experience
status: backlog
priority: medium
difficulty: low
dependencies: ["1-4-add-provider-connectivity-validation"]
---
# Story 6.6: Show Provider Status Indicators
## Story Statement
As a user, I want to see provider status indicators, so that I know the health of my connection.
## Acceptance Criteria
**Given** connected to provider, **When** TUI displayed, **Then** status shows "Connected: {provider} ({model})" in top-right.
**Given** connection lost, **When** detected, **Then** status changes to "Disconnected" with red color.
**Given** reconnecting, **When** in progress, **Then** status shows "Reconnecting..." with spinning animation.
## FRs: FR27
## Tasks
- [ ] Add status indicator to TUI
- [ ] Display connected state with provider and model
- [ ] Display disconnected state with red color
- [ ] Add reconnecting animation
- [ ] Update status on connection changes
- [ ] Add unit tests
