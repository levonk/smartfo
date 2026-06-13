---
story_id: "04-001"
story_title: "Privacy mode + tests"
story_name: "privacy-mode-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 4
parallel_id: 1
branch: "feature/current/requirements-gaps/story-04-001-privacy-mode-tests"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["03-003"]
parallel_safe: true
modules: ["privacy", "config", "tests"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "privacy", "config", "test"]
due: "2026-07-22"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Implement privacy mode with explicit ignore lists for identifiers to never log or process. Distinguish between "unknown" (logged but not assigned) and "anonymous" (ignored entirely). Add configurable privacy toggles to disable specific data collection. Addresses ADR #33 from CLI standards.

## Sub-Tasks

- [x] Create privacy module in `src/privacy.rs`
- [x] Implement ignore list pattern matching
- [x] Add privacy mode config options
- [x] Add `--privacy` flag to CLI
- [x] Implement "unknown" vs "anonymous" distinction
- [x] Add privacy toggles for specific data collection
- [x] Integrate privacy mode with audit logging
- [x] Integrate privacy mode with session hooks
- [x] Add privacy mode to output formatting
- [x] Write unit tests for ignore list matching
- [ ] Write integration tests for privacy mode
- [x] Write tests for "unknown" vs "anonymous" behavior
- [x] Update man pages with privacy documentation
- [x] Update help text with --privacy flag
- [x] Document privacy mode in README

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/privacy.rs` — New privacy module
- `src/config.rs` — Add privacy config options
- `src/cli.rs` — Add --privacy flag
- `src/audit.rs` — Integrate privacy with audit logging
- `src/hooks.rs` — Integrate privacy with session hooks
- `src/output.rs` — Integrate privacy with output
- `tests/privacy_test.rs` — New test file for privacy mode
- `src/man.rs` — Update man pages
- `README.md` — Document privacy mode
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [ ] Privacy mode ignores specified patterns
- [ ] "Unknown" vs "anonymous" distinction works correctly
- [ ] Privacy toggles disable specific data collection
- [ ] --privacy flag enables privacy for single operation
- [ ] Privacy mode integrates with audit logging
- [ ] Privacy mode integrates with session hooks
- [ ] All tests pass
- [ ] Documentation complete

## Test Plan

- Unit: `devbox run cargo test privacy_test`
- Integration: Test privacy mode with various operations
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log privacy mode activation
- Track privacy violations
- Monitor ignored patterns

## Compliance

- Follow ADR #33 (Privacy Mode with Anonymous Lists)
- Ensure privacy mode doesn't break core functionality

## Risks & Mitigations

- Risk: Privacy mode may break audit trail — Mitigation: Document privacy implications, require explicit opt-in
- Risk: Ignore patterns may be too broad — Mitigation: Use precise patterns, provide examples

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-03-003-resource-limits-tests]]
- Unblocks: 04-002, 04-003

## Definition of Done

- Privacy mode implemented and tested
- Ignore lists working
- Privacy toggles functional
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(privacy): add privacy mode with ignore lists`

## Changelog

- 2026-06-11: initialized story file
