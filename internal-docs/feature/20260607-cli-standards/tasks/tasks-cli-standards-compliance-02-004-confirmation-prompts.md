---
story_id: "02-004"
story_title: "Confirmation Prompts"
story_name: "confirmation-prompts"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 2
parallel_id: 4
branch: "feature/current/cli-standards-compliance/story-02-004-confirmation-prompts"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["mv.rs", "rm.rs"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "confirmation"]
due: "2026-07-05"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement confirmation prompts as specified in ADR #11. Require confirmation for destructive operations (delete, overwrite). Add --force flag to bypass confirmation prompts, and --interactive/-i flag to enable prompts even for non-destructive operations. Ensure prompts are clear about what will happen, and support batch confirmation (yes to all, no to all).

## Sub-Tasks

- [ ] Add --force flag to clap parsers in cli.rs for mv and rm modes
- [ ] Add --interactive/-i flag to clap parsers in cli.rs for mv and rm modes
- [ ] Implement confirmation prompt function for destructive operations
- [ ] Add confirmation prompts for file deletion in rm.rs
- [ ] Add confirmation prompts for file overwrite in mv.rs
- [ ] Implement --force flag to bypass all confirmation prompts
- [ ] Implement --interactive/-i flag to enable prompts for non-destructive operations
- [ ] Ensure prompts are clear about what will happen
- [ ] Implement batch confirmation (yes to all, no to all)
- [ ] Add confirmation prompt for moving tracked files outside VCS repo
- [ ] Add confirmation prompt for deleting VCS-tracked files
- [ ] Ensure prompts work correctly with --dry-run mode
- [ ] Ensure prompts are suppressed in --quiet mode (assume yes)
- [ ] Add unit tests for confirmation prompts
- [ ] Add unit tests for --force flag behavior
- [ ] Add unit tests for --interactive flag behavior
- [ ] Add unit tests for batch confirmation
- [ ] Add integration tests for confirmation scenarios

## Relevant Files

- `src/cli.rs` — Add --force and --interactive flags
- `src/mv.rs` — Implement confirmation prompts for overwrite operations
- `src/rm.rs` — Implement confirmation prompts for delete operations
- `tests/confirmation_tests.rs` — Add tests for confirmation prompts

## Acceptance Criteria

- [ ] Confirmation prompts are required for destructive operations (delete, overwrite)
- [ ] --force flag bypasses all confirmation prompts
- [ ] --interactive/-i flag enables prompts for non-destructive operations
- [ ] Prompts are clear about what will happen
- [ ] Batch confirmation (yes to all, no to all) is supported
- [ ] Prompts work correctly with --dry-run mode
- [ ] Prompts are suppressed in --quiet mode (assume yes)
- [ ] All tests pass

## Test Plan

- Unit: `cargo test confirmation_tests::delete_prompt`
- Unit: `cargo test confirmation_tests::overwrite_prompt`
- Unit: `cargo test confirmation_tests::force_flag`
- Unit: `cargo test confirmation_tests::interactive_flag`
- Unit: `cargo test confirmation_tests::batch_confirmation`
- Unit: `cargo test confirmation_tests::quiet_mode`
- Integration: `cargo test confirmation_tests::destructive_operations`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log confirmation prompt requests (debug level)
- Log user confirmation responses (debug level)

## Compliance

- Follows ADR #11: Confirmation Prompts

## Risks & Mitigations

- Risk: Prompts may break automation scripts — Mitigation: Ensure --force and --quiet flags bypass prompts
- Risk: Batch confirmation may lead to unintended mass operations — Mitigation: Clear warning before applying batch confirmation

## Dependencies

None

## Notes

- Use dialoguer crate for interactive prompts (already a dependency)
- Consider adding --yes flag as alias for --force
- Ensure prompts are consistent with POSIX rm -i behavior