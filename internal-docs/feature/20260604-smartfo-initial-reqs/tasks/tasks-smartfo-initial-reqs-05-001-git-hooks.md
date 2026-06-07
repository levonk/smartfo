---
story_id: "05-001"
story_title: "Git hooks — pre-commit and pre-receive"
story_name: "git-hooks"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 5
parallel_id: 1
branch: "feature/current/smartfo-initial-reqs/story-05-001-git-hooks"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["01-003", "02-002", "03-001", "03-002"]
parallel_safe: true
modules: ["hooks/"]
priority: "SHOULD"
risk_level: "medium"
tags: ["feat", "hooks", "git"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Implement the client-side `pre-commit` and server-side `pre-receive` Git hooks that block raw deletions and raw renames by cross-referencing the repo-local audit log.

## Sub-Tasks

- [x] Implement `smartfo-git-hook-client` subcommand that reads staged changes
- [x] Detect raw deletions (deleted file with no matching audit entry)
- [x] Detect raw renames (removed + added with similar content, no audit entry)
- [x] Implement `smartfo-git-hook-server` subcommand that reads incoming push
- [x] Cross-reference repo-local audit log at `{REPO_ROOT}/.smartfo/audit/operations.jsonl`
- [x] Generate clear error messages pointing users to use `smartfo` commands
- [x] Write integration tests with temp Git repos

## Relevant Files

- `src/main.rs` — Hook subcommand dispatch
- `src/hooks.rs` — Hook implementations (new file)
- `src/audit.rs` — Audit log reading for verification

## Acceptance Criteria

- [x] `pre-commit` blocks raw `rm` deletions with clear message
- [x] `pre-commit` blocks raw `mv` renames with clear message
- [x] `pre-receive` blocks pushes containing raw deletions
- [x] `pre-receive` blocks pushes containing raw renames
- [x] Valid smartfo operations pass hook verification
- [x] Server-side hook reads from repo-local audit log

## Test Plan

- Integration: tests with temp Git repos
- Unit: `cargo test hooks::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log hook invocation and decision at `debug` level

## Compliance

- Hooks must not leak sensitive path info in server-side errors beyond repo scope

## Risks & Mitigations

- Risk: Large repos may slow down pre-commit — Mitigation: Limit diff scope and use efficient parsing

## Dependencies & Sequencing

- Depends on: 01-003, 02-002, 03-001, 03-002
- Unblocks: 06-001, 06-002

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(hooks): add pre-commit raw deletion detection`

## Changelog

- 2026-06-05: initialized story file
