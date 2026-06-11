---
story_id: "03-001"
story_title: "Daemon Enhancements"
story_name: "daemon-enhancements"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 3
parallel_id: 1
branch: "feature/current/cli-standards-compliance/story-03-001-daemon-enhancements"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["02-002"]
parallel_safe: true
modules: ["daemon.rs", "queue.rs"]
priority: "MUST"
risk_level: "high"
tags: ["feat", "daemon"]
due: "2026-07-19"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Enhance daemon process support as specified in ADR #13. Provide --daemon and --no-daemon flags as opposites, maintain auto-spawn on first async operation, add --list-jobs command with optional job ID filtering, add --cancel-job <id> command, and implement platform fallback behavior with config variable override for noisy fallback messages.

## Sub-Tasks

- [x] Add --daemon flag to clap parsers in cli.rs for all modes
- [x] Add --no-daemon flag to clap parsers in cli.rs for all modes
- [x] Implement --daemon flag to pre-launch daemon in background and wait for jobs
- [x] Implement --no-daemon flag to force synchronous in-process operation (disable auto-spawning)
- [x] Add --list-jobs command to show background job status
- [x] Implement optional job ID list filtering for --list-jobs command
- [x] Add --cancel-job <id> command to cancel specific background jobs
- [x] Modify daemon.rs to return job ID immediately when daemon performs operation
- [x] Implement platform detection for daemon mode support
- [x] Implement fallback to synchronous processing when daemon mode not supported
- [x] Add clear error message explaining limitation when daemon mode not supported
- [x] Add suggestions for alternatives when --daemon is used on unsupported platform
- [x] Add daemon_fallback_quiet config variable to control noisy fallback behavior
- [x] Implement config variable override for platform-specific fallback messages
- [x] Add instructions for monitoring job progress in help output
- [x] Maintain existing Unix socket and PID lockfile infrastructure
- [x] Add unit tests for --daemon flag behavior
- [x] Add unit tests for --no-daemon flag behavior
- [x] Add unit tests for --list-jobs command
- [x] Add unit tests for --list-jobs with job ID filtering
- [x] Add unit tests for --cancel-job command
- [x] Add integration tests for platform fallback behavior
- [x] Add integration tests for daemon_fallback_quiet config variable

## Relevant Files

- `src/cli.rs` — Add --daemon, --no-daemon, --list-jobs, --cancel-job flags
- `src/daemon.rs` — Enhance with new daemon control flags and job management
- `src/queue.rs` — Add job filtering and cancellation logic
- `src/config.rs` — Add daemon_fallback_quiet config variable
- `tests/daemon_tests.rs` — Add tests for daemon enhancements

## Acceptance Criteria

- [ ] --daemon and --no-daemon flags work as opposites
- [ ] Auto-spawn daemon on first async operation is maintained
- [ ] --daemon flag pre-launches daemon in background and waits for jobs
- [ ] --no-daemon flag forces synchronous in-process operation
- [ ] Job ID is returned immediately when daemon performs operation
- [ ] --list-jobs command shows background job status
- [ ] --list-jobs supports optional job ID list filtering
- [ ] --cancel-job <id> command cancels specific background jobs
- [ ] Platform fallback to synchronous processing works correctly
- [ ] Clear error message explains limitation when daemon mode not supported
- [ ] Suggestions for alternatives are provided when --daemon is used on unsupported platform
- [ ] daemon_fallback_quiet config variable controls noisy fallback behavior
- [ ] All tests pass

## Test Plan

- Unit: `cargo test daemon_tests::daemon_flag`
- Unit: `cargo test daemon_tests::no_daemon_flag`
- Unit: `cargo test daemon_tests::list_jobs`
- Unit: `cargo test daemon_tests::list_jobs_filtering`
- Unit: `cargo test daemon_tests::cancel_job`
- Integration: `cargo test daemon_tests::platform_fallback`
- Integration: `cargo test daemon_tests::daemon_fallback_quiet_config`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log daemon spawn events (info level)
- Log job queue events (debug level)
- Log job cancellation events (info level)

## Compliance

- Follows ADR #13: Daemon Process Support

## Risks & Mitigations

- Risk: --no-daemon may break existing async-by-default behavior — Mitigation: Ensure auto-spawn is only disabled when --no-daemon is explicitly used
- Risk: Platform fallback may confuse users — Mitigation: Clear error messages and suggestions for alternatives

## Dependencies

- 02-002 (Signals & Exit Codes) — ensures proper signal handling for daemon operations

## Notes

- Maintain backward compatibility with existing daemon model
- Consider adding --daemon-status command for quick daemon health check
- Ensure job IDs are UUIDs for uniqueness
