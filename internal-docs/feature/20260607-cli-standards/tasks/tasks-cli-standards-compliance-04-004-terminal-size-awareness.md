---
story_id: "04-004"
story_title: "Terminal Size Awareness"
story_name: "terminal-size-awareness"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 4
parallel_id: 4
branch: "feature/current/cli-standards-compliance/story-04-004-terminal-size-awareness"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["output.rs"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "terminal"]
due: "2026-08-02"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement terminal size awareness as specified in ADR #22. Detect terminal size on startup, format output based on terminal width, handle terminal resize events where possible, and provide reasonable defaults for non-terminal output.

## Sub-Tasks

- [ ] Add terminal-size crate to Cargo.toml dependencies
- [ ] Implement terminal size detection on startup
- [ ] Detect terminal width and height
- [ ] Implement output formatting based on terminal width
- [ ] Add line wrapping for long output based on terminal width
- [ ] Implement table formatting that adapts to terminal width
- [ ] Handle terminal resize events where possible
- [ ] Provide reasonable defaults for non-terminal output (e.g., 80 columns)
- [ ] Ensure output formatting works with --json mode (disable wrapping)
- [ ] Ensure output formatting works with --quiet mode
- [ ] Add unit tests for terminal size detection
- [ ] Add unit tests for output formatting based on width
- [ ] Add unit tests for terminal resize handling
- [ ] Add integration tests for terminal size awareness

## Relevant Files

- `Cargo.toml` — Add terminal-size dependency
- `src/output.rs` — Implement terminal size detection and formatting
- `src/main.rs` — Initialize terminal size on startup
- `tests/terminal_tests.rs` — Add tests for terminal size awareness

## Acceptance Criteria

- [ ] Terminal size is detected on startup
- [ ] Output is formatted based on terminal width
- [ ] Line wrapping works based on terminal width
- [ ] Table formatting adapts to terminal width
- [ ] Terminal resize events are handled where possible
- [ ] Reasonable defaults are provided for non-terminal output
- [ ] Output formatting works with --json mode
- [ ] Output formatting works with --quiet mode
- [ ] All tests pass

## Test Plan

- Unit: `cargo test terminal_tests::size_detection`
- Unit: `cargo test terminal_tests::output_formatting`
- Unit: `cargo test terminal_tests::resize_handling`
- Integration: `cargo test terminal_tests::non_terminal_defaults`
- Integration: `cargo test terminal_tests::json_mode_compatibility`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log terminal size detection (debug level)
- Log terminal resize events (debug level)

## Compliance

- Follows ADR #22: Terminal Size Awareness

## Risks & Mitigations

- Risk: Terminal size detection may fail on some systems — Mitigation: Provide reasonable fallback defaults
- Risk: Resize handling may be complex — Mitigation: Implement basic resize handling, defer complex scenarios if needed

## Dependencies

None

## Notes

- Use terminal-size crate for cross-platform terminal detection
- Consider adding --width flag to override terminal width detection
- Ensure output remains readable on very small terminals (e.g., 40 columns)