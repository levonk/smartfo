---
story_id: "06-004"
story_title: "Signal-Based Config Reload"
story_name: "sighup-config-reload"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 6
parallel_id: 4
branch: "feature/current/cli-standards-compliance/story-06-004-sighup-config-reload"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["02-002", "04-003"]
parallel_safe: true
modules: ["main.rs", "daemon.rs", "config.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "signals"]
due: "2026-08-30"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement signal-based config reload as specified in ADR #31. Support SIGHUP to reload config files without restart, validate new config before applying, log reload events to audit log, handle validation errors gracefully (keep old config active), and apply to both CLI and daemon processes.

## Sub-Tasks

- [ ] Implement SIGHUP signal handler using nix crate
- [ ] Implement config reload logic in config.rs
- [ ] Add config validation before applying new config
- [ ] Implement graceful error handling for invalid config (keep old config active)
- [ ] Log config reload events to audit log
- [ ] Log config reload success/failure to standard log
- [ ] Apply SIGHUP handling to CLI process
- [ ] Apply SIGHUP handling to daemon process
- [ ] Add --reload-config flag to manually trigger config reload
- [ ] Implement config diff logging (show what changed)
- [ ] Ensure in-flight operations are not disrupted by config reload
- [ ] Add unit tests for SIGHUP signal handling
- [ ] Add unit tests for config reload logic
- [ ] Add unit tests for config validation before reload
- [ ] Add unit tests for graceful error handling
- [ ] Add integration tests for CLI config reload
- [ ] Add integration tests for daemon config reload

## Relevant Files

- `src/main.rs` — Implement SIGHUP handler for CLI
- `src/daemon.rs` — Implement SIGHUP handler for daemon
- `src/config.rs` — Implement config reload and validation logic
- `src/audit.rs` — Log config reload events
- `tests/signal_tests.rs` — Add tests for SIGHUP handling

## Acceptance Criteria

- [ ] SIGHUP signal triggers config reload
- [ ] New config is validated before applying
- [ ] Invalid config is rejected and old config remains active
- [ ] Config reload events are logged to audit log
- [ ] Config reload success/failure is logged to standard log
- [ ] SIGHUP handling works for CLI process
- [ ] SIGHUP handling works for daemon process
- [ ] --reload-config flag manually triggers config reload
- [ ] Config diff shows what changed
- [ ] In-flight operations are not disrupted by config reload
- [ ] All tests pass

## Test Plan

- Unit: `cargo test signal_tests::sighup_handler`
- Unit: `cargo test signal_tests::config_reload_logic`
- Unit: `cargo test signal_tests::config_validation_before_reload`
- Unit: `cargo test signal_tests::graceful_error_handling`
- Integration: `cargo test signal_tests::cli_config_reload`
- Integration: `cargo test signal_tests::daemon_config_reload`
- Integration: `cargo test signal_tests::reload_config_flag`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log SIGHUP signal reception (info level)
- Log config reload attempts (info level)
- Log config validation results (debug level)
- Log config diff (info level)

## Compliance

- Follows ADR #31: Signal-Based Config Reload

## Risks & Mitigations

- Risk: Config reload may disrupt in-flight operations — Mitigation: Ensure operations complete with old config before new config applies
- Risk: Invalid config may leave system in inconsistent state — Mitigation: Keep old config active on validation failure

## Dependencies

- 02-002 (Signals & Exit Codes) — ensures signal handling infrastructure exists
- 04-003 (Configuration Validation) — ensures config validation logic exists

## Notes

- Use nix crate for signal handling (already a dependency)
- Consider adding SIGUSR1/SIGUSR2 for other reload operations
- Document SIGHUP behavior in man pages