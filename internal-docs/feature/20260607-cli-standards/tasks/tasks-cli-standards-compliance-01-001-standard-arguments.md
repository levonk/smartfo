---
story_id: "01-001"
story_title: "Standard Arguments Implementation"
story_name: "standard-arguments"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 1
parallel_id: 1
branch: "feature/current/cli-standards-compliance/story-01-001-standard-arguments"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["cli.rs", "main.rs"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "cli"]
due: "2026-06-21"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement standard CLI arguments (--help/-h, --version/-v, --usage) for all modes (mv, rm, smartfo) as specified in ADR #1. These flags must work at the root command and for each mode, providing comprehensive help, version information, and brief usage summaries.

## Sub-Tasks

- [x] Add --help/-h flag to clap parsers in cli.rs for all modes (mv, rm, smartfo)
- [x] Add --version/-v flag to clap parsers in cli.rs for all modes
- [x] Add --usage flag to clap parsers in cli.rs for all modes
- [x] Implement comprehensive help text for each mode including all available flags and their descriptions
- [x] Implement version display showing version string from Cargo.toml
- [x] Implement brief usage summary showing basic command syntax
- [x] Ensure flags work at root command level (smartfo --help, smartfo --version, smartfo --usage)
- [x] Ensure flags work for each mode (mv --help, rm --help, etc.)
- [x] Add unit tests for help output for all modes
- [x] Add unit tests for version output for all modes
- [x] Add unit tests for usage output for all modes

## Relevant Files

- `src/cli.rs` — Add clap flag definitions for --help, --version, --usage
- `src/main.rs` — Handle flag dispatch and output generation
- `tests/cli_tests.rs` — Add tests for standard arguments

## Acceptance Criteria

- [x] --help/-h displays comprehensive help for current mode
- [x] --version/-v displays version information
- [x] --usage displays brief usage summary
- [x] All three flags work at root command and for each mode (mv, rm, install)
- [x] Help text is comprehensive and covers all available flags
- [x] Version information matches Cargo.toml version
- [x] Usage summary shows basic command syntax
- [x] All tests pass

## Test Plan

- Unit: `cargo test cli_tests::help_output`
- Unit: `cargo test cli_tests::version_output`
- Unit: `cargo test cli_tests::usage_output`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Add logging when help/version/usage flags are triggered (debug level)

## Compliance

- Follows ADR #1: Standard Arguments

## Risks & Mitigations

- Risk: Help text may become outdated as flags are added — Mitigation: Document process for updating help text when adding new flags

## Dependencies

None

## Notes

- Use clap's built-in help/version generation where possible
- Ensure help text is consistent across all modes
- Consider adding examples in help text for common use cases