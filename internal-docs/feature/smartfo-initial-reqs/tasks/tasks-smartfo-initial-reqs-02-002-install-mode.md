---
story_id: "02-002"
story_title: "Install mode with symlinks and Git hooks"
story_name: "install-mode"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 2
parallel_id: 2
branch: "feature/current/smartfo-initial-reqs/story-02-002-install-mode"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-001"]
parallel_safe: true
modules: ["cli.rs", "main.rs"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "install"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Implement `smartfo --install` to create symlinks (`mv`, `rm`, `smv`, `srm`) and optionally install Git client-side and server-side hooks. Handle existing files with `--force` and respect `$XDG_BIN_HOME`.

## Sub-Tasks

- [ ] Implement symlink target directory resolution (`$XDG_BIN_HOME`, `~/.local/bin`, `/usr/local/bin` for root)
- [ ] Create symlinks for `mv`, `rm`, `smv`, `srm` pointing to the smartfo binary
- [ ] Detect existing non-smartfo files and refuse unless `--force` is passed
- [ ] Detect if invoked inside a Git repo and conditionally install hooks
- [ ] Implement `smartfo-git-hook-client` subcommand (pre-commit hook)
- [ ] Implement `smartfo-git-hook-server` subcommand (pre-receive hook)
- [ ] Support `--hooks client`, `--hooks server`, `--hooks client,server`, `--no-hooks`
- [ ] Write unit tests for install logic with temp directories

## Relevant Files

- `src/main.rs` — Install mode dispatch and hook subcommands
- `src/cli.rs` — `--install` flag and hook options
- `src/hooks/` — Hook script templates (if separated)

## Acceptance Criteria

- [ ] `smartfo --install` creates symlinks in the correct directory
- [ ] Existing non-smartfo binaries are refused without `--force`
- [ ] `--hooks client` installs only `.git/hooks/pre-commit`
- [ ] `--hooks server` installs only `.git/hooks/pre-receive`
- [ ] `--no-hooks` skips all hook installation
- [ ] Root install targets `/usr/local/bin` when on PATH

## Test Plan

- Unit: `cargo test install::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log each symlink and hook created at `info` level
- Log warnings for skipped existing files

## Compliance

- None

## Risks & Mitigations

- Risk: Installing hooks outside a git repo — Mitigation: Skip hooks with a warning when not in a repo

## Dependencies & Sequencing

- Depends on: 01-001
- Unblocks: 05-001

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(install): add symlink creation logic`

## Changelog

- 2026-06-05: initialized story file
