---
story_id: "05-001"
story_title: "Content-first behavior + tests"
story_name: "content-first-behavior-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 5
parallel_id: 1
branch: "feature/current/requirements-gaps/story-05-001-content-first-behavior-tests"
status: "todo"
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

- [ ] Design content-first state summary in `src/output/content_first.rs`
- [ ] Implement state summary generation for current directory
- [ ] Add context-aware state (git repo info, operations, queue)
- [ ] Modify no-args behavior in `src/main.rs` to show state
- [ ] Add state summary for agent mode
- [ ] Add state summary for human mode
- [ ] Include contextual suggestions in state output
- [ ] Add TOON format for state summary
- [ ] Add human-readable format for state summary
- [ ] Write unit tests for state generation
- [ ] Write integration tests for no-args behavior
- [ ] Write tests for context-aware state
- [ ] Update help text to reference state summary
- [ ] Document content-first behavior in README

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/output/content_first.rs` — New module for content-first state
- `src/main.rs` — Modify no-args behavior
- `src/output.rs` — Integrate state summary with output
- `src/hooks.rs` — Use session context for state
- `tests/content_first_test.rs` — New test file for content-first behavior
- `README.md` — Document content-first behavior
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [ ] No-args shows live state summary instead of help
- [ ] State summary includes relevant context (git repo, operations, queue)
- [ ] State summary includes contextual suggestions
- [ ] TOON format works for agent mode
- [ ] Human format works for human mode
- [ ] All tests pass
- [ ] Documentation complete

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
