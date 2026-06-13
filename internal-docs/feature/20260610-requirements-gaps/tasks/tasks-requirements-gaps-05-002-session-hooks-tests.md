---
story_id: "05-002"
story_title: "Session hooks + tests"
story_name: "session-hooks-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 5
parallel_id: 2
branch: "feature/current/requirements-gaps/story-05-002-session-hooks-tests"
status: "done"
assignee: ""
reviewer: ""
dependencies: ["05-001"]
parallel_safe: true
modules: ["hooks", "session", "tests"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "axi", "hooks", "test"]
due: "2026-07-29"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Implement session hooks lifecycle capture including session-end hooks for transcripts, files touched, and VCS commands. Store session metadata in local cache for future context enrichment. Addresses AXI Requirement #8.

## Sub-Tasks

- [x] Implement session-end hook registration in `src/hooks.rs`
- [x] Add session metadata capture (transcripts, files, VCS commands)
- [x] Implement session metadata caching
- [x] Add session metadata loading for context enrichment
- [x] Integrate session-end hooks with Claude Code
- [x] Integrate session-end hooks with Codex
- [x] Add session lifecycle tracking
- [x] Implement session metadata cleanup (retention policy)
- [x] Write unit tests for session hooks
- [x] Write integration tests for session-end hooks
- [x] Write tests for session metadata caching
- [x] Document session hooks in README
- [x] Add examples of session metadata

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/hooks.rs` — Enhance with session-end hooks
- `src/session.rs` — New module for session lifecycle
- `tests/session_hooks_test.rs` — New test file for session hooks
- `README.md` — Document session hooks
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [x] Session-end hooks capture transcripts
- [x] Session-end hooks capture files touched
- [x] Session-end hooks capture VCS commands
- [x] Session metadata cached locally
- [x] Session metadata loaded for context enrichment
- [x] Session metadata cleanup works (retention policy)
- [x] All tests pass
- [x] Documentation complete

## Test Plan

- Unit: `devbox run cargo test session_hooks_test`
- Integration: Test session hooks with Claude Code and Codex
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log session hook executions
- Track session metadata cache size

## Compliance

- Follow AXI Requirement #8 (Ambient Context via Session Integrations)
- Ensure session metadata doesn't grow unbounded

## Risks & Mitigations

- Risk: Session metadata may grow too large — Mitigation: Implement retention policy, automatic cleanup
- Risk: Session hooks may slow down agent startup — Mitigation: Make hooks async, use efficient storage

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-05-001-content-first-behavior-tests]]
- Unblocks: None

## Definition of Done

- Session hooks implemented and tested
- Session metadata capture working
- Session metadata caching functional
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(hooks): add session hooks lifecycle capture`

## Changelog

- 2026-06-11: initialized story file
