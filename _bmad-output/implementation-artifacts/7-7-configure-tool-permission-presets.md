---
story_id: 7.7
story_key: 7-7-configure-tool-permission-presets
epic: 7
epic_title: Advanced Configuration & Admin
status: backlog
priority: medium
difficulty: low
dependencies: ["3-1-create-tool-registry-with-permission-system"]
---
# Story 7.7: Configure Tool Permission Presets
## Story Statement
As an administrator, I want to configure tool permission presets, so that I can set default permissions for different tool categories.
## Acceptance Criteria
**Given** config with [tool_permissions] default=ASK, read_commands=ALWAYS, **When** read command invoked, **Then** executes without prompt.
**Given** write command invoked, **When** executed, **Then** permission prompt shown.
**Given** dangerous command invoked, **When** executed, **Then** execution blocked.
## FRs: FR62
## Tasks
- [ ] Add tool_permissions config section
- [ ] Implement permission preset categories
- [ ] Map commands to categories
- [ ] Apply presets to tools
- [ ] Add override capability
- [ ] Add unit tests
