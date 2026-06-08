---
story_id: "01-002"
story_title: "Config Initialization & System Config"
story_name: "config-initialization"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 1
parallel_id: 2
branch: "feature/current/cli-standards-compliance/story-01-002-config-initialization"
status: "in_progress"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["config.rs"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "config"]
due: "2026-06-21"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement config file initialization on first run and add system config support as specified in ADR #2 and ADR #3. Create default config files with commented settings, add system config locations, and implement the daemon_fallback_quiet config variable.

## Sub-Tasks

- [x] Add system config path detection for Linux (/etc/smartfo/config.toml)
- [x] Add system config path detection for macOS (/Library/Application Support/smartfo/config.toml)
- [x] Add system config path detection for Windows (C:\ProgramData\smartfo\config.toml)
- [x] Update config precedence chain to include system config: CLI args > env vars > project config > user config > system config > defaults
- [x] Implement config file existence check on first run
- [x] Create default config file template with all settings commented out
- [x] Include default values and explanations for each option in template comments
- [x] Implement config file creation logic for missing configs
- [x] Add daemon_fallback_quiet config option to config schema
- [x] Implement --init-config flag to explicitly create/recreate default config
- [x] Add unit tests for config precedence with system config
- [x] Add unit tests for config file initialization
- [x] Add unit tests for --init-config flag
- [x] Add unit tests for daemon_fallback_quiet config option

## Relevant Files

- `src/config.rs` — Add system config paths, initialization logic, daemon_fallback_quiet option
- `src/main.rs` — Add --init-config flag handling
- `tests/config_tests.rs` — Add tests for config initialization and precedence

## Acceptance Criteria

- [x] System config is loaded from platform-specific location
- [x] Config precedence includes system config in correct position
- [x] Default config file is created on first run with all settings commented out
- [x] Default values and explanations are included in comments
- [x] --init-config flag creates/recreates default config
- [x] daemon_fallback_quiet config option is available
- [x] All tests pass

## Test Plan

- Unit: `cargo test config_tests::system_config_loading`
- Unit: `cargo test config_tests::config_initialization`
- Unit: `cargo test config_tests::init_config_flag`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log config file creation events (info level)
- Log which config files are loaded and from where (debug level)

## Compliance

- Follows ADR #2: Configuration Precedence
- Follows ADR #3: Config File Initialization

## Risks & Mitigations

- Risk: System config directory may not exist — Mitigation: Create directory if it doesn't exist, or fail gracefully with clear error message
- Risk: User may not have write permissions for system config — Mitigation: Fall back to user config with warning message

## Dependencies

None

## Notes

- Use XDG conventions where applicable
- Ensure config template is comprehensive and easy to understand
- Consider adding examples in config comments