---
story_id: "08-004"
story_title: "Release Preparation"
story_name: "release-preparation"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 8
parallel_id: 4
branch: "feature/current/cli-standards-compliance/story-08-004-release-preparation"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["08-001", "08-002", "08-003"]
parallel_safe: false
modules: ["Cargo.toml", "CHANGELOG.md"]
priority: "MUST"
risk_level: "low"
tags: ["release", "prep"]
due: "2026-09-27"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Prepare for release of CLI Standards Compliance features. Update Cargo.toml with new version, update CHANGELOG.md with all changes, perform final code review, ensure all tests pass, verify documentation is complete, create release notes, tag release, and prepare release artifacts.

## Sub-Tasks

- [ ] Update Cargo.toml with new version number
- [ ] Update version in all man pages
- [ ] Update version in README.md
- [ ] Update CHANGELOG.md with all CLI Standards Compliance changes
- [ ] Add release notes for new features
- [ ] Add migration notes for config changes
- [ ] Add breaking changes documentation (if any)
- [ ] Perform final code review of all changes
- [ ] Ensure all unit tests pass
- [ ] Ensure all integration tests pass
- [ ] Ensure all cross-platform tests pass
- [ ] Verify code coverage meets requirements
- [ ] Run final lint check (cargo clippy)
- [ ] Run final type check (cargo check)
- [ ] Verify documentation is complete and accurate
- [ ] Verify all examples in documentation work
- [ ] Verify shell completion scripts work
- [ ] Verify man pages install correctly
- [ ] Verify TUI mode works correctly
- [ ] Verify daemon operations work correctly
- [ ] Perform final security audit
- [ ] Check for any sensitive data in code or logs
- [ ] Verify all dependencies are up-to-date
- [ ] Verify no deprecated dependencies are used
- [ ] Create release branch
- [ ] Tag release with version number
- [ ] Create GitHub release with release notes
- [ ] Build release binaries for Linux
- [ ] Build release binaries for macOS
- [ ] Build release binaries for Windows
- [ ] Upload release artifacts to GitHub
- [ ] Update internal documentation with release information
- [ ] Notify stakeholders of release

## Relevant Files

- `Cargo.toml` — Update version number
- `CHANGELOG.md` — Document all changes
- `README.md` — Update version and features
- `docs/smartfo.1` — Update version
- `docs/smartfo-mv.1` — Update version
- `docs/smartfo-rm.1` — Update version
- `RELEASE_NOTES.md` — Create release notes
- `.github/workflows/release.yml` — Set up release workflow

## Acceptance Criteria

- [ ] Cargo.toml version is updated
- [ ] Man pages have correct version
- [ ] README.md has correct version
- [ ] CHANGELOG.md documents all changes
- [ ] Release notes are comprehensive
- [ ] Migration notes are included
- [ ] Breaking changes are documented (if any)
- [ ] Code review is complete
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All cross-platform tests pass
- [ ] Code coverage meets requirements
- [ ] Lint check passes
- [ ] Type check passes
- [ ] Documentation is complete and accurate
- [ ] All examples work
- [ ] Shell completion scripts work
- [ ] Man pages install correctly
-- [ ] TUI mode works correctly
- [ ] Daemon operations work correctly
- [ ] Security audit is complete
- [ ] No sensitive data in code or logs
- [ ] Dependencies are up-to-date
- [ ] No deprecated dependencies
- [ ] Release branch is created
- [ ] Release is tagged
- [ ] GitHub release is created with notes
- [ ] Release binaries are built for all platforms
- [ ] Release artifacts are uploaded
- [ ] Internal documentation is updated
- [ ] Stakeholders are notified

## Test Plan

- Unit: `cargo test` (all unit tests pass)
- Integration: `cargo test --test integration` (all integration tests pass)
- Cross-platform: Verify CI/CD pipelines pass on all platforms
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`
- Build: `cargo build --release` (all platforms)
- Manual: Verify all features work as documented
- Manual: Verify installation process works
- Manual: Verify upgrade process works

## Observability

- Log release preparation steps
- Log test results for final verification

## Compliance

- Ensures all ADR standards (ADR #1-#34) are ready for release

## Risks & Mitigations

- Risk: Breaking changes may affect existing users — Mitigation: Provide clear migration guide and support
- Risk: Release may have critical bugs — Mitigation: Comprehensive testing and beta release if needed

## Dependencies

- 08-001 (Integration Testing) — ensures all tests pass
- 08-002 (Documentation Completion) — ensures documentation is complete
- 08-003 (Cross-Platform Testing) — ensures cross-platform compatibility

## Notes

- Use semantic versioning for version number
- Consider creating a beta release for testing
- Prepare rollback plan in case of critical issues
- Document release process for future releases
- Ensure release notes highlight CLI Standards Compliance features