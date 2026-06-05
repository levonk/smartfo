---
story_id: "06-002"
story_title: "Property tests"
story_name: "property-tests"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 6
parallel_id: 2
branch: "feature/current/smartfo-initial-reqs/story-06-002-property-tests"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["03-001", "03-002", "04-003", "05-001", "05-002"]
parallel_safe: true
modules: ["tests/property/"]
priority: "SHOULD"
risk_level: "medium"
tags: ["test", "property"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Write property-based tests for safety guarantees: no data loss, directory tree preservation, VCS consistency, same-file deletion history, disk space guard ordering, audit validity, and install idempotency.

## Sub-Tasks

- [ ] Set up `proptest` or `quickcheck` framework
- [ ] Property: No data loss under any move/delete path
- [ ] Property: Directory trees preserved in trash
- [ ] Property: VCS state consistent after move (no uncommitted tracked file loss)
- [ ] Property: Same-file deletion history preserved across multiple deletes
- [ ] Property: Disk space guard culls oldest entries first
- [ ] Property: Audit log contains valid metadata for every operation
- [ ] Property: Git hooks correctly detect and block raw deletions and raw renames
- [ ] Property: `--install` correctly creates symlinks and hooks without overwriting existing files
- [ ] Property: `--force-delete` bypasses trash regardless of `trash_mode` or disk space
- [ ] Property: `trash_mode = "never"` performs direct delete without trash
- [ ] Property: `trash_mode = "auto"` falls back to direct delete when trash is full

## Relevant Files

- `tests/property/no_data_loss.rs`
- `tests/property/trash_preserve.rs`
- `tests/property/vcs_consistency.rs`
- `tests/property/audit_validity.rs`
- `tests/property/disk_space_culling.rs`
- `tests/property/hook_detection.rs`

## Acceptance Criteria

- [ ] All property tests pass with 100+ iterations each
- [ ] Shrinking produces minimal failing cases
- [ ] Coverage includes edge cases (empty files, deep nesting, unicode paths)
- [ ] Property tests run in CI with reasonable duration (<5 minutes total)

## Test Plan

- Property: `cargo test --test property`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Property test output should show iterations and shrinking behavior

## Compliance

- None

## Risks & Mitigations

- Risk: Property tests may be flaky due to filesystem timing — Mitigation: Use deterministic temp directory names and retry with backoff

## Dependencies & Sequencing

- Depends on: 03-001, 03-002, 04-003, 05-001, 05-002
- Unblocks: None

## Definition of Done

- All property tests pass; CI integration verified; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `test(property): add no-data-loss proptest`

## Changelog

- 2026-06-05: initialized story file
