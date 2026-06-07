---
story_id: "04-001"
story_title: "Pager Integration"
story_name: "pager-integration"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 4
parallel_id: 1
branch: "feature/current/cli-standards-compliance/story-04-001-pager-integration"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["cli.rs", "output.rs"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "pager"]
due: "2026-08-02"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement pager integration as specified in ADR #19. Auto-pager for long output (help, config, job lists). Respect PAGER environment variable, default to `less`. Add --no-pager flag to bypass paging. Detect if output is interactive before enabling pager.

## Sub-Tasks

- [ ] Add pager crate to Cargo.toml dependencies
- [ ] Implement pager detection logic
- [ ] Add --no-pager flag to clap parsers in cli.rs
- [ ] Implement auto-pager for help output
- [ ] Implement auto-pager for config display
- [ ] Implement auto-pager for job lists (--list-jobs)
- [ ] Implement auto-pager for man page display (--man)
- [ ] Respect PAGER environment variable
- [ ] Default to `less` if PAGER not set
- [ ] Detect if output is interactive before enabling pager
- [ ] Ensure pager is not used when stdout is not a TTY
- [ ] Ensure pager is not used in --quiet mode
- [ ] Ensure pager is not used in --json mode
- [ ] Add unit tests for pager detection
- [ ] Add unit tests for PAGER environment variable handling
- [ ] Add unit tests for --no-pager flag
- [ ] Add integration tests for auto-pager behavior

## Relevant Files

- `Cargo.toml` — Add pager dependency
- `src/cli.rs` — Add --no-pager flag
- `src/output.rs` — Implement pager integration logic
- `src/main.rs` — Integrate pager with output commands
- `tests/pager_tests.rs` — Add tests for pager integration

## Acceptance Criteria

- [ ] Auto-pager is enabled for long output (help, config, job lists)
- [ ] PAGER environment variable is respected
- [ ] Default pager is `less` when PAGER not set
- [ ] --no-pager flag bypasses paging
- [ ] Pager is only used when output is interactive
- [ ] Pager is not used when stdout is not a TTY
- [ ] Pager is not used in --quiet mode
- [ ] Pager is not used in --json mode
- [ ] All tests pass

## Test Plan

- Unit: `cargo test pager_tests::pager_detection`
- Unit: `cargo test pager_tests::pager_env_var`
- Unit: `cargo test pager_tests::no_pager_flag`
- Unit: `cargo test pager_tests::tty_detection`
- Integration: `cargo test pager_tests::help_pager`
- Integration: `cargo test pager_tests::config_pager`
- Integration: `cargo test pager_tests::job_list_pager`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log pager activation events (debug level)
- Log --no-pager flag usage (debug level)

## Compliance

- Follows ADR #19: Pager Integration

## Risks & Mitigations

- Risk: Pager may not be available on all systems — Mitigation: Graceful fallback to no paging if pager not found
- Risk: Auto-pager may interfere with piping — Mitigation: Disable pager when stdout is not a TTY

## Dependencies

None

## Notes

- Use pager crate for cross-platform pager support
- Consider adding --pager flag to force pager even for short output
- Ensure pager is configured with appropriate flags for color support