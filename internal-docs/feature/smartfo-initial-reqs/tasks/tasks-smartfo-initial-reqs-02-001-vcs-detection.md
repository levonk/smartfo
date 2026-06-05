---
story_id: "02-001"
story_title: "VCS detection and tracked-file logic"
story_name: "vcs-detection"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 2
parallel_id: 1
branch: "feature/current/smartfo-initial-reqs/story-02-001-vcs-detection"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["01-001"]
parallel_safe: true
modules: ["vcs.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "vcs"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Implement VCS detection for Git, Mercurial, SVN, and Jujutsu. Discover repo roots and determine whether a given path is tracked by the detected VCS. This module is the core intelligence behind VCS-aware moves and deletes.

## Sub-Tasks

- [x] Define `VcsType` enum (Git, Hg, Svn, Jj) and `VcsInfo` struct
- [x] Implement Git repo root detection (`git rev-parse --show-toplevel`)
- [x] Implement Git tracked-file check (`git ls-files --error-unmatch`)
- [x] Implement Mercurial repo root detection (`hg root`)
- [x] Implement Mercurial tracked-file check (`hg status -mu` exclusion)
- [x] Implement SVN repo root detection (`svn info`)
- [x] Implement SVN tracked-file check (`svn list` or `svn status`)
- [x] Implement Jujutsu repo root detection (`jj root`)
- [x] Implement Jujutsu tracked-file check (`jj file list`)
- [x] Add fallback chain when multiple VCS are present
- [ ] Write unit tests with mock repo fixtures

## Relevant Files

- `src/vcs.rs` — VCS detection and tracked-file logic (created)
- `src/main.rs` — Added vcs module declaration

## Acceptance Criteria

- [ ] Git repo root is correctly discovered for nested directories
- [ ] Tracked files return `true`; untracked return `false`
- [ ] Non-repo directories return `None` for VCS info
- [ ] Each supported VCS (git, hg, svn, jj) has working detection
- [ ] Config `vcs_list` filters which VCS are attempted
- [ ] Performance: detection completes in <100ms for typical repos

## Test Plan

- Unit: `cargo test vcs::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log detected VCS and repo root at `debug` level
- Log tracked/untracked determination at `trace` level

## Compliance

- None

## Risks & Mitigations

- Risk: VCS CLI tools may not be installed — Mitigation: Gracefully handle `CommandNotFound` and skip that VCS
- Risk: Large repos may be slow for tracked-file checks — Mitigation: Use VCS-native fast paths (`git ls-files` over `git status`)

## Dependencies & Sequencing

- Depends on: 01-001
- Unblocks: 03-001, 03-002

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(vcs): add git repo detection`

## Changelog

- 2026-06-05: initialized story file
