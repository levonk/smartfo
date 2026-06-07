---
story_id: "06-001"
story_title: "Collection/Processing Separation"
story_name: "collection-processing-separation"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 6
parallel_id: 1
branch: "feature/current/cli-standards-compliance/story-06-001-collection-processing-separation"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["03-001"]
parallel_safe: true
modules: ["daemon.rs", "export.rs (new)"]
priority: "SHOULD"
risk_level: "medium"
tags: ["feat", "architecture"]
due: "2026-08-30"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement collection vs processing separation as specified in ADR #28. Separate daemon collection (background job processing) from CLI processing, allow data collection in one environment and processing in another, add export commands for collected job data, add analysis commands that operate on exported data without requiring daemon, and enhance existing daemon model with explicit export/analysis commands.

## Sub-Tasks

- [ ] Create new src/export.rs module for data export functionality
- [ ] Define data export format (JSON, CSV, or custom format)
- [ ] Implement export command for collected job data
- [ ] Add --export <format> flag to export daemon job data
- [ ] Add --export-file <path> flag to specify export destination
- [ ] Implement analysis command for exported data
- [ ] Add --analyze <file> flag to analyze exported data
- [ ] Ensure analysis commands work without daemon running
- [ ] Separate collection logic from processing logic in daemon.rs
- [ ] Add metadata to exported data (collection time, environment, etc.)
- [ ] Implement data filtering in export command
- [ ] Implement data aggregation in analysis command
- [ ] Add unit tests for export functionality
- [ ] Add unit tests for analysis functionality
- [ ] Add integration tests for collection/processing separation
- [ ] Document export and analysis commands in help output

## Relevant Files

- `src/export.rs` (new) — Implement data export and analysis logic
- `src/daemon.rs` — Separate collection from processing
- `src/cli.rs` — Add --export and --analyze flags
- `src/queue.rs` — Enhance with export metadata
- `tests/export_tests.rs` — Add tests for export/analysis

## Acceptance Criteria

- [ ] Daemon collection is separated from CLI processing
- [ ] Data can be collected in one environment and processed in another
- [ ] Export command exists for collected job data
- [ ] Analysis commands exist for exported data
- [ ] Analysis commands work without daemon running
- [ ] Export format is well-defined and documented
- [ ] Export data includes metadata (collection time, environment, etc.)
- [ ] Data filtering works in export command
- [ ] Data aggregation works in analysis command
- [ ] All tests pass

## Test Plan

- Unit: `cargo test export_tests::export_functionality`
- Unit: `cargo test export_tests::analysis_functionality`
- Unit: `cargo test export_tests::data_filtering`
- Integration: `cargo test export_tests::collection_separation`
- Integration: `cargo test export_tests::cross_environment`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log export events (info level)
- Log analysis events (info level)
- Log collection/processing separation (debug level)

## Compliance

- Follows ADR #28: Collection vs Processing Separation

## Risks & Mitigations

- Risk: Export format may change over time — Mitigation: Add version field to export format
- Risk: Analysis commands may become complex — Mitigation: Keep analysis focused and extensible

## Dependencies

- 03-001 (Daemon Enhancements) — ensures daemon model is enhanced with job management

## Notes

- Use JSON as primary export format for compatibility
- Consider adding --export-format flag to support multiple formats
- Ensure export data can be imported back into daemon if needed