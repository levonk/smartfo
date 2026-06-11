---
story_id: "02-003"
story_title: "Terminal size awareness + tests"
story_name: "terminal-size-awareness-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 2
parallel_id: 3
branch: "feature/current/requirements-gaps/story-02-003-terminal-size-awareness-tests"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-002"]
parallel_safe: true
modules: ["tui", "terminal", "tests"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "terminal", "test"]
due: "2026-07-08"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Implement terminal size detection and resize handling for TUI mode and output formatting. Detect terminal size on startup, format output based on width, and handle resize events where possible. Addresses ADR #22 from CLI standards.

## Sub-Tasks

- [ ] Add terminal size detection utilities in `src/terminal.rs`
- [ ] Implement terminal size detection on startup
- [ ] Add output formatting based on terminal width
- [ ] Implement terminal resize event handling (SIGWINCH)
- [ ] Add fallback for non-terminal output (reasonable defaults)
- [ ] Add terminal size awareness to TUI mode
- [ ] Add terminal size awareness to help text formatting
- [ ] Add terminal size awareness to table/list output
- [ ] Write unit tests for terminal size detection
- [ ] Write integration tests for resize handling
- [ ] Document terminal size behavior in README

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/terminal.rs` — New module for terminal utilities
- `src/tui.rs` — Add terminal size awareness to TUI
- `src/output.rs` — Add terminal size awareness to output formatting
- `tests/terminal_test.rs` — New test file for terminal utilities
- `README.md` — Document terminal size behavior
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [ ] Terminal size detected correctly on startup
- [ ] Output formatted based on terminal width
- [ ] Resize events handled gracefully
- [ ] Fallback defaults work for non-terminal output
- [ ] TUI mode respects terminal size
- [ ] All tests pass

## Test Plan

- Unit: `devbox run cargo test terminal_test`
- Integration: Test with various terminal sizes
- Integration: Test resize events with SIGWINCH
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log terminal size detection
- Track resize events

## Compliance

- Follow ADR #22 (Terminal Size Awareness)
- Ensure cross-platform compatibility (Linux, macOS, Windows)

## Risks & Mitigations

- Risk: Terminal size detection may fail on some platforms — Mitigation: Use cross-platform libraries, provide fallback
- Risk: Resize handling may be complex — Mitigation: Keep it simple, only update on next output

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-01-002-cross-platform-test-harness]]
- Unblocks: 03-001

## Definition of Done

- Terminal size detection implemented and tested
- Output formatting respects terminal size
- Resize handling works
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(terminal): add terminal size awareness`

## Changelog

- 2026-06-11: initialized story file
