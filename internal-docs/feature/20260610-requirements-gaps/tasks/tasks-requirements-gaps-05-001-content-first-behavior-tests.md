---
story_id: "05-001"
story_title: "Content-first behavior + tests"
story_name: "content-first-behavior-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 5
parallel_id: 1
branch: "feature/current/requirements-gaps/story-05-001-content-first-behavior-tests"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: ["03-001"]
parallel_safe: true
modules: ["cli", "output", "tests"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "axi", "cli", "test"]
due: "2026-07-29"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Implement content-first no-args behavior where running CLI with no arguments shows most relevant live state instead of usage manual. When agent sees actual state, it can act immediately. Addresses AXI Requirement #10.

## Sub-Tasks

- [x] Design content-first state summary in `src/output/content_first.rs`
- [x] Implement state summary generation for current directory
- [x] Add context-aware state (git repo info, operations, queue)
- [x] Modify no-args behavior in `src/main.rs` to show state
- [x] Add state summary for agent mode
- [x] Add state summary for human mode
- [x] Include contextual suggestions in state output
- [x] Add TOON format for state summary
- [x] Add human-readable format for state summary
- [x] Write unit tests for state generation
- [x] Write integration tests for no-args behavior
- [x] Write tests for context-aware state
- [x] Update help text to reference state summary
- [x] Document content-first behavior in README

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/output/content_first.rs` — New module for content-first state summary generation
- `src/output/mod.rs` — Added content_first module and exports
- `src/main.rs` — Modified no-args behavior to use StateSummary
- `src/lib.rs` — Added vcs, daemon, queue, mv, rm modules for content_first
- `tests/cli_tests.rs` — Updated test to match TOON format output
- `README.md` — Added documentation for content-first behavior
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [x] No-args shows live state summary instead of help
- [x] State summary includes relevant context (git repo, operations, queue)
- [x] State summary includes contextual suggestions
- [x] TOON format works for agent mode
- [x] Human format works for human mode
- [x] All tests pass
- [x] Documentation complete

## Test Plan

- Unit: `devbox run cargo test content_first_test`
- Integration: Test no-args behavior in various contexts
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log state summary generation
- Track no-args invocations

## Compliance

- Follow AXI Requirement #10 (Content First)
- Ensure state summary is token-efficient for agents

## Risks & Mitigations

- Risk: State summary may be too verbose — Mitigation: Keep it concise, use TOON format for agents
- Risk: State summary may be slow to generate — Mitigation: Cache state, use efficient queries

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-03-001-tui-mode-framework-tests]]
- Unblocks: 05-002, 05-003

## Definition of Done

- Content-first behavior implemented and tested
- State summary working in both modes
- Contextual suggestions included
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(output): add content-first no-args behavior`

## Changelog

- 2026-06-11: initialized story file
