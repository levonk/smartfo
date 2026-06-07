---
story_id: "05-002"
story_title: "Credential/Secret Handling"
story_name: "credential-handling"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 5
parallel_id: 2
branch: "feature/current/cli-standards-compliance/story-05-002-credential-handling"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["config.rs", "logging.rs"]
priority: "MUST"
risk_level: "high"
tags: ["feat", "security"]
due: "2026-08-16"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement credential and secret handling as specified in ADR #25. Ensure no logging of secrets or sensitive data, provide secure storage options for any credentials (if needed in future), add clear warnings about insecure config methods, and sanitize audit logs to remove sensitive paths if privacy mode enabled.

## Sub-Tasks

- [ ] Audit all logging for potential secret leakage
- [ ] Implement secret redaction in logging module
- [ ] Redact sensitive paths from logs (e.g., ~/.ssh/, ~/.aws/, etc.)
- [ ] Redact environment variables containing secrets from logs
- [ ] Add warning for insecure config methods (e.g., plaintext credentials)
- [ ] Implement secure storage options for credentials (if needed)
- [ ] Sanitize audit logs to remove sensitive paths when privacy mode enabled
- [ ] Add secret detection patterns (e.g., API keys, passwords, tokens)
- [ ] Implement secret redaction in error messages
- [ ] Implement secret redaction in debug output
- [ ] Add unit tests for secret redaction in logs
- [ ] Add unit tests for secret redaction in audit logs
- [ ] Add unit tests for secret detection patterns
- [ ] Add integration tests for privacy mode sanitization
- [ ] Document security best practices for configuration

## Relevant Files

- `src/logging.rs` — Implement secret redaction in logging
- `src/config.rs` — Add warnings for insecure config methods
- `src/audit.rs` — Implement audit log sanitization
- `tests/security_tests.rs` — Add tests for credential handling

## Acceptance Criteria

- [ ] No secrets or sensitive data are logged
- [ ] Sensitive paths are redacted from logs
- [ ] Environment variables containing secrets are redacted from logs
- [ ] Clear warnings are added for insecure config methods
- [ ] Secure storage options are provided for credentials (if needed)
- [ ] Audit logs are sanitized to remove sensitive paths when privacy mode enabled
- [ ] Secret detection patterns work correctly
- [ ] Error messages do not contain secrets
- [ ] Debug output does not contain secrets
- [ ] All tests pass

## Test Plan

- Unit: `cargo test security_tests::secret_redaction_logs`
- Unit: `cargo test security_tests::secret_redaction_audit`
- Unit: `cargo test security_tests::secret_detection_patterns`
- Unit: `cargo test security_tests::insecure_config_warnings`
- Integration: `cargo test security_tests::privacy_mode_sanitization`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log secret redaction events (debug level, with redacted placeholders)

## Compliance

- Follows ADR #25: Credential/Secret Handling

## Risks & Mitigations

- Risk: Secret detection may have false positives — Mitigation: Use conservative patterns and allow whitelist
- Risk: Secure storage may add complexity — Mitigation: Implement only if credentials are actually needed

## Dependencies

None

## Notes

- Use common secret patterns: API keys, passwords, tokens, private keys
- Consider adding keyring support for secure credential storage
- Document which paths are considered sensitive and redacted