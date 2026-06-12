---
story_id: "03-001"
story_title: "TUI mode framework + tests"
story_name: "tui-mode-framework-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 3
parallel_id: 1
branch: "feature/current/requirements-gaps/story-03-001-tui-mode-framework-tests"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: ["02-003"]
parallel_safe: true
modules: ["tui", "cli", "tests"]
priority: "MUST"
risk_level: "high"
tags: ["feat", "tui", "test"]
due: "2026-07-15"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Implement TUI mode framework triggered by `--interactive` or `--tui` flag. Allow users to view and modify arguments before execution, support interactive configuration, and provide TUI for complex operations. Addresses ADR #9 from CLI standards.

## Sub-Tasks

- [x] Add ratatui dependency to Cargo.toml
- [x] Add crossterm dependency to Cargo.toml
- [x] Create TUI framework module in `src/tui.rs`
- [x] Implement TUI event loop
- [x] Implement TUI layout system
- [x] Implement TUI argument editor
- [x] Implement TUI config editor
- [x] Add `--interactive` flag to CLI
- [x] Add `--tui` flag to CLI
- [x] Integrate TUI with argument parsing
- [x] Add TUI for install operations
- [x] Add TUI for config editing
- [x] Add TUI for batch operations
- [x] Implement terminal resize handling in TUI
- [x] Write unit tests for TUI components
- [x] Write integration tests for TUI mode
- [x] Update man pages with TUI documentation
- [x] Document TUI usage in README

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/tui.rs` — New TUI framework module
- `src/cli.rs` — Add --interactive and --tui flags
- `src/config.rs` — Integrate TUI config editor
- `src/install.rs` — Integrate TUI for install
- `tests/tui_test.rs` — New test file for TUI
- `Cargo.toml` — Add ratatui and crossterm dependencies
- `src/man.rs` — Update man pages
- `README.md` — Document TUI usage
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [x] TUI mode launches with --interactive or --tui flag
- [x] Users can view and modify arguments before execution
- [x] TUI supports interactive configuration
- [x] TUI handles terminal resize events
- [x] TUI works for install, config editing, and batch operations
- [x] All tests pass
- [x] Documentation complete

## Test Plan

- Unit: `devbox run cargo test tui_test`
- Integration: Manual TUI testing
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log TUI mode activation
- Track TUI errors and crashes

## Compliance

- Follow ADR #9 (TUI Mode)
- Ensure TUI respects terminal size (ADR #22)

## Risks & Mitigations

- Risk: TUI may be complex and error-prone — Mitigation: Keep TUI simple, use well-tested libraries, extensive testing
- Risk: TUI may not work on all terminals — Mitigation: Test on various terminals, provide fallback to CLI mode

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-02-003-terminal-size-awareness-tests]]
- Unblocks: 05-001

## Definition of Done

- TUI framework implemented and tested
- --interactive and --tui flags working
- TUI supports all required operations
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(tui): add TUI mode framework`

## Changelog

- 2026-06-11: initialized story file
