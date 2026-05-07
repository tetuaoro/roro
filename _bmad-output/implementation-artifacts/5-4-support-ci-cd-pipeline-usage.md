---
story_id: 5.4
story_key: 5-4-support-ci-cd-pipeline-usage
epic: 5
epic_title: Complete CLI Interface
status: backlog
priority: medium
difficulty: medium
dependencies: ["5-1-implement-all-cli-flags-and-arguments"]
---
# Story 5.4: Support CI/CD Pipeline Usage
## Story Statement
As a developer, I want to use roro in CI/CD pipelines, so that I can automate tasks with LLM assistance.
## Acceptance Criteria
**Given** roro in CI environment (no TTY), **When** executed, **Then** automatically uses non-interactive mode, reads from stdin, outputs to stdout.
**Given** `roro --batch` with multiple prompts via stdin, **When** executed, **Then** all prompts processed sequentially, results output to stdout.
**Given** roro in GitHub Actions, **When** workflow runs, **Then** executes successfully, respects resource limits, returns appropriate exit codes.
## FRs: FR68
## Tasks
- [ ] Detect non-TTY environment
- [ ] Force non-interactive mode in CI
- [ ] Implement --batch flag
- [ ] Process multiple prompts from stdin
- [ ] Output results sequentially
- [ ] Respect CI resource limits
- [ ] Add unit tests
