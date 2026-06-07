---
story_id: "06-002"
story_title: "Config File Versioning"
story_name: "config-versioning"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 6
parallel_id: 2
branch: "feature/current/cli-standards-compliance/story-06-002-config-versioning"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["04-003"]
parallel_safe: true
modules: ["config.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "config"]
due: "2026-08-30"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement config file versioning as specified in ADR #29. Add schema version field to config files for future evolution, validate config schema version on load, reject configs with unsupported schema versions with clear error message, and document config schema version in upgrade notes.

## Sub-Tasks

- [ ] Define config schema version format (semantic versioning)
- [ ] Add schema_version field to config file structure
- [ ] Set initial schema version to 1.0.0
- [ ] Implement schema version validation on config load
- [ ] Implement support for multiple supported schema versions
- [ ] Add clear error message for unsupported schema versions
- [ ] Add upgrade path suggestions for outdated configs
- [ ] Document current schema version in config file comments
- [ ] Document config schema version in man pages
- [ ] Document config schema version in upgrade notes
- [ ] Add --check-config-version flag to validate config version
- [ ] Implement config migration logic for future version changes
- [ ] Add unit tests for schema version validation
- [ ] Add unit tests for unsupported version rejection
- [ ] Add integration tests for config version handling
- [ ] Test config migration scenarios

## Relevant Files

- `src/config.rs` — Implement schema version field and validation
- `src/cli.rs` — Add --check-config-version flag
- `docs/smartfo.1` — Document config schema version
- `CHANGELOG.md` — Document schema version changes
- `tests/config_tests.rs` — Add tests for config versioning

## Acceptance Criteria

- [ ] Schema version field is present in config files
- [ ] Schema version is validated on config load
- [ ] Configs with unsupported schema versions are rejected
- [ ] Clear error message is provided for unsupported versions
- [ ] Upgrade path suggestions are provided for outdated configs
- [ ] Schema version is documented in config file comments
- [ ] Schema version is documented in man pages
- [ ] Schema version is documented in upgrade notes
- [ ] --check-config-version flag validates config version
- [ ] Config migration logic exists for future version changes
- [ ] All tests pass

## Test Plan

- Unit: `cargo test config_tests::schema_version_validation`
- Unit: `cargo test config_tests::unsupported_version_rejection`
- Unit: `cargo test config_tests::version_migration`
- Integration: `cargo test config_tests::config_version_check`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log config schema version on load (info level)
- Log config version validation results (debug level)

## Compliance

- Follows ADR #29: Config File Versioning

## Risks & Mitigations

- Risk: Version validation may break existing configs — Mitigation: Support multiple versions and provide migration path
- Risk: Schema changes may be frequent — Mitigation: Use semantic versioning and document breaking changes

## Dependencies

- 04-003 (Configuration Validation) — ensures config validation infrastructure exists

## Notes

- Use semantic versioning (MAJOR.MINOR.PATCH) for schema versions
- Consider adding --migrate-config flag to automatically migrate configs
- Document migration steps for each schema version change