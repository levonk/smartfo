---
story_id: "05-003"
story_title: "Contextual disclosure + tests"
story_name: "contextual-disclosure-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 5
parallel_id: 3
branch: "feature/current/requirements-gaps/story-05-003-contextual-disclosure-tests"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["05-001"]
parallel_safe: true
modules: ["output", "suggestions", "tests"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "axi", "output", "test"]
due: "2026-07-29"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Implement contextual disclosure engine that provides next steps suggestions based on current output. Include few relevant suggestions (2-4 maximum) that are actionable and complete commands. Format as structured `help[]` array in TOON output. Addresses AXI Requirement #11.

## Sub-Tasks

- [ ] Enhance suggestion engine in `src/output/suggestions.rs`
- [ ] Add context-aware suggestion generation
- [ ] Implement suggestion logic for each command type
- [ ] Add suggestions for viewing operations
- [ ] Add suggestions for empty lists
- [ ] Add suggestions for queue status
- [ ] Add suggestions for install operations
- [ ] Format suggestions as structured `help[]` array in TOON
- [ ] Ensure suggestions are complete commands with flags
- [ ] Limit suggestions to 2-4 maximum
- [ ] Rank suggestions by relevance
- [ ] Write unit tests for suggestion engine
- [ ] Write integration tests for contextual suggestions
- [ ] Document suggestion behavior in README

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/output/suggestions.rs` — Enhance suggestion engine
- `src/output/toon.rs` — Add help[] array formatting
- `src/cli.rs` — Integrate suggestions with commands
- `tests/suggestions_test.rs` — New test file for suggestions
- `README.md` — Document suggestion behavior
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [ ] Suggestions are context-aware based on current state
- [ ] Suggestions are actionable complete commands
- [ ] Suggestions limited to 2-4 maximum
- [ ] Suggestions ranked by relevance
- [ ] TOON format includes structured `help[]` array
- [ ] All tests pass
- [ ] Documentation complete

## Test Plan

- Unit: `devbox run cargo test suggestions_test`
- Integration: Test suggestions with various command outputs
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log suggestion generation
- Track suggestion usage

## Compliance

- Follow AXI Requirement #11 (Contextual Disclosure)
- Ensure suggestions are token-efficient for agents

## Risks & Mitigations

- Risk: Suggestions may be irrelevant — Mitigation: Use smart context detection, test with real scenarios
- Risk: Suggestions may be too verbose — Mitigation: Keep commands concise, use aliases where appropriate

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-05-001-content-first-behavior-tests]]
- Unblocks: 06-002

## Definition of Done

- Contextual disclosure implemented and tested
- Suggestion engine working
- TOON help[] array functional
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(output): add contextual disclosure engine`

## Changelog

- 2026-06-11: initialized story file
