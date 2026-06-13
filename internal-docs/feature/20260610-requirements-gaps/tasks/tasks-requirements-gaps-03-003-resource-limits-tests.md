---
story_id: "03-003"
story_title: "Resource limits + tests"
story_name: "resource-limits-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 3
parallel_id: 3
branch: "feature/current/requirements-gaps/story-03-003-resource-limits-tests"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["02-002"]
parallel_safe: true
modules: ["daemon", "config", "tests"]
priority: "SHOULD"
risk_level: "medium"
tags": ["feat", "daemon", "config", "test"]
due: "2026-07-15"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Implement resource limits for memory and CPU-intensive operations. Add `--max-memory` and `--max-cpu` flags, document resource usage guidelines, and implement resource limiting for daemon operations. Addresses ADR #26 from CLI standards.

## Sub-Tasks

- [x] Add `--max-memory` flag to CLI
- [x] Add `--max-cpu` flag to CLI
- [x] Implement memory limit enforcement in daemon
- [x] Implement CPU limit enforcement in daemon
- [x] Add resource limit config options
- [x] Add resource monitoring to daemon
- [x] Add resource limit violation handling
- [x] Document resource usage guidelines
- [x] Write unit tests for resource limits
- [ ] Write integration tests for memory limits
- [ ] Write integration tests for CPU limits
- [x] Update man pages with resource limit documentation
- [x] Update help text with resource limit flags

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/cli.rs` — Add --max-memory and --max-cpu flags
- `src/daemon.rs` — Implement resource limiting
- `src/worker.rs` — Add resource monitoring
- `src/config.rs` — Add resource limit config options
- `tests/resource_limits_test.rs` — New test file for resource limits
- `src/man.rs` — Update man pages
- `README.md` — Document resource usage guidelines
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [x] --max-memory flag enforces memory limits
- [x] --max-cpu flag enforces CPU limits
- [x] Resource limits work in daemon operations
- [x] Resource limit violations handled gracefully
- [x] Resource usage guidelines documented
- [x] All tests pass
- [x] Documentation complete

## Test Plan

- Unit: `devbox run cargo test resource_limits_test`
- Integration: Test with operations that exceed limits
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log resource limit violations
- Track resource usage over time
- Monitor daemon resource consumption

## Compliance

- Follow ADR #26 (Resource Limits)
- Ensure resource limiting doesn't break critical operations

## Risks & Mitigations

- Risk: Resource limiting may be platform-specific — Mitigation: Use cross-platform libraries, provide graceful degradation
- Risk: Resource limits may be too restrictive — Mitigation: Use sensible defaults, allow override

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-02-002-health-check-mechanism-tests]]
- Unblocks: 04-001

## Definition of Done

- Resource limits implemented and tested
- Flags and config options working
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(daemon): add resource limits`

## Changelog

- 2026-06-11: initialized story file
