---
story_id: "06-003"
story_title: "Cross-platform validation"
story_name: "cross-platform-validation"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 6
parallel_id: 3
branch: "feature/current/requirements-gaps/story-06-003-cross-platform-validation"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-002", "04-003"]
parallel_safe: true
modules: ["tests", "platform"]
priority: "MUST"
risk_level: "high"
tags: ["test", "platform", "validation"]
due: "2026-08-05"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## SummaryValidate smartfo works correctly on Linux, macOS, and Windows. Ensure consistent path handling across platforms, test all new features on all three platforms, and validate cross-platform test harness. Addresses ADR #24 from CLI standards.

## Sub-Tasks

- [ ] Run full test suite on Linux
- [ ] Run full test suite on macOS
- [ ] Run full test suite on Windows
- [ ] Test path handling on all platforms
- [ ] Test daemon operations on all platforms
- [ ] Test VCS operations on all platforms
- [ ] Test TUI mode on all platforms
- [ ] Test privacy mode on all platforms
- [ ] Test health checks on all platforms
- [ ] Fix platform-specific bugs found during validation
- [ ] Document platform-specific behavior
- [ ] Update cross-platform test harness based on findings
- [ ] Add platform-specific workarounds if needed
- [ ] Validate CI matrix for all platforms

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `tests/platform_validation_test.rs` — New test file for cross-platform validation
- `tests/platform/` — Update platform-specific utilities
- `.github/workflows/ci.yml` — Update CI matrix
- `README.md` — Document platform-specific behavior
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [ ] All tests pass on Linux, macOS, and Windows
- [ ] Path handling consistent across platforms
- [ ] Daemon operations work on all platforms
- [ ] VCS operations work on all platforms
- [ ] TUI mode works on all platforms
- [ ] Privacy mode works on all platforms
- [ ] Health checks work on all platforms
- [ ] Platform-specific behavior documented
- [ ] CI matrix validated

## Test Plan

- Integration: Run tests on all three platforms
- Integration: Manual testing of GUI features
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Track test results per platform
- Log platform-specific issues

## Compliance

- Follow ADR #24 (Cross-Platform Path Handling)
- Ensure consistent behavior across platforms

## Risks & Mitigations

- Risk: Windows testing may be difficult — Mitigation: Use GitHub Actions Windows runners, test frequently
- Risk: Platform-specific bugs may be hard to fix — Mitigation: Use cross-platform libraries, minimize platform-specific code

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-01-002-cross-platform-test-harness]], [[tasks-requirements-gaps-04-003-secret-handling-tests]]
- Unblocks: None

## Definition of Done

- Cross-platform validation complete
- All tests pass on all platforms
- Platform-specific behavior documented
- CI matrix validated
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `test(platform): add cross-platform validation`

## Changelog

- 2026-06-11: initialized story file
