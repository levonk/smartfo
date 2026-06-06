---
story_id: "03-002"
story_title: "rm mode — trash enqueueing and VCS-aware delete"
story_name: "rm-mode"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 3
parallel_id: 2
branch: "feature/current/smartfo-initial-reqs/story-03-002-rm-mode"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: ["01-001", "01-002", "02-001"]
parallel_safe: true
modules: ["rm.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "rm"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Implement rm mode that enqueues trash moves by default, handles VCS-committed files (VCS-aware delete when clean), ignored files (direct delete), and dirty files (trash + VCS delete). Support `--plain`, `--force-delete`, and `--blocking`.

## Sub-Tasks

- [x] Implement file classification: VCS-committed clean, VCS-committed dirty, ignored, untracked
- [x] Implement trash enqueue for untracked/non-ignored files (default async)
- [x] Implement VCS-aware remove (`git rm`) for clean committed files when `backup_vcs_committed = false`
- [x] Implement trash + VCS remove for dirty committed files
- [x] Implement direct delete for ignored files when `backup_ignored_files = false`
- [x] Implement `--force-delete` bypass (direct delete, no trash)
- [x] Implement `--plain` bypass (exact POSIX rm behavior)
- [x] Implement `--blocking` flag to wait for completion
- [x] Integrate audit logging for every delete
- [x] Write unit tests for classification and delete paths

## Relevant Files

- `src/rm.rs` — Trash enqueueing and VCS-aware delete logic
- `src/rm.test.rs` — Unit tests for rm mode
- `src/audit.rs` — Audit logging integration

## Acceptance Criteria

- [x] Untracked files are moved to trash by default (async)
- [x] Clean committed files use `git rm` without trash when configured
- [x] Dirty committed files go to trash AND get `git rm`
- [x] Ignored files are deleted directly without trash when configured
- [x] `--force-delete` bypasses trash entirely
- [x] `--plain` behaves exactly like POSIX `rm`
- [x] `--blocking` waits for the daemon to complete

## Test Plan

- Unit: `cargo test rm::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log classification decision at `debug` level
- Log enqueue status at `info` level

## Compliance

- None

## Risks & Mitigations

- Risk: Accidental deletion of uncommitted work — Mitigation: Trash is default; VCS-committed files only bypass trash when clean

## Dependencies & Sequencing

- Depends on: 01-001, 01-002, 02-001
- Unblocks: 04-003, 05-001, 06-001, 06-002

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(rm): add trash enqueueing logic`

## Changelog

- 2026-06-05: initialized story file
