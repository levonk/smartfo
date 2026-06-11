---
story_id: "02-001"
story_title: "--init-config flag + tests"
story_name: "init-config-flag-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 2
parallel_id: 1
branch: "feature/current/requirements-gaps/story-02-001-init-config-flag-tests"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-001"]
parallel_safe: true
modules: ["install", "cli", "tests"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "config", "test"]
due: "2026-07-08"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Implement the `--init-config` flag to explicitly create or recreate the default config file with all settings commented out. This addresses ADR #3 from CLI standards. Includes comprehensive tests for config initialization behavior.

## Sub-Tasks

- [ ] Add `--init-config` flag to CLI arguments in `src/cli.rs`
- [ ] Implement config initialization logic in `src/config.rs`
- [ ] Add config template with all settings commented out
- [ ] Add environment variable expansion documentation in config template
- [ ] Implement config file validation before initialization
- [ ] Add error handling for config initialization failures
- [ ] Write unit tests for config initialization
- [ ] Write integration tests for --init-config flag
- [ ] Write tests for config recreation (overwriting existing config)
- [ ] Update man pages with --init-config documentation
- [ ] Update help text with --init-config flag
- [ ] Test config initialization with various XDG locations

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/cli.rs` — Add --init-config flag
- `src/config.rs` — Implement config initialization logic
- `src/install.rs` — May need updates for config initialization
- `tests/config_init_test.rs` — New test file for config initialization
- `src/man.rs` — Update man pages
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [ ] --init-config flag creates default config file with all settings commented out
- [ ] Config file includes default values and explanations in comments
- [ ] --init-config recreates config if it already exists
- [ ] Config initialization works with various XDG locations
- [ ] All tests pass for config initialization
- [ ] Man pages and help text updated

## Test Plan

- Unit: `devbox run cargo test config_init_test`
- Integration: `devbox run cargo test --test config_init_test`
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log config initialization actions
- Track config file creation success/failure

## Compliance

- Follow ADR #3 (Config File Initialization)
- Ensure config file follows TOML best practices

## Risks & Mitigations

- Risk: Config initialization may overwrite user's custom config — Mitigation: Add warning prompt before overwriting, require --force flag
- Risk: Config template may become outdated — Mitigation: Add CI check to validate config template matches Config struct

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-01-001-test-framework-enhancements]]
- Unblocks: 07-001

## Definition of Done

- --init-config flag implemented and tested
- Config template complete and documented
- All tests pass
- Man pages and help updated
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(config): add --init-config flag`

## Changelog

- 2026-06-11: initialized story file
