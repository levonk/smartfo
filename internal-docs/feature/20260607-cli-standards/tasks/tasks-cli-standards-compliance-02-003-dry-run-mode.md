---
story_id: "02-003"
story_title: "Dry-Run Mode"
story_name: "dry-run-mode"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 2
parallel_id: 3
branch: "feature/current/cli-standards-compliance/story-02-003-dry-run-mode"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["mv.rs", "rm.rs"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "dry-run"]
due: "2026-07-05"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement dry-run mode as specified in ADR #10. Add --dry-run flag to preview changes without executing them. Show exactly what would be done for each operation, including VCS commands that would be executed, file moves/deletes that would occur, and daemon operations that would be queued. Ensure dry-run mode has no side effects (no file system changes, no VCS commands).

## Sub-Tasks

- [ ] Add --dry-run flag to clap parsers in cli.rs for mv and rm modes
- [ ] Implement dry-run context struct to track dry-run state
- [ ] Modify mv.rs to check dry-run flag before executing file operations
- [ ] Modify rm.rs to check dry-run flag before enqueuing trash operations
- [ ] Modify vcs.rs to check dry-run flag before executing VCS commands
- [ ] Modify daemon.rs to check dry-run flag before enqueueing jobs
- [ ] Implement dry-run output showing what would be done
- [ ] Display VCS commands that would be executed in dry-run mode
- [ ] Display file moves that would occur in dry-run mode
- [ ] Display file deletes that would occur in dry-run mode
- [ ] Display daemon operations that would be queued in dry-run mode
- [ ] Ensure no file system changes occur in dry-run mode
- [ ] Ensure no VCS commands execute in dry-run mode
- [ ] Ensure no daemon jobs are enqueued in dry-run mode
- [ ] Add unit tests for dry-run mode in mv operations
- [ ] Add unit tests for dry-run mode in rm operations
- [ ] Add unit tests for dry-run mode with VCS operations
- [ ] Add unit tests for dry-run mode with daemon operations
- [ ] Verify no side effects in dry-run mode with integration tests

## Relevant Files

- `src/cli.rs` — Add --dry-run flag
- `src/mv.rs` — Implement dry-run checks and output
- `src/rm.rs` — Implement dry-run checks and output
- `src/vcs.rs` — Implement dry-run checks for VCS commands
- `src/daemon.rs` — Implement dry-run checks for job enqueueing
- `tests/dry_run_tests.rs` — Add tests for dry-run mode

## Acceptance Criteria

- [ ] --dry-run flag is available for mv and rm modes
- [ ] Dry-run mode displays exactly what would be done for each operation
- [ ] VCS commands that would be executed are displayed
- [ ] File moves that would occur are displayed
- [ ] File deletes that would occur are displayed
- [ ] Daemon operations that would be queued are displayed
- [ ] No file system changes occur in dry-run mode
- [ ] No VCS commands execute in dry-run mode
- [ ] No daemon jobs are enqueued in dry-run mode
- [ ] All tests pass

## Test Plan

- Unit: `cargo test dry_run_tests::mv_dry_run`
- Unit: `cargo test dry_run_tests::rm_dry_run`
- Unit: `cargo test dry_run_tests::vcs_dry_run`
- Unit: `cargo test dry_run_tests::daemon_dry_run`
- Integration: `cargo test dry_run_tests::no_side_effects`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log when dry-run mode is enabled (info level)
- Log operations that would be performed (info level in dry-run mode)

## Compliance

- Follows ADR #10: Dry-Run Mode

## Risks & Mitigations

- Risk: Dry-run mode may have unintended side effects — Mitigation: Comprehensive testing to ensure no state changes
- Risk: Dry-run output may be confusing — Mitigation: Clear, descriptive output with "would" phrasing

## Dependencies

None

## Notes

- Consider adding --dry-run to smartfo mode for install operations
- Ensure dry-run output is compatible with --json mode
- Use clear phrasing like "Would move file.txt to /dest/file.txt"