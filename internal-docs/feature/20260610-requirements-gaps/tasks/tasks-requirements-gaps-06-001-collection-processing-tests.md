---
story_id: "06-001"
story_title: "Collection/processing separation + tests"
story_name: "collection-processing-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 6
parallel_id: 1
branch: "feature/current/requirements-gaps/story-06-001-collection-processing-tests"
status: "completed"
assignee: ""
reviewer: ""
dependencies: ["03-002"]
parallel_safe: true
modules: ["daemon", "export", "tests"]
priority: "SHOULD"
risk_level: "medium"
tags: ["feat", "daemon", "export", "test"]
due: "2026-08-05"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Separate daemon collection (background job processing) from CLI processing. Allow data collection in one environment and processing in another. Add export commands for collected job data and analysis commands that operate on exported data without requiring daemon. Addresses ADR #28 from CLI standards.

## Sub-Tasks

- [x] Design export format for job data in `src/export.rs`
- [x] Implement job data export command
- [x] Add export filters (date range, status, type)
- [x] Implement export to JSON format
- [x] Implement export to TOON format
- [x] Add analysis commands for exported data
- [x] Implement analysis without daemon requirement
- [x] Add import command for exported data
- [x] Separate collection logic from processing logic
- [x] Write unit tests for export functionality
- [x] Write integration tests for export/import
- [x] Write tests for analysis commands
- [x] Document export/import workflow in README
- [x] Add examples of export/analysis usage

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/export.rs` — New module for export/import (created)
- `src/daemon.rs` — Separate collection from processing
- `src/queue.rs` — Add export functionality
- `src/cli.rs` — Add export/analysis commands
- `tests/export_test.rs` — New test file for export
- `README.md` — Document export/import workflow
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [x] Job data can be exported to JSON and TOON formats
- [x] Export filters work (date range, status, type)
- [x] Analysis commands work without daemon
- [x] Import command restores exported data
- [x] Collection separated from processing
- [x] All tests pass
- [x] Documentation complete

## Test Plan

- Unit: `devbox run cargo test export_test`
- Integration: Test export/import cycle
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log export operations
- Track export file sizes

## Compliance

- Follow ADR #28 (Collection vs Processing Separation)
- Ensure export format is stable and versioned

## Risks & Mitigations

- Risk: Export format may change breaking compatibility — Mitigation: Version export format, provide migration tools
- Risk: Analysis without daemon may be limited — Mitigation: Document limitations clearly

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-03-002-daemon-process-support-tests]]
- Unblocks: 07-001

## Definition of Done

- Collection/processing separation implemented
- Export/import commands working
- Analysis commands functional
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(daemon): separate collection from processing`

## Changelog

- 2026-06-11: initialized story file
