---
story_id: "02-005"
story_title: "Progress Indicators"
story_name: "progress-indicators"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 2
parallel_id: 5
branch: "feature/current/cli-standards-compliance/story-02-005-progress-indicators"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["02-001"]
parallel_safe: true
modules: ["worker.rs", "indicatif integration"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "progress"]
due: "2026-07-05"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement progress indicators as specified in ADR #12. Show progress bars or spinners for long-running operations. Display progress for async daemon operations when using --no-daemon mode (synchronous), and for large file copies (cross-device moves). Respect --quiet flag (no progress indicators in quiet mode). Use indicatif crate for progress bars.

## Sub-Tasks

- [x] Add indicatif crate to Cargo.toml dependencies
- [x] Implement progress bar wrapper for file operations
- [x] Add progress indicator for large file copies in worker.rs
- [x] Add progress indicator for cross-device moves in worker.rs
- [x] Add progress indicator for synchronous daemon operations (--blocking mode)
- [x] Add progress indicator for batch file operations
- [x] Ensure progress indicators respect --quiet flag (suppress in quiet mode)
- [x] Ensure progress indicators go to stderr (not stdout)
- [x] Implement spinner for short-duration operations
- [x] Implement progress bar for long-duration operations with percentage
- [x] Add estimated time remaining for long operations
- [x] Ensure progress indicators work with --json mode (suppress or format appropriately)
- [x] Add unit tests for progress bar functionality
- [x] Add unit tests for spinner functionality
- [x] Add integration tests for progress indicators with --quiet flag
- [x] Add integration tests for progress indicators with --json mode

## Relevant Files

- `Cargo.toml` — Add indicatif dependency
- `src/worker.rs` — Implement progress indicators for file operations
- `src/daemon.rs` — Implement progress indicators for synchronous operations
- `src/logging.rs` — Integrate progress indicator suppression with --quiet
- `tests/progress_tests.rs` — Add tests for progress indicators

## Acceptance Criteria

- [x] Progress bars or spinners are shown for long-running operations
- [x] Progress is displayed for async daemon operations in --no-daemon mode
- [x] Progress is displayed for large file copies (cross-device moves)
- [x] Progress indicators respect --quiet flag (suppressed in quiet mode)
- [x] Progress indicators go to stderr
- [x] Progress indicators work correctly with --json mode
- [x] Estimated time remaining is shown for long operations
- [x] All tests pass

## Test Plan

- Unit: `cargo test progress_tests::progress_bar_large_file`
- Unit: `cargo test progress_tests::spinner_short_operation`
- Unit: `cargo test progress_tests::quiet_mode_suppression`
- Unit: `cargo test progress_tests::json_mode_compatibility`
- Integration: `cargo test progress_tests::cross_device_move_progress`
- Integration: `cargo test progress_tests::synchronous_daemon_progress`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log progress indicator start/end (debug level)
- Log progress updates (trace level)

## Compliance

- Follows ADR #12: Progress Indicators

## Risks & Mitigations

- Risk: Progress indicators may impact performance — Mitigation: Use efficient progress updates (throttle to reasonable frequency)
- Risk: Progress indicators may clutter output in scripts — Mitigation: Respect --quiet and --json flags to suppress

## Dependencies

- 02-001 (Logging Modes Implementation) — ensures --quiet flag is available

## Notes

- Use indicatif crate for progress bars (industry standard)
- Consider adding --progress flag to force progress indicators even in quiet mode
- Ensure progress bars are disabled when output is not a TTY
