---
story_id: "05-002"
story_title: "Disk space guard and auto-culling"
story_name: "disk-space-guard"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 5
parallel_id: 2
branch: "feature/current/smartfo-initial-reqs/story-05-002-disk-space-guard"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["03-003", "04-003"]
parallel_safe: true
modules: ["trash.rs", "worker.rs"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "trash", "safety"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Implement the disk space guard that checks free space before trash operations, auto-culls oldest trash entries when space is low, and respects `on_trash_full` and `allow_last_version_cull` config options.

## Sub-Tasks

- [ ] Implement free space check on the trash filesystem before each operation
- [ ] Implement oldest-first culling of trash entries when below `min_free_space_percent`
- [ ] Implement `allow_last_version_cull` gate (protect last version unless config allows)
- [ ] Implement `on_trash_full` behaviors: `"refuse"` and `"delete"`
- [ ] Integrate culling into the worker before trash moves
- [ ] Write unit tests for space guard and culling logic

## Relevant Files

- `src/trash.rs` — Culling logic
- `src/worker.rs` — Pre-operation space check integration

## Acceptance Criteria

- [ ] Operation is refused when free space < 20% and culling cannot free enough
- [ ] Oldest entries are culled first when space is low
- [ ] `on_trash_full = "delete"` bypasses trash with a warning
- [ ] Last version is preserved unless `allow_last_version_cull = true`
- [ ] Clear error message includes `--force-delete` hint

## Test Plan

- Unit: `cargo test trash::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log free space check and culling actions at `warn` level
- Log refused operations at `error` level

## Compliance

- None

## Risks & Mitigations

- Risk: Culling may remove data the user wanted — Mitigation: Default to refuse; only cull when configured and space is critical

## Dependencies & Sequencing

- Depends on: 03-003, 04-003
- Unblocks: 06-001, 06-002

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(trash): add disk space guard`

## Changelog

- 2026-06-05: initialized story file
