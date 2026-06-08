---
story_id: "02-002"
story_title: "Pre-computed Aggregates"
story_name: "pre-computed-aggregates"
prd_name: "cli-axi"
prd_file: "internal-docs/feature/20260608-cli-axi/prd-20260608-cli-axi.md"
phase: 2
parallel_id: 2
branch: "feature/current/cli-axi/story-02-002-pre-computed-aggregates"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-003"]
parallel_safe: true
modules: ["cli.rs", "output.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "backend"]
due: "2025-01-15"
created_at: "2025-01-08"
updated_at: "2025-01-08"
---

## Summary

Implement pre-computed aggregate counts and derived status fields in list outputs. Include total counts (not just page size) and lightweight summaries (operations completed, queue pending). Compute counts efficiently at query time.

## Sub-Tasks

- [ ] Implement aggregate count computation for list queries
- [ ] Add total count to list output format (count: X of Y total)
- [ ] Implement derived status fields (operations: 3/3 completed, queue: 7 pending)
- [ ] Optimize count queries for efficiency
- [ ] Add derived fields to detail views where relevant
- [ ] Update list command to include operation count aggregates
- [ ] Update status command to include queue and daemon aggregates
- [ ] Add aggregate computation tests
- [ ] Ensure aggregates work with both TOON and JSON
- [ ] Update CLI help text to document aggregate fields

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/output/aggregates.rs` (new) — Aggregate computation logic
- `src/cli/list.rs` — Add operation count aggregates
- `src/cli/status.rs` — Add queue and daemon aggregates
- `tests/aggregates_test.rs` (new) — Aggregate tests

## Acceptance Criteria

- [ ] List outputs show total counts (count: X of Y total)
- [ ] Derived status fields are included inline
- [ ] Aggregate computation is efficient
- [ ] Aggregates work with both TOON and JSON
- [ ] List command shows operation count
- [ ] Status command shows queue and daemon aggregates
- [ ] Aggregates are computed at query time

## Test Plan

- Unit: `cargo test output::aggregates`
- Integration: Test aggregates for all list commands
- Performance: Benchmark aggregate computation
- Lint: `cargo clippy`
- Types: `cargo check`

## Observability

- Log aggregate computation performance
- Track aggregate field usage

## Compliance

- Follow AXI pre-computed aggregate requirements
- Ensure aggregates don't significantly impact performance

## Risks & Mitigations

- Risk: Performance impact from aggregate computation — Mitigation: Optimize queries and cache where appropriate
- Risk: Complex aggregate logic — Mitigation: Keep aggregates simple and well-documented

## Dependencies

- 01-003 (Minimal Default Schemas) — Schemas define which aggregates to include

## Notes

- Aggregates reduce need for follow-up API calls
- Keep aggregates lightweight and cheap to compute
