---
story_id: "02-002"
story_title: "Signals & Exit Codes"
story_name: "signals-exit-codes"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 2
parallel_id: 2
branch: "feature/current/cli-standards-compliance/story-02-002-signals-exit-codes"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["main.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "signals"]
due: "2026-07-05"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement signal handling and standard exit codes as specified in ADR #8. Handle SIGINT gracefully with exit code 130, and define standard exit codes for all error scenarios (0: success, 1: generic error, 2: usage error, 3: network error, 4: validation error, 5: file not found, 6: permission denied, 7: VCS operation failed, 8: daemon operation failed).

## Sub-Tasks

- [x] Define exit code enum with all standard codes (0-8)
- [x] Implement SIGINT signal handler using nix crate
- [x] Set exit code 130 on SIGINT (standard Unix convention)
- [x] Implement graceful shutdown on SIGINT (in-flight jobs complete before exit)
- [x] Add exit code 0 for successful operations
- [x] Add exit code 1 for generic errors
- [x] Add exit code 2 for usage errors (invalid flags, missing arguments)
- [x] Add exit code 3 for network errors (if applicable)
- [x] Add exit code 4 for validation errors (config validation, argument validation)
- [x] Add exit code 5 for file not found errors
- [x] Add exit code 6 for permission denied errors
- [x] Add exit code 7 for VCS operation failures
- [x] Add exit code 8 for daemon operation failures
- [x] Audit all error paths to ensure appropriate exit codes
- [x] Add unit tests for each exit code scenario
- [x] Add integration tests for SIGINT handling

## Relevant Files

- `src/main.rs` — Implement signal handlers and exit code logic
- `src/exit.rs` — New module for exit codes and signal handling
- `src/cli.rs` — Return usage errors with exit code 2
- `src/config.rs` — Return validation errors with exit code 4
- `src/vcs.rs` — Return VCS errors with exit code 7
- `src/daemon.rs` — Return daemon errors with exit code 8
- `src/lib.rs` — Export exit module for testing
- `tests/signal_tests.rs` — Add tests for signal handling
- `Cargo.toml` — Add signal_tests test target

## Acceptance Criteria

- [x] SIGINT is handled gracefully with exit code 130
- [x] Exit code 0 is used for successful operations
- [x] Exit code 1 is used for generic errors
- [x] Exit code 2 is used for usage errors
- [x] Exit code 3 is used for network errors
- [x] Exit code 4 is used for validation errors
- [x] Exit code 5 is used for file not found errors
- [x] Exit code 6 is used for permission denied errors
- [x] Exit code 7 is used for VCS operation failures
- [x] Exit code 8 is used for daemon operation failures
- [x] All error paths use appropriate exit codes
- [x] All tests pass

## Test Plan

- Unit: `cargo test signal_tests::sigint_handler`
- Unit: `cargo test signal_tests::exit_code_success`
- Unit: `cargo test signal_tests::exit_code_usage_error`
- Unit: `cargo test signal_tests::exit_code_validation_error`
- Unit: `cargo test signal_tests::exit_code_file_not_found`
- Unit: `cargo test signal_tests::exit_code_permission_denied`
- Unit: `cargo test signal_tests::exit_code_vcs_failed`
- Unit: `cargo test signal_tests::exit_code_daemon_failed`
- Integration: `cargo test signal_tests::graceful_shutdown`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log signal reception (info level)
- Log exit codes on termination (debug level)

## Compliance

- Follows ADR #8: Signals & Exit Codes

## Risks & Mitigations

- Risk: SIGINT may leave daemon in inconsistent state — Mitigation: Ensure graceful shutdown completes in-flight jobs before exit
- Risk: Exit codes may conflict with custom scripts — Mitigation: Document all exit codes clearly in man pages

## Dependencies

None

## Notes

- Use nix crate for signal handling (already a dependency)
- Consider adding SIGTERM handler for daemon mode
- Ensure exit codes are documented in help output