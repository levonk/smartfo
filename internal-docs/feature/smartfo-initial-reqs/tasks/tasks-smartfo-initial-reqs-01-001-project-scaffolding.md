---
story_id: "01-001"
story_title: "Project scaffolding and CLI framework"
story_name: "project-scaffolding"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 1
parallel_id: 1
branch: "feature/current/smartfo-initial-reqs/story-01-001-project-scaffolding"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["main.rs", "cli.rs", "Cargo.toml"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "cli", "scaffold"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Set up the Rust project scaffolding with Cargo, create the single-binary entrypoint with `argv[0]` dispatch, and implement POSIX-compatible flag parsers for both `mv` and `rm` modes. This story establishes the foundation that all other stories build upon.

## Sub-Tasks

- [x] Initialize Cargo project with appropriate dependencies (clap, serde, toml, sqlite, tracing, etc.)
- [x] Create `src/main.rs` with `argv[0]` dispatch logic (mv mode, rm mode, install mode, hook subcommands)
- [x] Create `src/cli.rs` with separate POSIX-compatible flag parsers for mv mode (`-f`, `-n`, `-i`, `-v`, `-T`, `-t`, `--backup`, `--plain`, etc.)
- [x] Create `src/cli.rs` rm mode parser (`-f`, `-i`, `-I`, `-r`, `-R`, `-d`, `--preserve-root`, `--one-file-system`, `--plain`, etc.)
- [x] Add `--install` flag and subcommand routing in main.rs
- [x] Add `--json` and `--dry-run` global output flags
- [x] Write unit tests for dispatch logic and flag parsing

## Relevant Files

- `Cargo.toml` — Project manifest with dependencies
- `src/main.rs` — Entry point and argv[0] dispatch
- `src/cli.rs` — CLI flag parsers for mv, rm, and install modes
- `src/cli.test.rs` — Unit tests for CLI parsing

## Acceptance Criteria

- [x] `cargo build` succeeds with no warnings
- [x] `./target/debug/smartfo --help` shows mv, rm, and install mode help
- [x] Invoking as `mv` (via symlink) dispatches to mv mode
- [x] Invoking as `rm` (via symlink) dispatches to rm mode
- [x] `--plain` flag is recognized in both mv and rm parsers
- [x] All standard POSIX flags for mv and rm are parsed correctly

## Test Plan

- Unit: `cargo test cli::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Add `tracing` initialization in main.rs for structured logging
- `--json` flag should emit structured operation previews in dry-run mode

## Compliance

- None

## Risks & Mitigations

- Risk: clap derive macro may not support all POSIX flag combinations — Mitigation: Use clap builder API where derive is insufficient

## Dependencies & Sequencing

- Depends on: None
- Unblocks: 02-001, 02-002, 03-001, 03-002

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(cli): add POSIX mv flag parser`

## Changelog

- 2026-06-05: initialized story file
