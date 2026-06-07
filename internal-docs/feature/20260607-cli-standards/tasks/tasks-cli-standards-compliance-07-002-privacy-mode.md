---
story_id: "07-002"
story_title: "Privacy Mode Implementation"
story_name: "privacy-mode"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 7
parallel_id: 2
branch: "feature/current/cli-standards-compliance/story-07-002-privacy-mode"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["06-002"]
parallel_safe: true
modules: ["config.rs", "audit.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "privacy"]
due: "2026-09-13"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement privacy mode with anonymous lists as specified in ADR #33. Support privacy mode with explicit ignore lists (identifiers to never log or process), distinguish between "unknown" (logged but not assigned) and "anonymous" (ignored entirely), add configurable privacy toggles to disable specific data collection, sanitize audit logs to remove sensitive paths when privacy mode enabled, add --privacy flag to enable privacy mode for single operation, and add privacy settings to config file.

## Sub-Tasks

- [ ] Define privacy mode states (disabled, enabled, strict)
- [ ] Add privacy_mode config option to config file
- [ ] Add privacy_ignore_list config option for explicit ignore patterns
- [ ] Add privacy_toggles config option for granular data collection control
- [ ] Add --privacy flag to clap parsers for all modes
- [ ] Implement identifier classification (unknown vs anonymous)
- [ ] Implement path sanitization based on privacy ignore list
- [ ] Sanitize audit logs to remove sensitive paths when privacy mode enabled
- [ ] Implement "unknown" handling (logged but not assigned)
- [ ] Implement "anonymous" handling (ignored entirely)
- [ ] Add privacy toggles for specific data types (file paths, VCS info, timestamps, etc.)
- [ ] Add unit tests for privacy mode logic
- [ ] Add unit tests for identifier classification
- [ ] Add unit tests for path sanitization
- [ ] Add unit tests for audit log sanitization
- [ ] Add integration tests for --privacy flag
- [ ] Add integration tests for privacy mode config options
- [ ] Document privacy mode in help output and man pages

## Relevant Files

- `src/config.rs` — Add privacy mode config options
- `src/audit.rs` — Implement audit log sanitization
- `src/cli.rs` — Add --privacy flag
- `src/main.rs` — Handle privacy mode state
- `tests/privacy_tests.rs` — Add tests for privacy mode

## Acceptance Criteria

- [ ] Privacy mode is supported with explicit ignore lists
- [ ] Identifiers are classified as "unknown" or "anonymous"
- [ ] "Unknown" identifiers are logged but not assigned
- [ ] "Anonymous" identifiers are ignored entirely
- [ ] Configurable privacy toggles disable specific data collection
- [ ] Audit logs are sanitized to remove sensitive paths when privacy mode enabled
- [ ] --privacy flag enables privacy mode for single operation
- [ ] Privacy settings are available in config file
- [ ] Path sanitization works correctly
- [ ] All tests pass

## Test Plan

- Unit: `cargo test privacy_tests::privacy_mode_logic`
- Unit: `cargo test privacy_tests::identifier_classification`
- Unit: `cargo test privacy_tests::path_sanitization`
- Unit: `cargo test privacy_tests::audit_log_sanitization`
- Integration: `cargo test privacy_tests::privacy_flag`
- Integration: `cargo test privacy_tests::privacy_config`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log privacy mode activation (info level)
- Log privacy sanitization events (debug level)

## Compliance

- Follows ADR #33: Privacy Mode with Anonymous Lists

## Risks & Mitigations

- Risk: Privacy mode may impact debugging — Mitigation: Provide clear guidance on when to disable privacy mode
- Risk: Ignore lists may be too complex — Mitigation: Use simple pattern matching with clear documentation

## Dependencies

- 06-002 (Config File Versioning) — ensures config structure supports new privacy options

## Notes

- Use glob patterns for ignore lists (e.g., ~/.ssh/*, /tmp/*)
- Consider adding --privacy-strict flag for maximum privacy
- Document which data types can be toggled in privacy mode