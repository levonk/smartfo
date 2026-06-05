---
story_id: "03-001"
story_title: "mv mode — POSIX-compatible VCS-aware move"
story_name: "mv-mode"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 3
parallel_id: 1
branch: "feature/current/smartfo-initial-reqs/story-03-001-mv-mode"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-001", "01-002", "02-001"]
parallel_safe: true
modules: ["mv.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "mv"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Implement the move mode that handles all six VCS scenarios: tracked→same-repo (VCS mv), tracked→outside (refuse/force), outside→inside (fs move), both outside (rename), neither tracked (rename), and src==dest (no-op). Support `--plain` for exact POSIX behavior.

## Sub-Tasks

- [ ] Implement scenario router that classifies source/dest combinations
- [ ] Implement VCS-native move (`git mv`, `hg mv`, etc.) for tracked→same-repo
- [ ] Implement tracked→outside refusal with `--force-outside-vcs` override
- [ ] Implement outside→inside and both-outside pure filesystem rename
- [ ] Implement neither-tracked-in-repo filesystem rename
- [ ] Implement src==dest no-op with exit 0
- [ ] Implement dest-already-exists handling: `-n` refuse, `-f` overwrite, `-i` prompt, `--backup` suffix
- [ ] Implement `--plain` bypass (no VCS detection, exact POSIX behavior)
- [ ] Integrate audit logging for every move
- [ ] Write unit tests for all six scenarios

## Relevant Files

- `src/mv.rs` — Move logic and scenario routing
- `src/mv.test.rs` — Unit tests for move scenarios
- `src/audit.rs` — Audit logging integration

## Acceptance Criteria

- [ ] `git mv` is used when source is tracked and dest is in the same repo
- [ ] Moving tracked file outside repo refuses without `--force-outside-vcs`
- [ ] `--plain` behaves identically to GNU `mv` with no VCS awareness
- [ ] `-n` refuses when dest exists; `-f` overwrites; `-i` prompts
- [ ] `--backup` creates suffixed backup before overwriting
- [ ] Cross-device moves are detected via statfs (preparation for async)
- [ ] All scenarios return correct POSIX exit codes

## Test Plan

- Unit: `cargo test mv::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log scenario classification and chosen action at `debug` level
- Log VCS command output on failure at `warn` level

## Compliance

- None

## Risks & Mitigations

- Risk: VCS `mv` may fail with uncommitted changes — Mitigation: Fall back to filesystem move when `fallback_to_fs` config is true

## Dependencies & Sequencing

- Depends on: 01-001, 01-002, 02-001
- Unblocks: 04-003, 05-001, 05-003, 06-001, 06-002

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(mv): add VCS-aware move scenarios`

## Changelog

- 2026-06-05: initialized story file
