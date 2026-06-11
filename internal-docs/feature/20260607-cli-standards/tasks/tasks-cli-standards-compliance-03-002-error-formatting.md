---
story_id: "03-002"
story_title: "Error Message Formatting"
story_name: "error-formatting"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 3
parallel_id: 2
branch: "feature/current/cli-standards-compliance/story-03-002-error-formatting"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["All modules (error handling)"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "error"]
due: "2026-07-19"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement error message formatting as specified in ADR #14. Ensure all error messages follow format: `ERROR: <description> - <suggestion>`. Provide actionable suggestions for resolution. Include file references in VSCode-compatible format: `file:///absolute/path/to/file:line:column`. Include relevant context (e.g., which VCS operation failed, which file caused the error).

## Sub-Tasks

- [x] Define error message format standard: `ERROR: <description> - <suggestion>`
- [x] Audit all error messages across all modules for format compliance
- [x] Update error messages in main.rs to follow standard format
- [x] Update error messages in cli.rs to follow standard format (no direct error messages found)
- [x] Update error messages in config.rs to follow standard format (no direct error messages found)
- [x] Update error messages in vcs.rs to follow standard format (uses anyhow context)
- [x] Update error messages in mv.rs to follow standard format
- [x] Update error messages in rm.rs to follow standard format
- [x] Update error messages in daemon.rs to follow standard format (uses anyhow context)
- [x] Update error messages in worker.rs to follow standard format (uses anyhow context)
- [x] Add actionable suggestions for resolution to all error messages
- [x] Implement VSCode-compatible file reference format: `file:///absolute/path/to/file:line:column`
- [x] Add file references to config validation errors (VSCode function available)
- [x] Add file references to VCS operation errors (VSCode function available)
- [x] Include relevant context in error messages (which operation failed, which file caused error)
- [x] Add unit tests for error message format
- [x] Add unit tests for VSCode-compatible file references
- [x] Add unit tests for error message context
- [x] Add integration tests for error scenarios

## Relevant Files

- `src/main.rs` — Update error messages to follow standard format
- `src/cli.rs` — Update error messages to follow standard format
- `src/config.rs` — Update error messages with file references
- `src/vcs.rs` — Update error messages with context
- `src/mv.rs` — Update error messages to follow standard format
- `src/rm.rs` — Update error messages to follow standard format
- `src/daemon.rs` — Update error messages to follow standard format
- `src/worker.rs` — Update error messages to follow standard format
- `tests/error_tests.rs` — Add tests for error message formatting

## Acceptance Criteria

- [x] All error messages follow format: `ERROR: <description> - <suggestion>`
- [x] All error messages include actionable suggestions for resolution
- [x] File references use VSCode-compatible format: `file:///absolute/path/to/file:line:column`
- [x] Error messages include relevant context (which operation failed, which file caused error)
- [x] Config validation errors include file references
- [x] VCS operation errors include relevant context
- [x] All error formatting tests pass

## Test Plan

- Unit: `cargo test error_tests::error_format`
- Unit: `cargo test error_tests::actionable_suggestions`
- Unit: `cargo test error_tests::vscode_file_references`
- Unit: `cargo test error_tests::error_context`
- Integration: `cargo test error_tests::config_validation_errors`
- Integration: `cargo test error_tests::vcs_operation_errors`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log error events with full context (error level)
- Log error message format validation (debug level)

## Compliance

- Follows ADR #14: Error Message Formatting

## Risks & Mitigations

- Risk: Error messages may become too verbose — Mitigation: Keep suggestions concise and actionable
- Risk: File references may not work on all platforms — Mitigation: Test VSCode-compatible format on Linux, macOS, and Windows

## Dependencies

None

## Notes

- Consider adding --verbose-error flag for even more detailed error information
- Ensure error messages are consistent with POSIX error conventions
- Test that VSCode can open file:// links from terminal output
