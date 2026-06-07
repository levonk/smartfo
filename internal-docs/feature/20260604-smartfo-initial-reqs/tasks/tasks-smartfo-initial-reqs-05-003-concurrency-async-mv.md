---
story_id: "05-003"
story_title: "Concurrency and async mv triggers"
story_name: "concurrency-async-mv"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 5
parallel_id: 3
branch: "feature/current/smartfo-initial-reqs/story-05-003-concurrency-async-mv"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["03-001", "04-003"]
parallel_safe: true
modules: ["mv.rs", "worker.rs", "daemon.rs"]
priority: "SHOULD"
risk_level: "medium"
tags: ["feat", "concurrency", "async"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Implement async triggers for mv mode (cross-device, size threshold, `--async` flag) and the concurrency model that serializes same-dir moves, parallelizes cross-dir moves, and limits network-mounted destinations.

## Sub-Tasks

- [ ] Implement cross-device detection via `statfs` for mv mode
- [ ] Implement file size threshold check (default 100MB) for async mv
- [ ] Implement `--async` flag to force async for any mv
- [ ] Implement `--blocking` flag to force synchronous wait
- [ ] Implement same-filesystem same-directory serialization
- [ ] Implement same-filesystem different-directory parallelism (up to cpu_cores)
- [ ] Implement cross-device parallelism (up to destination_drive_count)
- [ ] Implement network-mounted destination limit (`network_concurrency`)
- [ ] Implement global `max_concurrent_jobs` ceiling
- [ ] Write unit tests for concurrency decisions

## Relevant Files

- `src/mv.rs` — Async trigger logic
- `src/worker.rs` — Constrained parallelism
- `src/daemon.rs` — Worker pool orchestration

## Acceptance Criteria

- [ ] Cross-device mv is automatically async
- [ ] Mv of file >100MB is automatically async
- [ ] `--async` forces async even for small/same-fs moves
- [ ] `--blocking` waits for completion in both mv and rm modes
- [ ] Same-dir moves are serialized
- [ ] Cross-dir moves are parallelized up to cpu cores
- [ ] Network mounts limited to `network_concurrency` (default 2)

## Test Plan

- Unit: `cargo test mv::` and `cargo test worker::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log async decision and concurrency level at `debug` level
- Log worker pool state periodically

## Compliance

- None

## Risks & Mitigations

- Risk: Too many parallel workers saturate IO — Mitigation: Cap at cpu cores and drive count

## Dependencies & Sequencing

- Depends on: 03-001, 04-003
- Unblocks: 06-001, 06-002

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(mv): add async threshold triggers`

## Changelog

- 2026-06-05: initialized story file
