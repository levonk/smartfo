---
story_id: "04-005"
story_title: "Environment Variable Naming"
story_name: "env-var-naming"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 4
parallel_id: 5
branch: "feature/current/cli-standards-compliance/story-04-005-env-var-naming"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-002"]
parallel_safe: true
modules: ["config.rs"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "config"]
due: "2026-08-02"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement environment variable naming as specified in ADR #23. Ensure all environment variables use consistent SMARTFO_ prefix, document all supported environment variables, ensure env var naming follows section_key pattern (e.g., SMARTFO_BEHAVIOR_DEFAULT_BLOCKING), and complete coverage for all config options (partially implemented).

## Sub-Tasks

- [ ] Audit all existing environment variables for SMARTFO_ prefix consistency
- [ ] Document all supported environment variables in help output
- [ ] Document all supported environment variables in man pages
- [ ] Ensure env var naming follows section_key pattern (e.g., SMARTFO_BEHAVIOR_DEFAULT_BLOCKING)
- [ ] Add environment variable for all vcs config options
- [ ] Add environment variable for all trash config options
- [ ] Add environment variable for all concurrency config options
- [ ] Add environment variable for all behavior config options
- [ ] Add environment variable for all logging config options
- [ ] Add environment variable for all paths config options
- [ ] Ensure environment variable expansion works in config file values
- [ ] Add unit tests for environment variable parsing
- [ ] Add unit tests for environment variable expansion
- [ ] Add integration tests for environment variable precedence
- [ ] Verify environment variables work correctly with all config sections

## Relevant Files

- `src/config.rs` — Complete environment variable coverage for all config options
- `src/cli.rs` — Document environment variables in help output
- `docs/smartfo.1` — Document environment variables in man pages
- `tests/env_var_tests.rs` — Add tests for environment variables

## Acceptance Criteria

- [ ] All environment variables use consistent SMARTFO_ prefix
- [ ] All environment variables are documented in help output
- [ ] All environment variables are documented in man pages
- [ ] Env var naming follows section_key pattern (e.g., SMARTFO_BEHAVIOR_DEFAULT_BLOCKING)
- [ ] Environment variables exist for all config options
- [ ] Environment variable expansion works in config file values
- [ ] All tests pass

## Test Plan

- Unit: `cargo test env_var_tests::naming_convention`
- Unit: `cargo test env_var_tests::parsing`
- Unit: `cargo test env_var_tests::expansion`
- Integration: `cargo test env_var_tests::precedence`
- Integration: `cargo test env_var_tests::config_coverage`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log environment variable usage (debug level)

## Compliance

- Follows ADR #23: Environment Variable Naming

## Risks & Mitigations

- Risk: Too many environment variables may be overwhelming — Mitigation: Group related variables and document clearly
- Risk: Environment variable names may become too long — Mitigation: Keep section and key names concise

## Dependencies

- 01-002 (Config Initialization & System Config) — ensures config file structure is established

## Notes

- Use uppercase for all environment variable names
- Use double underscore for nested structures if needed
- Consider adding --env flag to list all supported environment variables