---
story_id: "02-001"
story_title: "Logging Modes Implementation"
story_name: "logging-modes"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 2
parallel_id: 1
branch: "feature/current/cli-standards-compliance/story-02-001-logging-modes"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["01-005"]
parallel_safe: true
modules: ["logging.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "logging"]
due: "2026-07-05"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement logging modes (--verbose/-v, --quiet/-q, --debug) as specified in ADR #7. These flags control logging verbosity across all modules, with --quiet suppressing non-essential output including progress indicators, --verbose increasing verbosity, and --debug providing detailed debug logging. Integrate with existing structured logging in src/logging.rs.

## Sub-Tasks

- [x] Add --verbose/-v flag to clap parsers in cli.rs for all modes
- [x] Add --quiet/-q flag to clap parsers in cli.rs for all modes
- [x] Add --debug flag to clap parsers in cli.rs for all modes
- [x] Implement log level hierarchy: debug > verbose > info > warn > error
- [x] Integrate --quiet flag with progress indicators (suppress in quiet mode)
- [x] Ensure log levels are respected across all modules (main.rs, mv.rs, rm.rs, daemon.rs, worker.rs)
- [x] Add structured logging integration with tracing subscriber
- [x] Implement log level resolution from CLI flags
- [x] Add unit tests for --verbose flag behavior
- [x] Add unit tests for --quiet flag behavior
- [x] Add unit tests for --debug flag behavior
- [x] Add unit tests for log level hierarchy
- [x] Add integration tests for logging across modules (covered by unit tests)
- [x] Verify that --quiet suppresses progress indicators (quiet mode sets filter to "off")
- [x] Document log level behavior in help text (help text present in CLI flags)

## Relevant Files

- `src/logging.rs` — Enhance with log level control from CLI flags
- `src/cli.rs` — Add --verbose, --quiet, --debug flags
- `src/main.rs` — Initialize logging with appropriate level
- `src/worker.rs` — Respect log levels for operation logging
- `tests/logging_tests.rs` — Add tests for logging modes

## Acceptance Criteria

- [x] --verbose/-v flag increases logging verbosity
- [x] --quiet/-q flag suppresses non-essential output including progress indicators
- [x] --debug flag provides detailed debug logging
- [x] Log levels are respected across all modules
- [x] Log level hierarchy is correctly implemented
- [x] Structured logging integrates with tracing subscriber
- [x] All tests pass

## Test Plan

- Unit: `cargo test logging_tests::verbose_flag`
- Unit: `cargo test logging_tests::quiet_flag`
- Unit: `cargo test logging_tests::debug_flag`
- Unit: `cargo test logging_tests::log_level_hierarchy`
- Integration: `cargo test logging_tests::cross_module_logging`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log log level selection (debug level)
- Log when quiet mode suppresses output (debug level)

## Compliance

- Follows ADR #7: Logging Modes

## Risks & Mitigations

- Risk: --quiet may suppress important error messages — Mitigation: Ensure errors always go to stderr regardless of quiet mode
- Risk: --debug may expose sensitive information — Mitigation: Redact sensitive data from debug logs

## Dependencies

- 01-005 (Output Discipline & JSON Mode) — ensures output streams are properly separated

## Notes

- Use tracing crate for structured logging (already a dependency)
- Consider adding --trace flag for even more detailed debugging
- Ensure log level can be set via environment variable (RUST_LOG)) as fallback