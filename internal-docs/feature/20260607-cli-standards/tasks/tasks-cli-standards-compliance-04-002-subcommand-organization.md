---
story_id: "04-002"
story_title: "Subcommand Organization"
story_name: "subcommand-organization"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 4
parallel_id: 2
branch: "feature/current/cli-standards-compliance/story-04-002-subcommand-organization"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-001"]
parallel_safe: true
modules: ["cli.rs"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "cli"]
due: "2026-08-02"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement subcommand organization as specified in ADR #20. Maintain hierarchical command structure, group related commands under logical subcommands, ensure consistency in command naming patterns, and document command hierarchy in help output. This ensures the argv[0] dispatch model (mv/rm/smartfo) works seamlessly with any new subcommands.

## Sub-Tasks

- [ ] Audit current command structure for consistency
- [ ] Document existing command hierarchy
- [ ] Identify opportunities for logical subcommand grouping
- [ ] Group related commands under logical subcommands (e.g., config commands, job commands)
- [ ] Ensure command naming patterns are consistent
- [ ] Document command hierarchy in help output
- [ ] Add subcommand help for each logical group
- [ ] Ensure argv[0] dispatch (mv/rm/smartfo) remains the primary entry point
- [ ] Add smartfo subcommands for operations not covered by mv/rm modes
- [ ] Ensure subcommand help is accessible via --help
- [ ] Ensure subcommand help shows hierarchy
- [ ] Add unit tests for subcommand structure
- [ ] Add unit tests for command naming consistency
- [ ] Add integration tests for subcommand help output
- [ ] Verify argv[0] dispatch still works correctly

## Relevant Files

- `src/cli.rs` — Organize subcommands and command hierarchy
- `src/main.rs` — Ensure dispatch logic works with subcommands
- `tests/subcommand_tests.rs` — Add tests for subcommand organization

## Acceptance Criteria

- [ ] Command structure is hierarchical and logical
- [ ] Related commands are grouped under logical subcommands
- [ ] Command naming patterns are consistent
- [ ] Command hierarchy is documented in help output
- [ ] argv[0] dispatch (mv/rm/smartfo) works seamlessly with subcommands
- [ ] Subcommand help is accessible via --help
- [ ] Subcommand help shows command hierarchy
- [ ] All tests pass

## Test Plan

- Unit: `cargo test subcommand_tests::command_hierarchy`
- Unit: `cargo test subcommand_tests::naming_consistency`
- Integration: `cargo test subcommand_tests::subcommand_help`
- Integration: `cargo test subcommand_tests::argv0_dispatch`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log subcommand dispatch events (debug level)

## Compliance

- Follows ADR #20: Subcommand Organization

## Risks & Mitigations

- Risk: Subcommand reorganization may break existing scripts — Mitigation: Maintain backward compatibility for argv[0] dispatch
- Risk: Too many subcommands may confuse users — Mitigation: Keep subcommand hierarchy shallow and intuitive

## Dependencies

- 01-001 (Standard Arguments Implementation) — ensures command structure is stable

## Notes

- Keep argv[0] dispatch as the primary model (mv/rm/smartfo)
- Consider adding smartfo subcommands for: config management, job management, health check
- Ensure subcommand names are short and memorable