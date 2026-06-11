---
story_id: "01-003"
story_title: "Property test extensions"
story_name: "property-test-extensions"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 1
parallel_id: 3
branch: "feature/current/requirements-gaps/story-01-003-property-test-extensions"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["tests", "property"]
priority: "SHOULD"
risk_level: "low"
tags: ["test", "property"]
due: "2026-07-01"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Extend property-based testing to cover new features including privacy mode, resource limits, daemon operations, and AXI features. Property tests provide safety guarantees by testing invariants across many random inputs.

## Sub-Tasks

- [x] Add property tests for privacy mode invariants in `tests/property/privacy_mode.rs`
- [x] Add property tests for resource limits invariants in `tests/property/resource_limits.rs`
- [x] Add property tests for daemon queue invariants in `tests/property/daemon_queue.rs`
- [x] Add property tests for TOON format invariants in `tests/property/toon_format.rs`
- [x] Add property tests for audit log invariants in `tests/property/audit_log.rs`
- [x] Add property tests for config reload invariants in `tests/property/config_reload.rs`
- [x] Add property tests for session hooks invariants in `tests/property/session_hooks.rs`
- [x] Add property tests for content truncation invariants in `tests/property/content_truncation.rs`
- [x] Add property tests for cross-platform path invariants in `tests/property/cross_platform_paths.rs`
- [x] Document property test patterns in `tests/property/README.md`
- [x] Run property tests with increased iterations to verify robustness

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `tests/property/privacy_mode.rs` — Property tests for privacy mode (already existed)
- `tests/property/resource_limits.rs` — Property tests for resource limits (already existed)
- `tests/property/daemon_queue.rs` — Property tests for daemon queue (already existed)
- `tests/property/toon_format.rs` — Property tests for TOON format (already existed)
- `tests/property/audit_log.rs` — Property tests for audit log (already existed)
- `tests/property/config_reload.rs` — New property tests for config reload
- `tests/property/session_hooks.rs` — New property tests for session hooks
- `tests/property/content_truncation.rs` — New property tests for content truncation
- `tests/property/cross_platform_paths.rs` — New property tests for cross-platform paths
- `tests/property/README.md` — Property test documentation
- `tests/property/mod.rs` — Updated to include new test modules

## Acceptance Criteria

- [x] All new property tests pass with high iteration counts (160 passed)
- [x] Property tests cover critical invariants for new features (config reload, session hooks, content truncation, cross-platform paths)
- [x] Property test patterns are documented (README.md created)
- [x] Property tests run efficiently in CI (completed in 511s)
- [x] Property tests find bugs (or validate correctness) (validated correctness of invariants)

## Test Plan

- Property: `devbox run cargo test --test property`
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Track property test execution time
- Log property test failures with seed for reproduction

## Compliance

- Ensure property tests follow proptest best practices
- No security vulnerabilities in test code

## Risks & Mitigations

- Risk: Property tests may be slow — Mitigation: Use appropriate iteration counts, run in parallel
- Risk: Property tests may not find meaningful bugs — Mitigation: Focus on critical invariants, review test strategies

## Dependencies & Sequencing

- Depends on: None
- Unblocks: 04-001, 04-002, 04-003, 07-001

## Definition of Done

- All property tests implemented and passing
- Property test patterns documented
- CI runs property tests efficiently
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `test(property): add privacy mode property tests`

## Changelog

- 2026-06-11: initialized story file
