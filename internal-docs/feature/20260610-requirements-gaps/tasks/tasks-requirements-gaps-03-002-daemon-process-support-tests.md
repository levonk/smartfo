---
story_id: "03-002"
story_title: "Daemon process support + tests"
story_name: "daemon-process-support-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 3
parallel_id: 2
branch: "feature/current/requirements-gaps/story-03-002-daemon-process-support-tests"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["02-002"]
parallel_safe: true
modules: ["daemon", "cli", "tests"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "daemon", "test"]
due: "2026-07-15"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Enhance daemon process support with `--daemon` and `--no-daemon` flags, `--list-jobs` command, and `--cancel-job <id>` command. Addresses ADR #13 from CLI standards. Includes platform fallback behavior with configurable quiet mode.

## Sub-Tasks

- [ ] Add `--daemon` flag to CLI for pre-launching daemon
- [ ] Add `--no-daemon` flag to CLI for synchronous operation
- [ ] Implement `--list-jobs` command with optional job ID filtering
- [ ] Implement `--cancel-job <id>` command
- [ ] Add job status display in list command
- [ ] Implement daemon platform fallback behavior
- [ ] Add `daemon_fallback_quiet` config option
- [ ] Add error messages for unsupported platforms
- [ ] Add instructions for monitoring job progress
- [ ] Write unit tests for daemon flags
- [ ] Write integration tests for --list-jobs
- [ ] Write integration tests for --cancel-job
- [ ] Write tests for platform fallback behavior
- [ ] Update man pages with daemon documentation
- [ ] Update help text with new commands

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/cli.rs` — Add daemon flags and commands
- `src/daemon.rs` — Enhance daemon with list/cancel operations
- `src/queue.rs` — Add job listing and cancellation
- `src/config.rs` — Add daemon_fallback_quiet config
- `tests/daemon_test.rs` — New tests for daemon enhancements
- `src/man.rs` — Update man pages
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [ ] --daemon flag pre-launches daemon in background
- [ ] --no-daemon flag forces synchronous operation
- [ ] --list-jobs shows background job status with filtering
- [ ] --cancel-job cancels specific jobs by ID
- [ ] Platform fallback works with clear error messages
- [ ] daemon_fallback_quiet config option works
- [ ] All tests pass
- [ ] Documentation complete

## Test Plan

- Unit: `devbox run cargo test daemon_test`
- Integration: Test daemon flags and commands
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log daemon flag usage
- Track job cancellations
- Monitor platform fallback occurrences

## Compliance

- Follow ADR #13 (Daemon Process Support)
- Ensure daemon operations are crash-safe

## Risks & Mitigations

- Risk: Job cancellation may leave inconsistent state — Mitigation: Implement graceful cancellation with cleanup
- Risk: Platform fallback may be noisy — Mitigation: Use daemon_fallback_quiet config to suppress warnings

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-02-002-health-check-mechanism-tests]]
- Unblocks: 06-001

## Definition of Done

- Daemon flags and commands implemented and tested
- Platform fallback working
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(daemon): add daemon process support enhancements`

## Changelog

- 2026-06-11: initialized story file
