---
story_id: "06-001"
story_title: "Integration tests"
story_name: "integration-tests"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 6
parallel_id: 1
branch: "feature/current/smartfo-initial-reqs/story-06-001-integration-tests"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["03-001", "03-002", "04-003", "05-001", "05-002", "05-003"]
parallel_safe: true
modules: ["tests/integration/"]
priority: "MUST"
risk_level: "medium"
tags: ["test", "integration"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Write comprehensive integration tests covering Git/hg/svn/jj repos, all six move scenarios, cross-device mounts, large file async mv, rm async behavior, crash recovery, and dest-already-exists overwrite modes.

## Sub-Tasks

- [ ] Create temp repo fixtures for Git, Mercurial, SVN, Jujutsu
- [ ] Test all six move scenarios with tracked and untracked files
- [ ] Test cross-device behavior with bind mounts or temp filesystems
- [ ] Test large file async mv (prompt return, queue completion)
- [ ] Test rm async behavior (prompt return, trash arrival)
- [ ] Test crash recovery (restart mid-move, verify resume or cleanup)
- [ ] Test dest-already-exists for all overwrite modes (`-n`, `-f`, `-i`, `--backup`)
- [ ] Test `--install` creates symlinks and hooks without overwriting
- [ ] Test `--force-delete` bypasses trash
- [ ] Test `trash_mode = "never"` and `"auto"`

## Relevant Files

- `tests/integration/mv_scenarios.rs`
- `tests/integration/rm_async.rs`
- `tests/integration/crash_recovery.rs`
- `tests/integration/cross_device.rs`
- `tests/integration/git_hooks.rs`
- `tests/integration/install.rs`

## Acceptance Criteria

- [ ] All six move scenarios have passing tests
- [ ] Cross-device streaming move is verified
- [ ] Async rm returns prompt immediately and file arrives in trash
- [ ] Crash recovery resumes or cleans up correctly
- [ ] All overwrite modes behave per POSIX spec
- [ ] Git hooks block raw deletions and raw renames
- [ ] `--install` with `--force` overwrites existing files safely

## Test Plan

- Integration: `cargo test --test integration`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Test output should clearly indicate which scenario failed

## Compliance

- None

## Risks & Mitigations

- Risk: Integration tests may be slow — Mitigation: Use temp directories and small files; gate large-file tests behind feature flags

## Dependencies & Sequencing

- Depends on: 03-001, 03-002, 04-003, 05-001, 05-002, 05-003
- Unblocks: None

## Definition of Done

- All tests pass; test coverage report generated; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `test(integration): add git mv scenario tests`

## Changelog

- 2026-06-05: initialized story file
