---
story_id: "01-003"
story_title: "Logging and audit trail infrastructure"
story_name: "logging-audit"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 1
parallel_id: 3
branch: "feature/current/smartfo-initial-reqs/story-01-003-logging-audit"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["logging.rs", "audit.rs"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "observability"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Set up structured logging with `tracing` and implement the audit trail system that records every `mv` and `rm` operation to an append-only JSONL file.

## Sub-Tasks

- [x] Initialize `tracing` subscriber with configurable format (pretty/JSON) and log level
- [x] Create `src/logging.rs` with log level resolution order: env → CLI → config → default
- [x] Define `AuditEntry` struct with all required fields (op, source_path, trash_path, reason, timestamp, uuid, tool, vcs, repo_root, committed)
- [x] Implement `append_audit_log()` that appends JSON to `$XDG_DATA_HOME/smartfo/audit/operations.jsonl`
- [x] Ensure parent directories are created on first write
- [x] Write unit tests for audit entry serialization and appending

## Relevant Files

- `src/logging.rs` — Structured logging setup with tracing subscriber
- `src/audit.rs` — Audit entry types and append logic with unit tests
- `Cargo.toml` — Added uuid dependency

## Acceptance Criteria

- [ ] Log output respects `--json` flag for machine-readable format
- [ ] Audit log file is created automatically on first operation
- [ ] Each audit entry contains all required fields as valid JSON
- [ ] Audit entries are appended as JSONL (one JSON object per line)
- [ ] `--reason "..."` is captured in the audit entry
- [ ] Invalid log paths produce clear errors

## Test Plan

- Unit: `cargo test audit::` and `cargo test logging::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log startup banner with version and config path
- Log every operation at `info` level with key metadata

## Compliance

- Audit logs must not contain sensitive file content, only paths and metadata

## Risks & Mitigations

- Risk: Concurrent writes to audit log from multiple processes — Mitigation: Use file locking or atomic appends

## Dependencies & Sequencing

- Depends on: None
- Unblocks: 03-003, 04-001, 05-001, 06-001, 06-002

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(audit): add JSONL audit trail`

## Changelog

- 2026-06-05: initialized story file
