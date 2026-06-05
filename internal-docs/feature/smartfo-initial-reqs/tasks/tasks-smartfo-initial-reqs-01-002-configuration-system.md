---
story_id: "01-002"
story_title: "Configuration system with env var expansion"
story_name: "configuration-system"
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
phase: 1
parallel_id: 2
branch: "feature/current/smartfo-initial-reqs/story-01-002-configuration-system"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["config.rs"]
priority: "MUST"
risk_level: "low"
tags: ["feat", "config"]
due: ""
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

## Summary

Implement the TOML-based configuration system with POSIX-style environment variable expansion, config schema validation, and the full precedence hierarchy (CLI flags > env vars > config file > defaults).

## Sub-Tasks

- [ ] Define config structs for all sections: `vcs`, `trash`, `concurrency`, `behavior`, `logging`, `paths`
- [ ] Implement TOML file loader with `$VAR` and `${VAR}` expansion
- [ ] Implement env var override resolver (`SMARTFO_<SECTION>_<KEY>`)
- [ ] Define and document built-in defaults for every config key
- [ ] Create default config file template
- [ ] Write unit tests for config loading, env expansion, and precedence

## Relevant Files

- `src/config.rs` — Config structs, loader, and env expansion
- `src/config.test.rs` — Unit tests for config loading

## Acceptance Criteria

- [ ] Config file `$HOME/smartfo/config.toml` is parsed correctly
- [ ] `$XDG_DATA_HOME`, `$XDG_CACHE_HOME`, `$XDG_CONFIG_HOME`, `$HOME` are expanded
- [ ] Env var `SMARTFO_BEHAVIOR_DEFAULT_BLOCKING=true` overrides config file
- [ ] CLI flag `--blocking` overrides env var and config file
- [ ] Missing config file falls back to built-in defaults
- [ ] Invalid config values produce clear error messages

## Test Plan

- Unit: `cargo test config::`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log loaded config path and effective overrides at `debug` level

## Compliance

- None

## Risks & Mitigations

- Risk: Circular dependencies between config and logging — Mitigation: Keep logging config simple enough to bootstrap before full logger init

## Dependencies & Sequencing

- Depends on: None
- Unblocks: 02-001, 03-001, 03-002, 03-003, 04-001

## Definition of Done

- Code, tests, and docs updated; CI green; story file updated

## Commit Conventions

- Use conventional commits with module scoping, e.g., `feat(config): add TOML loader with env expansion`

## Changelog

- 2026-06-05: initialized story file
