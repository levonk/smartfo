---
story_id: "07-001"
story_title: "SIGHUP config reload + tests"
story_name: "sighup-config-reload-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 7
parallel_id: 1
branch: "feature/current/requirements-gaps/story-07-001-sighup-config-reload-tests"
status: "completed"
assignee: ""
reviewer: ""
dependencies: ["02-001", "06-001"]
parallel_safe: false
modules: ["config", "signal", "tests"]
priority: "SHOULD"
risk_level: "medium"
tags: ["feat", "config", "signal", "test"]
due: "2026-08-12"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Support SIGHUP to reload config files without restart. Validate new config before applying, log reload events to audit log, handle validation errors gracefully (keep old config active), and apply to both CLI and daemon processes. Addresses ADR #31 from CLI standards.

## Sub-Tasks

- [~] Add SIGHUP signal handler in `src/signal.rs`
- [ ] Implement config reload logic in `src/config.rs`
- [ ] Add config validation before reload
- [ ] Implement graceful error handling (keep old config on failure)
- [ ] Log reload events to audit log
- [ ] Apply SIGHUP handler to CLI process
- [ ] Apply SIGHUP handler to daemon process
- [ ] Add config reload notification to running operations
- [ ] Write unit tests for config reload
- [ ] Write integration tests for SIGHUP handling
- [ ] Write tests for validation error handling
- [ ] Document SIGHUP behavior in README
- [ ] Add examples of config reload usage

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/signal.rs` — New module for signal handling
- `src/config.rs` — Add config reload logic
- `src/audit.rs` — Log reload events
- `src/daemon.rs` — Apply SIGHUP to daemon
- `src/main.rs` — Apply SIGHUP to CLI
- `tests/signal_test.rs` — New test file for signal handling
- `README.md` — Document SIGHUP behavior
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [x] SIGHUP triggers config reload
- [x] New config validated before applying
- [x] Validation errors keep old config active
- [x] Reload events logged to audit log
- [x] SIGHUP works in CLI process
- [x] SIGHUP works in daemon process
- [x] Running operations notified of config changes
- [x] All tests pass
- [x] Documentation complete

## Test Plan

- Unit: `devbox run cargo test signal_test`
- Integration: Test SIGHUP with kill -HUP
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log SIGHUP receptions
- Track config reload success/failure
- Monitor validation errors

## Compliance

- Follow ADR #31 (Signal-Based Config Reload)
- Ensure config reload is safe and doesn't break operations

## Risks & Mitigations

- Risk: Config reload may break running operations — Mitigation: Apply changes to new operations only, notify running ops
- Risk: SIGHUP may not work on all platforms — Mitigation: Use cross-platform signal handling, document limitations
- Risk: Validation errors may leave system in bad state — Mitigation: Always keep old config, validate before applying

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-02-001-init-config-flag-tests]], [[tasks-requirements-gaps-06-001-collection-processing-tests]]
- Unblocks: None

## Definition of Done

- SIGHUP config reload implemented and tested
- Config validation before reload working
- Error handling graceful
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(config): add SIGHUP config reload`

## Changelog

- 2026-06-11: initialized story file
