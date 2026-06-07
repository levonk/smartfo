---
story_id: "07-001"
story_title: "Health Check Implementation"
story_name: "health-check"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 7
parallel_id: 1
branch: "feature/current/cli-standards-compliance/story-07-001-health-check"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["03-001"]
parallel_safe: true
modules: ["health.rs (new)", "daemon.rs"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "health"]
due: "2026-09-13"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement health check for containers as specified in ADR #32. Provide health check mechanism for container orchestration, support HTTP endpoint or signal-based health check, validate operational state without side effects, work with Docker HEALTHCHECK and Kubernetes probes, and return appropriate exit codes for health status.

## Sub-Tasks

- [ ] Create new src/health.rs module
- [ ] Define health check status (healthy, unhealthy, degraded)
- [ ] Implement HTTP endpoint for health check (using actix-web or hyper)
- [ ] Implement signal-based health check as fallback
- [ ] Add --health-check flag to trigger health check
- [ ] Implement operational state validation without side effects
- [ ] Check daemon status in health check
- [ ] Check queue status in health check
- [ ] Check disk space in health check
- [ ] Return exit code 0 for healthy status
- [ ] Return exit code 1 for unhealthy status
- [ ] Return exit code 2 for degraded status
- [ ] Ensure health check works with Docker HEALTHCHECK
- [ ] Ensure health check works with Kubernetes probes (liveness, readiness, startup)
- [ ] Add unit tests for health check logic
- [ ] Add integration tests for HTTP endpoint health check
- [ ] Add integration tests for signal-based health check
- [ ] Document health check in help output and man pages

## Relevant Files

- `src/health.rs` (new) — Implement health check logic
- `src/daemon.rs` — Integrate health check with daemon status
- `src/cli.rs` — Add --health-check flag
- `src/main.rs` — Handle health check command
- `tests/health_tests.rs` — Add tests for health check

## Acceptance Criteria

- [ ] Health check mechanism exists for container orchestration
- [ ] HTTP endpoint is available for health check
- [ ] Signal-based health check is available as fallback
- [ ] --health-check flag triggers health check
- [ ] Operational state is validated without side effects
- [ ] Daemon status is checked
- [ ] Queue status is checked
- [ ] Disk space is checked
- [ ] Exit code 0 is returned for healthy status
- [ ] Exit code 1 is returned for unhealthy status
- [ ] Exit code 2 is returned for degraded status
- [ ] Health check works with Docker HEALTHCHECK
- [ ] Health check works with Kubernetes probes
- [ ] All tests pass

## Test Plan

- Unit: `cargo test health_tests::health_check_logic`
- Unit: `cargo test health_tests::status_validation`
- Integration: `cargo test health_tests::http_endpoint`
- Integration: `cargo test health_tests::signal_based`
- Integration: `cargo test health_tests::docker_healthcheck`
- Integration: `cargo test health_tests::kubernetes_probes`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log health check requests (info level)
- Log health check results (debug level)

## Compliance

- Follows ADR #32: Health Check for Containers

## Risks & Mitigations

- Risk: HTTP endpoint may add security surface — Mitigation: Bind to localhost only, add authentication if needed
- Risk: Health check may be too slow — Mitigation: Keep health check lightweight and fast (<100ms)

## Dependencies

- 03-001 (Daemon Enhancements) — ensures daemon status is available for health check

## Notes

- Use actix-web or hyper for HTTP endpoint (lightweight)
- Consider adding --health-check-port flag to customize port
- Ensure health check works in both CLI and daemon modes