---
story_id: "03-004"
story_title: "Shell Completion Generation"
story_name: "shell-completion"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 3
parallel_id: 4
branch: "feature/current/cli-standards-compliance/story-03-004-shell-completion"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: ["01-001"]
parallel_safe: true
modules: ["completions.rs (new)"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "completion"]
due: "2026-07-19"
created_at: "2026-06-07"
updated_at: "2026-07-07"
---

## Summary

Implement shell completion generation as specified in ADR #17. Generate shell completion scripts for bash, zsh, and fish using clap's completion generation features. Ensure completions match current command structure, include completions for all modes (mv, rm, smartfo), include completions for config keys and values where applicable, and install completions via --install flag.

## Sub-Tasks

- [x] Create new src/completions.rs module
- [x] Add clap completion generation dependencies if needed
- [x] Implement bash completion generation for all modes
- [x] Implement zsh completion generation for all modes
- [x] Implement fish completion generation for all modes
- [x] Ensure completions match current command structure (argv[0] dispatch)
- [x] Add completions for mv mode flags and arguments
- [x] Add completions for rm mode flags and arguments
- [x] Add completions for smartfo mode flags and subcommands
- [x] Add completions for config keys where applicable
- [x] Add completions for config values where applicable
- [x] Add completions for file paths with globbing support
- [x] Implement --generate-completion <shell> flag to generate completion scripts
- [x] Integrate completion installation with --install flag
- [x] Add unit tests for bash completion generation
- [x] Add unit tests for zsh completion generation
- [x] Add unit tests for fish completion generation
- [ ] Add integration tests for completion installation

## Relevant Files

- `src/completions.rs` (new) — Implement completion generation logic
- `src/cli.rs` — Add --generate-completion flag
- `src/main.rs` — Handle completion generation command
- `src/install.rs` — Integrate completion installation
- `tests/completion_tests.rs` — Add tests for shell completions

## Acceptance Criteria

- [x] Shell completion scripts are generated for bash, zsh, and fish
- [x] Completions match current command structure
- [x] Completions are available for all modes (mv, rm, smartfo)
- [x] Completions include config keys and values where applicable
- [x] --generate-completion <shell> flag generates completion scripts
- [x] Completions are installed via --install flag
- [ ] All tests pass

## Test Plan

- Unit: `cargo test completion_tests::bash_completion`
- Unit: `cargo test completion_tests::zsh_completion`
- Unit: `cargo test completion_tests::fish_completion`
- Unit: `cargo test completion_tests::completion_structure`
- Integration: `cargo test completion_tests::install_completion`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log completion generation events (info level)
- Log completion installation events (info level)

## Compliance

- Follows ADR #17: Shell Completion

## Risks & Mitigations

- Risk: Completions may become outdated as flags change — Mitigation: Document process for regenerating completions
- Risk: Completion scripts may not work on all shell versions — Mitigation: Test on multiple shell versions

## Dependencies

- 01-001 (Standard Arguments Implementation) — ensures command structure is stable

## Notes

- Use clap's built-in completion generation features
- Consider adding completion for custom config values (e.g., VCS systems)
- Test completions interactively in bash, zsh, and fish
