---
story_id: "03-003"
story_title: "Trash directory manager and index tracking"
story_name: "trash-manager"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 3
parallel_id: 3
branch: "feature/current/smartfo-initial-reqs/story-03-003-trash-manager"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-002", "01-003"]
parallel_safe: true
modules: ["trash.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "trash"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Implement the trash directory manager that computes versioned trash paths, preserves directory trees, and maintains a `.smartfo-index` JSONL file recording the full deletion history for each source path.

## Sub-Tasks

- [ ] Implement trash path computation: `$TRASH_ROOT/<abs-path>/<iso-timestamp>-<counter>`
- [ ] Implement parent directory creation in trash
- [ ] Implement atomic move into trash (same fs) or copy+unlink (cross-device)
- [ ] Implement `.smartfo-index` JSONL format (original path, timestamp, UUID, reason)
- [ ] Implement same-file history preservation (multiple deletes create timestamped subdirs)
- [ ] Implement trash retention and cleanup hooks
- [ ] Write unit tests for path computation, moves, and index tracking

## Relevant Files

- `src/trash.rs` — Trash mover and versioned directory management
- `src/trash.test.rs` — Unit tests for trash operations

## Acceptance Criteria

- [ ] Deleting `/home/user/foo.txt` creates trash path like `.../home/user/foo.txt/2026-06-04T09:15:30Z-001/foo.txt`
- [ ] Same file deleted twice creates separate timestamped entries
- [ ] `.smartfo-index` records all metadata for each deletion
- [ ] Parent directories are created as needed
- [ ] Trash root defaults to `$XDG_DATA_HOME/smartfo/trash`
- [ ] Cross-device trash moves use copy+fsync+unlink

## Test Plan

- Unit: `cargo test trash::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log trash path and index update at `debug` level
- Log disk usage of trash periodically

## Compliance

- None

## Risks & Mitigations

- Risk: Name collisions from rapid successive deletes — Mitigation: Use counter suffix after timestamp

## Dependencies & Sequencing

- Depends on: 01-002, 01-003
- Unblocks: 04-003, 05-002

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(trash): add versioned path computation`

## Changelog

- 2026-06-05: initialized story file
