---
story_id: "04-003"
story_title: "Secret handling + tests"
story_name: "secret-handling-tests"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 4
parallel_id: 3
branch: "feature/current/requirements-gaps/story-04-003-secret-handling-tests"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: ["04-001"]
parallel_safe: true
modules: ["logging", "security", "tests"]
priority: "MUST"
risk_level: "high"
tags: ["feat", "security", "logging", "test"]
due: "2026-07-22"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Implement secret handling to ensure no logging of secrets or sensitive data. Add secure storage options for credentials (if needed), add clear warnings about insecure config methods, and sanitize audit logs in privacy mode. Addresses ADR #25 from CLI standards.

## Sub-Tasks

- [x] Add secret detection utilities in `src/secret.rs` (already implemented)
- [x] Implement secret redaction in logging
- [x] Add secret detection for common patterns (API keys, tokens, passwords)
- [x] Add secret detection in config values
- [x] Add warnings for insecure config methods
- [x] Implement secure credential storage (if needed)
- [x] Add secret sanitization to all log outputs
- [x] Add secret sanitization to error messages
- [x] Write unit tests for secret detection
- [x] Write integration tests for secret redaction
- [x] Write tests for insecure config warnings
- [x] Document secret handling in README
- [x] Add examples of secure vs insecure config

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/security.rs` — New security module for secret handling
- `src/logging.rs` — Add secret redaction to logging
- `src/config.rs` — Add insecure config warnings
- `tests/security_test.rs` — New test file for secret handling
- `README.md` — Document secret handling
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [x] Secrets are detected and redacted in logs
- [x] Secrets are detected in config values
- [x] Insecure config methods trigger warnings
- [x] Error messages don't leak secrets
- [x] All tests pass
- [x] Documentation complete

## Test Plan

- Unit: `devbox run cargo test security_test`
- Integration: Test logging with secrets
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log secret redactions (without the secrets)
- Track insecure config usage

## Compliance

- Follow ADR #25 (Credential/Secret Handling)
- Ensure no secrets in logs or error messages
- Follow security best practices

## Risks & Mitigations

- Risk: Secret detection may be too aggressive — Mitigation: Use precise patterns, allow false positive overrides
- Risk: Secret redaction may break debugging — Mitigation: Add debug mode that shows redacted secrets with warning

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-04-001-privacy-mode-tests]]
- Unblocks: None

## Definition of Done

- Secret handling implemented and tested
- Secret detection and redaction working
- Insecure config warnings functional
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(security): add secret handling and redaction`

## Changelog

- 2026-06-11: initialized story file
