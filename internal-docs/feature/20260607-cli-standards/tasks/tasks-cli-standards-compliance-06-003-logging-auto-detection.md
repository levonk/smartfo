---
story_id: "06-003"
story_title: "Structured Logging Auto-Detection"
story_name: "logging-auto-detection"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 6
parallel_id: 3
branch: "feature/current/cli-standards-compliance/story-06-003-logging-auto-detection"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["02-001"]
parallel_safe: true
modules: ["logging.rs"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "logging"]
due: "2026-08-30"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement structured logging with format auto-detection as specified in ADR #30. Use structured logging (JSON or structured text) with format auto-detection based on TTY, support language-native env filters (RUST_LOG), resolve log level from env vars > CLI flags > config file > defaults, and enhance existing structured logging in src/logging.rs.

## Sub-Tasks

- [ ] Audit existing structured logging implementation in src/logging.rs
- [ ] Implement TTY detection for logging format auto-detection
- [ ] Implement JSON logging format for non-TTY output
- [ ] Implement structured text logging format for TTY output
- [ ] Add --log-format flag to explicitly override auto-detection
- [ ] Support RUST_LOG environment variable for log level filtering
- [ ] Implement log level resolution: env vars > CLI flags > config file > defaults
- [ ] Ensure structured logging includes timestamps, levels, and metadata
- [ ] Ensure JSON logging is parseable by standard log aggregation tools
- [ ] Ensure structured text logging is human-readable
- [ ] Add unit tests for TTY detection
- [ ] Add unit tests for JSON logging format
- [ ] Add unit tests for structured text logging format
- [ ] Add unit tests for log level resolution
- [ ] Add integration tests for RUST_LOG support
- [ ] Add integration tests for --log-format flag

## Relevant Files

- `src/logging.rs` — Enhance with format auto-detection and log level resolution
- `src/cli.rs` — Add --log-format flag
- `src/config.rs` — Add log_format config option
- `tests/logging_tests.rs` — Add tests for logging auto-detection

## Acceptance Criteria

- [ ] Structured logging uses JSON format for non-TTY output
- [ ] Structured logging uses structured text format for TTY output
- [ ] Format auto-detection is based on TTY
- [ ] --log-format flag overrides auto-detection
- [ ] RUST_LOG environment variable is supported for log level filtering
- [ ] Log level resolution follows: env vars > CLI flags > config file > defaults
- [ ] Structured logging includes timestamps, levels, and metadata
- [ ] JSON logging is parseable by log aggregation tools
- [ ] Structured text logging is human-readable
- [ ] All tests pass

## Test Plan

- Unit: `cargo test logging_tests::tty_detection`
- Unit: `cargo test logging_tests::json_format`
- Unit: `cargo test logging_tests::structured_text_format`
- Unit: `cargo test logging_tests::log_level_resolution`
- Integration: `cargo test logging_tests::rust_log_support`
- Integration: `cargo test logging_tests::log_format_override`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log logging format selection (debug level)
- Log log level resolution (debug level)

## Compliance

- Follows ADR #30: Structured Logging with Format Auto-Detection

## Risks & Mitigations

- Risk: Auto-detection may not work in all environments — Mitigation: Provide explicit --log-format flag override
- Risk: JSON format may not be human-readable — Mitigation: Use structured text for TTY by default

## Dependencies

- 02-001 (Logging Modes Implementation) — ensures logging infrastructure exists

## Notes

- Use tracing crate for structured logging (already a dependency)
- Consider adding --pretty flag for pretty-printed JSON output
- Ensure JSON format is compatible with common log aggregators (ELK, Splunk, etc.)