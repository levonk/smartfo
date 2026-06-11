---
story_id: "01-001"
story_title: "Test framework enhancements"
story_name: "test-framework-enhancements"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 1
parallel_id: 1
branch: "feature/current/requirements-gaps/story-01-001-test-framework-enhancements"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["tests", "framework"]
priority: "MUST"
risk_level: "low"
tags: ["test", "infrastructure"]
due: "2026-07-01"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Enhance the test framework to support CLI standards compliance testing, including utilities for testing CLI flags, output formats, exit codes, and daemon operations. This provides the foundation for all subsequent feature testing.

## Sub-Tasks

- [x] Create CLI standards test utilities module in `tests/cli_standards_utils.rs`
- [x] Add helper functions for testing CLI flag parsing and validation
- [x] Add helper functions for testing output format validation (JSON, TOON, human)
- [x] Add helper functions for testing exit code behavior
- [x] Add helper functions for testing daemon mode operations
- [x] Add helper functions for testing TUI mode interactions
- [x] Add helper functions for testing health check endpoints
- [x] Add helper functions for testing privacy mode behavior
- [x] Add helper functions for testing config reload behavior
- [x] Add helper functions for testing session hooks
- [x] Add helper functions for testing skill generation
- [x] Add helper functions for testing cross-platform path handling
- [x] Write unit tests for all new helper functions
- [x] Document test utilities in `tests/README.md`

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `tests/cli_standards_utils.rs` — New module for CLI standards test utilities
- `tests/cli_standards_utils_test.rs` — Unit tests for test utilities
- `tests/README.md` — Documentation for test framework
- `Cargo.toml` — Add any new test dependencies if needed

## Acceptance Criteria

- [x] All CLI standards test utilities are implemented and tested
- [x] Helper functions cover all missing CLI standards (ADR #9, #13, #26, #31, #32, #33)
- [x] Test utilities are well-documented with examples
- [x] All utility functions have unit tests with >80% coverage
- [x] Test utilities can be used across all test modules

## Test Plan

- Unit: `devbox run cargo test cli_standards_utils_test`
- Integration: `devbox run cargo test --test cli_standards_utils`
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Add logging to test utilities for debugging test failures
- Track test execution time for performance regression detection

## Compliance

- Ensure test utilities follow Rust best practices
- No security vulnerabilities in test code

## Risks & Mitigations

- Risk: Test utilities may be too complex — Mitigation: Keep utilities simple and focused, use composition
- Risk: Test utilities may not cover all edge cases — Mitigation: Add comprehensive unit tests and examples

## Dependencies & Sequencing

- Depends on: None
- Unblocks: 02-001, 02-002, 02-003

## Definition of Done

- Test utilities module created and tested
- Documentation complete
- CI green for all new tests
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `test(tests): add CLI standards test utilities`

## Changelog

- 2026-06-11: initialized story file
