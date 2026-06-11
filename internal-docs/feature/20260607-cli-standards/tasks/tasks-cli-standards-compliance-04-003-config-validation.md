---
story_id: "04-003"
story_title: "Configuration Validation"
story_name: "config-validation"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 4
parallel_id: 3
branch: "feature/current/cli-standards-compliance/story-04-003-config-validation"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-002"]
parallel_safe: true
modules: ["config.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "config"]
due: "2026-08-02"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement configuration validation as specified in ADR #21. Validate config files on load, report clear, specific error messages with line numbers, provide suggestions for fixing config errors, validate config schema version, and ensure invalid config doesn't crash the application.

## Sub-Tasks

- [x] Define config schema validation rules
- [x] Implement config file validation on load
- [x] Add validation for all config sections (vcs, trash, concurrency, behavior, logging, paths)
- [x] Add validation for config value types (string, integer, boolean, arrays)
- [x] Add validation for config value ranges (e.g., positive integers)
- [x] Add validation for config value formats (e.g., file paths, URLs)
- [x] Implement error reporting with line numbers
- [x] Implement clear, specific error messages
- [x] Add actionable suggestions for fixing config errors
- [x] Add config schema version validation
- [x] Ensure invalid config doesn't crash the application
- [x] Implement graceful fallback to defaults on invalid config
- [x] Add --validate-config flag to explicitly validate config without loading
- [x] Add unit tests for config validation rules
- [x] Add unit tests for error message formatting
- [x] Add unit tests for schema version validation
- [x] Add integration tests for invalid config handling

## Relevant Files

- `src/config.rs` — Implement config validation logic
- `src/cli.rs` — Add --validate-config flag
- `src/main.rs` — Handle config validation errors gracefully
- `tests/config_tests.rs` — Add tests for config validation

## Acceptance Criteria

- [x] Config files are validated on load
- [x] Error messages are clear, specific, and include line numbers
- [x] Actionable suggestions are provided for fixing config errors
- [x] Config schema version is validated
- [x] Invalid config doesn't crash the application
- [x] Graceful fallback to defaults on invalid config
- [x] --validate-config flag validates config without loading
- [x] All tests pass

## Test Plan

- Unit: `cargo test config_tests::validation_rules`
- Unit: `cargo test config_tests::error_messages`
- Unit: `cargo test config_tests::schema_version_validation`
- Unit: `cargo test config_tests::graceful_fallback`
- Integration: `cargo test config_tests::invalid_config_handling`
- Integration: `cargo test config_tests::validate_config_flag`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log config validation events (info level)
- Log config validation errors (error level)

## Compliance

- Follows ADR #21: Configuration Validation

## Risks & Mitigations

- Risk: Strict validation may break existing configs — Mitigation: Provide migration path and clear error messages
- Risk: Validation may impact startup performance — Mitigation: Use efficient validation logic

## Dependencies

- 01-002 (Config Initialization & System Config) — ensures config file structure is established

## Notes

- Use toml crate for TOML parsing and validation
- Consider adding --fix-config flag to automatically fix common config errors
- Document all validation rules in man pages
