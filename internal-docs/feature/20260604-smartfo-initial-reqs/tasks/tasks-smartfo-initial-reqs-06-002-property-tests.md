---
story_id: "06-002"
story_title: "Property tests"
story_name: "property-tests"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 6
parallel_id: 2
branch: "feature/current/smartfo-initial-reqs/story-06-002-property-tests"
status: "in_progress"
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

- [x] Set up `proptest` or `quickcheck` framework
- [x] Property: No data loss under any move/delete path
- [x] Property: Directory trees preserved in trash
- [x] Property: VCS state consistent after move (no uncommitted tracked file loss)
- [x] Property: Same-file deletion history preserved across multiple deletes
- [x] Property: Disk space guard culls oldest entries first
- [x] Property: Audit log contains valid metadata for every operation
- [x] Property: Git hooks correctly detect and block raw deletions and raw renames
- [x] Property: `--install` correctly creates symlinks and hooks without overwriting existing files
- [x] Property: `--force-delete` bypasses trash regardless of `trash_mode` or disk space
- [x] Property: `trash_mode = "never"` performs direct delete without trash
- [x] Property: `trash_mode = "auto"` falls back to direct delete when trash is full

## Relevant Files

- `tests/property/mod.rs` - Property test module declaration
- `tests/property/no_data_loss.rs` - Property tests for no data loss on move/copy
- `tests/property/trash_preserve.rs` - Property tests for directory tree preservation
- `tests/property/vcs_consistency.rs` - Property tests for VCS state consistency
- `tests/property/same_file_history.rs` - Property tests for same-file deletion history
- `tests/property/audit_validity.rs` - Property tests for audit log metadata validity
- `tests/property/disk_space_culling.rs` - Property tests for disk space culling ordering
- `tests/property/hook_detection.rs` - Property tests for Git hook detection
- `tests/property/install_idempotency.rs` - Property tests for install idempotency
- `tests/property/force_delete.rs` - Property tests for --force-delete behavior
- `tests/property/trash_mode.rs` - Property tests for trash_mode behavior
- `Cargo.toml` - Added proptest, fs_extra, and serde_json dev dependencies

## Acceptance Criteria

- [x] All property tests pass with 100+ iterations each
- [x] Shrinking produces minimal failing cases
- [x] Coverage includes edge cases (empty files, deep nesting, unicode paths)
- [x] Property tests run in CI with reasonable duration (<5 minutes total)

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
