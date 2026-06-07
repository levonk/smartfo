---
story_id: "05-004"
story_title: "Testing Infrastructure"
story_name: "testing-infrastructure"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 5
parallel_id: 4
branch: "feature/current/cli-standards-compliance/story-05-004-testing-infrastructure"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: false
modules: ["tests/ (all test modules)"]
priority: "MUST"
risk_level: "medium"
tags: ["test", "infrastructure"]
due: "2026-08-16"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement comprehensive testing infrastructure as specified in ADR #27. Add tests for help output, globbing patterns, stdin input handling, config precedence, JSON vs human output modes, exit-code behavior, standard arguments, config file initialization, shell completion script generation, error handling and formatting, daemon mode, --list-jobs with optional job ID filtering, and daemon platform fallback behavior.

## Sub-Tasks

- [ ] Add tests for help output for all modes (mv, rm, smartfo)
- [ ] Add tests for globbing patterns (recursive **/* patterns)
- [ ] Add tests for stdin input handling
- [ ] Add tests for config precedence (CLI > env > project > user > system > defaults)
- [ ] Add tests for JSON vs human output modes
- [ ] Add tests for exit-code behavior for all error paths
- [ ] Add tests for standard arguments (--help, --version, --usage)
- [ ] Add tests for config file initialization
- [ ] Add tests for shell completion script generation
- [ ] Add tests for error handling and formatting
- [ ] Add tests for daemon mode where feasible
- [ ] Add tests for --list-jobs with optional job ID filtering
- [ ] Add tests for daemon platform fallback behavior
- [ ] Add tests for daemon_fallback_quiet config variable override
- [ ] Set up test fixtures and mock environments
- [ ] Add property-based tests for critical operations
- [ ] Add integration tests for end-to-end scenarios
- [ ] Set up CI/CD test pipeline
- [ ] Add test coverage reporting

## Relevant Files

- `tests/cli_tests.rs` — Add tests for CLI behavior
- `tests/globbing_tests.rs` — Add tests for globbing patterns
- `tests/config_tests.rs` — Add tests for config precedence
- `tests/output_tests.rs` — Add tests for output modes
- `tests/signal_tests.rs` — Add tests for exit codes
- `tests/completion_tests.rs` — Add tests for shell completions
- `tests/error_tests.rs` — Add tests for error handling
- `tests/daemon_tests.rs` — Add tests for daemon operations
- `tests/property/` — Add property-based tests
- `tests/integration/` — Add integration tests

## Acceptance Criteria

- [ ] Tests exist for help output for all modes
- [ ] Tests exist for globbing patterns
- [ ] Tests exist for stdin input handling
- [ ] Tests exist for config precedence
- [ ] Tests exist for JSON vs human output modes
- [ ] Tests exist for exit-code behavior for all error paths
- [ ] Tests exist for standard arguments
- [ ] Tests exist for config file initialization
- [ ] Tests exist for shell completion script generation
- [ ] Tests exist for error handling and formatting
- [ ] Tests exist for daemon mode where feasible
- [ ] Tests exist for --list-jobs with job ID filtering
- [ ] Tests exist for daemon platform fallback behavior
- [ ] Tests exist for daemon_fallback_quiet config variable
- [ ] Property-based tests exist for critical operations
- [ ] Integration tests exist for end-to-end scenarios
- [ ] Test coverage reporting is set up
- [ ] All tests pass

## Test Plan

- Unit: `cargo test` (all unit tests)
- Property: `cargo test --all-features` (property-based tests)
- Integration: `cargo test --test integration`
- Coverage: `cargo tarpaulin` or similar coverage tool
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log test execution results
- Log test coverage metrics

## Compliance

- Follows ADR #27: Testing

## Risks & Mitigations

- Risk: Test suite may become slow to run — Mitigation: Use test parallelization and selective test execution
- Risk: Daemon tests may be flaky — Mitigation: Use robust test fixtures and timeouts

## Dependencies

None

## Notes

- Use tempfile crate for filesystem test fixtures
- Use proptest crate for property-based testing
- Consider adding benchmark tests for performance critical paths
- Ensure tests can run in CI/CD environment