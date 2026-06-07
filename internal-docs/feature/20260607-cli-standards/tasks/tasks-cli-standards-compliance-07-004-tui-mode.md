---
story_id: "07-004"
story_title: "TUI Mode Implementation"
story_name: "tui-mode"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 7
parallel_id: 4
branch: "feature/current/cli-standards-compliance/story-07-004-tui-mode"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-001", "04-004"]
parallel_safe: true
modules: ["tui.rs (new)", "ratatui integration"]
priority: "SHOULD"
risk_level: "high"
tags: ["feat", "tui"]
due: "2026-09-13"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement TUI mode as specified in ADR #9. Implement TUI mode triggered by --interactive or --tui flag, allow users to view and modify all arguments before execution, support interactive configuration of smartfo settings, provide TUI for complex operations (install, config editing, batch operations), use ratatui library for terminal UI, and ensure TUI mode respects terminal size and handles resize events.

## Sub-Tasks

- [ ] Add ratatui and crossterm crates to Cargo.toml dependencies
- [ ] Create new src/tui.rs module
- [ ] Add --interactive/-i flag to clap parsers for all modes
- [ ] Add --tui flag to clap parsers for all modes (alias for --interactive)
- [ ] Implement TUI argument viewer for current operation
- [ ] Implement TUI argument editor for modifying arguments before execution
- [ ] Implement TUI config editor for smartfo settings
- [ ] Implement TUI for install operation
- [ ] Implement TUI for config editing
- [ ] Implement TUI for batch operations
- [ ] Implement terminal size detection in TUI
- [ ] Implement resize event handling in TUI
- [ ] Add TUI navigation controls (arrow keys, Enter, Esc, etc.)
- [ ] Add TUI help screen
- [ ] Ensure TUI mode respects --quiet flag (disable TUI)
- [ ] Ensure TUI mode respects --json flag (disable TUI)
- [ ] Add unit tests for TUI components
- [ ] Add integration tests for TUI mode
- [ ] Document TUI mode in help output and man pages

## Relevant Files

- `Cargo.toml` — Add ratatui and crossterm dependencies
- `src/tui.rs` (new) — Implement TUI mode logic
- `src/cli.rs` — Add --interactive and --tui flags
- `src/main.rs` — Handle TUI mode dispatch
- `tests/tui_tests.rs` — Add tests for TUI mode

## Acceptance Criteria

- [ ] TUI mode is triggered by --interactive or --tui flag
- [ ] Users can view all arguments before execution
- [ ] Users can modify all arguments before execution
- [ ] Interactive configuration of smartfo settings is supported
- [ ] TUI for install operation exists
- [ ] TUI for config editing exists
- [ ] TUI for batch operations exists
- [ ] TUI mode respects terminal size
- [ ] TUI mode handles resize events
- [ ] TUI navigation controls work correctly
- [ ] TUI help screen exists
- [ ] TUI mode is disabled in --quiet mode
- [ ] TUI mode is disabled in --json mode
- [ ] All tests pass

## Test Plan

- Unit: `cargo test tui_tests::argument_viewer`
- Unit: `cargo test tui_tests::argument_editor`
- Unit: `cargo test tui_tests::config_editor`
- Unit: `cargo test tui_tests::resize_handling`
- Integration: `cargo test tui_tests::tui_mode_dispatch`
- Integration: `cargo test tui_tests::tui_install`
- Integration: `cargo test tui_tests::tui_batch_operations`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log TUI mode activation (info level)
- Log TUI user actions (debug level)

## Compliance

- Follows ADR #9: TUI Mode

## Risks & Mitigations

- Risk: TUI mode may not work on all terminals — Mitigation: Provide clear error message and fallback to CLI mode
- Risk: TUI implementation may be complex — Mitigation: Keep TUI simple and focused on common operations

## Dependencies

- 01-001 (Standard Arguments Implementation) — ensures argument structure is stable
- 04-004 (Terminal Size Awareness) — ensures terminal size detection is available

## Notes

- Use ratatui (formerly tui-rs) for terminal UI
- Use crossterm for cross-platform terminal handling
- Keep TUI simple and intuitive for first-time users
- Consider adding keyboard shortcuts documentation in TUI