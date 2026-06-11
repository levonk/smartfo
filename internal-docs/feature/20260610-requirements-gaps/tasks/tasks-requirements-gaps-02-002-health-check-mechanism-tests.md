---
story_id: "02-002"
story_title: "Health check mechanism + tests"
story_name: "health-check-mechanism-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 2
parallel_id: 2
branch: "feature/current/requirements-gaps/story-02-002-health-check-mechanism-tests"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-001"]
parallel_safe: true
modules: ["daemon", "health", "tests"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "daemon", "health", "test"]
due: "2026-07-08"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Implement health check mechanism for container orchestration (Docker HEALTHCHECK, Kubernetes probes). Supports HTTP endpoint or signal-based health checks with appropriate exit codes. Addresses ADR #32 from CLI standards.

## Sub-Tasks

- [ ] Design health check interface in `src/health.rs`
- [ ] Implement HTTP health check endpoint in daemon
- [ ] Implement signal-based health check (SIGUSR1)
- [ ] Add health check command to CLI
- [ ] Add health check validation logic (daemon operational state)
- [ ] Add appropriate exit codes for health status (0=healthy, 1=unhealthy)
- [ ] Write unit tests for health check logic
- [ ] Write integration tests for HTTP health check endpoint
- [ ] Write integration tests for signal-based health check
- [ ] Add Docker HEALTHCHECK configuration
- [ ] Add Kubernetes probe configuration examples
- [ ] Document health check usage in README

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/health.rs` — New module for health check implementation
- `src/daemon.rs` — Add health check endpoint to daemon
- `src/cli.rs` — Add health check command
- `tests/health_test.rs` — New test file for health checks
- `Dockerfile` — Add HEALTHCHECK instruction
- `README.md` — Document health check usage
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [ ] HTTP health check endpoint returns 200 when healthy
- [ ] Signal-based health check works with SIGUSR1
- [ ] Health check validates operational state without side effects
- [ ] Health check returns appropriate exit codes
- [ ] Works with Docker HEALTHCHECK
- [ ] Works with Kubernetes probes
- [ ] All tests pass

## Test Plan

- Unit: `devbox run cargo test health_test`
- Integration: Test HTTP endpoint with curl
- Integration: Test signal-based check with kill -USR1
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log health check requests
- Track health check failures

## Compliance

- Follow ADR #32 (Health Check for Containers)
- Ensure health checks are lightweight and fast

## Risks & Mitigations

- Risk: HTTP endpoint may add security exposure — Mitigation: Bind to localhost only, add authentication if needed
- Risk: Health check may be too slow — Mitigation: Keep checks minimal, timeout after 5 seconds

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-01-001-test-framework-enhancements]]
- Unblocks: 03-002, 03-003

## Definition of Done

- Health check mechanism implemented and tested
- HTTP and signal-based checks working
- Docker and Kubernetes configurations added
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(health): add health check mechanism`

## Changelog

- 2026-06-11: initialized story file
