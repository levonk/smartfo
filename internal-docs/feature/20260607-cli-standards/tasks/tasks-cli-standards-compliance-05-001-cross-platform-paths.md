---
story_id: "05-001"
story_title: "Cross-Platform Path Handling"
story_name: "cross-platform-paths"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 5
parallel_id: 1
branch: "feature/current/cli-standards-compliance/story-05-001-cross-platform-paths"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["All modules (path operations)"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "cross-platform"]
due: "2026-08-16"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement cross-platform path handling as specified in ADR #24. Ensure consistent path handling across Windows, Linux, and macOS, use platform-appropriate separators, handle both forward and backward slashes, use Rust's std::path for cross-platform path operations, and test on all three platforms.

## Sub-Tasks

- [ ] Audit all path operations across all modules
- [ ] Ensure all path operations use std::path::Path and std::path::PathBuf
- [ ] Ensure no hardcoded path separators in code
- [ ] Test path handling on Linux
- [ ] Test path handling on macOS
- [ ] Test path handling on Windows (or Windows CI)
- [ ] Ensure forward slashes are handled correctly on Windows
- [ ] Ensure backward slashes are handled correctly on Linux/macOS
- [ ] Test absolute path handling on all platforms
- [ ] Test relative path handling on all platforms
- [ ] Test path normalization on all platforms
- [ ] Test path expansion (home directory, environment variables) on all platforms
- [ ] Test path joining operations on all platforms
- [ ] Add unit tests for cross-platform path operations
- [ ] Add integration tests for path handling scenarios
- [ ] Fix any platform-specific path issues found during testing

## Relevant Files

- `src/main.rs` — Audit and fix path operations
- `src/cli.rs` — Audit and fix path operations
- `src/config.rs` — Audit and fix path operations
- `src/mv.rs` — Audit and fix path operations
- `src/rm.rs` — Audit and fix path operations
- `src/trash.rs` — Audit and fix path operations
- `src/vcs.rs` — Audit and fix path operations
- `tests/path_tests.rs` — Add tests for cross-platform path handling

## Acceptance Criteria

- [ ] Path handling is consistent across Windows, Linux, and macOS
- [ ] Platform-appropriate separators are used
- [ ] Both forward and backward slashes are handled correctly
- [ ] All path operations use std::path::Path and std::path::PathBuf
- [ ] No hardcoded path separators in code
- [ ] Path operations work correctly on Linux
- [ ] Path operations work correctly on macOS
- [ ] Path operations work correctly on Windows
- [ ] All tests pass

## Test Plan

- Unit: `cargo test path_tests::path_separator_handling`
- Unit: `cargo test path_tests::absolute_paths`
- Unit: `cargo test path_tests::relative_paths`
- Unit: `cargo test path_tests::path_normalization`
- Integration: `cargo test path_tests::linux_paths`
- Integration: `cargo test path_tests::macos_paths`
- Integration: `cargo test path_tests::windows_paths`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log path operations (trace level for debugging)
- Log platform-specific path handling (debug level)

## Compliance

- Follows ADR #24: Cross-Platform Path Handling

## Risks & Mitigations

- Risk: Path handling may differ between platforms — Mitigation: Comprehensive testing on all three platforms
- Risk: Windows path handling may have edge cases — Mitigation: Test on Windows or Windows CI environment

## Dependencies

None

## Notes

- Use std::path::Path and std::path::PathBuf for all path operations
- Avoid string manipulation of paths; use path methods instead
- Consider adding --normalize-path flag for explicit path normalization