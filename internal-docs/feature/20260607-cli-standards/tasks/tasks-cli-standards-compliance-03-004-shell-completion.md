---
story_id: "03-004"
story_title: "Shell Completion Generation"
story_name: "shell-completion"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 3
parallel_id: 4
branch: "feature/current/cli-standards-compliance/story-03-004-shell-completion"
status: "todo"
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

- [ ] Create new src/completions.rs module
- [ ] Add clap completion generation dependencies if needed
- [ ] Implement bash completion generation for all modes
- [ ] Implement zsh completion generation for all modes
- [ ] Implement fish completion generation for all modes
- [ ] Ensure completions match current command structure (argv[0] dispatch)
- [ ] Add completions for mv mode flags and arguments
- [ ] Add completions for rm mode flags and arguments
- [ ] Add completions for smartfo mode flags and subcommands
- [ ] Add completions for config keys where applicable
- [ ] Add completions for config values where applicable
- [ ] Add completions for file paths with globbing support
- [ ] Implement --generate-completion <shell> flag to generate completion scripts
- [ ] Integrate completion installation with --install flag
- [ ] Add unit tests for bash completion generation
- [ ] Add unit tests for zsh completion generation
- [ ] Add unit tests for fish completion generation
- [ ] Add integration tests for completion installation

## Relevant Files

- `src/completions.rs` (new) — Implement completion generation logic
- `src/cli.rs` — Add --generate-completion flag
- `src/main.rs` — Handle completion generation command
- `src/install.rs` — Integrate completion installation
- `tests/completion_tests.rs` — Add tests for shell completions

## Acceptance Criteria

- [ ] Shell completion scripts are generated for bash, zsh, and fish
- [ ] Completions match current command structure
- [ ] Completions are available for all modes (mv, rm, smartfo)
- [ ] Completions include config keys and values where applicable
- [ ] --generate-completion <shell> flag generates completion scripts
- [ ] Completions are installed via --install flag
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