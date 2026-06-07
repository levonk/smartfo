---
story_id: "07-003"
story_title: "Audit Logging Enhancements"
story_name: "audit-logging-enhancements"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 7
parallel_id: 3
branch: "feature/current/cli-standards-compliance/story-07-003-audit-logging-enhancements"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["07-002"]
parallel_safe: true
modules: ["audit.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "audit"]
due: "2026-09-13"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Enhance audit logging with retention as specified in ADR #34. The existing audit logging in src/audit.rs needs enhancements: configurable retention period (default 90 days), automatic cleanup of old audit entries, audit log rotation to prevent unbounded growth, support for export of audit logs, and privacy mode integration to sanitize sensitive entries.

## Sub-Tasks

- [ ] Audit existing audit logging implementation in src/audit.rs
- [ ] Add audit_retention_days config option (default 90 days)
- [ ] Implement automatic cleanup of old audit entries based on retention period
- [ ] Implement audit log rotation to prevent unbounded growth
- [ ] Add audit_log_max_size config option for rotation threshold
- [ ] Add audit_log_max_files config option for number of rotated files to keep
- [ ] Implement audit log export functionality
- [ ] Add --export-audit <file> flag to export audit logs
- [ ] Add --export-audit-format <format> flag (JSON, CSV)
- [ ] Integrate privacy mode sanitization with audit log export
- [ ] Add audit log statistics command (--audit-stats)
- [ ] Add audit log search functionality (--audit-search <pattern>)
- [ ] Implement audit log compression for old entries
- [ ] Add unit tests for retention cleanup
- [ ] Add unit tests for log rotation
- [ ] Add unit tests for audit log export
- [ ] Add unit tests for privacy mode integration
- [ ] Add integration tests for audit log lifecycle

## Relevant Files

- `src/audit.rs` — Enhance with retention, rotation, and export
- `src/config.rs` — Add audit retention and rotation config options
- `src/cli.rs` — Add --export-audit, --audit-stats, --audit-search flags
- `tests/audit_tests.rs` — Add tests for audit logging enhancements

## Acceptance Criteria

- [ ] Configurable retention period exists (default 90 days)
- [ ] Automatic cleanup of old audit entries works
- [ ] Audit log rotation prevents unbounded growth
- [ ] Audit log export functionality exists
- [ ] --export-audit flag exports audit logs
- [ ] --export-audit-format flag supports JSON and CSV
- [ ] Privacy mode sanitization integrates with audit log export
- [ ] --audit-stats command shows audit log statistics
- [ ] --audit-search command searches audit logs
- [ ] Audit log compression works for old entries
- [ ] All tests pass

## Test Plan

- Unit: `cargo test audit_tests::retention_cleanup`
- Unit: `cargo test audit_tests::log_rotation`
- Unit: `cargo test audit_tests::audit_export`
- Unit: `cargo test audit_tests::privacy_integration`
- Integration: `cargo test audit_tests::audit_lifecycle`
- Integration: `cargo test audit_tests::export_flags`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log audit cleanup events (info level)
- Log audit rotation events (info level)
- Log audit export events (info level)

## Compliance

- Follows ADR #34: Audit Logging with Retention

## Risks & Mitigations

- Risk: Automatic cleanup may delete important data — Mitigation: Provide warning and backup option before cleanup
- Risk: Log rotation may impact performance — Mitigation: Use efficient rotation logic and async operations

## Dependencies

- 07-002 (Privacy Mode Implementation) — ensures privacy sanitization is available

## Notes

- Use JSONL format for audit logs (one JSON object per line)
- Consider adding --audit-backup flag to backup audit logs before cleanup
- Ensure audit log export includes all metadata fields