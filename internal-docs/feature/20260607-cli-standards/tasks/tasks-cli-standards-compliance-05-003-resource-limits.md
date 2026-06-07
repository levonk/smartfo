---
story_id: "05-003"
story_title: "Resource Limits Implementation"
story_name: "resource-limits"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 5
parallel_id: 3
branch: "feature/current/cli-standards-compliance/story-05-003-resource-limits"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["02-002"]
parallel_safe: true
modules: ["daemon.rs", "worker.rs"]
priority: "SHOULD"
risk_level: "medium"
tags: ["feat", "resources"]
due: "2026-08-16"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement resource limits as specified in ADR #26. Add --max-memory flag for memory-intensive operations, add --max-cpu flag for CPU-intensive operations, document memory/CPU usage guidelines, and implement resource limiting for daemon operations.

## Sub-Tasks

- [ ] Add --max-memory flag to clap parsers in cli.rs
- [ ] Add --max-cpu flag to clap parsers in cli.rs
- [ ] Implement memory tracking in worker.rs
- [ ] Implement CPU usage tracking in worker.rs
- [ ] Implement memory limit enforcement for daemon operations
- [ ] Implement CPU limit enforcement for daemon operations
- [ ] Add memory usage logging (debug level)
- [ ] Add CPU usage logging (debug level)
- [ ] Document memory usage guidelines in help output
- [ ] Document CPU usage guidelines in help output
- [ ] Document resource limits in man pages
- [ ] Add graceful degradation when limits are approached
- [ ] Add warning when limits are exceeded
- [ ] Add unit tests for memory limit enforcement
- [ ] Add unit tests for CPU limit enforcement
- [ ] Add integration tests for resource limiting behavior

## Relevant Files

- `src/cli.rs` — Add --max-memory and --max-cpu flags
- `src/worker.rs` — Implement resource tracking and limiting
- `src/daemon.rs` — Integrate resource limiting with daemon operations
- `docs/smartfo.1` — Document resource limits
- `tests/resource_tests.rs` — Add tests for resource limits

## Acceptance Criteria

- [ ] --max-memory flag is available and functional
- [ ] --max-cpu flag is available and functional
- [ ] Memory usage is tracked for daemon operations
- [ ] CPU usage is tracked for daemon operations
- [ ] Memory limits are enforced for daemon operations
- [ ] CPU limits are enforced for daemon operations
- [ ] Memory/CPU usage is logged (debug level)
- [ ] Memory/CPU usage guidelines are documented
- [ ] Graceful degradation occurs when limits are approached
- [ ] Warnings are issued when limits are exceeded
- [ ] All tests pass

## Test Plan

- Unit: `cargo test resource_tests::memory_tracking`
- Unit: `cargo test resource_tests::cpu_tracking`
- Unit: `cargo test resource_tests::memory_limit_enforcement`
- Unit: `cargo test resource_tests::cpu_limit_enforcement`
- Integration: `cargo test resource_tests::daemon_resource_limiting`
- Integration: `cargo test resource_tests::graceful_degradation`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log resource usage (debug level)
- Log limit violations (warn level)
- Log resource limit configuration (info level)

## Compliance

- Follows ADR #26: Resource Limits

## Risks & Mitigations

- Risk: Resource limiting may impact performance — Mitigation: Use efficient tracking mechanisms
- Risk: Limits may be too restrictive for some workloads — Mitigation: Provide sensible defaults and allow override

## Dependencies

- 02-002 (Signals & Exit Codes) — ensures proper handling when limits are exceeded

## Notes

- Use platform-specific APIs for resource limiting where available
- Consider adding resource usage statistics command
- Ensure resource limits work correctly with --no-daemon mode