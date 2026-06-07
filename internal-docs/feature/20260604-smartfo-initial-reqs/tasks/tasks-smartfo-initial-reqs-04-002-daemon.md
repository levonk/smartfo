---
story_id: "04-002"
story_title: "Self-spawning daemon with Unix socket"
story_name: "daemon"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 4
parallel_id: 2
branch: "feature/current/smartfo-initial-reqs/story-04-002-daemon"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: ["01-001", "04-001"]
parallel_safe: true
modules: ["daemon.rs"]
priority: "MUST"
risk_level: "high"
tags: ["feat", "daemon"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Implement the self-spawning daemon that double-forks on first async operation, writes a PID lockfile, and listens on a Unix domain socket for CLI-to-daemon communication. Handle graceful shutdown on SIGTERM.

## Sub-Tasks

- [x] Implement double-fork detachment for background daemon
- [x] Implement PID lockfile at `$XDG_DATA_HOME/smartfo/daemon.pid`
- [x] Implement Unix domain socket at `$XDG_DATA_HOME/smartfo/daemon.sock`
- [x] Implement CLI connection to existing daemon (spawn if absent)
- [x] Implement graceful shutdown on SIGTERM (complete in-flight jobs before exit)
- [x] Implement daemon health check / ping protocol
- [x] Write unit tests for lockfile and socket lifecycle

## Relevant Files

- `src/daemon.rs` — Daemon lifecycle, fork, socket, and signal handling
- `src/daemon.test.rs` — Unit tests for daemon mechanics

## Acceptance Criteria

- [x] First async operation spawns daemon automatically
- [x] Subsequent invocations connect to existing daemon via socket
- [x] Daemon survives parent process exit
- [x] `SIGTERM` triggers graceful shutdown after in-flight jobs complete
- [x] Dead daemon (stale PID) is detected and replaced automatically
- [x] Socket communication handles enqueue and status requests

## Test Plan

- Unit: `cargo test daemon::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log daemon start, fork, and shutdown at `info` level
- Log socket errors at `warn` level

## Compliance

- None

## Risks & Mitigations

- Risk: Stale lockfile prevents daemon startup — Mitigation: Check PID liveness before claiming lock
- Risk: Socket path too long for Unix domain socket — Mitigation: Use abstract namespace or hash-based short paths

## Dependencies & Sequencing

- Depends on: 01-001, 04-001
- Unblocks: 04-003, 05-003

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(daemon): add self-spawning double-fork`

## Changelog

- 2026-06-05: initialized story file
