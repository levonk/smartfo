---
story_id: "04-002"
story_title: "Audit log sanitization + tests"
story_name: "audit-log-sanitization-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 4
parallel_id: 2
branch: "feature/current/requirements-gaps/story-04-002-audit-log-sanitization-tests"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["04-001"]
parallel_safe: true
modules: ["audit", "privacy", "tests"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "audit", "privacy", "test"]
due: "2026-07-22"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Implement audit log sanitization to remove sensitive paths when privacy mode is enabled. Sanitize audit logs based on privacy ignore patterns and ensure sensitive data is not logged. Addresses ADR #33 and ADR #34 from CLI standards.

## Sub-Tasks

- [ ] Implement path sanitization in `src/audit.rs`
- [ ] Add sensitive path detection
- [ ] Integrate privacy ignore patterns with audit logging
- [ ] Implement path redaction (e.g., /home/user → /home/****)
- [ ] Add sanitization for file names
- [ ] Add sanitization for VCS repo paths
- [ ] Implement audit log export with sanitization option
- [ ] Write unit tests for path sanitization
- [ ] Write integration tests for audit log sanitization
- [ ] Write tests for sensitive path detection
- [ ] Document sanitization behavior in README
- [ ] Add examples of sanitized audit logs

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/audit.rs` — Add sanitization logic
- `src/privacy.rs` — Use privacy patterns for sanitization
- `tests/audit_sanitization_test.rs` — New test file for audit sanitization
- `README.md` — Document sanitization behavior
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [ ] Sensitive paths are sanitized in audit logs
- [ ] Privacy ignore patterns are respected
- [ ] Path redaction works correctly
- [ ] Audit log export supports sanitization option
- [ ] All tests pass
- [ ] Documentation complete

## Test Plan

- Unit: `devbox run cargo test audit_sanitization_test`
- Integration: Test audit logging with privacy mode
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log sanitization actions
- Track sanitized entries

## Compliance

- Follow ADR #33 (Privacy Mode)
- Follow ADR #34 (Audit Logging with Retention)
- Ensure sanitization doesn't break audit log integrity

## Risks & Mitigations

- Risk: Sanitization may make audit logs unusable — Mitigation: Use reversible redaction where possible, document clearly
- Risk: Sanitization may miss sensitive paths — Mitigation: Use comprehensive patterns, allow user customization

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-04-001-privacy-mode-tests]]
- Unblocks: None

## Definition of Done

- Audit log sanitization implemented and tested
- Sensitive paths properly redacted
- Privacy integration working
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(audit): add audit log sanitization`

## Changelog

- 2026-06-11: initialized story file
