---
story_id: "04-003"
story_title: "Background worker — move/copy/fsync/retry"
story_name: "worker"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 4
parallel_id: 3
branch: "feature/current/smartfo-initial-reqs/story-04-003-worker"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["03-003", "04-001", "04-002"]
parallel_safe: true
modules: ["worker.rs"]
priority: "MUST"
risk_level: "high"
tags: ["feat", "worker"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Implement the background worker that processes queued jobs: atomic rename for same-filesystem moves, chunked copy + fsync + unlink for cross-device moves, and retry with exponential backoff on failure.

## Sub-Tasks

- [ ] Implement same-filesystem atomic rename worker
- [ ] Implement cross-device detection via `statfs` (or `nix::sys::statfs`)
- [ ] Implement chunked streaming copy with configurable buffer size
- [ ] Implement `fsync` on destination file and containing directory
- [ ] Implement safe temp-file + rename fallback for atomic cross-device moves
- [ ] Implement exponential backoff retry for failed jobs
- [ ] Integrate with trash manager for rm jobs
- [ ] Write unit tests for copy, fsync, and retry logic

## Relevant Files

- `src/worker.rs` — Background worker implementation
- `src/worker.test.rs` — Unit tests for worker operations
- `src/trash.rs` — Trash manager integration

## Acceptance Criteria

- [ ] Same-filesystem jobs use atomic `rename()`
- [ ] Cross-device jobs use copy + fsync + unlink
- [ ] Failed jobs are retried up to max retry count with exponential backoff
- [ ] Partial writes are safe (temp file + rename pattern)
- [ ] Worker marks jobs done/failed in the queue
- [ ] Large file moves stream without loading entire file into memory

## Test Plan

- Unit: `cargo test worker::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log job start, completion, and failure at `info` level
- Log retry attempts at `warn` level

## Compliance

- None

## Risks & Mitigations

- Risk: Copying massive files may exhaust memory — Mitigation: Use fixed-size chunk buffer and stream

## Dependencies & Sequencing

- Depends on: 03-003, 04-001, 04-002
- Unblocks: 05-002, 05-003, 06-001, 06-002

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(worker): add cross-device chunked copy`

## Changelog

- 2026-06-05: initialized story file
