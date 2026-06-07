---
story_id: "01-005"
story_title: "Output Discipline & JSON Mode"
story_name: "output-discipline"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 1
parallel_id: 5
branch: "feature/current/cli-standards-compliance/story-01-005-output-discipline"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["logging.rs", "cli.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "output"]
due: "2026-06-21"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement output discipline as specified in ADR #6: ensure results go to stdout, logs/progress/errors go to stderr, add --json output mode for machine-readable output, implement --color=auto|always|never flag with smart TTY detection, add color config file setting, and honor NO_COLOR environment variable (takes precedence over all other color settings).

## Sub-Tasks

- [ ] Audit all output to ensure results go to stdout
- [ ] Audit all logging to ensure logs go to stderr
- [ ] Audit all progress indicators to ensure they go to stderr
- [ ] Audit all error messages to ensure they go to stderr
- [ ] Add --json flag to clap parsers for all modes
- [ ] Implement JSON output mode for operation results
- [ ] Implement structured JSON serialization for all result types
- [ ] Add --color=auto|always|never flag to clap parsers for all modes
- [ ] Implement smart TTY detection for auto mode color control
- [ ] Add color setting to config schema with modes: auto|always|never
- [ ] Implement color config file loading and application
- [ ] Add NO_COLOR environment variable detection
- [ ] Implement NO_COLOR precedence over all other color settings
- [ ] Add unit tests for output discipline (results to stdout, logs to stderr)
- [ ] Add unit tests for JSON output mode
- [ ] Add unit tests for --color flag with all modes
- [ ] Add unit tests for smart TTY detection in auto mode
- [ ] Add unit tests for color config file setting
- [ ] Add unit tests for NO_COLOR environment variable precedence

## Relevant Files

- `src/logging.rs` — Ensure all logging goes to stderr
- `src/cli.rs` — Add --json flag, TTY detection, NO_COLOR handling
- `src/main.rs` — Ensure output routing is correct
- `tests/output_tests.rs` — Add tests for output discipline

## Acceptance Criteria

- [ ] All results go to stdout
- [ ] All logs go to stderr
- [ ] All progress indicators go to stderr
- [ ] All error messages go to stderr
- [ ] --json flag produces machine-readable JSON output
- [ ] --color=auto|always|never flag works correctly with smart TTY detection
- [ ] Color config file setting (auto|always|never) is respected
- [ ] NO_COLOR environment variable takes precedence over all other color settings
- [ ] All tests pass

## Test Plan

- Unit: `cargo test output_tests::stdout_results`
- Unit: `cargo test output_tests::stderr_logs`
- Unit: `cargo test output_tests::json_output`
- Unit: `cargo test output_tests::tty_color_detection`
- Unit: `cargo test output_tests::no_color_env`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log output mode selection (debug level)
- Log TTY detection results (debug level)

## Compliance

- Follows ADR #6: Output Discipline

## Risks & Mitigations

- Risk: Existing scripts may depend on current output behavior — Mitigation: Maintain backward compatibility for default output format
- Risk: JSON schema may change unexpectedly — Mitigation: Document JSON schema and provide versioning

## Dependencies

None

## Notes

- Use serde for JSON serialization
- Consider adding --pretty flag for human-readable JSON output
- Ensure JSON output is parseable by standard tools (jq, etc.)