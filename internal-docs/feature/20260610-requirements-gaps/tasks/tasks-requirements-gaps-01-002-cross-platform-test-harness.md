---
story_id: "01-002"
story_title: "Cross-platform test harness setup"
story_name: "cross-platform-test-harness"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 1
parallel_id: 2
branch: "feature/current/requirements-gaps/story-01-002-cross-platform-test-harness"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["tests", "platform"]
priority: "MUST"
risk_level: "medium"
tags: ["test", "platform", "infrastructure"]
due: "2026-07-01"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Set up cross-platform test harness to enable testing on Linux, macOS, and Windows. This includes platform-specific test configuration, conditional compilation, and CI integration for multi-platform testing.

## Sub-Tasks

- [x] Add platform detection utilities in `tests/platform/mod.rs`
- [x] Create platform-specific test configuration files
- [x] Add conditional compilation attributes for platform-specific tests
- [x] Set up CI matrix for multi-platform testing (Linux, macOS, Windows)
- [x] Add platform-specific path handling test utilities
- [x] Add platform-specific daemon test utilities
- [x] Add platform-specific VCS test utilities
- [x] Create platform-specific test fixtures
- [x] Add platform-specific test environment setup scripts
- [x] Document cross-platform testing approach in `tests/README.md`
- [x] Write integration tests for platform detection
- [x] Verify CI matrix works for all platforms

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `tests/platform/mod.rs` — New module for platform-specific test utilities (created)
- `tests/platform/platform_test.rs` — Platform detection tests (created)
- `tests/platform/linux_config.rs` — Linux-specific test configuration (created)
- `tests/platform/macos_config.rs` — macOS-specific test configuration (created)
- `tests/platform/windows_config.rs` — Windows-specific test configuration (created)
- `tests/platform/fixtures/mod.rs` — Platform-specific test fixtures module (created)
- `tests/platform/fixtures/paths.rs` — Platform-specific path fixtures (created)
- `tests/platform/setup.sh` — Unix environment setup script (created)
- `tests/platform/setup.ps1` — Windows environment setup script (created)
- `.github/workflows/ci.yml` — CI matrix already configured for all platforms
- `tests/README.md` — Cross-platform testing documentation (updated)

## Acceptance Criteria

- [x] Platform detection utilities work correctly on Linux, macOS, and Windows
- [x] CI matrix successfully runs tests on all three platforms (already configured)
- [x] Platform-specific test utilities are well-documented
- [x] All platform-specific tests pass on their respective platforms (platform tests pass)
- [x] Cross-platform testing approach is documented

## Test Plan

- Unit: `devbox run cargo test platform_test`
- Integration: Run CI matrix manually to verify all platforms
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Track test results per platform in CI
- Log platform detection for debugging

## Compliance

- Ensure platform-specific code follows Rust best practices
- No security vulnerabilities in platform detection code

## Risks & Mitigations

- Risk: Windows testing may be difficult to set up — Mitigation: Use GitHub Actions Windows runners, document Windows-specific setup
- Risk: Platform-specific code may become complex — Mitigation: Keep platform differences minimal, use abstractions

## Dependencies & Sequencing

- Depends on: None
- Unblocks: 02-003, 06-003

## Definition of Done

- Platform detection utilities implemented and tested
- CI matrix configured and working
- Documentation complete
- All platform-specific tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `test(platform): add cross-platform test harness`

## Changelog

- 2026-06-11: initialized story file
