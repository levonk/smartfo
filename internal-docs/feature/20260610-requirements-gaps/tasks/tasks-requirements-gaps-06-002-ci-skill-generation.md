---
story_id: "06-002"
story_title: "CI skill generation integration"
story_name: "ci-skill-generation"
prd_name: "requirements-gaps"
prd_file: "internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md"
phase: 6
parallel_id: 2
branch: "feature/current/requirements-gaps/story-06-002-ci-skill-generation"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["05-003"]
parallel_safe: true
modules: ["ci", "skill"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "ci", "skill"]
due: "2026-08-05"
created_at: "2026-06-11"
updated_at: "2026-06-11"
---

## Summary

Add CI integration for skill generation with `--check-skill` build step that fails if committed skill is stale. Integrate skill generation into CI/CD pipeline for automatic skill updates. Addresses AXI Requirement #9.

## Sub-Tasks

- [ ] Add `--check-skill` command to CLI
- [ ] Implement skill staleness detection
- [ ] Compare generated skill with committed SKILL.md
- [ ] Add skill generation to CI workflow
- [ ] Add skill check to pre-commit hook
- [ ] Add skill check to CI pipeline
- [ ] Implement automatic skill regeneration in CI
- [ ] Add skill version tracking
- [ ] Write unit tests for skill check
- [ ] Write integration tests for CI integration
- [ ] Update CI workflow file
- [ ] Document skill generation in CI
- [ ] Add skill regeneration to release process

Status conventions: mark in-progress with `[~]`, done with `[x]`, blocked with `[!]`.

## Relevant Files

- `src/skill.rs` — Add --check-skill command
- `src/cli.rs` — Add skill check command
- `.github/workflows/ci.yml` — Add skill check to CI
- `.pre-commit-config.yaml` — Add skill check to pre-commit
- `tests/skill_ci_test.rs` — New test file for CI integration
- `README.md` — Document skill generation in CI
- `internal-docs/feature/20260610-requirements-gaps/20260610-requirements-gaps.md` — Reference requirements

## Acceptance Criteria

- [ ] --check-skill command detects stale skills
- [ ] CI fails if skill is stale
- [ ] Pre-commit hook checks skill
- [ ] Automatic skill regeneration works in CI
- [ ] Skill version tracking functional
- [ ] All tests pass
- [ ] Documentation complete

## Test Plan

- Unit: `devbox run cargo test skill_ci_test`
- Integration: Test CI workflow with stale skill
- Lint: `devbox run cargo clippy -- -D warnings`
- Format: `devbox run cargo fmt`

## Observability

- Log skill check results
- Track skill regeneration events

## Compliance

- Follow AXI Requirement #9 (Installable Agent Skill)
- Ensure skill generation is deterministic

## Risks & Mitigations

- Risk: Skill check may be flaky — Mitigation: Use stable comparison, ignore formatting differences
- Risk: Skill regeneration may break CI — Mitigation: Test thoroughly, add rollback option

## Dependencies & Sequencing

- Depends on: [[tasks-requirements-gaps-05-003-contextual-disclosure-tests]]
- Unblocks: None

## Definition of Done

- CI skill generation integrated
- --check-skill command working
- CI workflow updated
- Documentation complete
- All tests pass
- Story file updated with completion status

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(ci): add skill generation integration`

## Changelog

- 2026-06-11: initialized story file
