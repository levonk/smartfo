---
story_id: "04-001"
story_title: "Durable job queue with SQLite WAL"
story_name: "job-queue"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 4
parallel_id: 1
branch: "feature/current/smartfo-initial-reqs/story-04-001-job-queue"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-002", "01-003"]
parallel_safe: true
modules: ["queue.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "queue"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Implement the durable job queue using SQLite WAL mode. Each job tracks UUID, source path, destination path, status (queued/running/done/failed), retry count, and operation type. The queue survives process restarts.

## Sub-Tasks

- [ ] Define SQLite schema for jobs table (UUID, source, dest, status, retry_count, op_type, created_at, updated_at)
- [ ] Implement queue initialization with WAL mode
- [ ] Implement `enqueue()` for adding jobs
- [ ] Implement `dequeue()` for claiming the next pending job
- [ ] Implement `mark_done()` and `mark_failed()` with retry increment
- [ ] Implement `get_status()` for polling job state
- [ ] Implement crash recovery on startup (restart in-flight jobs or mark failed)
- [ ] Write unit tests for queue operations and crash recovery

## Relevant Files

- `src/queue.rs` — Job queue schema and operations
- `src/queue.test.rs` — Unit tests for queue logic

## Acceptance Criteria

- [ ] Jobs survive SQLite connection close and reopen
- [ ] WAL mode is enabled for concurrent read/write safety
- [ ] In-flight jobs are recovered on daemon restart
- [ ] Failed jobs are retried up to max retry count
- [ ] Queue supports at least 1000 pending jobs without performance degradation
- [ ] Job status transitions are atomic

## Test Plan

- Unit: `cargo test queue::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log queue depth and worker status at `debug` level
- Log job lifecycle events (enqueue, start, done, fail) at `trace` level

## Compliance

- None

## Risks & Mitigations

- Risk: SQLite file corruption on power loss — Mitigation: WAL mode provides crash safety; test recovery paths

## Dependencies & Sequencing

- Depends on: 01-002, 01-003
- Unblocks: 04-002, 04-003

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(queue): add SQLite WAL job queue`

## Changelog

- 2026-06-05: initialized story file
