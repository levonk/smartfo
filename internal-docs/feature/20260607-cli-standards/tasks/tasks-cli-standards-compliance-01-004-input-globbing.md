---
story_id: "01-004"
story_title: "Input & Globbing Support"
story_name: "input-globbing"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 1
parallel_id: 4
branch: "feature/current/cli-standards-compliance/story-01-004-input-globbing"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["cli.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "cli"]
due: "2026-06-21"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement recursive globbing support (`**/*`) and stdin input handling for file arguments as specified in ADR #5. Support processing files and stdin interchangeably where applicable, ensuring globbing works correctly with VCS-aware operations.

## Sub-Tasks

- [ ] Add glob pattern support to clap argument parsers for file arguments
- [ ] Implement recursive `**/*` globbing pattern expansion
- [ ] Add stdin input support via `-` argument for file operations
- [ ] Add piped input support for operations that accept file lists
- [ ] Implement file/stdin interchangeability logic in argument processing
- [ ] Ensure globbing works correctly with VCS-aware move operations
- [ ] Ensure globbing works correctly with VCS-aware remove operations
- [ ] Add unit tests for recursive globbing patterns
- [ ] Add unit tests for stdin input handling
- [ ] Add unit tests for piped input handling
- [ ] Add unit tests for VCS-aware operations with globbed paths

## Relevant Files

- `src/cli.rs` — Add globbing and stdin support to argument parsing
- `src/mv.rs` — Ensure globbing works with VCS-aware moves
- `src/rm.rs` — Ensure globbing works with VCS-aware removes
- `tests/globbing_tests.rs` — Add tests for globbing functionality

## Acceptance Criteria

- [ ] Recursive `**/*` globbing patterns are supported
- [ ] Stdin input via `-` argument works for file operations
- [ ] Piped input works for operations that accept file lists
- [ ] Files and stdin can be processed interchangeably
- [ ] Globbing works correctly with VCS-aware move operations
- [ ] Globbing works correctly with VCS-aware remove operations
- [ ] All tests pass

## Test Plan

- Unit: `cargo test globbing_tests::recursive_globbing`
- Unit: `cargo test globbing_tests::stdin_input`
- Unit: `cargo test globbing_tests::piped_input`
- Unit: `cargo test globbing_tests::vcs_globbing`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log glob pattern expansion results (debug level)
- Log stdin/piped input detection (debug level)

## Compliance

- Follows ADR #5: Input & Globbing

## Risks & Mitigations

- Risk: Globbing may match too many files unexpectedly — Mitigation: Add --dry-run support to preview matched files before operations
- Risk: Stdin handling may interfere with terminal interaction — Mitigation: Detect TTY context and provide appropriate prompts

## Dependencies

None

## Notes

- Use glob crate for pattern matching
- Consider adding --glob-warning flag to warn about large match sets
- Ensure glob patterns are cross-platform compatible