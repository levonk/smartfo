---
story_id: "01-003"
story_title: "Install/Uninstall Enhancement"
story_name: "install-uninstall"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 1
parallel_id: 3
branch: "feature/current/cli-standards-compliance/story-01-003-install-uninstall"
status: "todo"
assignee: ""
reviewer: ""
dependencies: []
parallel_safe: true
modules: ["main.rs", "install.rs (new)"]
priority: "MUST"
risk_level: "medium"
tags: ["feat", "install"]
due: "2026-06-21"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Enhance existing --install flag to generate shell completion scripts, initialize default config files, set up environment variables, and install man pages. Add --uninstall counterpart for cleanup with optional --force flag.

## Sub-Tasks

- [ ] Create new install.rs module for install/uninstall logic
- [ ] Implement shell completion script generation for bash using clap
- [ ] Implement shell completion script generation for zsh using clap
- [ ] Implement shell completion script generation for fish using clap
- [ ] Add completion installation to appropriate directories (bash_completion, zsh/functions, fish/completions)
- [ ] Integrate default config file initialization with --install
- [ ] Add environment variable setup instructions to --install output
- [ ] Implement man page installation to system man directory
- [ ] Add --uninstall flag to clap parser
- [ ] Implement symlink removal (mv, rm, smv, srm) for --uninstall
- [ ] Implement shell completion script removal for --uninstall
- [ ] Implement man page removal for --uninstall
- [ ] Add optional config file removal with confirmation prompt for --uninstall
- [ ] Add --force flag to bypass confirmation prompts during --uninstall
- [ ] Add unit tests for --install completion generation
- [ ] Add unit tests for --install config initialization
- [ ] Add unit tests for --uninstall cleanup
- [ ] Add unit tests for --force flag behavior

## Relevant Files

- `src/install.rs` — New module for install/uninstall logic
- `src/main.rs` — Add --uninstall flag, integrate install.rs
- `src/cli.rs` — Add --uninstall flag to smartfo mode parser
- `tests/install_tests.rs` — Add tests for install/uninstall functionality

## Acceptance Criteria

- [ ] --install generates bash, zsh, and fish completion scripts
- [ ] --install initializes default config files
- [ ] --install displays environment variable setup instructions
- [ ] --install installs man pages to system man directory
- [ ] --uninstall removes symlinks (mv, rm, smv, srm)
- [ ] --uninstall removes shell completion scripts
- [ ] --uninstall removes man pages
- [ ] --uninstall prompts for config file removal
- [ ] --uninstall --force bypasses confirmation prompts
- [ ] All tests pass

## Test Plan

- Unit: `cargo test install_tests::completion_generation`
- Unit: `cargo test install_tests::config_initialization`
- Unit: `cargo test install_tests::uninstall_cleanup`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log install/uninstall operations (info level)
- Log which files were created/removed (debug level)

## Compliance

- Follows ADR #4: Install/Uninstall Flag

## Risks & Mitigations

- Risk: Install may require root permissions for system directories — Mitigation: Detect permission errors and provide clear instructions for manual installation
- Risk: Uninstall may remove files shared with other tools — Mitigation: Only remove files created by smartfo install, verify file ownership before removal

## Dependencies

None

## Notes

- Use clap's built-in completion generation
- Detect shell type from environment for completion installation
- Provide clear error messages when installation fails due to permissions